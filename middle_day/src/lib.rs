// use chrono::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if is_leap_year(year) {
       return None ; 
    }
   let middle = chrono::NaiveDate::from_yo_opt(year as i32, 183);
   Some()
    // todo!()
}
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}