use std::str::FromStr;

use nom::bytes::complete::{tag, take_till, take_while};
use nom::character::complete::line_ending;
use nom::combinator::{opt, peek};
use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SyntaxKind {
    Property,
    Parameter,
}

#[derive(Debug, strum::EnumIter)]
pub enum PropertyName {
    // Meta properties, probably should be removed once this looks more like the nested structure
    Begin,
    End,
    // Calendar properties
    CalScale,
    Method,
    ProdId,
    Version,
    // Component properties, descriptive
    Attach,
    Categories,
    Class,
    Comment,
    Description,
    Geo,
    Location,
    PercentComplete,
    Priority,
    Resources,
    Status,
    Summary,
    // Component properties, date and time
    Completed,
    DtEnd,
    Due,
    DtStart,
    Duration,
    FreeBusy,
    Transp,
    // Component properties, time zone
    TzId,
    TzName,
    TzOffsetFrom,
    TzOffsetTo,
    TzUrl,
    // Component properties, relationship
    Attendee,
    Contact,
    Organizer,
    RecurrenceId,
    RelatedTo,
    Url,
    Uid,
    // Component properties, recurrence
    ExDate,
    RDate,
    RRule,
    // Component properties, alarm
    Action,
    Repeat,
    Trigger,
    // Component properties, change management
    Created,
    DtStamp,
    LastModified,
    Sequence,
}

impl PropertyName {
    pub fn to_property(&self) -> &'static dyn crate::properties::Property {
        match self {
            PropertyName::Begin => &crate::properties::Begin,
            PropertyName::End => &crate::properties::End,
            PropertyName::CalScale => &crate::properties::CalScale,
            PropertyName::Method => &crate::properties::Method,
            PropertyName::ProdId => &crate::properties::ProdId,
            PropertyName::Version => &crate::properties::Version,
            PropertyName::Attach => &crate::properties::Attach,
            PropertyName::Categories => &crate::properties::Categories,
            PropertyName::Class => &crate::properties::Class,
            PropertyName::Comment => &crate::properties::Comment,
            PropertyName::Description => &crate::properties::Description,
            PropertyName::Geo => &crate::properties::Geo,
            PropertyName::Location => &crate::properties::Location,
            PropertyName::PercentComplete => &crate::properties::PercentComplete,
            PropertyName::Priority => &crate::properties::Priority,
            PropertyName::Resources => &crate::properties::Resources,
            PropertyName::Status => &crate::properties::Status,
            PropertyName::Summary => &crate::properties::Summary,
            PropertyName::Completed => &crate::properties::Completed,
            PropertyName::DtEnd => &crate::properties::DtEnd,
            PropertyName::Due => &crate::properties::Due,
            PropertyName::DtStart => &crate::properties::DtStart,
            PropertyName::Duration => &crate::properties::Duration,
            PropertyName::FreeBusy => &crate::properties::FreeBusy,
            PropertyName::Transp => &crate::properties::Transp,
            PropertyName::TzId => &crate::properties::TzId,
            PropertyName::TzName => &crate::properties::TzName,
            PropertyName::TzOffsetFrom => &crate::properties::TzOffsetFrom,
            PropertyName::TzOffsetTo => &crate::properties::TzOffsetTo,
            PropertyName::TzUrl => &crate::properties::TzUrl,
            PropertyName::Attendee => &crate::properties::Attendee,
            PropertyName::Contact => &crate::properties::Contact,
            PropertyName::Organizer => &crate::properties::Organizer,
            PropertyName::RecurrenceId => &crate::properties::RecurrenceId,
            PropertyName::RelatedTo => &crate::properties::RelatedTo,
            PropertyName::Url => &crate::properties::Url,
            PropertyName::Uid => &crate::properties::Uid,
            PropertyName::ExDate => &crate::properties::ExDate,
            PropertyName::RDate => &crate::properties::RDate,
            PropertyName::RRule => &crate::properties::RRule,
            PropertyName::Action => &crate::properties::Action,
            PropertyName::Repeat => &crate::properties::Repeat,
            PropertyName::Trigger => &crate::properties::Trigger,
            PropertyName::Created => &crate::properties::Created,
            PropertyName::DtStamp => &crate::properties::DtStamp,
            PropertyName::LastModified => &crate::properties::LastModified,
            PropertyName::Sequence => &crate::properties::Sequence,
        }
    }
}

impl FromStr for PropertyName {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "begin" => Ok(Self::Begin),
            "end" => Ok(Self::End),
            "calscale" => Ok(Self::CalScale),
            "method" => Ok(Self::Method),
            "prodid" => Ok(Self::ProdId),
            "version" => Ok(Self::Version),
            "attach" => Ok(Self::Attach),
            "categories" => Ok(Self::Categories),
            "class" => Ok(Self::Class),
            "comment" => Ok(Self::Comment),
            "description" => Ok(Self::Description),
            "geo" => Ok(Self::Geo),
            "location" => Ok(Self::Location),
            "percent-complete" => Ok(Self::PercentComplete),
            "priority" => Ok(Self::Priority),
            "resources" => Ok(Self::Resources),
            "status" => Ok(Self::Status),
            "summary" => Ok(Self::Summary),
            "completed" => Ok(Self::Completed),
            "dtend" => Ok(Self::DtEnd),
            "due" => Ok(Self::Due),
            "dtstart" => Ok(Self::DtStart),
            "duration" => Ok(Self::Duration),
            "freebusy" => Ok(Self::FreeBusy),
            "transp" => Ok(Self::Transp),
            "tzid" => Ok(Self::TzId),
            "tzname" => Ok(Self::TzName),
            "tzoffsetfrom" => Ok(Self::TzOffsetFrom),
            "tzoffsetto" => Ok(Self::TzOffsetTo),
            "tzurl" => Ok(Self::TzUrl),
            "attendee" => Ok(Self::Attendee),
            "contact" => Ok(Self::Contact),
            "organizer" => Ok(Self::Organizer),
            "recurrence-id" => Ok(Self::RecurrenceId),
            "related-to" => Ok(Self::RelatedTo),
            "url" => Ok(Self::Url),
            "uid" => Ok(Self::Uid),
            "exdate" => Ok(Self::ExDate),
            "rdate" => Ok(Self::RDate),
            "rrule" => Ok(Self::RRule),
            "action" => Ok(Self::Action),
            "repeat" => Ok(Self::Repeat),
            "trigger" => Ok(Self::Trigger),
            "created" => Ok(Self::Created),
            "dtstamp" => Ok(Self::DtStamp),
            "last-modified" => Ok(Self::LastModified),
            "sequence" => Ok(Self::Sequence),
            _ => Err(()),
        }
    }
}

#[derive(Debug, strum::EnumIter)]
pub enum ParameterName {
    AltRep,
    CN,
    CUType,
    DelegatedFrom,
    DelegatedTo,
    Dir,
    Encoding,
    FmtType,
    FBType,
    Language,
    Member,
    PartStat,
    Range,
    Related,
    RelType,
    Role,
    RSVP,
    SentBy,
    TZId,
    Value,
}

impl ParameterName {
    pub fn to_parameter(&self) -> &'static dyn crate::parameters::Parameter {
        match self {
            ParameterName::AltRep => &crate::parameters::AltRep,
            ParameterName::CN => &crate::parameters::CN,
            ParameterName::CUType => &crate::parameters::CUType,
            ParameterName::DelegatedFrom => &crate::parameters::DelegatedFrom,
            ParameterName::DelegatedTo => &crate::parameters::DelegatedTo,
            ParameterName::Dir => &crate::parameters::Dir,
            ParameterName::Encoding => &crate::parameters::Encoding,
            ParameterName::FmtType => &crate::parameters::FmtType,
            ParameterName::FBType => &crate::parameters::FBType,
            ParameterName::Language => &crate::parameters::Language,
            ParameterName::Member => &crate::parameters::Member,
            ParameterName::PartStat => &crate::parameters::PartStat,
            ParameterName::Range => &crate::parameters::Range,
            ParameterName::Related => &crate::parameters::Related,
            ParameterName::RelType => &crate::parameters::RelType,
            ParameterName::Role => &crate::parameters::Role,
            ParameterName::RSVP => &crate::parameters::RSVP,
            ParameterName::SentBy => &crate::parameters::SentBy,
            ParameterName::TZId => &crate::parameters::TZId,
            ParameterName::Value => &crate::parameters::Value,
        }
    }
}

impl FromStr for ParameterName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "altrep" => Ok(Self::AltRep),
            "cn" => Ok(Self::CN),
            "cutype" => Ok(Self::CUType),
            "delegated-from" => Ok(Self::DelegatedFrom),
            "delegated-to" => Ok(Self::DelegatedTo),
            "dir" => Ok(Self::Dir),
            "encoding" => Ok(Self::Encoding),
            "fmttype" => Ok(Self::FmtType),
            "fbtype" => Ok(Self::FBType),
            "language" => Ok(Self::Language),
            "member" => Ok(Self::Member),
            "partstat" => Ok(Self::PartStat),
            "range" => Ok(Self::Range),
            "related" => Ok(Self::Related),
            "reltype" => Ok(Self::RelType),
            "role" => Ok(Self::Role),
            "rsvp" => Ok(Self::RSVP),
            "sent-by" => Ok(Self::SentBy),
            "tzid" => Ok(Self::TZId),
            "value" => Ok(Self::Value),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Parameter<'a> {
    pub name_raw: Span<'a>,
    pub name: Option<ParameterName>,
    pub value: Option<Span<'a>>,
}

#[derive(Debug)]
pub struct Property<'a> {
    pub name_raw: Span<'a>,
    pub name: Option<PropertyName>,
    pub params: Vec<Parameter<'a>>,
    pub value: Option<Span<'a>>,
}

impl<'a> Property<'a> {
    pub fn check_value_type(&self) -> Result<(), String> {
        let Some(value) = self.value else {
            return Ok(());
        };
        let Some(name) = &self.name else {
            return Ok(());
        };
        let value_raw = value.fragment();
        match name.to_property().value_type() {
            crate::value::ValueType::Binary => Ok(()),
            crate::value::ValueType::Boolean => {
                if matches!(value_raw.to_lowercase().as_str(), "true" | "false") {
                    Ok(())
                } else {
                    Err("Did not match \"true\" or \"false\"".to_owned())
                }
            }
            crate::value::ValueType::CalAddress => {
                if !value_raw.starts_with("mailto:") {
                    return Err("Does not start with \"mailto:\"".to_owned());
                }
                if !value_raw.trim_start_matches("mailto:").contains("@") {
                    return Err("Does not contain '@'".to_owned());
                }
                Ok(())
            }
            crate::value::ValueType::Date => check_date_type(value_raw),
            crate::value::ValueType::DateTime => {
                let Some((date, time)) = value_raw.split_once('T') else {
                    return Err("Did not contain 'T'".to_owned());
                };
                check_date_type(date)?;
                check_time_type(time)
            }
            crate::value::ValueType::Duration => Ok(()),
            crate::value::ValueType::Float => f64::from_str(value_raw)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            crate::value::ValueType::Integer => i64::from_str(value_raw)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            crate::value::ValueType::PeriodOfTime => Ok(()),
            crate::value::ValueType::RecurrenceRule => Ok(()),
            crate::value::ValueType::Text => Ok(()),
            crate::value::ValueType::Time => check_time_type(value_raw),
            crate::value::ValueType::Uri => Ok(()),
            crate::value::ValueType::UtcOffset => Ok(()),
        }
    }
}

fn check_date_type(s: &str) -> Result<(), String> {
    if s.len() != 8 {
        return Err("Length was not 8".to_owned());
    }
    if !s.chars().all(|c| c.is_numeric()) {
        return Err("Not all characters are numeric".to_owned());
    }
    Ok(())
}

fn check_time_type(s: &str) -> Result<(), String> {
    if !matches!(s.len(), 6 | 7) {
        return Err("Length was not 6 or 7".to_owned());
    }
    if s.len() == 7 && !s.ends_with('Z') {
        return Err("Length was 7 but did not end with 'Z'".to_owned());
    }
    if !s.chars().take(6).all(|c| c.is_numeric()) {
        return Err("Not all of the first 6 characters were numeric".to_owned());
    }
    Ok(())
}

pub fn parse_properties(s: Span) -> IResult<Span, Vec<Property>> {
    let mut sc = s;
    let mut properties = Vec::new();
    while !sc.is_empty() {
        let (s, p) = parse_property(sc)?;
        sc = s;
        properties.push(p);
    }
    Ok((sc, properties))
}

pub fn parse_property(s: Span) -> IResult<Span, Property> {
    let (s, name_raw) = take_till(|c| c == ';' || c == ':')(s)?;
    let name = PropertyName::from_str(name_raw.fragment()).ok();
    let mut params = Vec::new();
    let mut sc = s;
    loop {
        let (s, has_params) = peek(tag::<_, _, ()>(";"))(sc)
            .map(|(s, _)| (s, true))
            .unwrap_or((s, false));
        if has_params {
            // params
            let (s, _) = tag(";")(s)?;
            let (s, parameter) = parse_parameter(s)?;
            sc = s;
            params.push(parameter);
        } else {
            break;
        }
    }
    let s = sc;
    let (s, colon) = opt(tag(":"))(s)?;
    let (s, value) = if colon.is_some() {
        let (s, value) = take_till(|c| c == '\r' || c == '\n')(s)?;
        (s, Some(value))
    } else {
        (s, None)
    };
    let (s, _) = opt(line_ending)(s)?;
    Ok((
        s,
        Property {
            name_raw,
            name,
            params,
            value,
        },
    ))
}

fn parse_parameter(s: Span) -> IResult<Span, Parameter> {
    let (s, param_name) = take_while(|c: char| c.is_alphabetic() || c == '-')(s)?;
    let (s, equals) = opt(tag("="))(s)?;
    let (s, param_value) = if equals.is_some() {
        let (s, value) = take_till(|c| c == ';' || c == ':')(s)?;
        (s, Some(value))
    } else {
        (s, None)
    };
    Ok((
        s,
        Parameter {
            name_raw: param_name,
            name: ParameterName::from_str(param_name.fragment()).ok(),
            value: param_value,
        },
    ))
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn property_basic() {
        expect![[r#"
            Ok(
                (
                    LocatedSpan {
                        offset: 15,
                        line: 1,
                        fragment: "",
                        extra: (),
                    },
                    Property {
                        name_raw: LocatedSpan {
                            offset: 0,
                            line: 1,
                            fragment: "BEGIN",
                            extra: (),
                        },
                        name: Some(
                            Begin,
                        ),
                        params: [],
                        value: Some(
                            LocatedSpan {
                                offset: 6,
                                line: 1,
                                fragment: "VCALENDAR",
                                extra: (),
                            },
                        ),
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&parse_property(Span::new("BEGIN:VCALENDAR")));
    }

    #[test]
    fn property_with_param() {
        expect![[r#"
            Ok(
                (
                    LocatedSpan {
                        offset: 42,
                        line: 1,
                        fragment: "",
                        extra: (),
                    },
                    Property {
                        name_raw: LocatedSpan {
                            offset: 0,
                            line: 1,
                            fragment: "DTSTART",
                            extra: (),
                        },
                        name: Some(
                            DtStart,
                        ),
                        params: [
                            Parameter {
                                name_raw: LocatedSpan {
                                    offset: 8,
                                    line: 1,
                                    fragment: "TZID",
                                    extra: (),
                                },
                                name: Some(
                                    TZId,
                                ),
                                value: Some(
                                    LocatedSpan {
                                        offset: 13,
                                        line: 1,
                                        fragment: "Europe/London",
                                        extra: (),
                                    },
                                ),
                            },
                        ],
                        value: Some(
                            LocatedSpan {
                                offset: 27,
                                line: 1,
                                fragment: "20221008T170000",
                                extra: (),
                            },
                        ),
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&parse_property(Span::new(
            "DTSTART;TZID=Europe/London:20221008T170000",
        )));
    }

    #[test]
    fn properties() {
        expect![[r#"
            Ok(
                (
                    LocatedSpan {
                        offset: 72,
                        line: 3,
                        fragment: "",
                        extra: (),
                    },
                    [
                        Property {
                            name_raw: LocatedSpan {
                                offset: 0,
                                line: 1,
                                fragment: "BEGIN",
                                extra: (),
                            },
                            name: Some(
                                Begin,
                            ),
                            params: [],
                            value: Some(
                                LocatedSpan {
                                    offset: 6,
                                    line: 1,
                                    fragment: "VCALENDAR",
                                    extra: (),
                                },
                            ),
                        },
                        Property {
                            name_raw: LocatedSpan {
                                offset: 16,
                                line: 2,
                                fragment: "DTSTART",
                                extra: (),
                            },
                            name: Some(
                                DtStart,
                            ),
                            params: [
                                Parameter {
                                    name_raw: LocatedSpan {
                                        offset: 24,
                                        line: 2,
                                        fragment: "TZID",
                                        extra: (),
                                    },
                                    name: Some(
                                        TZId,
                                    ),
                                    value: Some(
                                        LocatedSpan {
                                            offset: 29,
                                            line: 2,
                                            fragment: "Europe/London",
                                            extra: (),
                                        },
                                    ),
                                },
                            ],
                            value: Some(
                                LocatedSpan {
                                    offset: 43,
                                    line: 2,
                                    fragment: "20221008T170000",
                                    extra: (),
                                },
                            ),
                        },
                        Property {
                            name_raw: LocatedSpan {
                                offset: 59,
                                line: 3,
                                fragment: "END",
                                extra: (),
                            },
                            name: Some(
                                End,
                            ),
                            params: [],
                            value: Some(
                                LocatedSpan {
                                    offset: 63,
                                    line: 3,
                                    fragment: "VCALENDAR",
                                    extra: (),
                                },
                            ),
                        },
                    ],
                ),
            )
        "#]]
        .assert_debug_eq(&parse_properties(Span::new(
            "BEGIN:VCALENDAR\nDTSTART;TZID=Europe/London:20221008T170000\nEND:VCALENDAR",
        )));
    }

    #[test]
    fn incomplete_parameter() {
        expect![[r#"
            Ok(
                (
                    LocatedSpan {
                        offset: 5,
                        line: 1,
                        fragment: ";",
                        extra: (),
                    },
                    Parameter {
                        name_raw: LocatedSpan {
                            offset: 0,
                            line: 1,
                            fragment: "DTEND",
                            extra: (),
                        },
                        name: None,
                        value: None,
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&parse_parameter(Span::new("DTEND;")));
    }

    #[test]
    fn incomplete_property() {
        expect![[r#"
            Ok(
                (
                    LocatedSpan {
                        offset: 12,
                        line: 1,
                        fragment: "",
                        extra: (),
                    },
                    Property {
                        name_raw: LocatedSpan {
                            offset: 0,
                            line: 1,
                            fragment: "DTEND",
                            extra: (),
                        },
                        name: Some(
                            DtEnd,
                        ),
                        params: [
                            Parameter {
                                name_raw: LocatedSpan {
                                    offset: 6,
                                    line: 1,
                                    fragment: "incomp",
                                    extra: (),
                                },
                                name: None,
                                value: None,
                            },
                        ],
                        value: None,
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&parse_property(Span::new("DTEND;incomp")));
    }
}
