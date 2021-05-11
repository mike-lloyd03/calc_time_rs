use chrono::Duration;
use chrono::NaiveTime;
use std::io;

fn main() {
    calc_time();
}

fn calc_time() {
    println!("Enter all times as four digit 24 hr times without colons (e.g. \"1415\").");

    println!("Enter start time: ");
    let start_time_obj = get_time_input();

    println!("Enter lunch start time: ");
    let start_lunch_obj = get_time_input();

    println!("Enter lunch end time: ");
    let end_lunch_obj = get_time_input();

    println!("Enter end time: ");
    let end_time_obj = get_time_input();

    // println!("start time = {}", start_time_obj.format("%H%M"));
    // println!("start_lunch = {}", start_lunch_obj.format("%H%M"));
    // println!("end_lunch = {}", end_lunch_obj.format("%H%M"));
    // println!("end_time = {}", end_time_obj.format("%H%M"));

    let total_time = end_time_obj - start_time_obj - (end_lunch_obj - start_lunch_obj);
    println!("Total time {}", format_duration(&total_time));
}

fn format_duration(d: &Duration) -> String {
    let h = d.num_hours();
    let m = d.num_minutes() % (h * 60);
    format!("{}:{:02}", h, m)
}

fn get_time_input() -> NaiveTime {
    let mut time = String::new();
    io::stdin().read_line(&mut time).unwrap();
    NaiveTime::parse_from_str(&time.trim(), "%H%M")
        .expect("Entered time format is invalid. Please enter time as \"HHMM\"")
}
