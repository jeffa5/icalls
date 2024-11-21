pub trait Property {
    fn name(&self) -> &'static str;
    fn purpose(&self) -> &'static str;
    fn value_type(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn examples(&self) -> Vec<&'static str>;
    fn keywords(&self) -> Vec<&'static str>;
}

pub fn properties() -> Vec<&'static dyn Property> {
    vec![&Summary, &Status, &Location]
}

macro_rules! property {
    ($prop:ident, $name:expr, $purpose:expr, $vt:expr, $desc:expr, $examples:expr, $($kw:expr),+) => {
        pub struct $prop;
        impl Property for $prop {
            fn name(&self) -> &'static str { $name }
            fn purpose(&self) -> &'static str { $purpose }
            fn value_type(&self) -> &'static str { $vt }
            fn description(&self) -> &'static str { $desc }
            fn examples(&self) -> Vec<&'static str> { $examples }
            fn keywords(&self) -> Vec<&'static str> {
                vec![$($kw),+]
            }
        }
    };
}

property! {
    Summary,
    "SUMMARY",
    "This property defines a short summary or subject for the calendar component.",
    "TEXT",
    r#"This property is used in the "VEVENT", "VTODO", and "VJOURNAL" calendar components to capture a short, one-line summary about the activity or journal entry.

This property is used in the "VALARM" calendar component to capture the subject of an EMAIL category of alarm."#,
    vec!["SUMMARY:Department Party"],
    "summary"
}

property! {
    Status,
    "STATUS",
    "This property defines the overall status or confirmation for the calendar component.",
    "TEXT",
    r#"In a group-scheduled calendar component, the property is used by the "Organizer" to provide a confirmation of the event to the "Attendees".  For example in a "VEVENT" calendar component, the "Organizer" can indicate that a meeting is tentative, confirmed, or cancelled.  In a "VTODO" calendar component, the "Organizer" can indicate that an action item needs action, is completed, is in process or being worked on, or has been cancelled.  In a "VJOURNAL" calendar component, the "Organizer" can indicate that a journal entry is draft, final, or has been cancelled or removed."#,
    vec!["STATUS:TENTATIVE", "STATUS:NEEDS-ACTION", "STATUS:DRAFT"],
    "status"
}

property! {
    Location,
    "LOCATION",
    "This property defines the intended venue for the activity defined by a calendar component.",
    "TEXT",
    "Specific venues such as conference or meeting rooms may be explicitly specified using this property.  An alternate representation may be specified that is a URI that points to directory information with more structured specification of the location.  For example, the alternate representation may specify either an LDAP URL [RFC4516] pointing to an LDAP server entry or a CID URL [RFC2392] pointing to a MIME body part containing a Virtual-Information Card (vCard) [RFC2426] for the location.",
    vec![
        "LOCATION:Conference Room - F123\\, Bldg. 002",
        "LOCATION;ALTREP=\"http://xyzcorp.com/conf-rooms/f123.vcf\": Conference Room - F123\\, Bldg. 002"
    ],
    "location"
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use super::*;

    #[test]
    fn macro_check() {
        expect!["SUMMARY"].assert_eq(Summary.name());
        expect!["This property defines a short summary or subject for the calendar component."]
            .assert_eq(Summary.purpose());
        expect!["TEXT"].assert_eq(Summary.value_type());
        expect![].assert_eq(Summary.description());
        expect![].assert_debug_eq(&Summary.examples());
        expect![[r#"
            [
                "summary",
            ]
        "#]]
        .assert_debug_eq(&Summary.keywords());
    }
}
