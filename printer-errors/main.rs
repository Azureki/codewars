fn printer_error(s: &str) -> String {
    let mut errors = 0;
    for c in s.chars(){
        if c > 'm'{
            errors += 1
        }
    }
    format!("{}/{}", errors, s.len())
}

pub fn main() {
    let s = "asdflkasjfd";
    println!("{}", 'b'>'a');
    println!("{}", printer_error(s));
}
