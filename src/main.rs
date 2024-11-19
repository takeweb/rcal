use chrono::{Datelike, Local};
use clap::Parser;

/// Simple Calender CLI
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Target Year (1-9999)
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)), default_value_t = Local::now().year())]
    year: i32,

    /// Target Month
    #[arg(short, long, value_parser, default_value_t = Local::now().month())]
    month: u32,

    /// Month Count
    #[arg(short, long, value_parser, default_value_t = 0)]
    count: u32,

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
    let mut args = Args::parse();
    if args.three {
        args.count = 1;
    }
    let cmd = rcal::CalCmd::new(args.count, args.year, args.month);

    if args.list {
        cmd.print_list();
    } else {
        cmd.print_cal();
    }
}

// fn parse_args() {}
