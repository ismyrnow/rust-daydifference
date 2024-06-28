use chrono::{DateTime, Duration, NaiveDate, Utc};
use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: <start_date> <end_date> <allowed_days_of_the_week> <excluded_dates>");
        std::process::exit(1);
    }

    let start_date_time = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Error parsing start_date").and_hms_opt(0,0,0).expect("Error parsing start_date");
    let start_date: i64 = start_date_time.and_utc().timestamp();
    let end_date_time = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Error parsing end_date").and_hms_opt(0,0,0).expect("Error parsing end_date");
    let end_date = end_date_time.and_utc().timestamp();
    let allowed_days_of_the_week = args[3].split(',')
        .map(|s| s.parse::<u32>().expect("Error parsing allowed_days_of_the_week"))
        .collect();
    let excluded_dates = args[4].split(',').map(String::from).collect();

    let difference = find_difference(start_date, end_date, allowed_days_of_the_week, excluded_dates);
    println!("Difference: {}", difference);
}

fn find_difference(start_stamp: i64, end_stamp: i64, allowed_days_of_the_week: Vec<u32>, excluded_dates: Vec<String>) -> i32 {
    let start_date = DateTime::<Utc>::from_timestamp(start_stamp, 0).expect("Invalid start timestamp").naive_utc().date();
    let end_date = DateTime::<Utc>::from_timestamp(end_stamp, 0).expect("Invalid end timestamp").naive_utc().date();
    let allowed_days_set: HashSet<String> = allowed_days_of_the_week.iter().map(|d| d.to_string()).collect();
    let excluded_dates_set: HashSet<String> = excluded_dates.into_iter().collect();

    let is_full_week = allowed_days_set.len() == 7;
    let has_exclusions = !excluded_dates_set.is_empty();
    let total_days = (end_stamp - start_stamp) / 86_400; // Seconds in one day

    if is_full_week && !has_exclusions {
        return total_days as i32;
    }

    let mut count = 0;
    let mut current_date = start_date;
    while current_date < end_date {
        let date_formats = current_date.format("%w,%Y-%m-%d,*-%m-%d").to_string();
        let date_formats_vec = date_formats.split(',').collect::<Vec<&str>>();
        let week_day = date_formats_vec[0].to_string();
        let specific_date = date_formats_vec[1].to_string();
        let wildcard_date = date_formats_vec[2].to_string();

        if allowed_days_set.contains(&week_day) && !excluded_dates_set.contains(&specific_date) && !excluded_dates_set.contains(&wildcard_date) {
            count += 1;
        }
        current_date += Duration::days(1);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module
    use chrono::TimeZone; // Make sure you have the `chrono` crate as a dependency

    #[test]
    fn test_same_start_end() {
        let start_stamp = Utc.with_ymd_and_hms(2023, 4, 1, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2023, 4, 1,0,0,0).single().unwrap().timestamp();
        let allowed_days_of_the_week = vec![0, 1, 2, 3, 4, 5, 6]; // All days of the week allowed
        let excluded_dates = vec![]; // No excluded dates

        let difference = find_difference(start_stamp, end_stamp, allowed_days_of_the_week, excluded_dates);

        assert_eq!(difference, 0); // Expecting 0 days difference
    }

    #[test]
    fn test_negative_diff() {
        let start_stamp = Utc.with_ymd_and_hms(2023, 4, 10, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2023, 4, 9,0,0,0).single().unwrap().timestamp();
        let allowed_days_of_the_week = vec![0, 1, 2, 3, 4, 5, 6]; // All days of the week allowed
        let excluded_dates = vec![]; // No excluded dates

        let difference = find_difference(start_stamp, end_stamp, allowed_days_of_the_week, excluded_dates);

        assert_eq!(difference, -1); // Expecting -1 days difference
    }

    #[test]
    fn test_two_weeks_no_exclusions() {
        let start_stamp = Utc.with_ymd_and_hms(2023, 4, 1, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2023, 4, 15,0,0,0).single().unwrap().timestamp();
        let allowed_days_of_the_week = vec![0, 1, 2, 3, 4, 5, 6]; // All days of the week allowed
        let excluded_dates = vec![]; // No excluded dates

        let difference = find_difference(start_stamp, end_stamp, allowed_days_of_the_week, excluded_dates);

        assert_eq!(difference, 14); // Expecting 7 days difference
    }

    #[test]
    fn test_skipping_specific_days_of_the_week() {
        let start_stamp = Utc.with_ymd_and_hms(2020, 1, 6, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2020, 1, 13,0,0,0).single().unwrap().timestamp();

        let test_cases = vec![
            (vec![1, 2, 3, 4, 5, 6], 6),
            (vec![0, 2, 3, 4, 5, 6], 6),
            (vec![0, 1, 3, 4, 5, 6], 6),
            (vec![0, 1, 2, 4, 5, 6], 6),
            (vec![0, 1, 2, 3, 5, 6], 6),
            (vec![0, 1, 2, 3, 4, 6], 6),
            (vec![0, 1, 2, 3, 4, 5], 6),
            (vec![2, 3, 4, 5, 6], 5),
            (vec![0, 3, 4, 5, 6], 5),
            (vec![0, 1, 4, 5, 6], 5),
            (vec![0, 1, 2, 5, 6], 5),
            (vec![0, 1, 2, 3, 6], 5),
            (vec![0, 1, 2, 3, 4], 5),
            (vec![0], 1),
            (vec![1], 1),
            (vec![2], 1),
            (vec![3], 1),
            (vec![4], 1),
            (vec![5], 1),
            (vec![6], 1),
        ];

        for (allowed_days_of_the_week, expected_difference) in test_cases {
            let difference = find_difference(start_stamp, end_stamp, allowed_days_of_the_week.clone(), vec![]);
            assert_eq!(difference, expected_difference, "Testing allowed days: {:?}", allowed_days_of_the_week);
        }
    }

    #[test]
    fn test_two_weeks_no_weekends() {
        let start_stamp = Utc.with_ymd_and_hms(2023, 4, 1, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2023, 4, 15,0,0,0).single().unwrap().timestamp();
        let allowed_days_of_the_week = vec![1, 2, 3, 4, 5]; // Only weekdays allowed
        let excluded_dates = vec![]; // No excluded dates

        let difference = find_difference(start_stamp, end_stamp, allowed_days_of_the_week, excluded_dates);

        assert_eq!(difference, 10); // Expecting 10 days difference
    }

    #[test]
    fn test_skipping_specific_dates() {
        let start_stamp = Utc.with_ymd_and_hms(2020, 1, 6, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2020, 1, 13,0,0,0).single().unwrap().timestamp();

        let excluded_dates = vec!["2020-01-07"].iter().map(|s| s.to_string()).collect();
        let difference = find_difference(start_stamp, end_stamp, vec![1, 2, 3, 4, 5], excluded_dates);
        assert_eq!(difference, 4);
    }

    #[test]
    fn test_skipping_yearly_repeating_dates() {
        let start_stamp = Utc.with_ymd_and_hms(2020, 1, 6, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2020, 1, 13,0,0,0).single().unwrap().timestamp();

        let excluded_dates = vec!["*-01-07"].iter().map(|s| s.to_string()).collect();
        let difference = find_difference(start_stamp, end_stamp, vec![1, 2, 3, 4, 5], excluded_dates);
        assert_eq!(difference, 4);
    }

    #[test]
    fn test_combining_all_the_things() {
        let start_stamp = Utc.with_ymd_and_hms(2020, 1, 6, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2020, 1, 13,0,0,0).single().unwrap().timestamp();

        let excluded_dates = vec!["2020-01-08", "*-01-07"].iter().map(|s| s.to_string()).collect();
        let difference = find_difference(start_stamp, end_stamp, vec![1, 2, 3, 4, 5], excluded_dates);
        assert_eq!(difference, 3);
    }

    #[test]
    fn test_skipping_defined_start_end_dates() {
        let start_stamp = Utc.with_ymd_and_hms(2020, 1, 6, 0,0,0).single().unwrap().timestamp();
        let end_stamp = Utc.with_ymd_and_hms(2020, 1, 13,0,0,0).single().unwrap().timestamp();

        let excluded_dates = vec!["2020-01-06", "*-01-13"].iter().map(|s| s.to_string()).collect();
        let difference = find_difference(start_stamp, end_stamp, vec![1, 2, 3, 4, 5], excluded_dates);
        assert_eq!(difference, 4);
    }
}