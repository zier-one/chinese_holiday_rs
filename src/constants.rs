use crate::{DayKind, Ymd};

pub const BASE_YEAR: u16 = 2000;

pub const SPECIAL_DAY_LIST: [(Ymd, DayKind); 28] = [
    // 2014 年部分节假日安排
    // https://zh.wikisource.org/zh-hans/国务院办公厅关于2004年部分节假日安排的通知
    // 一、元旦：1月1日放假。
    (Ymd::new(2004, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：1月22日———28日（即农历大年初一至初七）放假，共7天。
    //     其中，22日、23日、24日为法定假日，1月25日（星期日）照常公休，将1月17日（星期六）、18日（星期日）、24日（星期六）三个公休日
    //     调至1月26日（星期一）、27日（星期二）、28日（星期三），1月17日、18日上班。
    (Ymd::new(2004, 1, 17), DayKind::SpringFestivalWorkday),
    (Ymd::new(2004, 1, 18), DayKind::SpringFestivalWorkday),
    (Ymd::new(2004, 1, 22), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 23), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 25), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 26), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2004, 1, 28), DayKind::SpringFestivalHoliday),
    // 三、“五一”：5月1日———7日放假，共7天。
    //     其中，1日、2日、3日为法定假日，将5月1日（星期六）、2日（星期日）两个公休日调至5月4日（星期二）、5日（星期三），
    //     5月8日（星期六）、5月9日（星期日）两个公休日调至5月6日（星期四）、7日（星期五），5月8日、9日上班。
    (
        Ymd::new(2004, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 6),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 7),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2004, 5, 8),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2004, 5, 9),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 四、“十一”：10月1日———7日放假，共7天。
    //     其中，1日、2日、3日为法定假日，将10月2日（星期六）、3日（星期日）两个公休日调至10月4日（星期一）、5日（星期二），
    //     10月9日（星期六）、10日（星期日）两个公休日调至10月6日（星期三）、7日（星期四），10月9日、10日上班。
    (Ymd::new(2004, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2004, 10, 9), DayKind::NationalDayWorkday),
    (Ymd::new(2004, 10, 10), DayKind::NationalDayWorkday),
];

#[cfg(test)]
mod tests {
    use crate::Ymd;

    #[test]
    fn make_sure_special_day_list_is_sorted() {
        use super::SPECIAL_DAY_LIST;
        let mut last = Ymd::new(2000, 1, 1);
        for (ymd, _) in SPECIAL_DAY_LIST.iter() {
            assert!(last < *ymd);
            last = *ymd;
        }
    }
}
