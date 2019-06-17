/* Output system load info */
use getopts::Options;
use log::debug;
use std::io::{Error, ErrorKind};
use systemstat::{Platform, System};

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt(
        "n",
        "number",
        "number of load averages to show (1-3)",
        "NUMBER",
    );

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    let sys = System::new();
    let loadavg = sys.load_average()?;

    let num = matches.opt_str("n").unwrap_or_else(|| String::from("3"));

    match num.as_str() {
        "1" => println!("{:.2}", loadavg.one),
        "2" => println!("{:.2} {:.2}", loadavg.one, loadavg.five),
        "3" => println!(
            "{:.2} {:.2} {:.2}",
            loadavg.one, loadavg.five, loadavg.fifteen
        ),
        _ => {
            Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                format!("no command matches `{}`", num),
            )))?;
        }
    }
    Ok(())
}
