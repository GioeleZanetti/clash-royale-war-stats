use chrono::prelude::*;
use chrono::{Duration, Utc};

pub fn is_in_race_range(lastwar: &DateTime<Utc>) -> bool {
    let mut race_start = Utc.ymd(2022, 04, 25);
    let now = Utc.ymd(Utc::now().year(), Utc::now().month(), Utc::now().day());
    while race_start + Duration::days(7) < now {
        race_start = race_start + Duration::days(7);
    }
    let race_end = race_start + Duration::days(7);
    race_start <= lastwar.date() && lastwar.date() <= race_end
}