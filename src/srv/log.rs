pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
    }
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::Level::Error => {
                    eprintln!(
                        "{} # {} # {}",
                        chrono::Local::now().to_rfc3339(),
                        record.level(),
                        record.args()
                    )
                }
                _ => {
                    println!(
                        "{} # {} # {}",
                        chrono::Local::now().to_rfc3339(),
                        record.level(),
                        record.args()
                    )
                }
            }
        }
    }
    fn flush(&self) {}
}

static LOGGER: Logger = Logger;
pub fn init_logger(log_level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}
