pub trait Property {
    fn name(&self) -> &'static str;
    fn purpose(&self) -> &'static str;
    fn value_type(&self) -> ValueType;
    fn description(&self) -> &'static str;
    fn examples(&self) -> Vec<&'static str>;
    fn keywords(&self) -> Vec<&'static str>;
}

pub fn properties() -> Vec<&'static dyn Property> {
    vec![
        &Begin, &End, &Version, &DTStart, &DTEnd, &Uid, &Summary, &Status, &Location,
    ]
}

#[derive(Debug)]
pub enum ValueType {
    Binary,
    Boolean,
    CalAddress,
    Date,
    DateTime,
    Duration,
    Float,
    Integer,
    PeriodOfTime,
    RecurrenceRule,
    Text,
    Time,
    Uri,
    UtcOffset,
}

macro_rules! property {
    ($prop:ident, $name:expr, $purpose:expr, $vt:expr, $desc:expr, $examples:expr, $($kw:expr),+) => {
        pub struct $prop;
        impl Property for $prop {
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

property! {
    Begin,
    "BEGIN",
    "",
    ValueType::Text,
    "",
    vec!["BEGIN:VCALENDAR", "BEGIN:VEVENT"],
    "begin"
}

property! {
    End,
    "END",
    "",
    ValueType::Text,
    "",
    vec!["END:VCALENDAR", "END:VEVENT"],
    "end"
}

property! {
    Version,
    "VERSION",
    "This property specifies the identifier corresponding to the highest version number or the minimum and maximum range of the iCalendar specification that is required in order to interpret the iCalendar object.",
    ValueType::Text,
    r#"A value of "2.0" corresponds to this memo (rfc5545)."#,
    vec!["VERSION:2.0"],
    "version"
}

property! {
    DTStart,
    "DTSTART",
    "This property specifies when the calendar component begins.",
    ValueType::DateTime,
    r#"Within the "VEVENT" calendar component, this property defines the start date and time for the event.

      Within the "VFREEBUSY" calendar component, this property defines the start date and time for the free or busy time information. The time MUST be specified in UTC time.

      Within the "STANDARD" and "DAYLIGHT" sub-components, this property defines the effective start date and time for a time zone specification.  This property is REQUIRED within each "STANDARD" and "DAYLIGHT" sub-components included in "VTIMEZONE" calendar components and MUST be specified as a date with local time without the "TZID" property parameter."#,
    vec!["DTSTART:19980118T073000Z"],
    "dtstart",
    "begin"
}

property! {
    DTEnd,
    "DTEND",
    "This property specifies the date and time that a calendar component ends.",
    ValueType::DateTime,
    r#"Within the "VEVENT" calendar component, this property
      defines the date and time by which the event ends.  The value type
      of this property MUST be the same as the "DTSTART" property, and
      its value MUST be later in time than the value of the "DTSTART"
      property.  Furthermore, this property MUST be specified as a date
      with local time if and only if the "DTSTART" property is also
      specified as a date with local time.

      Within the "VFREEBUSY" calendar component, this property defines
      the end date and time for the free or busy time information.  The
      time MUST be specified in the UTC time format.  The value MUST be
      later in time than the value of the "DTSTART" property.
      "#,
    vec!["DTEND:19960401T150000Z", "DTEND;VALUE=DATE:19980704"],
    "dtend",
    "finish"
}

property! {
    Uid,
    "UID",
    "This property defines the persistent, globally unique identifier for the calendar component.",
ValueType::Text,
    r#"The "UID" itself MUST be a globally unique identifier.
      The generator of the identifier MUST guarantee that the identifier
      is unique.  There are several algorithms that can be used to
      accomplish this.  A good method to assure uniqueness is to put the
      domain name or a domain literal IP address of the host on which
      the identifier was created on the right-hand side of an "@", and
      on the left-hand side, put a combination of the current calendar
      date and time of day (i.e., formatted in as a DATE-TIME value)
      along with some other currently unique (perhaps sequential)
      identifier available on the system (for example, a process id
      number).  Using a DATE-TIME value on the left-hand side and a
      domain name or domain literal on the right-hand side makes it
      possible to guarantee uniqueness since no two hosts should be
      using the same domain name or IP address at the same time.  Though
      other algorithms will work, it is RECOMMENDED that the right-hand
      side contain some domain identifier (either of the host itself or
      otherwise) such that the generator of the message identifier can
      guarantee the uniqueness of the left-hand side within the scope of
      that domain.

      This is the method for correlating scheduling messages with the
      referenced "VEVENT", "VTODO", or "VJOURNAL" calendar component.

      The full range of calendar components specified by a recurrence
      set is referenced by referring to just the "UID" property value
      corresponding to the calendar component.  The "RECURRENCE-ID"
      property allows the reference to an individual instance within the
      recurrence set.

      This property is an important method for group-scheduling
      applications to match requests with later replies, modifications,
      or deletion requests.  Calendaring and scheduling applications
      MUST generate this property in "VEVENT", "VTODO", and "VJOURNAL"
      calendar components to assure interoperability with other group-
      scheduling applications.  This identifier is created by the
      calendar system that generates an iCalendar object.

      Implementations MUST be able to receive and persist values of at
      least 255 octets for this property, but they MUST NOT truncate
      values in the middle of a UTF-8 multi-octet sequence.
"#,
    vec!["UID:19960401T080045Z-4000F192713-0052@example.com"],
    "uid"
}

property! {
    Summary,
    "SUMMARY",
    "This property defines a short summary or subject for the calendar component.",
ValueType::Text,
    r#"This property is used in the "VEVENT", "VTODO", and "VJOURNAL" calendar components to capture a short, one-line summary about the activity or journal entry.

This property is used in the "VALARM" calendar component to capture the subject of an EMAIL category of alarm."#,
    vec!["SUMMARY:Department Party"],
    "summary"
}

property! {
    Status,
    "STATUS",
    "This property defines the overall status or confirmation for the calendar component.",
ValueType::Text,
    r#"In a group-scheduled calendar component, the property is used by the "Organizer" to provide a confirmation of the event to the "Attendees".  For example in a "VEVENT" calendar component, the "Organizer" can indicate that a meeting is tentative, confirmed, or cancelled.  In a "VTODO" calendar component, the "Organizer" can indicate that an action item needs action, is completed, is in process or being worked on, or has been cancelled.  In a "VJOURNAL" calendar component, the "Organizer" can indicate that a journal entry is draft, final, or has been cancelled or removed."#,
    vec!["STATUS:TENTATIVE", "STATUS:NEEDS-ACTION", "STATUS:DRAFT"],
    "status"
}

property! {
    Location,
    "LOCATION",
    "This property defines the intended venue for the activity defined by a calendar component.",
ValueType::Text,
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
        expect!["TEXT"].assert_debug_eq(Summary.value_type());
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
