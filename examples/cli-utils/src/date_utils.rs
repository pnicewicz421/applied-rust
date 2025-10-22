//! Date and time manipulation utility functions
//! 
//! This module provides date and time manipulation functionalities including:
//! - Date difference calculation
//! - Date format validation
//! - Date formatting in different styles

use chrono::{Duration, Local, NaiveDate};

/// Calculates the difference between two dates in days
/// 
/// # Arguments
/// * `date1` - The first date in YYYY-MM-DD format
/// * `date2` - The second date in YYYY-MM-DD format
/// 
/// # Returns
/// The number of days between the dates (positive if date1 > date2)
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::date_difference_days;
/// assert_eq!(date_difference_days("2023-01-10", "2023-01-05").unwrap(), 5);
/// assert_eq!(date_difference_days("2023-01-05", "2023-01-10").unwrap(), -5);
/// ```
pub fn date_difference_days(date1: &str, date2: &str) -> Result<i64, chrono::ParseError> {
    let d1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d")?;
    let d2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d")?;
    Ok((d1 - d2).num_days())
}

/// Validates if a date string matches a specific format
/// 
/// # Arguments
/// * `date_str` - The date string to validate
/// * `format` - The expected format (e.g., "%Y-%m-%d", "%d/%m/%Y")
/// 
/// # Returns
/// `true` if the date string matches the format, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::validate_date_format;
/// assert_eq!(validate_date_format("2023-12-25", "%Y-%m-%d"), true);
/// assert_eq!(validate_date_format("25/12/2023", "%d/%m/%Y"), true);
/// assert_eq!(validate_date_format("2023-13-25", "%Y-%m-%d"), false);
/// ```
pub fn validate_date_format(date_str: &str, format: &str) -> bool {
    NaiveDate::parse_from_str(date_str, format).is_ok()
}

/// Formats a date string from one format to another
/// 
/// # Arguments
/// * `date_str` - The input date string
/// * `input_format` - The format of the input date
/// * `output_format` - The desired output format
/// 
/// # Returns
/// The reformatted date string or an error
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::format_date;
/// assert_eq!(format_date("2023-12-25", "%Y-%m-%d", "%d/%m/%Y").unwrap(), "25/12/2023");
/// assert_eq!(format_date("25/12/2023", "%d/%m/%Y", "%Y-%m-%d").unwrap(), "2023-12-25");
/// ```
pub fn format_date(date_str: &str, input_format: &str, output_format: &str) -> Result<String, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(date_str, input_format)?;
    Ok(date.format(output_format).to_string())
}

/// Converts a date to DD/MM/YYYY format
/// 
/// # Arguments
/// * `date_str` - The input date string in YYYY-MM-DD format
/// 
/// # Returns
/// The date in DD/MM/YYYY format or an error
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::to_dd_mm_yyyy;
/// assert_eq!(to_dd_mm_yyyy("2023-12-25").unwrap(), "25/12/2023");
/// ```
pub fn to_dd_mm_yyyy(date_str: &str) -> Result<String, chrono::ParseError> {
    format_date(date_str, "%Y-%m-%d", "%d/%m/%Y")
}

/// Converts a date to YYYY-MM-DD format
/// 
/// # Arguments
/// * `date_str` - The input date string in DD/MM/YYYY format
/// 
/// # Returns
/// The date in YYYY-MM-DD format or an error
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::to_yyyy_mm_dd;
/// assert_eq!(to_yyyy_mm_dd("25/12/2023").unwrap(), "2023-12-25");
/// ```
pub fn to_yyyy_mm_dd(date_str: &str) -> Result<String, chrono::ParseError> {
    format_date(date_str, "%d/%m/%Y", "%Y-%m-%d")
}

/// Gets the current date in the specified format
/// 
/// # Arguments
/// * `format` - The desired format string
/// 
/// # Returns
/// The current date formatted as requested
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::current_date;
/// let date = current_date("%Y-%m-%d");
/// // Will return something like "2023-12-25"
/// assert!(date.len() == 10); // YYYY-MM-DD format
/// ```
pub fn current_date(format: &str) -> String {
    Local::now().format(format).to_string()
}

/// Adds days to a date
/// 
/// # Arguments
/// * `date_str` - The input date string in YYYY-MM-DD format
/// * `days` - The number of days to add (can be negative to subtract)
/// 
/// # Returns
/// The new date string or an error
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::add_days;
/// assert_eq!(add_days("2023-12-25", 7).unwrap(), "2024-01-01");
/// assert_eq!(add_days("2023-12-25", -5).unwrap(), "2023-12-20");
/// ```
pub fn add_days(date_str: &str, days: i64) -> Result<String, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let new_date = date + Duration::days(days);
    Ok(new_date.format("%Y-%m-%d").to_string())
}

/// Checks if a year is a leap year
/// 
/// # Arguments
/// * `year` - The year to check
/// 
/// # Returns
/// `true` if the year is a leap year, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::is_leap_year;
/// assert_eq!(is_leap_year(2024), true);
/// assert_eq!(is_leap_year(2023), false);
/// assert_eq!(is_leap_year(2000), true);
/// assert_eq!(is_leap_year(1900), false);
/// ```
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Gets the day of the week for a given date
/// 
/// # Arguments
/// * `date_str` - The date string in YYYY-MM-DD format
/// 
/// # Returns
/// The day of the week as a string or an error
/// 
/// # Examples
/// ```
/// use cli_utils::date_utils::day_of_week;
/// assert_eq!(day_of_week("2023-12-25").unwrap(), "Monday");
/// ```
pub fn day_of_week(date_str: &str) -> Result<String, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    Ok(date.format("%A").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_difference_days() {
        assert_eq!(date_difference_days("2023-01-10", "2023-01-05").unwrap(), 5);
        assert_eq!(date_difference_days("2023-01-05", "2023-01-10").unwrap(), -5);
        assert_eq!(date_difference_days("2023-01-01", "2023-01-01").unwrap(), 0);
    }

    #[test]
    fn test_validate_date_format() {
        assert_eq!(validate_date_format("2023-12-25", "%Y-%m-%d"), true);
        assert_eq!(validate_date_format("25/12/2023", "%d/%m/%Y"), true);
        assert_eq!(validate_date_format("2023-13-25", "%Y-%m-%d"), false);
        assert_eq!(validate_date_format("invalid", "%Y-%m-%d"), false);
    }

    #[test]
    fn test_format_date() {
        assert_eq!(format_date("2023-12-25", "%Y-%m-%d", "%d/%m/%Y").unwrap(), "25/12/2023");
        assert_eq!(format_date("25/12/2023", "%d/%m/%Y", "%Y-%m-%d").unwrap(), "2023-12-25");
    }

    #[test]
    fn test_to_dd_mm_yyyy() {
        assert_eq!(to_dd_mm_yyyy("2023-12-25").unwrap(), "25/12/2023");
    }

    #[test]
    fn test_to_yyyy_mm_dd() {
        assert_eq!(to_yyyy_mm_dd("25/12/2023").unwrap(), "2023-12-25");
    }

    #[test]
    fn test_current_date() {
        let date = current_date("%Y-%m-%d");
        assert!(date.len() == 10); // YYYY-MM-DD format
        assert!(validate_date_format(&date, "%Y-%m-%d"));
    }

    #[test]
    fn test_add_days() {
        assert_eq!(add_days("2023-12-25", 7).unwrap(), "2024-01-01");
        assert_eq!(add_days("2023-12-25", -5).unwrap(), "2023-12-20");
        assert_eq!(add_days("2023-12-25", 0).unwrap(), "2023-12-25");
    }

    #[test]
    fn test_is_leap_year() {
        assert_eq!(is_leap_year(2024), true);
        assert_eq!(is_leap_year(2023), false);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(2004), true);
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week("2023-12-25").unwrap(), "Monday");
        assert_eq!(day_of_week("2024-01-01").unwrap(), "Monday");
    }
}