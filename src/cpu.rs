/* Output cpu info */
use getopts::Options;

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

#[cfg(target_os = "linux")]
fn cpu_test() {
    use std::thread;
    use systemstat::{Duration, Platform, System};

    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!(
                "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0,
                cpu.nice * 100.0,
                cpu.system * 100.0,
                cpu.interrupt * 100.0,
                cpu.idle * 100.0
            );
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
}

#[cfg(target_os = "linux")]
fn get_cpu(interval: Option<u64>) {
    use std::thread;
    use systemstat::{Duration, Platform, System};

    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(interval.unwrap_or(1)));
            let cpu = cpu.done().unwrap();
            println!(
                "{:.1}%",
                (cpu.user + cpu.nice + cpu.system + cpu.interrupt) * 100_f32
            );
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }
}

#[cfg(target_os = "macos")]
fn cpu_test() {
    // use sys_info;
    // sys_info::
}

#[cfg(target_os = "macos")]
fn get_cpu() {
    // use sys_info;
    // sys_info::
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("t", "test", "test cpu load aggregate and print results");
    opts.optopt(
        "i",
        "interval",
        "interval length for sampling cpu (in seconds)",
        "SECS",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    if matches.opt_present("t") {
        cpu_test();
    }

    let interval: Option<u64> = matches.opt_get("i")?;
    get_cpu(interval);
    Ok(())
}
