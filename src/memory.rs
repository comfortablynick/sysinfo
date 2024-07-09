/* Output memory usage info */
use getopts::Options;
use log::debug;
use std::cmp;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct MemStats {
    total: usize,
    used:  usize,
}

impl MemStats {
    fn new(total: usize, used: usize) -> MemStats {
        MemStats { total, used }
    }
}

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

/// Convert bytes into human-readable string format
///
/// `si_units`: use 1024 instead of 1000 bytes/kilobyte
/// `display_byte_suffix`: show 'B' after unit (e.g., 'MB' vs 'M')
fn humanize_bytes(num: f64, si_units: bool, display_byte_suffix: bool) -> Result<String> {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];
    let byte_suffix = if display_byte_suffix { "B" } else { "" };
    if num < 1_f64 {
        return Ok(format!("{}{}{}", negative, num, byte_suffix));
    }
    let delimiter = if si_units { 1024_f64 } else { 1000_f64 };
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>()? * 1_f64;
    let unit = units[exponent as usize];
    let out = format!("{}{:.2}{}{}", negative, pretty_bytes, unit, byte_suffix);
    Ok(out)
}

#[cfg(target_os = "linux")]
fn get_memory() -> Result<MemStats> {
    use systemstat::{ByteSize, Platform, System};

    let sys = System::new();
    let mem = sys.memory()?;
    let meminfo = mem.platform_memory.meminfo;
    let mem_total = mem.total.as_u64() as usize;
    let shmem = meminfo
        .get("Shmem")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_u64() as usize;
    let mem_free = meminfo
        .get("MemFree")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_u64() as usize;
    let buffers = meminfo
        .get("Buffers")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_u64() as usize;
    let cached = meminfo
        .get("Cached")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_u64() as usize;
    let s_reclaimable = meminfo
        .get("SReclaimable")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_u64() as usize;

    let mem_used = mem_total - mem_free + shmem - buffers - cached - s_reclaimable;

    trace!("{:#?}", System::new().memory()?.platform_memory.meminfo);
    debug!(
        "Memory details:
        MemTotal:     {}
        MemFree:      {}
        Cached:       {}
        Buffers:      {}
        Shmem:        {}
        SReclaimable: {}",
        mem_total / 1024,
        mem_free / 1024,
        cached / 1024,
        buffers / 1024,
        shmem / 1024,
        s_reclaimable / 1024
    );
    Ok(MemStats::new(mem_total, mem_used))
}

#[cfg(target_os = "macos")]
fn get_memory() -> Result<MemStats> {
    let mem_info = sys_info::mem_info()?;
    debug!("{:#?}", mem_info);
    Ok(MemStats::new(
        (mem_info.total * 1024) as usize,
        ((mem_info.total - mem_info.free - mem_info.avail) * 1024) as usize,
    ))
}

pub fn main(args: &[String]) -> Result {
    debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "percent", "used mem as pct of total mem");
    opts.optflag("u", "used", "show used memory only");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    let used_pct = matches.opt_present("p");
    debug!("Opt: Used mem as pct: {}", used_pct);
    let used_only = matches.opt_present("u");
    debug!("Opt: Used mem only: {}", used_only);

    let stats = get_memory()?;

    debug!(
        "Used: {} ({:.2}%)",
        stats.used,
        (stats.used as f32 / stats.total as f32) * 100.0
    );

    // Write output
    if used_pct {
        println!("{:.1}%", (stats.used as f32 / stats.total as f32) * 100.0);
        return Ok(());
    }

    let used_fmt = humanize_bytes(stats.used as f64, true, false)?;

    if used_only {
        println!("{}", used_fmt);
        return Ok(());
    }

    let total_fmt = humanize_bytes(stats.total as f64, true, false)?;

    println!("{}/{}", used_fmt, total_fmt);

    Ok(())
}
