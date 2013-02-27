fn main() {
  io::println("===Beginning test run===");
  test(&"5", 5);
  test(&"99", 99);
  test(&"1,2", 3);
}

fn add(numbers: &str) -> int {
  let mut total = 0;
  for str::split_char(numbers, ',').each |x| {
    total += match int::from_str(*x) {
      Some(x) => x,
      None => 0
    };
  }
  return total; 
}




fn test (numbers: &str, expected_sum: int) {
  let message = fmt!("'%?' should return %?", numbers, expected_sum);
  is_true(add(numbers) == expected_sum, message);
}

fn is_true(x: bool, msg: &str) {
  if x != true {
    io::println("");
    io::println(fmt!("FAIL: %?", msg));
  } else
  {
    io::print(".");
  }
}
