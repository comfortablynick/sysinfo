use getopts::Options;
mod example;
mod logger;

#[derive(Debug)]
struct Command {
    name: String,
    help: String,
}

impl Command {
    fn new(name: &str, help: &str) -> Command {
        Command {
            name: String::from(name),
            help: String::from(help),
        }
    }
}

fn print_usage(program: &str, opts: Options, cmds: Vec<Command>) {
    // let brief = format!("Usage: {} [options]", program);
    print!("Usage: {} [options] COMMAND", program);
    print!("{}", opts.usage(""));
    println!("");
    println!("Commands:");
    for cmd in cmds {
        println!("    {:20}{}", cmd.name, cmd.help,);
    }
}

fn main() {
    let args: Vec<String> = if std::env::args().len() > 1 {
        std::env::args().collect()
    } else {
        vec!["sysinfo"].iter().map(|s| s.to_string()).collect()
    };
    let program = args[0].clone();

    // Define command line options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "example", "show example output of different commands");
    opts.optflagmulti("v", "verbose", "increase log verbosity (e.g., -vv/-vvv");
    opts.optflag("q", "quiet", "discard log output (overrides --verbose");

    let commands = vec![
        Command::new("memory", "output memory usage info"),
        Command::new("cpu", "output cpu usage percent"),
        Command::new("temp", "output cpu temp"),
    ];

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if !matches.opt_present("q") {
        // init logger
        logger::Logger::init().expect("error initializing logger");
        log::set_max_level(match matches.opt_count("v") {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        });
    }
    log::debug!("Args: {:#?}", args);

    if matches.opt_present("h") {
        print_usage(&program, opts, commands);
        return;
    }

    log::debug!("Free: {:?}", matches.free);
    if matches.opt_present("e") {
        let measure_cpu = false;
        example::run_all(measure_cpu);
        return;
    }

    print_usage(&program, opts, commands);
}
