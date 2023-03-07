fn main() {
    use chinese_holiday::*;

    assert_eq!(
        chinese_holiday(Ymd::new(2004, 1, 1)),
        DayKind::NewYearsDayHoliday
    );
    assert!(chinese_holiday(Ymd::new(2004, 1, 1)).is_holiday());

    assert_eq!(
        chinese_holiday(Ymd::new(2004, 5, 8)),
        DayKind::InternationalWorkersDayWorkday
    );
    assert!(chinese_holiday(Ymd::new(2004, 5, 8)).is_workday());
}
