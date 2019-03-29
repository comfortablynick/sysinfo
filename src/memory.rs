/* Output memory usage info */
use getopts::Options;
use number_prefix::{NumberPrefix, Prefixed, Standalone};

#[derive(Debug)]
struct MemStats {
    total: usize,
    used: usize,
}

impl MemStats {
    fn new(total: usize, used: usize) -> MemStats {
        MemStats {
            total: total,
            used: used,
        }
    }
}

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

#[cfg(target_os = "linux")]
fn get_memory() -> Result<MemStats, Box<std::error::Error>> {
    use systemstat::{ByteSize, Platform, System};

    let sys = System::new();
    let mem = sys.memory()?;
    let meminfo = mem.platform_memory.meminfo;
    let mem_total = mem.total.as_usize();
    let shmem = meminfo
        .get("Shmem")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_usize();
    let mem_free = meminfo
        .get("MemFree")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_usize();
    let buffers = meminfo
        .get("Buffers")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_usize();
    let cached = meminfo
        .get("Cached")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_usize();
    let s_reclaimable = meminfo
        .get("SReclaimable")
        .cloned()
        .unwrap_or_else(|| ByteSize::b(0))
        .as_usize();

    let mem_used = mem_total - mem_free + shmem - buffers - cached - s_reclaimable;

    log::trace!("{:#?}", System::new().memory()?.platform_memory.meminfo);
    log::debug!(
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
fn get_memory() -> Result<MemStats, Box<std::error::Error>> {
    let mem_info = sys_info::mem_info()?;
    log::debug!("{:#?}", mem_info);
    Ok(MemStats::new(
        mem_info.total as usize,
        (mem_info.total - mem_info.free - mem_info.avail) as usize,
    ))
}

pub fn main(args: Vec<String>) -> Result<(), Box<std::error::Error>> {
    log::debug!("Args: {:?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "percent", "used mem as pct of total mem");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return Ok(());
    }

    let used_pct = matches.opt_present("p");
    log::debug!("Opt: Used mem as pct: {}", used_pct);

    let stats = get_memory()?;

    log::debug!(
        "Used: {} ({:.2}%)",
        stats.used,
        (stats.used as f32 / stats.total as f32) * 100.0
    );

    if used_pct {
        println!("{:.1}%", (stats.used as f32 / stats.total as f32) * 100.0);
        return Ok(());
    }

    let used_fmt = match NumberPrefix::binary(stats.used as f32) {
        Standalone(b) => format!("{}B", b),
        Prefixed(prefix, n) => format!("{:.1}{}", n, prefix),
    };
    let total_fmt = match NumberPrefix::binary(stats.total as f32) {
        Standalone(b) => format!("{}B", b),
        Prefixed(prefix, n) => format!("{:.2}{}", n, prefix),
    };

    println!("{}/{}", used_fmt, total_fmt);

    Ok(())
}
