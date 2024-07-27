use chrono::{Datelike, Duration, Local, NaiveDateTime, NaiveTime};

pub struct Friday {
    pub prev: NaiveDateTime,
    pub next: NaiveDateTime,
}

pub fn get_fridays() -> Friday {
    let input_date = Local::now().date_naive();
    let current_weekday = input_date.weekday().num_days_from_monday();
    let zaras = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
    // Calculate the number of days until the next Friday (5th day of the week)
    let days_until_friday = (4 + 7 - current_weekday) % 7;
    // If the input date is already Friday, we need to move to the next week's Friday
    let days_until_friday = if current_weekday == 4 {
        0
    } else {
        days_until_friday
    };

    // Get the next Friday by adding the days until Friday to the input date
    let next_friday = input_date + Duration::days(days_until_friday.into());
    let last_friday = next_friday + Duration::days(-7);
    let next_friday_whole = NaiveDateTime::new(next_friday, zaras);
    let last_friday_whole = NaiveDateTime::new(last_friday, zaras);
    Friday {
        prev: last_friday_whole,
        next: next_friday_whole,
    }
}
