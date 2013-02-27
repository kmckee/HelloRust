fn main() {
  io::println("===Beginning test run===");
  test(&"5", 5);
  test(&"99", 99);
  test(&"1,2", 3);
}

fn add(numbers: &str) -> int {
  let mut total = 0;
  for str::split_char(numbers, ',').each |x| {
    let y = int::from_str(*x);
    let t = match y {
      Some(x) => x,
      None => 0
    };
    total += t;
  }
  return total; 
  //return int::from_str(numbers);
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
