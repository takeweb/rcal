use chrono::{Datelike, Local};
use clap::Parser;

/// Simple Calender CLI
#[derive(Parser, Debug)]
#[clap(
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
struct Args {
    /// Target Year
    #[clap(short, long, value_parser, default_value_t = Local::now().year())]
    year: i32,

    /// Target Month
    #[clap(short, long, value_parser, default_value_t = Local::now().month())]
    month: u32,

    /// Month Count
    #[clap(short, long, value_parser, default_value_t = 0)]
    count: u32,

    /// Show list
    #[clap(short, long, value_parser, default_value_t = false)]
    list: bool,
}

fn main() {
    let args = Args::parse();
    let cmd = rcal::CalCmd::new(args.count, args.year, args.month);

    if args.list {
        cmd.print_list();
    } else {
        cmd.print_cal();
    }
}
