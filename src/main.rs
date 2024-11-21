use clap::Parser;
use icalls::properties::Property;
use icalls::OpenFiles;
use lsp_server::ErrorCode;
use lsp_server::Message;
use lsp_server::Notification;
use lsp_server::Request;
use lsp_server::RequestId;
use lsp_server::Response;
use lsp_server::{Connection, IoThreads};
use lsp_types::notification::LogMessage;
use lsp_types::notification::Notification as _;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::notification::ShowMessage;
use lsp_types::request::Request as _;
use lsp_types::CompletionItem;
use lsp_types::CompletionItemKind;
use lsp_types::CompletionList;
use lsp_types::Diagnostic;
use lsp_types::DiagnosticSeverity;
use lsp_types::InitializeParams;
use lsp_types::InitializeResult;
use lsp_types::MarkupContent;
use lsp_types::PositionEncodingKind;
use lsp_types::PublishDiagnosticsParams;
use lsp_types::ServerCapabilities;
use lsp_types::ServerInfo;
use lsp_types::TextDocumentSyncKind;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Parser)]
struct Args {
    #[clap(long)]
    stdio: bool,
}

fn log(c: &Connection, message: impl Serialize) {
    c.sender
        .send(Message::Notification(Notification::new(
            LogMessage::METHOD.to_string(),
            message,
        )))
        .unwrap();
}

fn notify(c: &Connection, method: &str, params: impl Serialize) {
    c.sender
        .send(Message::Notification(Notification::new(
            method.to_owned(),
            params,
        )))
        .unwrap();
}

fn response_empty(id: RequestId) -> Message {
    Message::Response(Response {
        id,
        result: None,
        error: None,
    })
}

fn response_ok(id: RequestId, result: impl Serialize) -> Message {
    Message::Response(Response::new_ok(id, result))
}

fn response_err(id: RequestId, code: i32, message: String) -> Message {
    Message::Response(Response::new_err(id, code, message))
}

fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        hover_provider: Some(lsp_types::HoverProviderCapability::Simple(true)),
        completion_provider: Some(lsp_types::CompletionOptions {
            resolve_provider: Some(true),
            ..Default::default()
        }),
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Options(
            lsp_types::TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                ..Default::default()
            },
        )),
        ..Default::default()
    }
}

fn connect(stdio: bool) -> (lsp_types::InitializeParams, Connection, IoThreads) {
    let (connection, io) = if stdio {
        Connection::stdio()
    } else {
        panic!("No connection mode given, e.g. --stdio");
    };
    let (id, params) = connection.initialize_start().unwrap();
    let mut caps = server_capabilities();
    let init_params = serde_json::from_value::<InitializeParams>(params).unwrap();
    if let Some(general) = &init_params.capabilities.general {
        let pe = general
            .position_encodings
            .clone()
            .unwrap_or_default()
            .iter()
            .find(|&pe| *pe == PositionEncodingKind::UTF8)
            .cloned()
            .unwrap_or(PositionEncodingKind::UTF16);
        caps.position_encoding = Some(pe);
    }
    let init_opts = if let Some(io) = &init_params.initialization_options {
        match serde_json::from_value::<InitializationOptions>(io.clone()) {
            Ok(v) => v,
            Err(err) => {
                notify(
                    &connection,
                    ShowMessage::METHOD,
                    format!("Invalid initialization options: {err}"),
                );
                panic!("Invalid initialization options: {err}")
            }
        }
    } else {
        notify(
            &connection,
            ShowMessage::METHOD,
            "No initialization options given, need it for vcard directory location at least",
        );
        panic!("No initialization options given, need it for vcard directory location at least")
    };
    if !init_opts.enable_completion.unwrap_or(true) {
        caps.completion_provider = None;
    }
    if !init_opts.enable_hover.unwrap_or(true) {
        caps.hover_provider = None;
    }
    if !init_opts.enable_code_actions.unwrap_or(true) {
        caps.code_action_provider = None;
        caps.execute_command_provider = None;
    }
    let init_result = InitializeResult {
        capabilities: caps,
        server_info: Some(ServerInfo {
            name: "icalls".to_owned(),
            version: None,
        }),
    };
    connection
        .initialize_finish(id, serde_json::to_value(init_result).unwrap())
        .unwrap();
    // log(&c, format!("{:?}", params.initialization_options));
    (init_params, connection, io)
}

struct Server {
    open_files: OpenFiles,
    diagnostics: Vec<Diagnostic>,
    shutdown: bool,
}

#[derive(Serialize, Deserialize)]
struct InitializationOptions {
    enable_completion: Option<bool>,
    enable_hover: Option<bool>,
    enable_code_actions: Option<bool>,
}

impl Server {
    fn new(c: &Connection, params: lsp_types::InitializeParams) -> Self {
        let init_opts = if let Some(io) = params.initialization_options {
            match serde_json::from_value::<InitializationOptions>(io) {
                Ok(v) => v,
                Err(err) => {
                    notify(
                        c,
                        ShowMessage::METHOD,
                        format!("Invalid initialization options: {err}"),
                    );
                    panic!("Invalid initialization options: {err}")
                }
            }
        } else {
            notify(
                c,
                ShowMessage::METHOD,
                "No initialization options given, need it for vcard directory location at least",
            );
            panic!("No initialization options given, need it for vcard directory location at least")
        };

        Self {
            open_files: OpenFiles::default(),
            diagnostics: Vec::new(),
            shutdown: false,
        }
    }

    fn serve(mut self, c: Connection) -> Result<(), String> {
        loop {
            match c.receiver.recv().unwrap() {
                Message::Request(r) => {
                    // log(&c, format!("Got request {r:?}"));
                    if self.shutdown {
                        c.sender
                            .send(response_err(
                                r.id,
                                ErrorCode::InvalidRequest as i32,
                                String::from("received request after shutdown"),
                            ))
                            .unwrap();
                        continue;
                    }

                    let messages = match &r.method[..] {
                        lsp_types::request::HoverRequest::METHOD => self.handle_hover_request(r),
                        lsp_types::request::Completion::METHOD => self.handle_completion_request(r),
                        lsp_types::request::ResolveCompletionItem::METHOD => {
                            self.handle_resolve_completion_item_request(r)
                        }
                        lsp_types::request::CodeActionRequest::METHOD => {
                            self.handle_code_action_request(r)
                        }
                        lsp_types::request::ExecuteCommand::METHOD => {
                            self.handle_execute_command_request(r)
                        }
                        lsp_types::request::Shutdown::METHOD => {
                            self.shutdown = true;
                            vec![response_empty(r.id)]
                        }
                        _ => {
                            log(&c, format!("Unmatched request received: {}", r.method));
                            vec![]
                        }
                    };
                    for message in messages {
                        c.sender.send(message).unwrap();
                    }
                }
                Message::Response(r) => log(&c, format!("Unmatched response received: {}", r.id)),
                Message::Notification(n) => {
                    let messages = match &n.method[..] {
                        lsp_types::notification::DidOpenTextDocument::METHOD => {
                            self.handle_did_open_text_document_notification(n)
                        }
                        lsp_types::notification::DidChangeTextDocument::METHOD => {
                            self.handle_did_change_text_document_notification(n)
                        }
                        lsp_types::notification::DidCloseTextDocument::METHOD => {
                            self.handle_did_close_text_document_notification(n)
                        }
                        lsp_types::notification::Exit::METHOD => {
                            if self.shutdown {
                                return Ok(());
                            } else {
                                return Err(String::from(
                                    "Received exit notification before shutdown request",
                                ));
                            }
                        }
                        _ => {
                            log(&c, format!("Unmatched notification received: {}", n.method));
                            Vec::new()
                        }
                    };
                    for message in messages {
                        c.sender.send(message).unwrap()
                    }
                }
            }
        }
    }

    fn handle_hover_request(&mut self, request: Request) -> Vec<Message> {
        let tdp = serde_json::from_value::<lsp_types::TextDocumentPositionParams>(request.params)
            .unwrap();

        let word = self.get_word_from_document(&tdp);
        let response = if let Some(word) = word {
            let lower_word = word.to_lowercase();
            if let Some(property) = icalls::properties::properties()
                .into_iter()
                .find(|p| p.keywords().into_iter().any(|kw| *kw == lower_word))
            {
                let text = render_property(property);
                let resp = lsp_types::Hover {
                    contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                        kind: lsp_types::MarkupKind::Markdown,
                        value: text,
                    }),
                    range: None,
                };
                response_ok(request.id, resp)
            } else {
                response_empty(request.id)
            }
        } else {
            response_empty(request.id)
        };

        vec![response]
    }

    fn handle_completion_request(&mut self, request: Request) -> Vec<Message> {
        let mut tdp =
            serde_json::from_value::<lsp_types::TextDocumentPositionParams>(request.params)
                .unwrap();

        tdp.position.character = tdp.position.character.saturating_sub(1);
        let response = match self.get_word_from_document(&tdp) {
            Some(word) => {
                let lower_word = word.to_lowercase();
                let limit = 100;
                let completion_items: Vec<_> = icalls::properties::properties()
                    .into_iter()
                    .filter(|p| p.keywords().iter().any(|kw| kw.contains(&lower_word)))
                    .map(|p| CompletionItem {
                        label: p.name().to_owned(),
                        kind: Some(CompletionItemKind::TEXT),
                        ..Default::default()
                    })
                    .collect();
                let resp = lsp_types::CompletionResponse::List(CompletionList {
                    is_incomplete: completion_items.len() == limit,
                    items: completion_items,
                });
                response_ok(request.id, resp)
            }
            None => response_empty(request.id),
        };

        vec![response]
    }

    fn handle_resolve_completion_item_request(&mut self, request: Request) -> Vec<Message> {
        let mut ci = serde_json::from_value::<lsp_types::CompletionItem>(request.params).unwrap();

        ci.documentation = Some(lsp_types::Documentation::MarkupContent(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: icalls::properties::properties()
                .into_iter()
                .find(|p| p.name() == ci.label)
                .map(render_property)
                .unwrap_or_default(),
        }));
        let response = response_ok(request.id, ci);

        vec![response]
    }

    fn handle_code_action_request(&mut self, request: Request) -> Vec<Message> {
        let cap = serde_json::from_value::<lsp_types::CodeActionParams>(request.params).unwrap();

        // let action_list = Vec::new();
        // let response = response_ok(request.id, action_list);

        vec![]
    }

    fn handle_execute_command_request(&mut self, request: Request) -> Vec<Message> {
        let cap =
            serde_json::from_value::<lsp_types::ExecuteCommandParams>(request.params).unwrap();

        let mut messages = Vec::new();
        let response = match cap.command.as_str() {
            _ => response_err(
                request.id,
                ErrorCode::InvalidRequest as i32,
                String::from("unknown command"),
            ),
        };
        messages.push(response);

        messages
    }

    fn handle_did_open_text_document_notification(
        &mut self,
        notification: Notification,
    ) -> Vec<Message> {
        let dotdp =
            serde_json::from_value::<lsp_types::DidOpenTextDocumentParams>(notification.params)
                .unwrap();
        self.open_files.add(
            dotdp.text_document.uri.to_string(),
            dotdp.text_document.text,
        );
        let diagnostics = self.refresh_diagnostics(dotdp.text_document.uri.as_ref());
        let message = Message::Notification(Notification::new(
            PublishDiagnostics::METHOD.to_owned(),
            PublishDiagnosticsParams {
                uri: dotdp.text_document.uri,
                diagnostics,
                version: Some(dotdp.text_document.version),
            },
        ));
        vec![message]
        // log(
        //     &c,
        //     format!(
        //         "got open document notification for {:?}",
        //         dotdp.text_document.uri
        //     ),
        // );
    }

    fn handle_did_change_text_document_notification(
        &mut self,
        notification: Notification,
    ) -> Vec<Message> {
        let dctdp =
            serde_json::from_value::<lsp_types::DidChangeTextDocumentParams>(notification.params)
                .unwrap();
        let doc = dctdp.text_document.uri.to_string();
        self.open_files.apply_changes(&doc, dctdp.content_changes);
        let diagnostics = self.refresh_diagnostics(dctdp.text_document.uri.as_ref());
        let message = Message::Notification(Notification::new(
            PublishDiagnostics::METHOD.to_owned(),
            PublishDiagnosticsParams {
                uri: dctdp.text_document.uri,
                diagnostics,
                version: Some(dctdp.text_document.version),
            },
        ));
        vec![message]
        // log(&c, format!("got change document notification for {doc:?}"))
    }

    fn handle_did_close_text_document_notification(
        &mut self,
        notification: Notification,
    ) -> Vec<Message> {
        let dctdp =
            serde_json::from_value::<lsp_types::DidCloseTextDocumentParams>(notification.params)
                .unwrap();
        self.open_files.remove(dctdp.text_document.uri.as_ref());
        Vec::new()
        // log(
        //     &c,
        //     format!(
        //         "got close document notification for {:?}",
        //         dctdp.text_document.uri
        //     ),
        // );
    }

    fn get_word_from_document(
        &mut self,
        tdp: &lsp_types::TextDocumentPositionParams,
    ) -> Option<String> {
        let content = self.open_files.get(tdp.text_document.uri.as_ref());
        get_word_from_content(
            content,
            tdp.position.line as usize,
            tdp.position.character as usize,
        )
    }

    fn refresh_diagnostics(&mut self, file: &str) -> Vec<Diagnostic> {
        let content = self.open_files.get(file);
        let mut diagnostics = Vec::new();
        for (lineno, line) in content.lines().enumerate() {
            if line.starts_with(" ") {
                continue;
            }
            let property_end = line.find(":").unwrap_or(line.len());
            let property_part = &line[..property_end];
            let property_end = property_part.find(";").unwrap_or(property_part.len());
            let property = &property_part[..property_end];
            if !icalls::properties::properties()
                .iter()
                .any(|p| p.name() == property)
            {
                // unknown property
                diagnostics.push(Diagnostic {
                    range: lsp_types::Range {
                        start: lsp_types::Position {
                            line: lineno as u32,
                            character: 0,
                        },
                        end: lsp_types::Position {
                            line: lineno as u32,
                            character: property_end as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: format!("Unknown property {:?}", property),
                    ..Default::default()
                });
            }
        }
        diagnostics
    }
}

fn get_word_from_content(content: &str, line: usize, character: usize) -> Option<String> {
    let line = content.lines().nth(line)?;
    let word = get_word_from_line(line, character)?;
    Some(word)
}

const EMAIL_PUNC: &str = "._%+-@";

fn get_word_from_line(line: &str, character: usize) -> Option<String> {
    let mut current_word = String::new();
    let mut found = false;
    let mut match_chars = EMAIL_PUNC.to_owned();
    let word_char = |match_with: &str, c: char| c.is_alphanumeric() || match_with.contains(c);
    for (i, c) in line.chars().enumerate() {
        if word_char(&match_chars, c) {
            current_word.push(c);
        } else {
            if found {
                return Some(current_word);
            }
            current_word.clear();
        }

        if i == character {
            if word_char(&match_chars, c) {
                match_chars.push(' ');
                found = true
            } else {
                return None;
            }
        }

        if !word_char(&match_chars, c) && found {
            return Some(current_word);
        }
    }

    // got to end of line
    if found {
        return Some(current_word);
    }

    None
}

fn main() {
    let args = Args::parse();
    let (p, c, io) = connect(args.stdio);
    let server = Server::new(&c, p);
    let s = server.serve(c);
    io.join().unwrap();
    match s {
        Ok(()) => (),
        Err(s) => {
            eprintln!("{}", s);
            std::process::exit(1)
        }
    }
}

fn render_property(property: &dyn Property) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# {}", property.name()));
    lines.push(format!("_{}_", property.value_type()));
    lines.push(property.purpose().to_owned());
    if !property.examples().is_empty() {
        let mut examples = Vec::new();
        examples.push("## Examples\n".to_owned());
        for example in property.examples() {
            examples.push(format!("- {}", example));
        }
        lines.push(examples.join("\n"));
    }
    lines.join("\n\n")
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use icalls::properties::Summary;

    use super::*;

    #[test]
    fn hover_render() {
        expect![].assert_eq(&render_property(&Summary));
    }
}
