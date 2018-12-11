fn main() {
    race(720, 850, 70);
}
fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v2 <= v1 {
        let res: Option<Vec<i32>> = None;
        return res;
    }
    // let time:f64 = g/(v2-v1);
    let mut time = g * 3600 / (v2 - v1);
    // let mut res : Option<Vec<i32>> ;
    let sec = time % 60;
    time /= 60;
    let minutes = time % 60;
    let hours = time / 60;
    let res = vec![hours, minutes, sec];
    Some(res)
}
