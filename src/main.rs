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
    month: Option<u32>,

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
fn parse_args(args: &Args) -> Result<(NaiveDate, NaiveDate), Box<dyn std::error::Error>> {
    let today = Local::now().date_naive();
    let start_date = match args.year {
        None => NaiveDate::from_ymd_opt(today.year(), today.month(), 1),
        Some(y) => match args.month {
            None => NaiveDate::from_ymd_opt(y, 1, 1),
            Some(m) => NaiveDate::from_ymd_opt(y, m, 1),
        },
    };
    let mut start_date = start_date.unwrap();

    let end_date = match args.year {
        None => util_date::last_day_in_month_opt(start_date.year(), start_date.month()),
        Some(y) => match args.month {
            None => NaiveDate::from_ymd_opt(y, 12, 31),
            Some(m) => util_date::last_day_in_month_opt(y, m),
        },
    };
    let mut end_date = end_date.unwrap();

    if args.three {
        start_date = util_date::get_before_month(1, today.year(), today.month());
        end_date = util_date::get_next_month(1, today.year(), today.month());
    }
    Ok((start_date, end_date))
}
