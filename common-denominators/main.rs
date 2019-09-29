fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if l.len() == 0 {
        return l;
    }
    let mut res :Vec<(i64, i64)> = Vec::new();
    for tup in l.iter(){
        let tem = gcd(tup.0, tup.1);
        res.push((tup.0/tem, tup.1/tem));
    }
    let mut least_common_multiple = res[0].1;
    for i in 1..res.len(){
        least_common_multiple = lcm(least_common_multiple, res[i].1);
    }
    for tup in res.iter_mut(){
         tup.0 *= least_common_multiple/tup.1;
         tup.1 = least_common_multiple;
    }
    res
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a%b)
    }
}

fn lcm(a: i64, b: i64)->i64 {
    a*b/gcd(a,b)
}
