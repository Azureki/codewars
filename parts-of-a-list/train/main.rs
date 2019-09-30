fn part_list(arr: Vec<&str>) -> String {
    let mut res = vec![];
    for i in 1..arr.len(){
        res.push(format!("({}, {})", arr[..i].join(" "), arr[i..].join(" ")));
    }
    res.join("")
}
