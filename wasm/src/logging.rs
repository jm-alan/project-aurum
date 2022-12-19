macro_rules! console_log {
  ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()));
}

pub(crate) use console_log;
