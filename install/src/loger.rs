use log::{Record, Metadata, SetLoggerError, LevelFilter};

struct SimpleLogger(LevelFilter);

pub(crate) fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_boxed_logger(Box::new(SimpleLogger(level)))
        .map(|()| log::set_max_level(LevelFilter::Trace))
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.0
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}