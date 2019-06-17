use getopts::Options;
use log::debug;
use std::io::{Error, ErrorKind};
mod cpu;
mod example;
mod load;
mod logger;
mod memory;
mod temp;
mod uptime;

// Constants
pub const PROG: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESC: &str = env!("CARGO_PKG_DESCRIPTION");

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
    println!("{} v{}", PROG, VERSION);
    println!("{}", AUTHORS);
    println!();
    print!("Usage: {} [options] COMMAND", program);
    print!("\n\n{}", DESC);
    print!("{}", opts.usage(""));
    println!("\nCommands:");
    for cmd in cmds {
        println!("    {:20}{}", cmd.name, cmd.help,);
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = if std::env::args().len() > 1 {
        std::env::args().collect()
    } else {
        vec!["sysinfo"].iter().map(|s| s.to_string()).collect()
    };
    let program = args[0].clone();

    // Define command line options
    let mut opts = Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.optflag("h", "help", "print help for program or command");
    opts.optflag("V", "version", "print version and exit");
    opts.optflagmulti("v", "verbose", "increase log verbosity (e.g., -vv/-vvv)");
    opts.optflag("q", "quiet", "discard log output (overrides --verbose");

    let commands = vec![
        Command::new("m, memory", "output memory usage info"),
        Command::new("c, cpu", "output cpu usage info"),
        Command::new("l, load", "output load average"),
        Command::new("t, temp", "output cpu temp"),
        Command::new("u, uptime", "output system uptime"),
        Command::new("e, example", "show example output of different commands"),
    ];

    let matches = opts.parse(&args[1..])?;

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
    debug!("Main args: {:?}", args);
    debug!("Remaining args: {:?}", matches.free);

    if matches.opt_present("h") {
        print_help(&program, opts, commands);
        return Ok(());
    }

    if matches.opt_present("V") {
        println!("{} v{}", PROG, VERSION);
        return Ok(());
    }

    let cmd = if !matches.free.is_empty() {
        matches
            .free
            .get(0)
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "error getting command"))?
    } else {
        print_help(&program, opts, commands);
        return Ok(());
    };

    debug!("Command: '{}'", cmd);

    // Handle command
    match cmd.as_str() {
        "m" | "memory" => memory::main(matches.free)?,
        "l" | "load" => load::main(matches.free)?,
        "c" | "cpu" => cpu::main(matches.free)?,
        "t" | "temp" => temp::main(matches.free)?,
        "u" | "uptime" => uptime::main(matches.free)?,
        "e" | "example" => example::run_all(true),
        _ => {
            Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                format!("no command matches `{}`", cmd.as_str()),
            )))?;
        }
    }
    Ok(())
}
