use chrono::{Datelike, Local};
use clap::Parser;

/// Simple Calender CLI
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Target Year
    #[arg(short, long, value_parser, default_value_t = Local::now().year())]
    year: i32,

    /// Target Month
    #[arg(short, long, value_parser, default_value_t = Local::now().month())]
    month: u32,

    /// Month Count
    #[arg(short, long, value_parser, default_value_t = 0)]
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
