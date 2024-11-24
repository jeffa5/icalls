use strum::IntoEnumIterator as _;

use crate::ast;

pub trait Property {
    fn name(&self) -> &'static str;
    fn purpose(&self) -> &'static str;
    fn value_type(&self) -> ValueType;
    fn description(&self) -> &'static str;
    fn examples(&self) -> Vec<&'static str>;
    fn keywords(&self) -> Vec<&'static str>;
}

pub fn properties() -> Vec<&'static dyn Property> {
    ast::PropertyName::iter()
        .map(|pn| pn.to_property())
        .collect()
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

property! {
    CalScale,
    "CALSCALE",
    "This property defines the calendar scale used for the calendar information specified in the iCalendar object.",
    ValueType::Text,
    "This memo is based on the Gregorian calendar scale. The Gregorian calendar scale is assumed if this property is not specified in the iCalendar object.  It is expected that other calendar scales will be defined in other specifications or by future versions of this memo.",
    vec!["CALSCALE:GREGORIAN"],
    "calscale"
}

property! {
    Method,
    "METHOD",
    "This property defines the iCalendar object method associated with the calendar object.",
    ValueType::Text,
    "",
    vec!["METHOD:REQUEST"],
    "method"
}

property! {
    ProdId,
    "PRODID",
    "This property specifies the identifier for the product that created the iCalendar object.",
    ValueType::Text,
    "",
    vec!["PRODID:-//ABC Corporation//NONSGML My Product//EN"],
    "prodid", "product identifier"
}

property! {
    Attach,
    "ATTACH",
    "This property provides the capability to associate a document object with a calendar component.",
    ValueType::Text,
    "",
    vec![ "ATTACH:CID:jsmith.part3.960817T083000.xyzMail@example.com",
       "ATTACH;FMTTYPE=application/postscript:ftp://example.com/pub/reports/r-960812.ps"
],
    "attach"
}

property! {
    Categories,
    "CATEGORIES",
    "This property defines the categories for a calendar component.",
    ValueType::Text,
    "",
    vec!["CATEGORIES:APPOINTMENT,EDUCATION", "CATEGORIES:MEETING"],
    "categories"
}

property! {
    Class,
    "CLASS",
    "This property defines the access classification for a calendar component.",
    ValueType::Text,
    "",
    vec!["CLASS:PUBLIC"],
    "classification"
}

property! {
    Comment,
    "COMMENT",
    "This property specifies non-processing information intended to provide a comment to the calendar user.",
    ValueType::Text,
    "",
    vec![],
    "comment"
}

property! {
    Description,
    "DESCRIPTION",
    r#"This property provides a more complete description of the calendar component than that provided by the "SUMMARY" property."#,
    ValueType::Text,
    "",
    vec![],
    "description"
}

property! {
    Geo,
    "GEO",
    "This property specifies information related to the global position for the activity specified by a calendar component.",
    ValueType::Float,
    "",
    vec![],
    "geographic position"
}

property! {
    PercentComplete,
    "PERCENT-COMPLETE",
    r#"This property is used by an assignee or delegatee of a to-do to convey the percent completion of a to-do to the "Organizer"."#,
    ValueType::Integer,
    "",
    vec![],
    "percent complete"
}

property! {
    Priority,
    "PRIORITY",
    "This property defines the relative priority for a calendar component.",
    ValueType::Integer,
    "",
    vec![],
    "priority"
}

property! {
    Resources,
    "RESOURCES",
    "This property defines the equipment or resources anticipated for an activity specified by a calendar component.",
    ValueType::Text,
    "",
    vec!["RESOURCES:EASEL,PROJECTOR,VCR"],
    "resources"
}

property! {
    Completed,
    "COMPLETED",
    "This property defines the date and time that a to-do was actually completed.",
    ValueType::DateTime,
    "",
    vec!["COMPLETED:19960401T150000Z"],
    "completed", "done"
}

property! {
    DtEnd,
    "DTEND",
    "This property specifies the date and time that a calendar component ends.",
    ValueType::DateTime,
    "",
    vec!["DTEND:19960401T150000Z","DTEND;VALUE=DATE:19980704"],
    "dtend"
}

property! {
    Due,
    "DUE",
    "This property defines the date and time that a to-do is expected to be completed.",
    ValueType::DateTime,
    "",
    vec!["DUE:19980430T000000Z"],
    "due"
}

property! {
    DtStart,
    "DTSTART",
    "This property specifies when the calendar component begins.",
    ValueType::DateTime,
    "",
    vec!["DTSTART:19980118T073000Z"],
    "dtstart"
}

property! {
    Duration,
    "DURATION",
    "This property specifies a positive duration of time.",
    ValueType::Duration,
    "",
    vec!["DURATION:PT1H0M0S"],
    "duration"
}

property! {
    FreeBusy,
    "FREEBUSY",
    "This property defines one or more free or busy time intervals.",
    ValueType::PeriodOfTime,
    "",
    vec!["FREEBUSY;FBTYPE=BUSY-UNAVAILABLE:19970308T160000Z/PT8H30M"],
    "freebusy"
}

property! {
    Transp,
    "TRANSP",
    "This property defines whether or not an event is transparent to busy time searches.",
    ValueType::Text,
    "",
    vec!["TRANSP:TRANSPARENT", "TRANSP:OPAQUE"],
    "transparency", "transparent", "opaque"
}

property! {
    TzId,
    "TZID",
    r#"This property specifies the text value that uniquely identifies the "VTIMEZONE" calendar component in the scope of an iCalendar object."#,
    ValueType::Text,
    "",
    vec!["TZID:America/New_York"],
    "tzid", "timezone identifier"
}

property! {
    TzName,
    "TZNAME",
    "This property specifies the customary designation for a time zone description.",
    ValueType::Text,
    "",
    vec![],
    "tzname"
}

property! {
    TzOffsetFrom,
    "TZOFFSETFROM",
    "This property specifies the offset that is in use prior to this time zone observance.",
    ValueType::Text,
    "",
    vec![],
    "tzoffsetfrom"
}

property! {
    TzOffsetTo,
    "TZOFFSETTO",
    "This property specifies the offset that is in use in this time zone observance.",
    ValueType::Text,
    "",
    vec![],
    "tzoffsetto"
}

property! {
    TzUrl,
    "TZURL",
    r#"This property provides a means for a "VTIMEZONE" component to point to a network location that can be used to retrieve an up- to-date version of itself."#,
    ValueType::Uri,
    "",
    vec![],
    "tzurl"
}

property! {
    Attendee,
    "ATTENDEE",
    "",
    ValueType::CalAddress,
    "",
    vec![],
    "attendee"
}

property! {
    Contact,
    "CONTACT",
    "",
    ValueType::Text,
    "",
    vec![],
    "contact"
}

property! {
    Organizer,
    "ORGANIZER",
    "",
    ValueType::CalAddress,
    "",
    vec![],
    "organizer"
}

property! {
    RecurrenceId,
    "RECURRENCE-ID",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "recurrence-id"
}

property! {
    RelatedTo,
    "RELATED-TO",
    "",
    ValueType::Text,
    "",
    vec![],
    "related-to"
}

property! {
    Url,
    "URL",
    "",
    ValueType::Uri,
    "",
    vec![],
    "url"
}

property! {
    ExDate,
    "EXDATE",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "exdate"
}

property! {
    RDate,
    "RDATE",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "rdate"
}

property! {
    RRule,
    "RRULE",
    "",
    ValueType::RecurrenceRule,
    "",
    vec![],
    "rrule"
}

property! {
    Action,
    "ACTION",
    "",
    ValueType::Text,
    "",
    vec![],
    "action"
}

property! {
    Repeat,
    "REPEAT",
    "",
    ValueType::Integer,
    "",
    vec![],
    "repeat"
}

property! {
    Trigger,
    "TRIGGER",
    "",
    ValueType::Duration,
    "",
    vec![],
    "trigger"
}

property! {
    Created,
    "CREATED",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "created"
}

property! {
    DtStamp,
    "DTSTAMP",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "dtstamp"
}

property! {
    LastModified,
    "LAST-MODIFIED",
    "",
    ValueType::DateTime,
    "",
    vec![],
    "last-modified"
}

property! {
    Sequence,
    "SEQUENCE",
    "",
    ValueType::Integer,
    "",
    vec![],
    "sequence"
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
        expect![[r#"
            Text
        "#]]
        .assert_debug_eq(&Summary.value_type());
        expect![[r#"
            This property is used in the "VEVENT", "VTODO", and "VJOURNAL" calendar components to capture a short, one-line summary about the activity or journal entry.

            This property is used in the "VALARM" calendar component to capture the subject of an EMAIL category of alarm."#]].assert_eq(Summary.description());
        expect![[r#"
            [
                "SUMMARY:Department Party",
            ]
        "#]]
        .assert_debug_eq(&Summary.examples());
        expect![[r#"
            [
                "summary",
            ]
        "#]]
        .assert_debug_eq(&Summary.keywords());
    }
}
