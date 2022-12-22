#[macro_export]
macro_rules! console_log {
  ($($loggable_expression:expr ),+) => {
    let mut formatted_expression: String = "".to_string();
    $(
      formatted_expression += &format!("{:#?} ", $loggable_expression);
    )*
    $crate::log(&formatted_expression);
  };
}
