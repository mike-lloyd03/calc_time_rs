use std::io;
// use std::fmt;
use time::Time;
// use time::Duration;
use chrono::NaiveTime;
use chrono::Local;
use chrono::Duration;

fn main() {
    calc_time();
}

fn calc_time() {
    println!("Enter times as \"HHMM\"");

    let mut start_time = String::new();
    println!("Enter start time: ");
    io::stdin().read_line(&mut start_time).expect("Try again idiot");
    
    let mut start_lunch = String::new();
    println!("Enter lunch start time: ");
    io::stdin().read_line(&mut start_lunch).expect("Try again idiot");

    let mut end_lunch = String::new();
    println!("Enter lunch end time: ");
    io::stdin().read_line(&mut end_lunch).expect("Try again idiot");

    let mut end_time = String::new();
    println!("Enter end time: ");
    io::stdin().read_line(&mut end_time).expect("Try again idiot");

    println!("start time = {}", start_time.trim());
    println!("start_lunch = {}", start_lunch.trim());
    println!("end_lunch = {}", end_lunch.trim());
    println!("end_time = {}", end_time.trim());

    let start_time_obj = NaiveTime::parse_from_str(&start_time.trim(), "%H%M").unwrap();
    let end_time_obj = NaiveTime::parse_from_str(&end_time.trim(), "%H%M").unwrap();
    let total_time = end_time_obj - start_time_obj;
    println!("time {}", format_duration(&total_time));
}

fn format_duration(d: &Duration) -> String {
    let h = d.num_hours();
    let m = d.num_minutes() % (h * 60);
    format!("{}:{:02}", h, m)
}

// impl fmt::Display for Duration {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let h:i8 = self.seconds / 3600;
//         let m:u8 = self.seconds % (h * 3600) / 60;
//         write!(f, "{}:{}", h, m)
//     }
// }
