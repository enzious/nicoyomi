use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use slog::Drain;

lazy_static! {
  static ref LOG_GUARD: Arc<Mutex<Option<LoggingGuard>>> = Arc::new(Mutex::new(None));
}

pub struct LoggingGuard {
  _scope_guard: slog_scope::GlobalLoggerGuard,
  _log_guard: (),
}

lazy_static! {
  static ref FILTERED_MODULES: HashSet<&'static str> = {
    let mut modules = HashSet::new();
    modules.insert("tokio_io");
    modules.insert("tokio_reactor");
    modules
  };
}

fn filter_records(record: &slog::Record) -> bool {
  let mut pieces = record.module().split("::");
  let lcrate = pieces.nth(0);
  if let Some(lcrate) = lcrate {
    if FILTERED_MODULES.contains(lcrate) {
      return false;
    }
  }
  true
}

pub fn init() {
  let values = slog_o!("place" =>
    slog::FnValue(move |info| {
      format!(
        "{}:{} {}",
        info.file(),
        info.line(),
        info.module(),
      )
    })
  );

  let _scope_guard = {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain.fuse()).build().fuse();
    let drain = slog::LevelFilter::new(drain, slog::Level::Info)
      .filter(filter_records)
      .fuse();
    let logger = slog::Logger::root(drain, values);
    Some(slog_scope::set_global_logger(logger))
  };

  if let Some(_scope_guard) = _scope_guard {
    let _log_guard = slog_stdlog::init().unwrap();

    let mut log_guard = LOG_GUARD.lock().unwrap();
    *log_guard = Some(LoggingGuard {
      _scope_guard,
      _log_guard,
    });
  }
}
