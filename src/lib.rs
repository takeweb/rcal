use chrono::{Datelike, NaiveDate, Weekday};
use colored::{ColoredString, Colorize};
use jpholiday::jpholiday::JPHoliday;
use my_lib::util_date;
use my_lib::util_date::{MyCalendar, YearMonths};
use std::borrow::Borrow;

/// カレンダー期間
struct Period {
    start_date: NaiveDate,
    end_date: NaiveDate,
}
impl Period {
    /// カレンダー期間の初期化
    ///
    /// * `n` - 前後nカ月表示の指定。
    /// * `year` - 指定年。
    /// * `month` - 指定月。
    fn new(n: u32, year: i32, month: u32) -> Self {
        let start_date = util_date::get_before_month(n, year, month);
        let end_date = util_date::get_next_month(n, year, month);
        Period {
            start_date,
            end_date,
        }
    }
}

/// カレンダーコマンド
pub struct CalCmd<'a> {
    jpholiday: JPHoliday<'a>,
    mycalendar: MyCalendar,
}
impl CalCmd<'_> {
    /// カレンダーコマンドの初期化
    ///
    /// * `n` - 前後nカ月表示の指定。
    /// * `year` - 指定年。
    /// * `month` - 指定月。
    pub fn new(n: u32, year: i32, month: u32) -> Self {
        let jpholiday = JPHoliday::new();
        let period = Period::new(n, year, month);
        let mycalendar = MyCalendar::new(period.start_date, period.end_date);
        CalCmd {
            jpholiday,
            mycalendar,
        }
    }

    /// カレンダーのリスト表示
    pub fn print_list(&self) {
        let keys = self.mycalendar.year_months.keys();
        for year_month in keys {
            let calendar = self.mycalendar.year_months.get(year_month).unwrap();
            for date in calendar {
                let target_date =
                    NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();
                match self.get_holiday(target_date.borrow()) {
                    Some(holiday) => {
                        println!(
                            "{} {} {}",
                            date.to_string().red(),
                            util_date::get_jp_weekday(&date).red(),
                            holiday.red()
                        );
                    }
                    None => match date.weekday() {
                        Weekday::Sat => println!(
                            "{} {}",
                            date.to_string().blue(),
                            util_date::get_jp_weekday(&date).blue()
                        ),
                        Weekday::Sun => println!(
                            "{} {}",
                            date.to_string().red(),
                            util_date::get_jp_weekday(&date).red()
                        ),
                        _ => println!(
                            "{} {}",
                            date.to_string().white(),
                            util_date::get_jp_weekday(&date).white()
                        ),
                    },
                }
            }
        }
    }

    /// カレンダー表示
    pub fn print_cal(&self) {
        let mut keys: Vec<YearMonths> = self
            .mycalendar
            .year_months
            .keys()
            .cloned()
            .collect::<Vec<YearMonths>>();
        keys.sort();
        for year_month in keys {
            let calendar = self.mycalendar.year_months.get(&year_month).unwrap();
            let formatted_calendar: Vec<ColoredString> =
                self.make_cal(year_month.year, year_month.month, calendar);
            formatted_calendar.iter().for_each(|c| println!("{}", c));
        }
    }

    fn make_cal(
        &self,
        target_year: i32,
        target_month: u32,
        calendar: &Vec<NaiveDate>,
    ) -> Vec<ColoredString> {
        let mut result: Vec<ColoredString> = Vec::new();
        let end_date = NaiveDate::from_ymd_opt(
            target_year,
            target_month,
            util_date::get_days_from_ym(target_year, target_month),
        )
        .unwrap();
        let mut vec_cal = Vec::new();
        let mut row = self.get_empty_row();
        let mut week_end = false;

        let header1 = format!(
            "{}年({}){}月",
            target_year.to_string(),
            util_date::get_wareki(&NaiveDate::from_ymd_opt(target_year, target_month, 1).unwrap()),
            target_month.to_string()
        );
        let header2 = format!(
            "{} {} {} {} {} {} {}",
            String::from("日").red(),
            String::from("月"),
            String::from("火"),
            String::from("水"),
            String::from("木"),
            String::from("金"),
            String::from("土").blue()
        );
        result.push(header1.white());
        result.push(header2.white());

        for date in calendar {
            let target_date =
                NaiveDate::from_ymd_opt(date.year(), date.month(), date.day()).unwrap();
            let week_index = date.weekday().num_days_from_sunday() as usize;
            let colored_date = if self.jpholiday.is_holiday(target_date.borrow()) {
                date.day().to_string().red()
            } else {
                if week_index == 0 {
                    date.day().to_string().red()
                } else if week_index == 6 {
                    date.day().to_string().blue()
                } else {
                    date.day().to_string().white()
                }
            };

            if util_date::is_today(target_date.borrow()) {
                row[week_index] = colored_date.reversed();
            } else {
                row[week_index] = colored_date;
            }

            if week_index == 6 {
                week_end = true;
            }

            if week_end || end_date.day() == date.day() {
                vec_cal.push(row);
                week_end = false;
                row = self.get_empty_row();
            }
        }
        for row in vec_cal {
            let formatted_row = self.get_formatted_row(row);
            result.push(formatted_row);
        }
        result.push(self.get_formatted_row(self.get_empty_row()));
        result
    }

    fn get_formatted_row(&self, row: Vec<ColoredString>) -> ColoredString {
        let formatted_row = format!(
            "{:>02} {:>02} {:>02} {:>02} {:>02} {:>02} {:>02}",
            row[0], row[1], row[2], row[3], row[4], row[5], row[6]
        );
        formatted_row.white()
    }

    fn get_empty_row(&self) -> Vec<ColoredString> {
        let empty_row = vec![
            String::from("").white(),
            String::from("").white(),
            String::from("").white(),
            String::from("").white(),
            String::from("").white(),
            String::from("").white(),
            String::from("").white(),
        ];
        empty_row
    }

    fn get_holiday(&self, target_date: &NaiveDate) -> Option<String> {
        self.jpholiday.is_holiday_name(target_date)
    }
}
