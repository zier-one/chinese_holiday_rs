use crate::constants::BASE_YEAR;

/// [`Ymd`] is a packed representation of a year, month and day.
/// It uses a u16 to store the year, month and day, which can represent dates
/// between 2000-01-01 and 2127-12-31.
///
/// [`Ymd`] 是年月日的压缩表示。
/// 它使用一个 [`u16`] 来存储年月日，可以表达 `2000-01-01` 到 `2127-12-31` 之间的日期。
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Ymd(u16);

impl Ymd {
    /// Create a new [`Ymd`] from the given year, month and day.
    /// Note that the input parameters are not checked for validity.
    /// For example, `2020-02-31` is a invalid date,
    /// but it can be created successfully.
    ///
    /// 从给定的年月日创建一个新的 [`Ymd`]。
    /// 注意：输入参数不会被校验是否有效。
    /// 例如，即便 `2020-02-31` 是一个无效的日期，但是它仍然可以被成功创建。
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        assert!(
            year >= BASE_YEAR && year < BASE_YEAR + 128,
            "year out of range"
        );
        assert!(month >= 1 && month <= 12, "month out of range");
        assert!(day >= 1 && day <= 31, "day out of range");
        let mut ymd = (year - BASE_YEAR) << 9;
        ymd |= ((month & 0x0F) as u16) << 5;
        ymd |= (day & 0x1F) as u16;
        Ymd(ymd)
    }

    /// Unpack the [`Ymd`] into a tuple of year, month and day.
    ///
    /// 将 [`Ymd`] 解包为一个包含年月日的元组。
    pub const fn unpack(self) -> (u16, u8, u8) {
        let ymd = self.0;
        let year = BASE_YEAR + ((ymd >> 9) & 0x7F);
        let month = ((ymd >> 5) & 0x0F) as u8;
        let day = (ymd & 0x1F) as u8;
        (year, month, day)
    }
}

impl From<(u16, u8, u8)> for Ymd {
    fn from((year, month, day): (u16, u8, u8)) -> Self {
        Self::new(year, month, day)
    }
}

#[cfg(feature = "chrono-compatible")]
use chrono::*;
#[cfg(feature = "chrono-compatible")]
use chrono_tz::Asia::Shanghai;

#[cfg(feature = "chrono-compatible")]
#[allow(deprecated)]
impl<Tz: TimeZone> From<&Date<Tz>> for Ymd {
    fn from(date: &Date<Tz>) -> Self {
        let date = date.with_timezone(&Shanghai);
        Self::new(date.year() as u16, date.month() as u8, date.day() as u8)
    }
}

#[cfg(feature = "chrono-compatible")]
impl<Tz: TimeZone> From<&DateTime<Tz>> for Ymd {
    fn from(date: &DateTime<Tz>) -> Self {
        let date = date.with_timezone(&Shanghai);
        Self::new(date.year() as u16, date.month() as u8, date.day() as u8)
    }
}

#[cfg(feature = "chrono-compatible")]
impl From<&NaiveDate> for Ymd {
    fn from(date: &NaiveDate) -> Self {
        Self::new(date.year() as u16, date.month() as u8, date.day() as u8)
    }
}

#[cfg(feature = "chrono-compatible")]
impl From<&NaiveDateTime> for Ymd {
    fn from(date: &NaiveDateTime) -> Self {
        Self::new(date.year() as u16, date.month() as u8, date.day() as u8)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_ymd() {
        let cases = [(2020, 2, 16), (2000, 1, 1), (2127, 12, 31)];
        for c in cases {
            let ymd = Ymd::new(c.0, c.1, c.2);
            assert_eq!(ymd.unpack(), (c.0, c.1, c.2));
            assert_eq!(ymd, Ymd::from((c.0, c.1, c.2)));
        }
    }

    #[test]
    #[cfg(feature = "chrono-compatible")]
    #[allow(deprecated)]
    fn test_from_chorno() {
        use chrono_tz::Asia::Shanghai;
        let cases = [(2020, 2, 16), (2000, 1, 1), (2127, 12, 31)];
        for c in cases {
            let ymd = Ymd::new(c.0, c.1, c.2);
            assert_eq!(
                ymd,
                Ymd::from(&NaiveDate::from_ymd_opt(c.0 as i32, c.1 as u32, c.2 as u32).unwrap())
            );
            assert_eq!(
                ymd,
                Ymd::from(
                    &NaiveDate::from_ymd_opt(c.0 as i32, c.1 as u32, c.2 as u32)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                )
            );
            assert_eq!(
                ymd,
                Ymd::from(&Shanghai.ymd(c.0 as i32, c.1 as u32, c.2 as u32))
            );
            assert_eq!(
                ymd,
                Ymd::from(
                    &Shanghai
                        .with_ymd_and_hms(c.0 as i32, c.1 as u32, c.2 as u32, 0, 0, 0)
                        .unwrap()
                )
            );
        }
    }

    #[test]
    #[should_panic(expected = "year out of range")]
    fn test_ymd_panic_1() {
        Ymd::new(1999, 12, 31);
    }

    #[test]
    #[should_panic(expected = "year out of range")]
    fn test_ymd_panic_2() {
        Ymd::new(2128, 1, 1);
    }
}
