/* Output memory usage info */
#![allow(unused_imports)]
use getopts::Options;
use number_prefix::{NumberPrefix, Prefixed, Standalone};
use systemstat::{ByteSize, Platform, System};

fn print_help(command: &str, opts: Options) {
    let usage = format!("Usage: {} {} [options]", super::PROG, command);
    print!("{}", opts.usage(&usage));
}

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
        print_help(&args[0], opts);
        return Ok(());
    }

    let used_pct = matches.opt_present("p");
    log::debug!("Opt: Used mem as pct: {}", used_pct);

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
    log::debug!(
        "Used:         {} ({:.2}%)",
        ByteSize::b(mem_used),
        (mem_used as f32 / mem_total as f32) * 100.0
    );

    if used_pct {
        println!("{:.1}%", (mem_used as f32 / mem_total as f32) * 100.0);
        return Ok(());
    }

    let used_fmt = match NumberPrefix::binary(mem_used as f32) {
        Standalone(b) => format!("{} B", b),
        Prefixed(prefix, n) => format!("{:.1} {}B", n, prefix),
    };
    let total_fmt = match NumberPrefix::binary(mem_total as f32) {
        Standalone(b) => format!("{} B", b),
        Prefixed(prefix, n) => format!("{:.2} {}B", n, prefix),
    };

    println!("{}/{}", used_fmt, total_fmt);

    Ok(())
}
