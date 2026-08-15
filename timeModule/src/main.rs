// use std::{ops::Sub, time::{Duration, Instant}};

use chrono::{NaiveDate, Utc};

extern crate chrono;

fn main() {
    println!("Hello, world!");

    test_chrono();

}

// #[allow(dead_code)]
// fn test_std_time() {
//     let dur1 = Duration::from_secs(15);

//     println!("{}", dur1.as_millis());

//     let dur2: Duration = Duration::from_millis(14500);
//     let dur3: Option<Duration> = dur1.checked_sub(dur2);

//     println!("{}", dur3.unwrap_or_default().as_millis());


//     let now =  Instant::now();
//     std::thread::sleep(Duration::from_millis(200));

//     println!("{}", now.elapsed().as_micros());


// }


fn test_chrono() {

    let now: chrono::prelude::DateTime<Utc> = Utc::now();
    println!("{}", now.format("%Y %b %d %H %s"));


    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Z %Y %b %d %H"));



    let date1 = NaiveDate::from_isoywd_opt(2026, 3, chrono::Weekday::Sun);
    let unwrapped_date = date1.unwrap();

    unwrapped_date.iter_days().take(4).for_each(|d| println!("{}", d.format("%j") ));





}





