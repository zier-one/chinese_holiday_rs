#[allow(deprecated)]
fn main() {
    use chinese_holiday::*;
    use chrono::*;
    use chrono_tz::Asia::Shanghai;

    let date = NaiveDate::from_ymd_opt(2004, 1, 1).unwrap();
    assert_eq!(chinese_holiday(&date), DayKind::NewYearsDayHoliday);
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    assert_eq!(chinese_holiday(&datetime), DayKind::NewYearsDayHoliday);

    let date = Shanghai.ymd(2004, 5, 8);
    assert_eq!(
        chinese_holiday(&date),
        DayKind::InternationalWorkersDayWorkday
    );
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    assert_eq!(
        chinese_holiday(&datetime),
        DayKind::InternationalWorkersDayWorkday
    );
}
