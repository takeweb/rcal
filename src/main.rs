use core::fmt;

use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;
use my_lib::util_date;

/// Simple Calender CLI
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Target Year (1-9999)
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)))]
    year: Option<i32>,

    /// Target Month name or number (1-12)
    #[arg(short)]
    month: Option<String>,

    /// Show near 3 month
    #[arg(short('3'), long, value_parser, default_value_t = false)]
    three: bool,

    /// Show list
    #[clap(short, long, value_parser, default_value_t = false)]
    list: bool,

    /// Show the whole current year
    #[arg(short('y'), long("year"), conflicts_with_all(["month", "year"]))]
    show_current_year: bool,
}

// カスタムエラー型を定義
#[derive(Debug)]
enum ParseError {
    InvalidMonth(String),
}

// `Display`トレイトを実装して、エラーメッセージをカスタマイズ
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidMonth(month) => {
                write!(f, r#"month "{month}" not in the range 1 through 12"#)
            }
        }
    }
}

// `std::error::Error`トレイトを実装（任意）
impl std::error::Error for ParseError {}

fn main() {
    let args = Args::parse();
    let (start_date, end_date) = parse_args(&args).unwrap();
    let cmd = rcal::CalCmd::new(start_date, end_date);
    if args.list {
        cmd.print_list();
    } else {
        cmd.print_cal();
    }
}

/// コマンドライン引数の解析
fn parse_args(args: &Args) -> Result<(NaiveDate, NaiveDate), ParseError> {
    let today = Local::now().date_naive();
    let month = parse_month(args.month.clone());
    let start_date = match args.year {
        None => NaiveDate::from_ymd_opt(today.year(), today.month(), 1),
        Some(y) => match month {
            Err(_) => NaiveDate::from_ymd_opt(y, 1, 1),
            Ok(m) => NaiveDate::from_ymd_opt(y, m, 1),
        },
    };
    let mut start_date = start_date.unwrap();

    let end_date = match args.year {
        None => util_date::last_day_in_month_opt(start_date.year(), start_date.month()),
        Some(y) => match month {
            Err(_) => NaiveDate::from_ymd_opt(y, 12, 31),
            Ok(m) => util_date::last_day_in_month_opt(y, m),
        },
    };
    let mut end_date = end_date.unwrap();

    if args.three {
        start_date = util_date::get_before_month(1, today.year(), today.month());
        end_date = util_date::get_next_month(1, today.year(), today.month());
    }
    Ok((start_date, end_date))
}

// --------------------------------------------------
fn parse_month(month: Option<String>) -> Result<u32, ParseError> {
    let month = month.unwrap_or_default();
    match month.parse() {
        Ok(num) => {
            if (1..=12).contains(&num) {
                Ok(num)
            } else {
                Err(ParseError::InvalidMonth(month))
            }
        }
        _ => {
            let lower = &month.to_lowercase();
            let matches: Vec<_> = rcal::MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    if name.to_lowercase().starts_with(lower) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .collect();

            if matches.len() == 1 {
                Ok(matches[0] as u32)
            } else {
                Err(ParseError::InvalidMonth(month))
            }
        }
    }
}
