fn main() {
  for n in 0..100{
    let mut out = String::new();
    if n % 3 == 0{
      out.push_str("Fizz")
    }
    if n % 5 == 0{
      out.push_str("Buzz")
    };
    if(out.is_empty()){
      println!("{}", n);
    } else {
      println!("{}", out);
    }
  }
}
