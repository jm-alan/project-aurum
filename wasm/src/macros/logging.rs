#[macro_export]
macro_rules! console_log {
  ($($loggable_expression:expr),+) => {
    let mut formatted_expression: String = "".to_string();
    $(
      formatted_expression += &format!("{:#?} ", $loggable_expression);
    )*
    $crate::log(&formatted_expression);
  };
}

#[macro_export]
macro_rules! console_error {
  ($($loggable_expression:expr),+) => {
    let mut formatted_expression: String = "".to_string();
    $(
      formatted_expression += &format!("{:#?} ", $loggable_expression);
    )*
    $crate::err(&formatted_expression);
  };
}

#[macro_export]
macro_rules! js_panic {
  ($($loggable_expression:expr),+) => {
    $crate::console_error!($($loggable_expression),*);
    $crate::console_error!("The above error resulted in an immediate panic");
    $crate::console_error!("WASM execution will stop beyond this point and must be restarted");
    panic!();
  };
}
