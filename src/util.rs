use std::old_io;
use std::rand;

/// Checks if x in [a,b].
///
/// # Examples
///
/// ```
/// use twenty48::util::check_bounded;
/// assert!(check_bounded(5, 2, 7));
/// assert!(check_bounded(-3, -3, -3));
/// assert!(!check_bounded(12, 12, 11));
/// ```
pub fn check_bounded(x: i32, a: i32, b: i32) -> bool {
  return x >= a && x <= b;
}

/// Checks if x is a power of base b.
///
/// # Examples
///
/// ```
/// use twenty48::util::check_power_of_base;
/// assert!(!check_power_of_base(4, 0));
/// assert!(!check_power_of_base(5, 1));
/// assert!(check_power_of_base(1, 6));
/// assert!(check_power_of_base(16, 2));
/// assert!(!check_power_of_base(28, 3));
/// ```
pub fn check_power_of_base(x: u32, b: u32) -> bool {
  if x == 1 { true }
  else if x == 0 { false }
  else if b == 0 || b == 1 { false }
  else if x == 1 { return true; }
  else if x % b == 0 { check_power_of_base(x / b, b) }
  else { false }
}

/// Flips a coin with P(true) = p.
///
/// # Examples
///
/// ```
/// use twenty48::util::flip;
/// assert!(flip(1.0));
/// assert!(!flip(0.0));
/// ```
pub fn flip(p: f32) -> bool {
  let val: f32 = rand::random::<f32>();
  if val < p { true } else { false }
}

pub fn get_int_input<F: Fn(u32) -> bool>(prompt: &str, default: u32, check: F) -> u32 {
  old_io::stdio::println(prompt);
  let input: String = old_io::stdin().read_line()                     // Result<String, Error>
                                     .ok()                            // Option<String>
                                     .expect("Failed to read input"); // String
  let input_option: Option<u32> = input.trim()         // String
                                       .parse::<u32>() // Option<u32>
                                       .ok();          // Result<u32, Error>
  let input_number = match input_option {
    Some(num) => if check(num) { num } else { default },
    None => default,
  };
  return input_number;
}
