use getopts::Options;
mod example;
mod load;
mod logger;
mod memory;

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

fn print_help(program: &str, opts: Options, cmds: Vec<Command>) {
    print!("Usage: {} [options] COMMAND", program);
    print!("{}", opts.usage(""));
    println!("");
    println!("Commands:");
    for cmd in cmds {
        println!("    {:20}{}", cmd.name, cmd.help,);
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = if std::env::args().len() > 1 {
        std::env::args().collect()
    } else {
        vec!["sysinfo", "memory"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    };
    let program = args[0].clone();

    // Define command line options
    let mut opts = Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "example", "show example output of different commands");
    opts.optflagmulti("v", "verbose", "increase log verbosity (e.g., -vv/-vvv");
    opts.optflag("q", "quiet", "discard log output (overrides --verbose");

    let commands = vec![
        Command::new("memory", "output memory usage info"),
        Command::new("cpu", "output cpu usage info"),
        Command::new("load", "output load average"),
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
    log::debug!("Main args: {:?}", args);
    log::debug!("Remaining args: {:?}", matches.free);

    if matches.opt_present("h") {
        print_help(&program, opts, commands);
        return Ok(());
    }

    if matches.opt_present("e") {
        let measure_cpu = false;
        example::run_all(measure_cpu);
        return Ok(());
    }

    let cmd = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_help(&program, opts, commands);
        return Ok(());
    };

    log::debug!("Command: '{}'", cmd);

    // Handle command
    if cmd == "memory" {
        memory::main(matches.free)?;
        return Ok(());
    }
    if cmd == "load" {
        load::main(matches.free)?;
        return Ok(());
    }
    Ok(())
}
