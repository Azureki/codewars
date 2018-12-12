fn main() {
    let a1 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
    let res = new_avg(&a1, 4800.0);
    match res {
        None => {
            println!("123");
        }
        Some(i) => {
            println!("{}", i);
        }
    }
}
fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    let mut sum = 0 as f64;
    for i in arr {
        sum += i;
        // codewars need dereference
        // sum += *i;
    }
    let res = newavg * (arr.len() as f64 + 1 as f64) - sum;
    if res < 0.0 {
        None
    } else {
        Some(res.round() as i32)
    }
}

// WiebeCnossen
fn new_avg2(arr: &[f64], newavg: f64) -> Option<i32> {
  match (arr.len() as f64 + 1f64) * newavg - arr.iter().sum::<f64>() {
    n if n > 0f64 => Some(n.ceil() as i32),
    _ => None
  }
}
