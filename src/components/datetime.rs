//! Calendar dates and times of day, for the date and time pickers.
use std::fmt;
use std::str::FromStr;

/// A day in the proleptic Gregorian calendar, with no time or time zone.
///
/// Parses from and displays as ISO 8601 (`2026-09-19`), the format of an
/// HTML date input, so it round-trips through forms and JSON.
///
/// ```
/// use g3_ui::CalendarDate;
///
/// let date: CalendarDate = "2026-09-19".parse().unwrap();
/// assert_eq!(date.add_days(13).to_string(), "2026-10-02");
/// assert_eq!(date.weekday(), 6); // Saturday
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalendarDate {
    year: i32,
    month: u8,
    day: u8,
}

impl CalendarDate {
    /// The date, or `None` if the month or day is out of range.
    pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        ((1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month))
            .then_some(Self { year, month, day })
    }

    /// The year.
    pub fn year(self) -> i32 {
        self.year
    }

    /// The month, 1 to 12.
    pub fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, from 1.
    pub fn day(self) -> u8 {
        self.day
    }

    /// Day of the week, 0 for Sunday to 6 for Saturday.
    pub fn weekday(self) -> u8 {
        // 1970-01-01 was a Thursday.
        (self.days_since_epoch() + 4).rem_euclid(7) as u8
    }

    /// The date `days` later (or earlier, when negative).
    pub fn add_days(self, days: i64) -> Self {
        Self::from_days_since_epoch(self.days_since_epoch() + days)
    }

    /// The same day `months` later (or earlier), moved back to the last day
    /// of the month when that month is shorter.
    pub fn add_months(self, months: i32) -> Self {
        let index = self.year * 12 + i32::from(self.month) - 1 + months;
        let year = index.div_euclid(12);
        let month = (index.rem_euclid(12) + 1) as u8;
        let day = self.day.min(days_in_month(year, month));
        Self { year, month, day }
    }

    /// The first day of this date's month.
    pub fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    /// Clamp to the range `min..=max`, where each bound is optional.
    pub fn clamp_to(self, min: Option<Self>, max: Option<Self>) -> Self {
        let mut date = self;
        if let Some(min) = min {
            date = date.max(min);
        }
        if let Some(max) = max {
            date = date.min(max);
        }
        date
    }

    /// Today in the device's local time zone.
    ///
    /// On the web this reads the browser clock. Elsewhere it uses the system
    /// clock in UTC, which can be a day off near midnight.
    pub fn today() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let now = js_sys::Date::new_0();
            Self {
                year: now.get_full_year() as i32,
                month: now.get_month() as u8 + 1,
                day: now.get_date() as u8,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let seconds = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs() as i64);
            Self::from_days_since_epoch(seconds.div_euclid(86_400))
        }
    }

    /// Days since 1970-01-01, after Howard Hinnant's `days_from_civil`.
    fn days_since_epoch(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let month = i64::from(self.month);
        let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5
            + i64::from(self.day)
            - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }

    /// The inverse of [`days_since_epoch`](Self::days_since_epoch).
    fn from_days_since_epoch(days: i64) -> Self {
        let shifted = days + 719_468;
        let era = shifted.div_euclid(146_097);
        let day_of_era = shifted - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_index = (5 * day_of_year + 2) / 153;
        let day = (day_of_year - (153 * month_index + 2) / 5 + 1) as u8;
        let month = if month_index < 10 {
            month_index + 3
        } else {
            month_index - 9
        } as u8;
        let year = (year_of_era + era * 400 + i64::from(month <= 2)) as i32;
        Self { year, month, day }
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// Why a date or time string did not parse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseDateTimeError;

impl fmt::Display for ParseDateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected an ISO 8601 date (YYYY-MM-DD) or time (HH:MM)")
    }
}

impl std::error::Error for ParseDateTimeError {}

impl FromStr for CalendarDate {
    type Err = ParseDateTimeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.trim().splitn(3, '-');
        let (Some(year), Some(month), Some(day)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(ParseDateTimeError);
        };
        let number = |part: &str| part.parse::<u8>().map_err(|_| ParseDateTimeError);
        let year = year.parse().map_err(|_| ParseDateTimeError)?;
        Self::new(year, number(month)?, number(day)?).ok_or(ParseDateTimeError)
    }
}

/// Number of days in `month` (1 to 12) of `year`.
pub(crate) fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// A time of day to the minute, with no date or time zone.
///
/// Parses from and displays as 24-hour ISO 8601 (`14:05`), the format of an
/// HTML time input.
///
/// ```
/// use g3_ui::TimeOfDay;
///
/// let time: TimeOfDay = "14:05".parse().unwrap();
/// assert_eq!(time.hour12(), (2, true));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeOfDay {
    hour: u8,
    minute: u8,
}

impl TimeOfDay {
    /// The time, or `None` if the hour or minute is out of range.
    pub fn new(hour: u8, minute: u8) -> Option<Self> {
        (hour < 24 && minute < 60).then_some(Self { hour, minute })
    }

    /// The hour, 0 to 23.
    pub fn hour(self) -> u8 {
        self.hour
    }

    /// The minute, 0 to 59.
    pub fn minute(self) -> u8 {
        self.minute
    }

    /// The hour on a 12-hour clock (1 to 12), and whether it is after noon.
    pub fn hour12(self) -> (u8, bool) {
        let hour = match self.hour % 12 {
            0 => 12,
            hour => hour,
        };
        (hour, self.hour >= 12)
    }

    /// The time for a 12-hour clock reading: `hour` is 1 to 12.
    pub fn from_hour12(hour: u8, minute: u8, pm: bool) -> Option<Self> {
        if !(1..=12).contains(&hour) {
            return None;
        }
        Self::new(hour % 12 + if pm { 12 } else { 0 }, minute)
    }

    /// The time now, from the same clock as [`CalendarDate::today`].
    pub fn now() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let now = js_sys::Date::new_0();
            Self {
                hour: now.get_hours() as u8,
                minute: now.get_minutes() as u8,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let seconds = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs());
            let minutes = seconds / 60;
            Self {
                hour: (minutes / 60 % 24) as u8,
                minute: (minutes % 60) as u8,
            }
        }
    }
}

impl fmt::Display for TimeOfDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl FromStr for TimeOfDay {
    type Err = ParseDateTimeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.trim().split(':');
        let (Some(hour), Some(minute)) = (parts.next(), parts.next()) else {
            return Err(ParseDateTimeError);
        };
        let number = |part: &str| part.parse::<u8>().map_err(|_| ParseDateTimeError);
        Self::new(number(hour)?, number(minute)?).ok_or(ParseDateTimeError)
    }
}

/// Whether times show on a 12-hour clock with AM and PM, or a 24-hour one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum HourCycle {
    /// 1 to 12, with AM and PM.
    #[default]
    H12,
    /// 00 to 23.
    H24,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> CalendarDate {
        text.parse().unwrap()
    }

    #[test]
    fn dates_round_trip_through_day_counts() {
        for text in [
            "1970-01-01",
            "2000-02-29",
            "1600-03-01",
            "2026-09-19",
            "0001-01-01",
        ] {
            let parsed = date(text);
            assert_eq!(
                CalendarDate::from_days_since_epoch(parsed.days_since_epoch()),
                parsed
            );
            assert_eq!(parsed.to_string(), text);
        }
        assert_eq!(date("1970-01-01").days_since_epoch(), 0);
    }

    #[test]
    fn weekdays_and_day_arithmetic() {
        assert_eq!(date("1970-01-01").weekday(), 4);
        assert_eq!(date("2026-09-19").weekday(), 6);
        assert_eq!(date("2024-02-28").add_days(1), date("2024-02-29"));
        assert_eq!(date("2024-03-01").add_days(-1), date("2024-02-29"));
        assert_eq!(date("2026-12-31").add_days(1), date("2027-01-01"));
    }

    #[test]
    fn months_clamp_to_their_length() {
        assert_eq!(date("2026-01-31").add_months(1), date("2026-02-28"));
        assert_eq!(date("2024-01-31").add_months(1), date("2024-02-29"));
        assert_eq!(date("2026-01-15").add_months(-1), date("2025-12-15"));
        assert_eq!(date("2026-11-30").add_months(14), date("2028-01-30"));
    }

    #[test]
    fn invalid_dates_and_times_are_rejected() {
        assert!(CalendarDate::new(2026, 2, 29).is_none());
        assert!(CalendarDate::new(2026, 13, 1).is_none());
        assert!("2026-9".parse::<CalendarDate>().is_err());
        assert!("24:00".parse::<TimeOfDay>().is_err());
        assert!("7:60".parse::<TimeOfDay>().is_err());
    }

    #[test]
    fn twelve_hour_clock_readings() {
        assert_eq!(TimeOfDay::new(0, 5).unwrap().hour12(), (12, false));
        assert_eq!(TimeOfDay::new(12, 0).unwrap().hour12(), (12, true));
        assert_eq!(TimeOfDay::new(23, 59).unwrap().hour12(), (11, true));
        assert_eq!(TimeOfDay::from_hour12(12, 0, false), TimeOfDay::new(0, 0));
        assert_eq!(TimeOfDay::from_hour12(12, 30, true), TimeOfDay::new(12, 30));
        assert_eq!(TimeOfDay::from_hour12(7, 15, true), TimeOfDay::new(19, 15));
        assert_eq!("09:07".parse::<TimeOfDay>().unwrap().to_string(), "09:07");
    }

    #[test]
    fn clamping_respects_open_bounds() {
        let min = date("2026-01-01");
        let max = date("2026-12-31");
        assert_eq!(date("2025-05-05").clamp_to(Some(min), Some(max)), min);
        assert_eq!(date("2027-05-05").clamp_to(None, Some(max)), max);
        assert_eq!(
            date("2027-05-05").clamp_to(Some(min), None),
            date("2027-05-05")
        );
    }
}
