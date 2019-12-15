use std::str::FromStr;

pub fn init(level: &str) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{ts} {lvl:<5} [{thread:>25.25}] | userId requestId | {file:<40.40} {msg}",
                ts = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                lvl = record.level(),
                thread = std::thread::current().name().unwrap_or("main"),
                file = record.file().unwrap_or("no_file.rs"),
                msg = message
            ))
        })
        .level(log::LevelFilter::Off)
        .level_for(
            "rust_tide_backend",
            log::LevelFilter::from_str(level).unwrap_or(log::LevelFilter::Info),
        )
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
