use std::str::FromStr;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::line_ending;
use nom::combinator::{opt, peek};
use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

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

#[derive(Debug)]
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
    pub value: Span<'a>,
}

#[derive(Debug)]
pub struct Property<'a> {
    pub name_raw: Span<'a>,
    pub name: Option<PropertyName>,
    pub params: Vec<Parameter<'a>>,
    pub value: Span<'a>,
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
            let (s, param_name) = take_till(|c| c == '=')(s)?;
            let (s, _) = tag("=")(s)?;
            let (s, param_value) = take_till(|c| c == ';' || c == ':')(s)?;
            sc = s;
            params.push(Parameter {
                name_raw: param_name,
                name: ParameterName::from_str(param_name.fragment()).ok(),
                value: param_value,
            });
        } else {
            break;
        }
    }
    let s = sc;
    let (s, _) = tag(":")(s)?;
    let (s, value) = take_till(|c| c == '\r' || c == '\n')(s)?;
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
                        value: LocatedSpan {
                            offset: 6,
                            line: 1,
                            fragment: "VCALENDAR",
                            extra: (),
                        },
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
                                value: LocatedSpan {
                                    offset: 13,
                                    line: 1,
                                    fragment: "Europe/London",
                                    extra: (),
                                },
                            },
                        ],
                        value: LocatedSpan {
                            offset: 27,
                            line: 1,
                            fragment: "20221008T170000",
                            extra: (),
                        },
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
                            value: LocatedSpan {
                                offset: 6,
                                line: 1,
                                fragment: "VCALENDAR",
                                extra: (),
                            },
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
                                    value: LocatedSpan {
                                        offset: 29,
                                        line: 2,
                                        fragment: "Europe/London",
                                        extra: (),
                                    },
                                },
                            ],
                            value: LocatedSpan {
                                offset: 43,
                                line: 2,
                                fragment: "20221008T170000",
                                extra: (),
                            },
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
                            value: LocatedSpan {
                                offset: 63,
                                line: 3,
                                fragment: "VCALENDAR",
                                extra: (),
                            },
                        },
                    ],
                ),
            )
        "#]]
        .assert_debug_eq(&parse_properties(Span::new(
            "BEGIN:VCALENDAR\nDTSTART;TZID=Europe/London:20221008T170000\nEND:VCALENDAR",
        )));
    }
}
