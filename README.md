# chinese_holiday

[English](README.md) | [中文](README_ZH.md)

> **A Rust library for determining Chinese holidays. Holiday data from the [Chinese Government Information Disclosure Platform](http://www.gov.cn/zhengce/xxgk/index.htm)**

Licensed under [Apache 2.0](LICENSE).

## Install

```
cargo add chinese_holiday
```

## Usage

```
let date = NaiveDate::from_ymd_opt(2004, 1, 1).unwrap();
assert_eq!(chinese_holiday(&date), DayKind::NewYearsDayHoliday);
```

For more details, see:
- [docs.rs](https://docs.rs/chinese_holiday/)
- [examples](examples/)