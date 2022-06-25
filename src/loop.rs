fn main() {
  let b = {
    let mut i = 0;

    loop {
      i += 1;

      if i == 10 { break i }
    }
  };

  println!("{b}");
}