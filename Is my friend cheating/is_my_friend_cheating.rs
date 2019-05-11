fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let n = m as i64;
    let num=(n*n+n)/2+1;
    let mut res:Vec<(i32,i32)>=Vec::new();
    //for i in 2..((NUM as f64).sqrt() as i32+1){
    for i in 2..n{
        if num%i==0&& num/i<n{
            res.push(((i - 1) as i32,(num/i-1) as i32));
        }
    }
    res
}