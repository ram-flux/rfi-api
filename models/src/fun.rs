//
//  Copyright 2024 Ram Flux, LLC.
//



use chrono::{NaiveDateTime, Utc};


pub fn get_current_date() -> NaiveDateTime {
    Utc::now().naive_utc()
}
