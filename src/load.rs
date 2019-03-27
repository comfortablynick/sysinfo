use getopts::Options;
use systemstat::{Platform, System};

pub fn main(args: Vec<String>) -> Result<(), std::io::Error> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "percent", "used mem as pct of total mem");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        // print_help(&args[0], opts);
        return Ok(());
    }

    let sys = System::new();
    let loadavg = sys.load_average()?;

    println!(
        "{:.2} {:.2} {:.2}",
        loadavg.one, loadavg.five, loadavg.fifteen
    );
    Ok(())
}
