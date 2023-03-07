/// # English Version
/// [`DayKind`] is a enum type, used to represent whether a day is a holiday,
/// and which holiday it is.
///
/// ## Enum Value Description
///
/// ### Holiday Enum Value
/// - [`DayKind::NormalHoliday`]：Normal holiday, that is,
/// Saturday and Sunday without makeup workday.
/// - [`DayKind::NewYearsDayHoliday`]：New Year's Day holiday.
/// - [`DayKind::SpringFestivalHoliday`]：Spring Festival holiday.
/// - [`DayKind::ChingMingFestivalHoliday`]：Ching Ming Festival holiday.
/// - [`DayKind::InternationalWorkersDayHoliday`]：International Workers' Day
/// holiday.
/// - [`DayKind::DragonBoatFestivalHoliday`]：Dragon Boat Festival holiday.
/// - [`DayKind::MidAutumnFestivalHoliday`]：Mid-Autumn Festival holiday.
/// - [`DayKind::NationalDayHoliday`]：National Day holiday.
///
/// ### Workday Enum Value
/// - [`DayKind::NormalWorkday`]：Normal workday, that is, Monday to Friday
/// without statutory holiday.
/// - [`DayKind::NewYearsDayWorkday`]：New Year's Day makeup workday.
/// - [`DayKind::SpringFestivalWorkday`]：Spring Festival makeup workday.
/// - [`DayKind::ChingMingFestivalWorkday`]：Ching Ming Festival makeup workday.
/// - [`DayKind::InternationalWorkersDayWorkday`]：International Workers' Day
/// makeup workday.
/// - [`DayKind::DragonBoatFestivalWorkday`]：Dragon Boat Festival makeup workday.
/// - [`DayKind::MidAutumnFestivalWorkday`]：Mid-Autumn Festival makeup workday.
/// - [`DayKind::NationalDayWorkday`]：National Day makeup workday.
///
///
/// # 中文版说明
/// [`DayKind`] 是一个枚举类型，用于表示某一天是否是假期，处于哪种假期。
///
/// ## 枚举值说明
///
/// ### 假期类枚举值
/// - [`DayKind::NormalHoliday`]：普通假期，即没有补班安排的星期六、星期日。
/// - [`DayKind::NewYearsDayHoliday`]：元旦假期。
/// - [`DayKind::SpringFestivalHoliday`]：春节假期。
/// - [`DayKind::ChingMingFestivalHoliday`]：清明节假期。
/// - [`DayKind::InternationalWorkersDayHoliday`]：国际劳动节假期。
/// - [`DayKind::DragonBoatFestivalHoliday`]：端午节假期。
/// - [`DayKind::MidAutumnFestivalHoliday`]：中秋节假期。
/// - [`DayKind::NationalDayHoliday`]：国庆节假期。
///
/// ### 工作日类枚举值
/// - [`DayKind::NormalWorkday`]：普通工作日，即没有调休安排的星期一至星期五。
/// - [`DayKind::NewYearsDayWorkday`]：元旦补班。
/// - [`DayKind::SpringFestivalWorkday`]：春节补班。
/// - [`DayKind::ChingMingFestivalWorkday`]：清明节补班。
/// - [`DayKind::InternationalWorkersDayWorkday`]：国际劳动节补班。
/// - [`DayKind::DragonBoatFestivalWorkday`]：端午节补班。
/// - [`DayKind::MidAutumnFestivalWorkday`]：中秋节补班。
/// - [`DayKind::NationalDayWorkday`]：国庆节补班。
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayKind {
    NormalWorkday,
    NormalHoliday,
    NewYearsDayHoliday,
    NewYearsDayWorkday,
    SpringFestivalHoliday,
    SpringFestivalWorkday,
    ChingMingFestivalHoliday,
    ChingMingFestivalWorkday,
    InternationalWorkersDayHoliday,
    InternationalWorkersDayWorkday,
    DragonBoatFestivalHoliday,
    DragonBoatFestivalWorkday,
    MidAutumnFestivalHoliday,
    MidAutumnFestivalWorkday,
    NationalDayHoliday,
    NationalDayWorkday,
}

impl DayKind {
    /// Returns whether the current [`DayKind`] is a workday.
    ///
    /// 返回当前的 [`DayKind`] 是否是工作日。
    pub fn is_workday(&self) -> bool {
        matches!(
            self,
            DayKind::NormalWorkday
                | DayKind::NewYearsDayWorkday
                | DayKind::SpringFestivalWorkday
                | DayKind::ChingMingFestivalWorkday
                | DayKind::InternationalWorkersDayWorkday
                | DayKind::DragonBoatFestivalWorkday
                | DayKind::MidAutumnFestivalWorkday
                | DayKind::NationalDayWorkday
        )
    }

    /// Returns whether the current [`DayKind`] is a holiday.
    ///
    /// 返回当前的 [`DayKind`] 是否是假期。
    pub fn is_holiday(&self) -> bool {
        matches!(
            self,
            DayKind::NormalHoliday
                | DayKind::NewYearsDayHoliday
                | DayKind::SpringFestivalHoliday
                | DayKind::ChingMingFestivalHoliday
                | DayKind::InternationalWorkersDayHoliday
                | DayKind::DragonBoatFestivalHoliday
                | DayKind::MidAutumnFestivalHoliday
                | DayKind::NationalDayHoliday
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day_kind() {
        use super::DayKind;
        assert_eq!(DayKind::NormalWorkday.is_workday(), true);
        assert_eq!(DayKind::NormalWorkday.is_holiday(), false);
        assert_eq!(DayKind::NormalHoliday.is_workday(), false);
        assert_eq!(DayKind::NormalHoliday.is_holiday(), true);
        assert_eq!(DayKind::NewYearsDayHoliday.is_workday(), false);
        assert_eq!(DayKind::NewYearsDayHoliday.is_holiday(), true);
        assert_eq!(DayKind::NewYearsDayWorkday.is_workday(), true);
        assert_eq!(DayKind::NewYearsDayWorkday.is_holiday(), false);
        assert_eq!(DayKind::SpringFestivalHoliday.is_workday(), false);
        assert_eq!(DayKind::SpringFestivalHoliday.is_holiday(), true);
        assert_eq!(DayKind::SpringFestivalWorkday.is_workday(), true);
        assert_eq!(DayKind::SpringFestivalWorkday.is_holiday(), false);
    }
}
