use strum::IntoEnumIterator as _;

use crate::{ast, value::ValueType};

pub trait Parameter {
    fn name(&self) -> &'static str;
    fn purpose(&self) -> &'static str;
    fn value_type(&self) -> ValueType;
    fn description(&self) -> &'static str;
    fn examples(&self) -> Vec<&'static str>;
    fn keywords(&self) -> Vec<&'static str>;
}

pub fn parameters() -> Vec<&'static dyn Parameter> {
    ast::ParameterName::iter()
        .map(|pn| pn.to_parameter())
        .collect()
}

macro_rules! parameter {
    ($param:ident, $name:expr, $purpose:expr, $vt:expr, $desc:expr, $examples:expr, $($kw:expr),+) => {
        pub struct $param;
        impl Parameter for $param {
            fn name(&self) -> &'static str { $name }
            fn purpose(&self) -> &'static str { $purpose }
            fn value_type(&self) -> ValueType { $vt }
            fn description(&self) -> &'static str { $desc }
            fn examples(&self) -> Vec<&'static str> { $examples }
            fn keywords(&self) -> Vec<&'static str> {
                vec![$($kw),+]
            }
        }
    };
}

parameter! {
    AltRep,
    "ALTREP",
    "To specify an alternate text representation for the property value.",
    ValueType::Uri,
    "",
    vec![],
    "altrep"
}

parameter! {
    CN,
    "CN",
    "To specify the common name to be associated with the calendar user specified by the property.",
    ValueType::Text,
    "",
    vec![],
    "cn", "common name"
}

parameter! {
    CUType,
    "CUTYPE",
    "To identify the type of calendar user specified by the property",
    ValueType::Text,
    "",
    vec![],
    "cutype"
}

parameter! {
    DelegatedFrom,
    "DELEGATED-FROM",
    "",
    ValueType::Text,
    "",
    vec![],
    "delegated-from"
}

parameter! {
    DelegatedTo,
    "DELEGATED-TO",
    "",
    ValueType::Text,
    "",
    vec![],
    "delegated-to"
}

parameter! {
    Dir,
    "DIR",
    "",
    ValueType::Text,
    "",
    vec![],
    "dir"
}

parameter! {
    Encoding,
    "ENCODING",
    "",
    ValueType::Text,
    "",
    vec![],
    "encoding"
}

parameter! {
    FmtType,
    "FMTTYPE",
    "",
    ValueType::Text,
    "",
    vec![],
    "fmttype"
}

parameter! {
    FBType,
    "FBTYPE",
    "",
    ValueType::Text,
    "",
    vec![],
    "fbtype"
}

parameter! {
    Language,
    "LANGUAGE",
    "",
    ValueType::Text,
    "",
    vec![],
    "language"
}

parameter! {
    Member,
    "MEMBER",
    "",
    ValueType::Text,
    "",
    vec![],
    "member"
}

parameter! {
    PartStat,
    "PARTSTAT",
    "",
    ValueType::Text,
    "",
    vec![],
    "partstat"
}

parameter! {
    Range,
    "RANGE",
    "",
    ValueType::Text,
    "",
    vec![],
    "range"
}

parameter! {
    Related,
    "RELATED",
    "",
    ValueType::Text,
    "",
    vec![],
    "related"
}

parameter! {
    RelType,
    "RELTYPE",
    "",
    ValueType::Text,
    "",
    vec![],
    "reltype"
}

parameter! {
    Role,
    "ROLE",
    "",
    ValueType::Text,
    "",
    vec![],
    "role"
}

parameter! {
    RSVP,
    "RSVP",
    "",
    ValueType::Text,
    "",
    vec![],
    "rsvp"
}

parameter! {
    SentBy,
    "SENT-BY",
    "",
    ValueType::Text,
    "",
    vec![],
    "sent-by"
}

parameter! {
    TZId,
    "TZID",
    "",
    ValueType::Text,
    "",
    vec![],
    "tzid"
}

parameter! {
    Value,
    "VALUE",
    "",
    ValueType::Text,
    "",
    vec![],
    "value"
}
