use chrono::*;
mod calendar;

fn main() {
    println!("Getting date");
    let cal = calendar::Calendar::default();
    let date = cal.get_date();
    println!("{:?}", date);
    if let Some(date) = date {
        println!("{:?}", date.year());
        println!("{:?}", date.month());
        println!("{:?}", date.day());
    }
}