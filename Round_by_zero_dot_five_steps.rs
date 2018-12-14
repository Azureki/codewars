fn main() {
    let res = solution(4.25);
    println!("result is {}", res);
}
fn solution(n: f64) -> f64 {
    (n * 2.0).round() / 2.0
}
