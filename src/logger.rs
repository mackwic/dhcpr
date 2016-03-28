use colored;
use fern;
use log;
use time;

pub fn init_logger(is_debug: bool) {
    use log::LogLevel::*;
    use colored::*;

    fn level_to_color(level: &log::LogLevel, string: &str) -> colored::ColoredString {
        match *level {
            Error => string.red(),
            Warn => string.yellow(),
            Info => string.green(),
            Debug => string.cyan(),
            Trace => string.magenta()
        }
    }

    let log_level = if is_debug {
        log::LogLevelFilter::Trace
    } else {
        log::LogLevelFilter::Info
    };

    let log_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            format!(
                "[{}{}{}][{}]\t{}",
                // ISO compatible time display. Use `CLICOLOR=0` for automatic parsing
                t.strftime("%Y-%m-%dT").unwrap().to_string().cyan(),
                t.strftime("%T").unwrap().to_string().yellow(),
                t.strftime("%zUTC").unwrap().to_string().cyan(),
                level_to_color(level, &level.to_string()).bold(),
                level_to_color(level, msg)
            )
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log_level
    };
    fern::init_global_logger(log_config, log_level).expect("unable to init logger");

    info!("Logger init success");
    debug!("Running in debug mode")
}
