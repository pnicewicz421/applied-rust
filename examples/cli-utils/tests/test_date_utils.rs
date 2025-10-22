use cli_utils::date_utils::*;

#[test]
fn test_date_difference_integration() {
    assert_eq!(date_difference_days("2023-01-10", "2023-01-05").unwrap(), 5);
    assert_eq!(date_difference_days("2023-01-05", "2023-01-10").unwrap(), -5);
    assert_eq!(date_difference_days("2023-12-25", "2023-12-25").unwrap(), 0);
}

#[test]
fn test_date_validation_integration() {
    assert_eq!(validate_date_format("2023-12-25", "%Y-%m-%d"), true);
    assert_eq!(validate_date_format("25/12/2023", "%d/%m/%Y"), true);
    assert_eq!(validate_date_format("2023-13-25", "%Y-%m-%d"), false);
    assert_eq!(validate_date_format("invalid-date", "%Y-%m-%d"), false);
}

#[test]
fn test_date_formatting_integration() {
    assert_eq!(format_date("2023-12-25", "%Y-%m-%d", "%d/%m/%Y").unwrap(), "25/12/2023");
    assert_eq!(format_date("25/12/2023", "%d/%m/%Y", "%Y-%m-%d").unwrap(), "2023-12-25");
}

#[test]
fn test_date_conversion_integration() {
    assert_eq!(to_dd_mm_yyyy("2023-12-25").unwrap(), "25/12/2023");
    assert_eq!(to_yyyy_mm_dd("25/12/2023").unwrap(), "2023-12-25");
}

#[test]
fn test_date_arithmetic_integration() {
    assert_eq!(add_days("2023-12-25", 7).unwrap(), "2024-01-01");
    assert_eq!(add_days("2023-12-25", -5).unwrap(), "2023-12-20");
    assert_eq!(add_days("2023-12-31", 1).unwrap(), "2024-01-01");
}

#[test]
fn test_leap_year_integration() {
    assert_eq!(is_leap_year(2024), true);
    assert_eq!(is_leap_year(2023), false);
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(1900), false);
}

#[test]
fn test_day_of_week_integration() {
    // December 25, 2023 was a Monday
    assert_eq!(day_of_week("2023-12-25").unwrap(), "Monday");
    // January 1, 2024 was also a Monday
    assert_eq!(day_of_week("2024-01-01").unwrap(), "Monday");
}