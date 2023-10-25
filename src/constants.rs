use crate::{DayKind, Ymd};

pub const BASE_YEAR: u16 = 2000;

pub const SPECIAL_DAY_LIST: [(Ymd, DayKind); 722] = [
    // 2004 年节假日安排
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
    //
    //
    // 2005 年节假日安排
    // 一、元旦：1月1日～3日放假，共3天。其中1月1日为法定假日，将1月1日(星期六)公休日调至1月3日(星期一)，1月2日(星期日)照常公休。
    (Ymd::new(2005, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2005, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2005, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月9日～15日(农历大年初一至初七)放假，共7天。其中，9日、10日、11日为法定假日，
    // 2月12日(星期六)、13日(星期日)照常公休，将2月5日(星期六)、6日(星期日)两个公休日调至2月14日(星期一)、15日(星期二)，
    // 2月5日、6日上班。
    (Ymd::new(2005, 2, 5), DayKind::SpringFestivalWorkday),
    (Ymd::new(2005, 2, 6), DayKind::SpringFestivalWorkday),
    (Ymd::new(2005, 2, 9), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 10), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 12), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 14), DayKind::SpringFestivalHoliday),
    (Ymd::new(2005, 2, 15), DayKind::SpringFestivalHoliday),
    // 三、“五一”：5月1日～7日放假，共7天。其中，1日、2日、3日为法定假日，将4月30日(星期六)、5月1日(星期日)、8日(星期日)三个公休日
    // 调至5月4日(星期三)、5日(星期四)、6日(星期五)，5月7日(星期六)照常公休，4月30日、5月8日上班。
    (
        Ymd::new(2005, 4, 30),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2005, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 6),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 7),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2005, 5, 8),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 四、“十一”：10月1日～7日放假，共7天。其中，1日、2日、3日为法定假日，将10月1日(星期六)、2日(星期日)两个公休日
    // 调至10月4日(星期二)、5日(星期三)，10月8日(星期六)、9日(星期日)两个公休日调至10月6日(星期四)、7日(星期五)，10月8日、9日上班。
    (Ymd::new(2005, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2005, 10, 8), DayKind::NationalDayWorkday),
    (Ymd::new(2005, 10, 9), DayKind::NationalDayWorkday),
    // 2006 年节假日安排
    // http://www.gov.cn/jrzg/2005-12/22/content_133837.htm
    // 一、元旦：1月1日—3日放假，共3天。
    // 其中1月1日为法定假日，将12月31日(星期六)、1月1日(星期日)两个公休日调至1月2日(星期一)、3日(星期二)，12月31日(星期六)上班。
    (Ymd::new(2005, 12, 31), DayKind::NewYearsDayWorkday),
    (Ymd::new(2006, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2006, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2006, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：1月29日—2月4日(即农历大年初一至初七)放假，共7天。
    // 其中，29日、30日、31日为法定假日，将1月28日(星期六)、29日(星期日)、2月5日(星期日)三个公休日调至2月1日(星期三)、2日(星期四)、3日(星期五)，2月4日(星期六)照常公休，1月28日、2月5日上班。
    (Ymd::new(2006, 1, 28), DayKind::SpringFestivalWorkday),
    (Ymd::new(2006, 1, 29), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 1, 30), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 2, 1), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 2, 2), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 2, 3), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 2, 4), DayKind::SpringFestivalHoliday),
    (Ymd::new(2006, 2, 5), DayKind::SpringFestivalWorkday),
    // 三、“五一”：5月1日—7日放假，共7天。
    // 其中，1日、2日、3日为法定假日，将4月29日(星期六)、30日(星期日)两个公休日调至5月4日(星期四)、5日(星期五)，5月6日(星期六)、7日(星期日)照常公休，4月29日、30日上班。
    (
        Ymd::new(2006, 4, 29),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2006, 4, 30),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2006, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 6),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2006, 5, 7),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 四、“十一”：10月1日—7日放假，共7天。
    // 其中，1日、2日、3日为法定假日，将9月30日(星期六)、10月1日(星期日)、8日(星期日)三个公休日调至10月4日(星期三)、5日(星期四)、6日(星期五)，10月7日(星期六)照常公休，9月30日、10月8日上班。
    (Ymd::new(2006, 9, 30), DayKind::NationalDayWorkday),
    (Ymd::new(2006, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2006, 10, 8), DayKind::NationalDayWorkday),
    //
    //
    // 2007 年节假日安排
    // http://www.gov.cn/fwxx/sh/2006-12/18/content_471877.htm
    // 一、元旦： 1月1日－3日放假，共三天。
    // 其中1月1日为法定假日，将2006年12月30日（星期六）、31日（星期日）两个公休日分别调至2007年1月2日、3日，2006年12月30日（星期六）、12月31日（星期日）上班。
    (Ymd::new(2006, 12, 30), DayKind::NewYearsDayWorkday),
    (Ymd::new(2006, 12, 31), DayKind::NewYearsDayWorkday),
    (Ymd::new(2007, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2007, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2007, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月18日—24日（即农历初一至初七）放假，共7天。
    // 其中18日、19日、20日为法定假日，将17日（星期六）、18日（星期日）、25日（星期日）三个公休日分别调至21日（星期三）、22日（星期四）、23日（星期五）；24日（星期六）照常公休，17日、25日上班。
    (Ymd::new(2007, 2, 17), DayKind::SpringFestivalWorkday),
    (Ymd::new(2007, 2, 18), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 19), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 20), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 21), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 22), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 23), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2007, 2, 25), DayKind::SpringFestivalWorkday),
    // 三、“五一”：5月1日—7日放假，共7天。
    // 其中，1日、2日、3日为法定假日，将4月28日（星期六）、29日（星期日）两个公休日调至5月4日（星期五）、7日（星期一）；5月5日（星期六）、6日（星期日）照常公休，4月28日、29日上班。
    (
        Ymd::new(2007, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2007, 4, 29),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2007, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 6),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2007, 5, 7),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 四、“十一”：10月1日—7日放假，共7天。
    // 其中，1日、2日、3日为法定假日，将9月29日（星期六）、30日（星期日）两个公休日调至10月4日（星期四）、5日（星期五）；10月6日（星期六）、7日（星期日）照常公休，9月29日、30日上班。
    (Ymd::new(2007, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2007, 9, 30), DayKind::NationalDayWorkday),
    (Ymd::new(2007, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2007, 10, 7), DayKind::NationalDayHoliday),
    //
    //
    // 2008 年节假日安排
    // http://www.gov.cn/zwgk/2007-12/18/content_837184.htm
    // 一、元旦：2007年12月30日—2008年1月1日放假，共3天。
    // 其中，1月1日（星期二）为法定节假日，12月30日（星期日）为公休日，12月29日（星期六）公休日调至12月31日（星期一），12月29日（星期六）上班。
    (Ymd::new(2007, 12, 29), DayKind::NewYearsDayWorkday),
    (Ymd::new(2007, 12, 30), DayKind::NewYearsDayHoliday),
    (Ymd::new(2007, 12, 31), DayKind::NewYearsDayHoliday),
    (Ymd::new(2008, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：2月6日—12日（农历除夕至正月初六）放假，共7天。
    // 其中，2月6日（除夕）、2月7日（春节）、2月8日（正月初二）为法定节假日，2月9日（星期六）、2月10日（星期日）照常公休，2月2日（星期六）、2月3日（星期日）两个公休日调至2月11日（星期一）、2月12日（星期二），2月2
    // 日（星期六）、2月3日（星期日）上班。
    (Ymd::new(2008, 2, 2), DayKind::SpringFestivalWorkday),
    (Ymd::new(2008, 2, 3), DayKind::SpringFestivalWorkday),
    (Ymd::new(2008, 2, 6), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 7), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 8), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 9), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 10), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2008, 2, 12), DayKind::SpringFestivalHoliday),
    // 三、清明节：4月4日—6日放假，共3天。
    // 其中，4月4日（清明节）为法定节假日，4月5日（星期六）、4月6日（星期日）照常公休。
    (Ymd::new(2008, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2008, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2008, 4, 6), DayKind::ChingMingFestivalHoliday),
    // 四、“五一”国际劳动节：5月1日—3日放假，共3天。
    // 其中，5月1日为法定节假日，5月3日（星期六）为公休日，5月4日（星期日）公休日调至5月2日（星期五），5月4日（星期日）上班。
    (
        Ymd::new(2008, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2008, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2008, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2008, 5, 4),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月7日—9日放假，共3天。
    // 其中，6月7日（星期六）照常公休，6月8日（农历五月初五，端午节）为法定节假日，6月8日（星期日）公休日调至6月9日（星期一）。
    (Ymd::new(2008, 6, 7), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2008, 6, 8), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2008, 6, 9), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月13日—15日放假，共3天。
    // 其中，9月13日（星期六）为公休日，9月14日（农历八月十五，中秋节）为法定节假日，9月14日（星期日）公休日调至9月15日（星期一）。
    (Ymd::new(2008, 9, 13), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2008, 9, 14), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2008, 9, 15), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：9月29日—10月5日放假，共7天。
    // 其中，10月1日、2日、3日为法定节假日，9月27日（星期六）、9月28日（星期日）两个公休日调至9月29日（星期一）、30日（星期二），10月4日（星期六）、5日（星期日）照常公休。
    (Ymd::new(2008, 9, 27), DayKind::NationalDayWorkday),
    (Ymd::new(2008, 9, 28), DayKind::NationalDayWorkday),
    (Ymd::new(2008, 9, 29), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 9, 30), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2008, 10, 5), DayKind::NationalDayHoliday),
    //
    //
    // 2009 年节假日安排
    // http://www.gov.cn/zwgk/2008-12/10/content_1174014.htm
    // 一、元旦：1月1日至3日放假，共3天。
    // 其中，1月1日（星期四、新年）为法定节假日，1月3日（星期六）为公休日。
    // 1月4日（星期日）公休日调至1月2日（星期五）。
    // 1月4日（星期日）上班。
    (Ymd::new(2009, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2009, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2009, 1, 3), DayKind::NewYearsDayHoliday),
    (Ymd::new(2009, 1, 4), DayKind::NewYearsDayWorkday),
    // 二、春节：1月25日至31日放假，共7天。
    // 其中，1月25日（星期日、农历除夕）、1月26日（星期一、农历正月初一）、1月27日（星期二、农历正月初二）为法定节假日，1月31日（星期六）照常公休；1月25日（星期日）公休日调至1月28日（星期三），1月24日（星期六）、2月1
    // 日（星期日）两个公休日调至1月29日（星期四）、1月30日（星期五）。
    // 1月24日（星期六）、2月1日（星期日）上班。
    (Ymd::new(2009, 1, 24), DayKind::SpringFestivalWorkday),
    (Ymd::new(2009, 1, 25), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 26), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 28), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 29), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 30), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2009, 2, 1), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月4日至6日放假，共3天。
    // 其中，4月4日（星期六、农历清明当日）为法定节假日，4月5日（星期日）照常公休。
    // 4月4日（星期六）公休日调至4月6日（星期一）。
    (Ymd::new(2009, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2009, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2009, 4, 6), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至3日放假，共3天。
    // 其中，5月1日（星期五、“五一”国际劳动节）为法定节假日，5月2日（星期六）、5月3日（星期日）照常公休。
    (
        Ymd::new(2009, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2009, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2009, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：5月28日至30日放假，共3天。
    // 其中，5月28日（星期四、农历端午当日）为法定节假日，5月30日（星期六）照常公休；5月31日（星期日）公休日调至5月29日（星期五）。
    // 5月31日（星期日）上班。
    (Ymd::new(2009, 5, 28), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2009, 5, 29), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2009, 5, 30), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2009, 5, 31), DayKind::DragonBoatFestivalWorkday),
    // 六、国庆节、中秋节：10月1日至8日放假，共8天。
    // 其中，10月1日（星期四）、10月2日（星期五）、10月3日（星期六）为国庆节法定节假日，10月4日（星期日）照常公休；10月3日（星期六）公休日及中秋节分别调至10月5日（星期一）、10月6日（星期二），9月27日（星期日）、10
    // 月10日（星期六）公休日调至10月7日（星期三）、10月8日（星期四）。
    // 9月27日（星期日）、10月10日（星期六）上班。
    (Ymd::new(2009, 9, 27), DayKind::NationalDayWorkday),
    (Ymd::new(2009, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 3), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2009, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 8), DayKind::NationalDayHoliday),
    (Ymd::new(2009, 10, 10), DayKind::NationalDayWorkday),
    //
    //
    // 2010 年节假日安排
    // http://www.gov.cn/zwgk/2009-12/08/content_1482691.htm
    // 一、元旦：1月1日至3日放假公休，共3天。
    (Ymd::new(2010, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2010, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2010, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月13日至19日放假调休，共7天。2月20日（星期六）、21日（星期日）上班。
    (Ymd::new(2010, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 14), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 15), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 16), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 17), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 18), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 19), DayKind::SpringFestivalHoliday),
    (Ymd::new(2010, 2, 20), DayKind::SpringFestivalWorkday),
    (Ymd::new(2010, 2, 21), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月3日至5日放假公休，共3天。
    (Ymd::new(2010, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2010, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2010, 4, 5), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至3日放假公休，共3天。
    (
        Ymd::new(2010, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2010, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2010, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月14日至16日放假调休，共3天。6月12日（星期六）、13日（星期日）上班。
    (Ymd::new(2010, 6, 12), DayKind::DragonBoatFestivalWorkday),
    (Ymd::new(2010, 6, 13), DayKind::DragonBoatFestivalWorkday),
    (Ymd::new(2010, 6, 14), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2010, 6, 15), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2010, 6, 16), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月22日至24日放假调休，共3天。9月19日（星期日）、25日（星期六）上班。
    (Ymd::new(2010, 9, 19), DayKind::MidAutumnFestivalWorkday),
    (Ymd::new(2010, 9, 22), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2010, 9, 23), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2010, 9, 24), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2010, 9, 25), DayKind::MidAutumnFestivalWorkday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月26日（星期日）、10月9日（星期六）上班。
    (Ymd::new(2010, 9, 26), DayKind::NationalDayWorkday),
    (Ymd::new(2010, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2010, 10, 9), DayKind::NationalDayWorkday),
    //
    //
    // 2011 年节假日安排
    // http://www.gov.cn/zwgk/2010-12/10/content_1762643.htm
    // 一、元旦：1月1日至3日放假公休，共3天。
    (Ymd::new(2011, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2011, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2011, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月2日（农历除夕）至8日放假调休，共7天。1月30日（星期日）、2月12日（星期六）上班。
    (Ymd::new(2011, 1, 30), DayKind::SpringFestivalWorkday),
    (Ymd::new(2011, 2, 2), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 3), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 4), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 5), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 6), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 7), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 8), DayKind::SpringFestivalHoliday),
    (Ymd::new(2011, 2, 12), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月3日至5日放假调休，共3天。4月2日（星期六）上班。
    (Ymd::new(2011, 4, 2), DayKind::ChingMingFestivalWorkday),
    (Ymd::new(2011, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2011, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2011, 4, 5), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：4月30日至5月2日放假公休，共3天。
    (
        Ymd::new(2011, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2011, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2011, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月4日至6日放假公休，共3天。
    (Ymd::new(2011, 6, 4), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2011, 6, 5), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2011, 6, 6), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月10日至12日放假公休，共3天。
    (Ymd::new(2011, 9, 10), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2011, 9, 11), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2011, 9, 12), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。10月8日（星期六）、10月9日（星期日）上班。
    (Ymd::new(2011, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2011, 10, 8), DayKind::NationalDayWorkday),
    (Ymd::new(2011, 10, 9), DayKind::NationalDayWorkday),
    //
    //
    // 2012 年节假日安排
    // http://www.gov.cn/zwgk/2011-12/06/content_2012097.htm
    // 一、元旦：2012年1月1日至3日放假调休，共3天。2011年12月31日（星期六）上班。
    (Ymd::new(2011, 12, 31), DayKind::NewYearsDayWorkday),
    (Ymd::new(2012, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2012, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2012, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：1月22日至28日放假调休，共7天。1月21日（星期六）、1月29日（星期日）上班。
    (Ymd::new(2012, 1, 21), DayKind::SpringFestivalWorkday),
    (Ymd::new(2012, 1, 22), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 23), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 25), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 26), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 28), DayKind::SpringFestivalHoliday),
    (Ymd::new(2012, 1, 29), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月2日至4日放假调休，共3天。3月31日（星期六）、4月1日（星期日）上班。
    (Ymd::new(2012, 3, 31), DayKind::ChingMingFestivalWorkday),
    (Ymd::new(2012, 4, 1), DayKind::ChingMingFestivalWorkday),
    (Ymd::new(2012, 4, 2), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2012, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2012, 4, 4), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：4月29日至5月1日放假调休，共3天。4月28日（星期六）上班。
    (
        Ymd::new(2012, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2012, 4, 29),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2012, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2012, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月22日至24日放假公休，共3天。
    (Ymd::new(2012, 6, 22), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2012, 6, 23), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2012, 6, 24), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节、国庆节：9月30日至10月7日放假调休，共8天。9月29日（星期六）上班。
    (Ymd::new(2012, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2012, 9, 30), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2012, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2012, 10, 7), DayKind::NationalDayHoliday),
    //
    //
    // 2013 年节假日安排
    // http://www.gov.cn/zwgk/2012-12/10/content_2286598.htm
    // 一、元旦：1月1日至3日放假调休，共3天。1月5日（星期六）、1月6日（星期日）上班。
    (Ymd::new(2013, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2013, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2013, 1, 3), DayKind::NewYearsDayHoliday),
    (Ymd::new(2013, 1, 5), DayKind::NewYearsDayWorkday),
    (Ymd::new(2013, 1, 6), DayKind::NewYearsDayWorkday),
    // 二、春节：2月9日至15日放假调休，共7天。2月16日（星期六）、2月17日（星期日）上班。
    (Ymd::new(2013, 2, 9), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 10), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 12), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 14), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 15), DayKind::SpringFestivalHoliday),
    (Ymd::new(2013, 2, 16), DayKind::SpringFestivalWorkday),
    (Ymd::new(2013, 2, 17), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月4日至6日放假调休，共3天。4月7日（星期日）上班。
    (Ymd::new(2013, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2013, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2013, 4, 6), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2013, 4, 7), DayKind::ChingMingFestivalWorkday),
    // 四、劳动节：4月29日至5月1日放假调休，共3天。4月27日（星期六）、4月28日（星期日）上班。
    (
        Ymd::new(2013, 4, 27),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2013, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2013, 4, 29),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2013, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2013, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月10日至12日放假调休，共3天。6月8日（星期六）、6月9日（星期日）上班。
    (Ymd::new(2013, 6, 8), DayKind::DragonBoatFestivalWorkday),
    (Ymd::new(2013, 6, 9), DayKind::DragonBoatFestivalWorkday),
    (Ymd::new(2013, 6, 10), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2013, 6, 11), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2013, 6, 12), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月19日至21日放假调休，共3天。9月22日（星期日）上班。
    (Ymd::new(2013, 9, 19), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2013, 9, 20), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2013, 9, 21), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2013, 9, 22), DayKind::MidAutumnFestivalWorkday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月29日（星期日）、10月12日（星期六）上班。
    (Ymd::new(2013, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2013, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2013, 10, 12), DayKind::NationalDayWorkday),
    //
    //
    // 2014 年节假日安排
    // http://www.gov.cn/zwgk/2013-12/11/content_2546204.htm
    // 一、元旦：1月1日放假1天。
    (Ymd::new(2014, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：1月31日至2月6日放假调休，共7天。1月26日（星期日）、2月8日（星期六）上班。
    (Ymd::new(2014, 1, 26), DayKind::SpringFestivalWorkday),
    (Ymd::new(2014, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 1), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 2), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 3), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 4), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 5), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 6), DayKind::SpringFestivalHoliday),
    (Ymd::new(2014, 2, 8), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月5日放假，4月7日（星期一）补休。
    (Ymd::new(2014, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2014, 4, 6), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2014, 4, 7), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至3日放假调休，共3天。5月4日（星期日）上班。
    (
        Ymd::new(2014, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2014, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2014, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2014, 5, 4),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月2日放假，与周末连休。
    (Ymd::new(2014, 5, 31), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2014, 6, 1), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2014, 6, 2), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月8日放假，与周末连休。
    (Ymd::new(2014, 9, 6), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2014, 9, 7), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2014, 9, 8), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月28日（星期日）、10月11日（星期六）上班。
    (Ymd::new(2014, 9, 28), DayKind::NationalDayWorkday),
    (Ymd::new(2014, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2014, 10, 11), DayKind::NationalDayWorkday),
    //
    //
    // 2015 年节假日安排
    // http://www.gov.cn/zhengce/content/2014-12/16/content_9302.htm
    // http://www.gov.cn/zhengce/content/2015-05/13/content_9742.htm
    // 一、元旦：1月1日至3日放假调休，共3天。1月4日（星期日）上班。
    (Ymd::new(2015, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2015, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2015, 1, 3), DayKind::NewYearsDayHoliday),
    (Ymd::new(2015, 1, 4), DayKind::NewYearsDayWorkday),
    // 二、春节：2月18日至24日放假调休，共7天。2月15日（星期日）、2月28日（星期六）上班。
    (Ymd::new(2015, 2, 15), DayKind::SpringFestivalWorkday),
    (Ymd::new(2015, 2, 18), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 19), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 20), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 21), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 22), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 23), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2015, 2, 28), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月5日放假，4月6日（星期一）补休。
    (Ymd::new(2015, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2015, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2015, 4, 6), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日放假，与周末连休。
    (
        Ymd::new(2015, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2015, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2015, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月20日放假，6月22日（星期一）补休。
    (Ymd::new(2015, 6, 20), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2015, 6, 21), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2015, 6, 22), DayKind::DragonBoatFestivalHoliday),
    // 八、参见《国务院关于中国人民抗日战争暨世界反法西斯战争胜利70周年纪念日调休放假的通知》
    // 额外的放假安排如下：
    // 9月3日至5日调休放假，共3天。其中9月3日（星期四）放假，9月4日（星期五）调休，9月6日（星期日）上班。
    (Ymd::new(2015, 9, 3), DayKind::OtherHoliday),
    (Ymd::new(2015, 9, 4), DayKind::OtherHoliday),
    (Ymd::new(2015, 9, 5), DayKind::OtherHoliday),
    (Ymd::new(2015, 9, 6), DayKind::OtherWorkday),
    // 六、中秋节：9月27日放假。
    (Ymd::new(2015, 9, 27), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。10月10日（星期六）上班。
    (Ymd::new(2015, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2015, 10, 10), DayKind::NationalDayWorkday),
    //
    //
    // 2016 年节假日安排
    // http://www.gov.cn/zhengce/content/2015-12/10/content_10394.htm
    // 一、元旦：1月1日放假，与周末连休。
    (Ymd::new(2016, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2016, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2016, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月7日至13日放假调休，共7天。2月6日（星期六）、2月14日（星期日）上班。
    (Ymd::new(2016, 2, 6), DayKind::SpringFestivalWorkday),
    (Ymd::new(2016, 2, 7), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 8), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 9), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 10), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 12), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2016, 2, 14), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月4日放假，与周末连休。
    (Ymd::new(2016, 4, 2), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2016, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2016, 4, 4), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日放假，5月2日（星期一）补休。
    (
        Ymd::new(2016, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2016, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2016, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月9日至11日放假调休，共3天。6月12日（星期日）上班。
    (Ymd::new(2016, 6, 9), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2016, 6, 10), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2016, 6, 11), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2016, 6, 12), DayKind::DragonBoatFestivalWorkday),
    // 六、中秋节：9月15日至17日放假调休，共3天。9月18日（星期日）上班。
    (Ymd::new(2016, 9, 15), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2016, 9, 16), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2016, 9, 17), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2016, 9, 18), DayKind::MidAutumnFestivalWorkday),
    // 七、国庆节：10月1日至7日放假调休，共7天。10月8日（星期六）、10月9日（星期日）上班。
    (Ymd::new(2016, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2016, 10, 8), DayKind::NationalDayWorkday),
    //
    //
    // 2017 年节假日安排
    // http://www.gov.cn/zhengce/content/2016-12/01/content_5141603.htm
    // 一、元旦：1月1日放假，1月2日（星期一）补休。
    (Ymd::new(2016, 12, 30), DayKind::NewYearsDayHoliday),
    (Ymd::new(2017, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2017, 1, 2), DayKind::NewYearsDayHoliday),
    // 二、春节：1月27日至2月2日放假调休，共7天。1月22日（星期日）、2月4日（星期六）上班。
    (Ymd::new(2017, 1, 22), DayKind::SpringFestivalWorkday),
    (Ymd::new(2017, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 1, 28), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 1, 29), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 1, 30), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 2, 1), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 2, 2), DayKind::SpringFestivalHoliday),
    (Ymd::new(2017, 2, 4), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月2日至4日放假调休，共3天。4月1日（星期六）上班。
    (Ymd::new(2017, 4, 1), DayKind::ChingMingFestivalWorkday),
    (Ymd::new(2017, 4, 2), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2017, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2017, 4, 4), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日放假，与周末连休。
    (
        Ymd::new(2017, 4, 29),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2017, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2017, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：5月28日至30日放假调休，共3天。5月27日（星期六）上班。
    (Ymd::new(2017, 5, 27), DayKind::DragonBoatFestivalWorkday),
    (Ymd::new(2017, 5, 28), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2017, 5, 29), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2017, 5, 30), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节、国庆节：10月1日至8日放假调休，共8天。9月30日（星期六）上班。
    (Ymd::new(2017, 9, 30), DayKind::NationalDayWorkday),
    (Ymd::new(2017, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 4), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2017, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2017, 10, 8), DayKind::NationalDayHoliday),
    //
    //
    // 2018 年节假日安排
    // http://www.gov.cn/zhengce/content/2017-11/30/content_5243579.htm
    // 一、元旦：1月1日放假，与周末连休。
    (Ymd::new(2017, 12, 30), DayKind::NewYearsDayHoliday),
    (Ymd::new(2017, 12, 31), DayKind::NewYearsDayHoliday),
    (Ymd::new(2018, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：2月15日至21日放假调休，共7天。2月11日（星期日）、2月24日（星期六）上班。
    (Ymd::new(2018, 2, 11), DayKind::SpringFestivalWorkday),
    (Ymd::new(2018, 2, 15), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 16), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 17), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 18), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 19), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 20), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 21), DayKind::SpringFestivalHoliday),
    (Ymd::new(2018, 2, 24), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月5日至7日放假调休，共3天。4月8日（星期日）上班。
    (Ymd::new(2018, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2018, 4, 6), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2018, 4, 7), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2018, 4, 8), DayKind::ChingMingFestivalWorkday),
    // 四、劳动节：4月29日至5月1日放假调休，共3天。4月28日（星期六）上班。
    (
        Ymd::new(2018, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2018, 4, 29),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2018, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2018, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    // 五、端午节：6月18日放假，与周末连休。
    (Ymd::new(2018, 6, 16), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2018, 6, 17), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2018, 6, 18), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月24日放假，与周末连休。
    (Ymd::new(2018, 9, 22), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2018, 9, 23), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2018, 9, 24), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月29日（星期六）、9月30日（星期日）上班。
    (Ymd::new(2018, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2018, 9, 30), DayKind::NationalDayWorkday),
    (Ymd::new(2018, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2018, 10, 7), DayKind::NationalDayHoliday),
    //
    //
    // 2019 年节假日安排
    // http://www.gov.cn/xinwen/2018-12/06/content_5346287.htm
    // http://www.gov.cn/zhengce/content/2019-03/22/content_5375877.htm
    // 一、元旦：12月30日至1月1日放假，共3天。 12月29日（星期六）上班。
    (Ymd::new(2018, 12, 29), DayKind::NewYearsDayWorkday),
    (Ymd::new(2018, 12, 30), DayKind::NewYearsDayHoliday),
    (Ymd::new(2018, 12, 31), DayKind::NewYearsDayHoliday),
    (Ymd::new(2019, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：2月4日至10日放假调休，共7天。2月2日（星期六）、2月3日（星期天）上班。
    (Ymd::new(2019, 2, 2), DayKind::SpringFestivalWorkday),
    (Ymd::new(2019, 2, 3), DayKind::SpringFestivalWorkday),
    (Ymd::new(2019, 2, 4), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 5), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 6), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 7), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 8), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 9), DayKind::SpringFestivalHoliday),
    (Ymd::new(2019, 2, 10), DayKind::SpringFestivalHoliday),
    // 三、清明节：4月5日放假，与周末连休。
    (Ymd::new(2019, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2019, 4, 6), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2019, 4, 7), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至4日放假调休，共4天。4月28日（星期日）、5月5日（星期日）上班。
    (
        Ymd::new(2019, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2019, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2019, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2019, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2019, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2019, 5, 5),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月7日放假，与周末连休。
    (Ymd::new(2019, 6, 7), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2019, 6, 8), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2019, 6, 9), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月13日放假，与周末连休。
    (Ymd::new(2019, 9, 13), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2019, 9, 14), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2019, 9, 15), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月29日（星期天）、10月12日（周六）上班。
    (Ymd::new(2019, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2019, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2019, 10, 12), DayKind::NationalDayWorkday),
    //
    //
    // 2020 年节假日安排
    // http://www.gov.cn/zhengce/content/2019-11/21/content_5454164.htm
    // http://www.gov.cn/zhengce/content/2020-01/27/content_5472352.htm
    // 一、元旦：2020年1月1日放假，共1天。
    (Ymd::new(2020, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：1月24日至2月2日放假调休，共10天。1月19日（星期日）上班。
    (Ymd::new(2020, 1, 19), DayKind::SpringFestivalWorkday),
    (Ymd::new(2020, 1, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 25), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 26), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 28), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 29), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 30), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 2, 1), DayKind::SpringFestivalHoliday),
    (Ymd::new(2020, 2, 2), DayKind::SpringFestivalHoliday),
    // 三、清明节：4月4日至6日放假调休，共3天。
    (Ymd::new(2020, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2020, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2020, 4, 6), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至5日放假调休，共5天。4月26日（星期日）、5月9日（星期六）上班。
    (
        Ymd::new(2020, 4, 26),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2020, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2020, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2020, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2020, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2020, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2020, 5, 9),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月25日至27日放假调休，共3天。6月28日（星期日）上班。
    (Ymd::new(2020, 6, 25), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2020, 6, 26), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2020, 6, 27), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2020, 6, 28), DayKind::DragonBoatFestivalWorkday),
    // 六、国庆节、中秋节：10月1日至8日放假调休，共8天。9月27日（星期日）、10月10日（星期六）上班。
    (Ymd::new(2020, 9, 27), DayKind::NationalDayWorkday),
    (Ymd::new(2020, 10, 1), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2020, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 8), DayKind::NationalDayHoliday),
    (Ymd::new(2020, 10, 10), DayKind::NationalDayWorkday),
    //
    //
    // 2021 年节假日安排
    // http://www.gov.cn/zhengce/content/2020-11/25/content_5564127.htm
    // 一、元旦：2021年1月1日至3日放假，共3天。
    (Ymd::new(2021, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2021, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2021, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：2月11日至17日放假调休，共7天。2月7日（星期日）、2月20日（星期六）上班。
    (Ymd::new(2021, 2, 7), DayKind::SpringFestivalWorkday),
    (Ymd::new(2021, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 12), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 14), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 15), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 16), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 17), DayKind::SpringFestivalHoliday),
    (Ymd::new(2021, 2, 20), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月3日至5日放假调休，共3天。
    (Ymd::new(2021, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2021, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2021, 4, 5), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：5月1日至5日放假调休，共5天。4月25日（星期日）、5月8日（星期六）上班。
    (
        Ymd::new(2021, 4, 25),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2021, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2021, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2021, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2021, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2021, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2021, 5, 8),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月12日至14日放假，共3天。
    (Ymd::new(2021, 6, 12), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2021, 6, 13), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2021, 6, 14), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月19日至21日放假调休，共3天。9月18日（星期六）上班。
    (Ymd::new(2021, 9, 18), DayKind::MidAutumnFestivalWorkday),
    (Ymd::new(2021, 9, 19), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2021, 9, 20), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2021, 9, 21), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月26日（星期日）、10月9日（星期六）上班。
    (Ymd::new(2021, 9, 26), DayKind::NationalDayWorkday),
    (Ymd::new(2021, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2021, 10, 9), DayKind::NationalDayWorkday),
    //
    //
    // 2022 年节假日安排
    // http://www.gov.cn/zhengce/content/2021-10/25/content_5644835.htm
    // 一、元旦：2022年1月1日至3日放假，共3天。
    (Ymd::new(2022, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2022, 1, 2), DayKind::NewYearsDayHoliday),
    (Ymd::new(2022, 1, 3), DayKind::NewYearsDayHoliday),
    // 二、春节：1月31日至2月6日放假调休，共7天。1月29日（星期六）、1月30日（星期日）上班。
    (Ymd::new(2022, 1, 29), DayKind::SpringFestivalWorkday),
    (Ymd::new(2022, 1, 30), DayKind::SpringFestivalWorkday),
    (Ymd::new(2022, 1, 31), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 1), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 2), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 3), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 4), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 5), DayKind::SpringFestivalHoliday),
    (Ymd::new(2022, 2, 6), DayKind::SpringFestivalHoliday),
    // 三、清明节：4月3日至5日放假调休，共3天。4月2日（星期六）上班。
    (Ymd::new(2022, 4, 2), DayKind::ChingMingFestivalWorkday),
    (Ymd::new(2022, 4, 3), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2022, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2022, 4, 5), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：4月30日至5月4日放假调休，共5天。4月24日（星期日）、5月7日（星期六）上班。
    (
        Ymd::new(2022, 4, 24),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2022, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2022, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2022, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2022, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2022, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2022, 5, 7),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月3日至5日放假，共3天。
    (Ymd::new(2022, 6, 3), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2022, 6, 4), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2022, 6, 5), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月10日至12日放假，共3天。
    (Ymd::new(2022, 9, 10), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2022, 9, 11), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2022, 9, 12), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。10月8日（星期六）、10月9日（星期日）上班。
    (Ymd::new(2022, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2022, 10, 8), DayKind::NationalDayWorkday),
    (Ymd::new(2022, 10, 9), DayKind::NationalDayWorkday),
    //
    //
    // 2023 年节假日安排
    // http://www.gov.cn/zhengce/content/2022-12/08/content_5730844.htm
    // 一、元旦：2022年12月31日至2023年1月2日放假调休，共3天。
    (Ymd::new(2022, 12, 31), DayKind::NewYearsDayHoliday),
    (Ymd::new(2023, 1, 1), DayKind::NewYearsDayHoliday),
    (Ymd::new(2023, 1, 2), DayKind::NewYearsDayHoliday),
    // 二、春节：1月21日至27日放假调休，共7天。1月28日（星期六）、1月29日（星期日）上班。
    (Ymd::new(2023, 1, 21), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 22), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 23), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 24), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 25), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 26), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 27), DayKind::SpringFestivalHoliday),
    (Ymd::new(2023, 1, 28), DayKind::SpringFestivalWorkday),
    (Ymd::new(2023, 1, 29), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月5日放假，共1天。
    (Ymd::new(2023, 4, 5), DayKind::ChingMingFestivalHoliday),
    // 四、劳动节：4月29日至5月3日放假调休，共5天。4月23日（星期日）、5月6日（星期六）上班。
    (
        Ymd::new(2023, 4, 23),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2023, 4, 29),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2023, 4, 30),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2023, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2023, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2023, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2023, 5, 6),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月22日至24日放假调休，共3天。6月25日（星期日）上班。
    (Ymd::new(2023, 6, 22), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2023, 6, 23), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2023, 6, 24), DayKind::DragonBoatFestivalHoliday),
    (Ymd::new(2023, 6, 25), DayKind::DragonBoatFestivalWorkday),
    // 六、中秋节、国庆节：9月29日至10月6日放假调休，共8天。10月7日（星期六）、10月8日（星期日）上班。
    (Ymd::new(2023, 9, 29), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2023, 9, 30), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2023, 10, 7), DayKind::NationalDayWorkday),
    (Ymd::new(2023, 10, 8), DayKind::NationalDayWorkday),
    // 2024 年节假日安排
    // https://www.gov.cn/zhengce/content/202310/content_6911527.htm
    // 一、元旦：1月1日放假，与周末连休。
    (Ymd::new(2024, 1, 1), DayKind::NewYearsDayHoliday),
    // 二、春节：2月10日至17日放假调休，共8天。2月4日（星期日）、2月18日（星期日）上班。鼓励各单位结合带薪年休假等制度落实，安排职工在除夕（2月9日）休息。
    (Ymd::new(2024, 2, 4), DayKind::SpringFestivalWorkday),
    (Ymd::new(2024, 2, 10), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 11), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 12), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 13), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 14), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 15), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 16), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 17), DayKind::SpringFestivalHoliday),
    (Ymd::new(2024, 2, 18), DayKind::SpringFestivalWorkday),
    // 三、清明节：4月4日至6日放假调休，共3天。4月7日（星期日）上班。
    (Ymd::new(2024, 4, 4), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2024, 4, 5), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2024, 4, 6), DayKind::ChingMingFestivalHoliday),
    (Ymd::new(2024, 4, 7), DayKind::ChingMingFestivalWorkday),
    // 四、劳动节：5月1日至5日放假调休，共5天。4月28日（星期日）、5月11日（星期六）上班。
    (
        Ymd::new(2024, 4, 28),
        DayKind::InternationalWorkersDayWorkday,
    ),
    (
        Ymd::new(2024, 5, 1),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2024, 5, 2),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2024, 5, 3),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2024, 5, 4),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2024, 5, 5),
        DayKind::InternationalWorkersDayHoliday,
    ),
    (
        Ymd::new(2024, 5, 11),
        DayKind::InternationalWorkersDayWorkday,
    ),
    // 五、端午节：6月10日放假，与周末连休。
    (Ymd::new(2024, 6, 10), DayKind::DragonBoatFestivalHoliday),
    // 六、中秋节：9月15日至17日放假调休，共3天。9月14日（星期六）上班。
    (Ymd::new(2024, 9, 14), DayKind::MidAutumnFestivalWorkday),
    (Ymd::new(2024, 9, 15), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2024, 9, 16), DayKind::MidAutumnFestivalHoliday),
    (Ymd::new(2024, 9, 17), DayKind::MidAutumnFestivalHoliday),
    // 七、国庆节：10月1日至7日放假调休，共7天。9月29日（星期日）、10月12日（星期六）上班。
    (Ymd::new(2024, 9, 29), DayKind::NationalDayWorkday),
    (Ymd::new(2024, 10, 1), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 2), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 3), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 4), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 5), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 6), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 7), DayKind::NationalDayHoliday),
    (Ymd::new(2024, 10, 12), DayKind::NationalDayWorkday),
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
