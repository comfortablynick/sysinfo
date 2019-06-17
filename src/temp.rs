/* Output cpu info */
use getopts::Options;
use std::io::{self, Write};
use systemstat::{Platform, System};

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag(
        "c",
        "celcius",
        "show result in degrees celcius (not fahrenheit)",
    );

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    let sys = System::new();
    let temp = sys.cpu_temp()?;

    let stdout = io::stdout();
    let mut cout = stdout.lock();

    if matches.opt_present("c") {
        write!(cout, "{:.0}º", temp)?;
    } else {
        write!(cout, "{:.0}º", ((temp * 9.0) / 5.0 + 32.0))?;
    }
    Ok(())
}
