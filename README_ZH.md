# chinese_holiday

[English](README.md) | [中文](README_ZH.md)

`chinese_holiday` 是一个用于判断中国节假日的 Rust 库。节假日数据来源于[中国政府信息公开平台](http://www.gov.cn/zhengce/xxgk/index.htm)。`chinese_holiday` 收录了自 2004 年到 2023 年的中国节假日特例数据，此后每年将持续更新。

Licensed under [Apache 2.0](LICENSE).

## 安装

```
cargo add chinese_holiday
```

## 用例

```
use chinese_holiday::*;
use chrono::*;

let date = NaiveDate::from_ymd_opt(2004, 1, 1).unwrap();
assert_eq!(chinese_holiday(&date), DayKind::NewYearsDayHoliday);
```

更多信息请参考:
- [docs.rs](https://docs.rs/chinese_holiday/)
- [examples](examples/)