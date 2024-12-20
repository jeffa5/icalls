use clap::Parser;
use icalls::ast;
use icalls::ast::parse_properties;
use icalls::ast::parse_value;
use icalls::ast::SyntaxKind;
use icalls::parameters::Parameter;
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
use lsp_types::Position;
use lsp_types::PositionEncodingKind;
use lsp_types::PublishDiagnosticsParams;
use lsp_types::Range;
use lsp_types::ServerCapabilities;
use lsp_types::ServerInfo;
use lsp_types::TextDocumentSyncKind;
use nom_locate::LocatedSpan;
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
    shutdown: bool,
}

#[derive(Serialize, Deserialize)]
struct InitializationOptions {
    enable_completion: Option<bool>,
    enable_hover: Option<bool>,
}

impl Server {
    fn new(_c: &Connection, _params: lsp_types::InitializeParams) -> Self {
        Self {
            open_files: OpenFiles::default(),
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

        let content = self.open_files.get(tdp.text_document.uri.as_ref());
        let Ok((_, ast)) = ast::parse_properties(LocatedSpan::new(content)) else {
            return vec![response_empty(request.id)];
        };

        'outer: for property in ast {
            if property.name_raw.location_line() - 1 < tdp.position.line {
                continue;
            }
            if property.name_raw.location_line() - 1 > tdp.position.line {
                break;
            }

            let ns = property.name_raw.get_utf8_column() - 1;
            let nl = property.name_raw.fragment().len();
            if (ns..(ns + nl)).contains(&(tdp.position.character as usize)) {
                if let Some(name) = property.name {
                    let text = render_property(name.to_property());
                    let resp = lsp_types::Hover {
                        contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                            kind: lsp_types::MarkupKind::Markdown,
                            value: text,
                        }),
                        range: None,
                    };
                    return vec![response_ok(request.id, resp)];
                } else {
                    break;
                }
            }

            for param in property.params {
                let ps = param.name_raw.get_utf8_column() - 1;
                let pl = param.name_raw.fragment().len();
                if (ps..(ps + pl)).contains(&(tdp.position.character as usize)) {
                    if let Some(name) = param.name {
                        let text = render_parameter(name.to_parameter());
                        let resp = lsp_types::Hover {
                            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                                kind: lsp_types::MarkupKind::Markdown,
                                value: text,
                            }),
                            range: None,
                        };
                        return vec![response_ok(request.id, resp)];
                    } else {
                        break 'outer;
                    }
                }
            }

            if let Some(value) = property.value {
                let ns = value.get_utf8_column() - 1;
                let nl = value.fragment().len();
                if (ns..(ns + nl)).contains(&(tdp.position.character as usize)) {
                    if let Some(name) = property.name {
                        match parse_value(value, name.to_property().value_type()) {
                            Ok((_, v)) => {
                                return vec![response_ok(
                                    request.id,
                                    lsp_types::Hover {
                                        contents: lsp_types::HoverContents::Markup(
                                            lsp_types::MarkupContent {
                                                kind: lsp_types::MarkupKind::Markdown,
                                                value: v.prettify(),
                                            },
                                        ),
                                        range: None,
                                    },
                                )];
                            }
                            Err(e) => {
                                return vec![response_ok(
                                    request.id,
                                    lsp_types::Hover {
                                        contents: lsp_types::HoverContents::Markup(
                                            lsp_types::MarkupContent {
                                                kind: lsp_types::MarkupKind::Markdown,
                                                value: e.to_string(),
                                            },
                                        ),
                                        range: None,
                                    },
                                )];
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        vec![response_empty(request.id)]
    }

    fn handle_completion_request(&mut self, request: Request) -> Vec<Message> {
        let mut tdp =
            serde_json::from_value::<lsp_types::TextDocumentPositionParams>(request.params)
                .unwrap();

        let limit = 100;

        tdp.position.character = tdp.position.character.saturating_sub(1);

        let content = self.open_files.get(tdp.text_document.uri.as_ref());
        let Ok((_, ast)) = ast::parse_properties(LocatedSpan::new(content)) else {
            return vec![response_empty(request.id)];
        };

        for property in ast {
            if property.name_raw.location_line() - 1 < tdp.position.line {
                continue;
            }
            if property.name_raw.location_line() - 1 > tdp.position.line {
                break;
            }

            let ns = property.name_raw.get_utf8_column() - 1;
            let nl = property.name_raw.fragment().len();
            if (ns..(ns + nl)).contains(&(tdp.position.character as usize)) {
                let lower_word = property
                    .name_raw
                    .fragment()
                    .chars()
                    .take(tdp.position.character as usize - ns)
                    .collect::<String>()
                    .to_lowercase();
                let completion_items: Vec<_> = icalls::properties::properties()
                    .into_iter()
                    .filter(|p| p.keywords().iter().any(|kw| kw.contains(&lower_word)))
                    .map(|p| CompletionItem {
                        label: p.name().to_owned(),
                        kind: Some(CompletionItemKind::TEXT),
                        data: Some(serde_json::to_value(SyntaxKind::Property).unwrap()),
                        ..Default::default()
                    })
                    .collect();
                let resp = lsp_types::CompletionResponse::List(CompletionList {
                    is_incomplete: completion_items.len() == limit,
                    items: completion_items,
                });
                return vec![response_ok(request.id, resp)];
            }

            for param in property.params {
                let ps = param.name_raw.get_utf8_column() - 1;
                let pl = param.name_raw.fragment().len();
                if (ps..(ps + pl)).contains(&(tdp.position.character as usize)) {
                    let lower_word = param
                        .name_raw
                        .fragment()
                        .chars()
                        .take(tdp.position.character as usize - ns)
                        .collect::<String>()
                        .to_lowercase();
                    let completion_items: Vec<_> = icalls::parameters::parameters()
                        .into_iter()
                        .filter(|p| p.keywords().iter().any(|kw| kw.contains(&lower_word)))
                        .map(|p| CompletionItem {
                            label: p.name().to_owned(),
                            kind: Some(CompletionItemKind::TEXT),
                            data: Some(serde_json::to_value(SyntaxKind::Parameter).unwrap()),
                            ..Default::default()
                        })
                        .collect();
                    let resp = lsp_types::CompletionResponse::List(CompletionList {
                        is_incomplete: completion_items.len() == limit,
                        items: completion_items,
                    });
                    return vec![response_ok(request.id, resp)];
                }
            }
        }
        vec![response_empty(request.id)]
    }

    fn handle_resolve_completion_item_request(&mut self, request: Request) -> Vec<Message> {
        let mut ci = serde_json::from_value::<lsp_types::CompletionItem>(request.params).unwrap();

        let value = match ci
            .data
            .as_ref()
            .and_then(|d| serde_json::from_value::<SyntaxKind>(d.clone()).ok())
        {
            Some(v) => match v {
                SyntaxKind::Property => icalls::properties::properties()
                    .into_iter()
                    .find(|p| p.name() == ci.label)
                    .map(render_property)
                    .unwrap_or_default(),
                SyntaxKind::Parameter => icalls::parameters::parameters()
                    .into_iter()
                    .find(|p| p.name() == ci.label)
                    .map(render_parameter)
                    .unwrap_or_default(),
            },
            None => String::new(),
        };

        ci.documentation = Some(lsp_types::Documentation::MarkupContent(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value,
        }));
        let response = response_ok(request.id, ci);

        vec![response]
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

    fn refresh_diagnostics(&mut self, file: &str) -> Vec<Diagnostic> {
        let content = self.open_files.get(file);
        let (_, ast) = match parse_properties(LocatedSpan::new(content)) {
            Ok(ast) => ast,
            Err(nom::Err::Error(err)) => {
                return vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: err.input.location_line(),
                            character: 0,
                        },
                        end: Position {
                            line: err.input.location_line(),
                            character: 0,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    ..Default::default()
                }]
            }
            Err(nom::Err::Failure(err)) => {
                return vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: err.input.location_line(),
                            character: 0,
                        },
                        end: Position {
                            line: err.input.location_line(),
                            character: 0,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    ..Default::default()
                }]
            }
            Err(nom::Err::Incomplete(_)) => {
                unreachable!()
            }
        };
        let mut diagnostics = Vec::new();
        for property in ast {
            if property.name.is_none() {
                let line = property.name_raw.location_line() - 1;
                let character_start = property.name_raw.get_utf8_column() - 1;
                let character_end = character_start + property.name_raw.fragment().len();
                diagnostics.push(Diagnostic {
                    range: lsp_types::Range {
                        start: lsp_types::Position {
                            line,
                            character: character_start as u32,
                        },
                        end: lsp_types::Position {
                            line,
                            character: character_end as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    message: format!("Unknown property {:?}", property.name_raw.fragment()),
                    ..Default::default()
                });
            } else if let Some(value) = property.value {
                let line = value.location_line() - 1;
                let character_start = value.get_utf8_column() - 1;
                let character_end = character_start + value.fragment().len();
                if let Err(e) = property.check_value_type() {
                    diagnostics.push(Diagnostic {
                        range: lsp_types::Range {
                            start: lsp_types::Position {
                                line,
                                character: character_start as u32,
                            },
                            end: lsp_types::Position {
                                line,
                                character: character_end as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: format!(
                            "Failed to match expected type: {:?}\n\n{}",
                            property.name.unwrap().to_property().value_type(),
                            e
                        ),
                        ..Default::default()
                    });
                }
            }

            for parameter in property.params {
                if parameter.name.is_none() {
                    let line = parameter.name_raw.location_line() - 1;
                    let character_start = parameter.name_raw.get_utf8_column() - 1;
                    let character_end = character_start + parameter.name_raw.fragment().len();
                    diagnostics.push(Diagnostic {
                        range: lsp_types::Range {
                            start: lsp_types::Position {
                                line,
                                character: character_start as u32,
                            },
                            end: lsp_types::Position {
                                line,
                                character: character_end as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        message: format!("Unknown parameter {:?}", parameter.name_raw.fragment()),
                        ..Default::default()
                    });
                }
            }
        }

        diagnostics
    }
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
    lines.push(format!("_{:?}_", property.value_type()));
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

fn render_parameter(parameter: &dyn Parameter) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# {}", parameter.name()));
    lines.push(format!("_{:?}_", parameter.value_type()));
    lines.push(parameter.purpose().to_owned());
    if !parameter.examples().is_empty() {
        let mut examples = Vec::new();
        examples.push("## Examples\n".to_owned());
        for example in parameter.examples() {
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
        expect![[r#"
            # SUMMARY

            _Text_

            This property defines a short summary or subject for the calendar component.

            ## Examples

            - SUMMARY:Department Party"#]]
        .assert_eq(&render_property(&Summary));
    }
}
