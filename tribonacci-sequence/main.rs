fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut res = signature.to_vec();
    res.resize(n, 0.0);
    for i in 3..n{
        res[i] = res[i-1]+res[i-2]+res[i-3];
    }
    res
}
