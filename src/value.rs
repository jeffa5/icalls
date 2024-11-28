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

#[derive(Debug)]
pub enum Value {
    Binary(Vec<u8>),
    Boolean(bool),
    CalAddress(String),
    Date(Date),
    DateTime(Date, Time),
    Duration(String),
    Float(f64),
    Integer(i64),
    PeriodOfTime(String),
    RecurrenceRule(String),
    Text(String),
    Time(Time),
    Uri(String),
    UtcOffset(String),
}

impl Value {
    pub fn prettify(&self) -> String {
        match self {
            Value::Binary(_) => "binary".to_owned(),
            Value::Boolean(v) => v.to_string(),
            Value::CalAddress(v) => v.to_string(),
            Value::Date(date) => date.prettify(),
            Value::DateTime(d, t) => format!("{} {}", t.prettify(), d.prettify()),
            Value::Duration(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::Integer(v) => v.to_string(),
            Value::PeriodOfTime(v) => v.to_string(),
            Value::RecurrenceRule(v) => v.to_string(),
            Value::Text(v) => v.to_string(),
            Value::Time(v) => v.prettify(),
            Value::Uri(v) => v.to_string(),
            Value::UtcOffset(v) => v.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    fn prettify(&self) -> String {
        format!(
            "{} {} {}",
            pretty_day(self.day),
            pretty_month(self.month),
            self.year
        )
    }
}

fn pretty_day(d: u8) -> String {
    if matches!(d, 1 | 21 | 31) {
        format!("{}st", d)
    } else if matches!(d, 2 | 22) {
        format!("{}nd", d)
    } else if matches!(d, 3 | 23) {
        format!("{}rd", d)
    } else {
        format!("{}th", d)
    }
}

fn pretty_month(m: u8) -> &'static str {
    [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ][m as usize]
}

#[derive(Debug)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub utc: bool,
}

impl Time {
    fn prettify(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}
