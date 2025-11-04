//! # The Chinese Holiday Library
//!
//! ## English Version
//! `chinese_holiday` is a Rust library for determining Chinese holidays. It can
//! determine whether a given date is a holiday, and the type of holiday.
//!
//! ### Features
//!
//! - `chrono-compatible`: Enable this feature to make the library compatible with
//!  the [`chrono`](https://crates.io/crates/chrono) crate.
//!  This feature is enabled by default.
//!
//! ### Example
//!
//! ```rust
#![doc = include_str!("../examples/simple.rs")]
//! ```
//!
//! The library is compatible with the [`chrono`](https://crates.io/crates/chrono),
//! for example:
//!
#![cfg_attr(not(feature = "chrono-compatible"), doc = " ```ignore")]
#![cfg_attr(feature = "chrono-compatible", doc = " ```rust")]
#![doc = include_str!("../examples/chrono-compatible.rs")]
//! ```
//!
//! ## 中文版说明
//! `chinese_holiday` 是一个用于判断中国节假日的 Rust 库，可以判断给定日期是否是节假日，
//! 以及节假日的类型。
//!
//! ### Features
//!
//! - `chrono-compatible`: 启用此特性可以使此库与
//!  [`chrono`](https://crates.io/crates/chrono) crate 兼容。此特性默认启用。
//!
//! ### 示例
//!
//! ```rust
#![doc = include_str!("../examples/simple.rs")]
//! ```
//!
//! 本库与 [`chrono`](https://crates.io/crates/chrono) crate 兼容，例如：
//!
#![cfg_attr(not(feature = "chrono-compatible"), doc = " ```ignore")]
#![cfg_attr(feature = "chrono-compatible", doc = " ```rust")]
#![doc = include_str!("../examples/chrono-compatible.rs")]
//! ```

mod constants;
mod day_kind;
mod ymd;

pub use day_kind::DayKind;
pub use ymd::Ymd;

use constants::SPECIAL_DAY_LIST;

/// Return the [`DayKind`] of the given date. According to [`DayKind`], you can
/// determine whether this day is a workday or a holiday.
///
/// 返回给定日期的 [`DayKind`]，根据 [`DayKind`] 可以判断这一天是工作日还是节假日。
pub fn chinese_holiday<T: Into<Ymd>>(ymd: T) -> DayKind {
    let ymd = ymd.into();
    assert!(
        ymd >= Ymd::new(2004, 1, 1) && ymd <= Ymd::new(2026, 12, 25),
        "The library only supports dates from 2004-01-01 to 2026-12-25"
    );
    match SPECIAL_DAY_LIST.binary_search_by_key(&ymd, |(ymd, _)| *ymd) {
        Ok(i) => SPECIAL_DAY_LIST[i].1,
        Err(_) => {
            let (year, month, day) = ymd.unpack();
            match weekday(year, month, day) {
                5 | 6 => DayKind::NormalHoliday,
                _ => DayKind::NormalWorkday,
            }
        }
    }
}

/// Calculate the weekday of the given date.
/// Returns a day-of-week number starting from Monday = 0.
///
/// `w`:                | `Mon` | `Tue` | `Wed` | `Thu` | `Fri` | `Sat` | `Sun`
/// ------------------- | ----- | ----- | ----- | ----- | ----- | ----- | -----
/// `weekday(y, m, d)`: | 0     | 1     | 2     | 3     | 4     | 5     | 6
///
/// Reference: https://en.wikipedia.org/wiki/Zeller%27s_congruence
#[inline]
fn weekday(y: u16, m: u8, d: u8) -> u8 {
    if m <= 2 {
        return weekday(y - 1, m + 12, d);
    }
    let h = (d + (13 * (m + 1)) / 5) as u16 + y + y / 4 - y / 100 + y / 400 + 5;
    (h % 7) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;

    #[test]
    fn test_weekday() {
        for year in 2000..2128 {
            test_weekday_by_year(year);
        }
    }

    fn test_weekday_by_year(year: i32) {
        let mut case = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        while case < NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() {
            assert_eq!(
                weekday(case.year() as u16, case.month() as u8, case.day() as u8),
                case.weekday().num_days_from_monday() as u8
            );
            case += Duration::days(1);
        }
    }

    #[test]
    fn test_chinese_holiday() {
        let cases = [
            (Ymd::new(2004, 1, 1), DayKind::NewYearsDayHoliday),
            (Ymd::new(2004, 1, 22), DayKind::SpringFestivalHoliday),
            (Ymd::new(2004, 10, 10), DayKind::NationalDayWorkday),
            (Ymd::new(2004, 12, 31), DayKind::NormalWorkday),
            (Ymd::new(2024, 10, 6), DayKind::NationalDayHoliday),
            (Ymd::new(2024, 12, 20), DayKind::NormalWorkday),
            (Ymd::new(2025, 10, 11), DayKind::NationalDayWorkday),
            (Ymd::new(2026, 9, 20), DayKind::NationalDayWorkday),
        ];
        for c in cases {
            assert_eq!(chinese_holiday(c.0), c.1);
        }
    }
}
