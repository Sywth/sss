#[macro_export]
macro_rules! dbg_boxed {
  ($($arg:tt)*) => {
      if cfg!(debug_assertions) {
          eprintln!("---------------------");
          eprintln!($($arg)*);
          eprintln!("---------------------");
      }
  };
}
