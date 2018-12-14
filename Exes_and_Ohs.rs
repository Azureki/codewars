fn main() {}
fn xo(string: &'static str) -> bool {
    let bytes = string.as_bytes();
    let mut num1 = 0;
    let mut num2 = 0;
    for &i in bytes.iter() {
        if i == b'o' || i == b'O' {
            num1 += 1;
        } else if i == b'x' || i == b'X' {
            num2 += 1;
        }
    }
    num1 == num2
}

// DGolubets
fn xo1(string: &'static str) -> bool {
  string.chars().fold(0, |a, c|{
    match c {
      'x' | 'X' => a + 1,
      'o' | 'O' => a - 1,
      _ => a
    }
  }) == 0
}

// trit
fn xo2(s: &str) -> bool {
    let s2 = s.to_lowercase();
    s2.matches("x").count() == s2.matches("o").count()
}
