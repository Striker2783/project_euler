pub fn run() {
    let mut start_date = Date::new(1900, 1, DayOfWeek::Monday, Month::January);
    let starting = Date::new(1901, 1, DayOfWeek::Monday, Month::January);
    start_date.increment_to(&starting);
    let end_date = Date::new(2000, 31, DayOfWeek::Monday, Month::December);
    let sundays = start_date.counting_sundays(&end_date);
    println!("{sundays}");
}
#[derive(PartialEq, Eq)]
enum Month {
    January,
    Feburary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
impl Month {
    pub fn days(&self, is_leap: bool) -> u32 {
        match self {
            Month::Feburary => {
                if is_leap {
                    29
                } else {
                    28
                }
            }
            Month::April => 30,
            Month::June => 30,
            Month::September => 30,
            Month::November => 30,
            _ => 31,
        }
    }
    pub fn next_month(&self) -> Self {
        match self {
            Month::January => Self::Feburary,
            Month::Feburary => Self::March,
            Month::March => Self::April,
            Month::April => Self::May,
            Month::May => Self::June,
            Month::June => Self::July,
            Month::July => Self::August,
            Month::August => Self::September,
            Month::September => Self::October,
            Month::October => Self::November,
            Month::November => Self::December,
            Month::December => Self::January,
        }
    }
}
impl ToString for Month {
    fn to_string(&self) -> String {
        match self {
            Month::January => "January",
            Month::Feburary => "Feburary",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        }
        .into()
    }
}
#[derive(PartialEq, Eq)]
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
impl DayOfWeek {
    pub fn next(&self) -> Self {
        match self {
            DayOfWeek::Sunday => Self::Monday,
            DayOfWeek::Monday => Self::Tuesday,
            DayOfWeek::Tuesday => Self::Wednesday,
            DayOfWeek::Wednesday => Self::Thursday,
            DayOfWeek::Thursday => Self::Friday,
            DayOfWeek::Friday => Self::Saturday,
            DayOfWeek::Saturday => Self::Sunday,
        }
    }
}
impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
            DayOfWeek::Saturday => "Saturday",
        }
        .into()
    }
}
#[derive(Eq)]
struct Date {
    year: u32,
    day: u32,
    day_of_week: DayOfWeek,
    month: Month,
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.day == other.day && self.month == other.month
    }
}

impl Date {
    pub fn new(year: u32, day: u32, day_of_week: DayOfWeek, month: Month) -> Self {
        Self {
            year,
            day,
            day_of_week,
            month,
        }
    }
    pub fn is_leap_year(year: u32) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }
    pub fn increment_to(&mut self, end_date: &Self) {
        while self != end_date {
            self.next_day();
        }
    }
    fn next_day(&mut self) {
        self.day += 1;
        let is_leap_year = Self::is_leap_year(self.year);
        if self.day > self.month.days(is_leap_year) {
            self.month = self.month.next_month();
            if self.month == Month::January {
                self.year += 1;
            }
            self.day = 1;
        }
        self.day_of_week = self.day_of_week.next();
    }
    fn counting_sundays(&mut self, end_date: &Self) -> u32 {
        let mut count = 0;
        while (self != end_date) {
            self.next_day();
            if self.day == 1 && self.day_of_week == DayOfWeek::Sunday {
                count += 1;
            }
        }

        count
    }
}
impl ToString for Date {
    fn to_string(&self) -> String {
        "".to_string()
            + &self.day_of_week.to_string()
            + ", "
            + &self.month.to_string()
            + " "
            + &self.day.to_string()
            + ", "
            + &self.year.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::problems::one_hundred::nineteen::Date;

    #[test]
    fn test_is_leap_year() {
        assert!(Date::is_leap_year(2000));
        assert!(!Date::is_leap_year(1900));
        assert!(Date::is_leap_year(2004));
    }
}
