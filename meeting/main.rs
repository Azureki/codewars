fn meeting2(s: &str) -> String {
    let mut res: Vec<String> = s.to_ascii_uppercase().split(';')
        .map(|s|s.split(':').collect::<Vec<&str>>())
        .map(|s|format!("({}, {})", s[1], s[0]))
        .collect();
    res.sort();
    res.concat()
}

// use join twice, format once
fn meeting(s: &str) -> String {
    let mut res: Vec<String> = s.split(';')
        .map(|s|s.to_ascii_uppercase().split(':').rev().collect::<Vec<&str>>().join(", "))
        .collect();
    res.sort();
    res.iter().map(|s|format!("({})", s)).collect::<Vec<String>>().join("")
}

pub fn main() {
    let res = meeting("Alexis:Wahl;John:Bell;Victoria:Schwarz");
    println!("{}", res);
}
