fn main() {
  io::println("===Beginning test run===");
  test(&"5", 5);
  test(&"99", 99);
  test(&"1,2", 3);
}

fn add(numbers: &str) -> int {
  vec::foldl(0, str::split_char(numbers, ','), |a, b| a + match int::from_str(*b) { Some(x) => x, None => 0 })
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
