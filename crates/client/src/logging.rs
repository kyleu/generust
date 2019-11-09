macro_rules! debug {
  ($($t:tt)*) => (crate::js::log(&format_args!($($t)*).to_string().replacen("", "%c [debug] ", 1), "color: #999;"))
}

macro_rules! info {
  ($($t:tt)*) => (crate::js::log(&format_args!($($t)*).to_string().replacen("", "%c [info] ", 1), "color: #003049;"))
}

macro_rules! warn {
  ($($t:tt)*) => (crate::js::log(&format_args!($($t)*).to_string().replacen("", "%c [warn] ", 1), "color: #fcbf49;"))
}

macro_rules! error {
  ($($t:tt)*) => (crate::js::log(&format_args!($($t)*).to_string().replacen("", "%c [error] ", 1), "color: #d62828;"))
}
