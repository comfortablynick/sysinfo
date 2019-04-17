/* Output system uptime */
// use chrono::format;
use getopts::Options;
use std::{
    io::{self, Write},
    ops::Sub,
};
use systemstat::{Platform, System};

fn print_help(command: &str, opts: Options) {
    const DESC: &str = "Print uptime in specified format.";
    let usage = format!("Usage: {} {} [options]\n\n{}", super::PROG, command, DESC);
    print!("{}", opts.usage(&usage));
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("w", "weeks", "format as weeks instead of days");
    opts.optflag("p", "precise", "show hours or minutes (if < 1 hour)");

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
    let stdout = io::stdout();
    let mut cout = stdout.lock();
    write!(cout, "↑")?;
    // push weeks
    let fmt_weeks = matches.opt_present("weeks");
    if duration.num_weeks() > 0 && fmt_weeks {
        write!(cout, "{}w", duration.num_weeks())?;
    }
    // push days
    let days = if fmt_weeks {
        duration
            .sub(chrono::Duration::weeks(duration.num_weeks()))
            .num_days()
    } else {
        duration.num_days()
    };

    if days > 0 {
        write!(cout, "{}d", days)?;
    }
    // push hours if < 1 week
    let hours = duration
        .sub(chrono::Duration::days(duration.num_days()))
        .num_hours();
    if hours > 0 && (duration.num_weeks() == 0 || matches.opt_present("precise")) {
        write!(cout, "{}h", hours)?;
    }
    // push minutes if < 1 hour
    if duration.num_hours() < 1 {
        let minutes = duration
            .sub(chrono::Duration::hours(duration.num_hours()))
            .num_minutes();
        write!(cout, "{}m", minutes)?;
    }
    Ok(())
}
