/* Output system uptime */
// use chrono::format;
use getopts::Options;
use std::ops::Sub;
use systemstat::{Platform, System};

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    let sys = System::new();
    let uptime = sys.uptime()?;
    let duration = chrono::Duration::from_std(uptime)?;
    // push weeks
    if duration.num_weeks() > 0 {
        print!("{}w", duration.num_weeks());
    }
    // push days
    let days = duration
        .sub(chrono::Duration::weeks(duration.num_weeks()))
        .num_days();
    if days > 0 {
        print!("{}d", days);
    }
    // push hours
    let hours = duration
        .sub(chrono::Duration::days(duration.num_days()))
        .num_hours();
    if hours > 0 {
        print!("{}h", hours);
    }
    // push minutes if < 1 hour
    if duration.num_hours() < 1 {
        let minutes = duration
            .sub(chrono::Duration::hours(duration.num_hours()))
            .num_minutes();
        print!("{}m", minutes);
    }
    Ok(())
}
