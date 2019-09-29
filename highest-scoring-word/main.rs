fn high(input: &str) -> &str {
    let mut pos = vec![0, 0];
    let mut score = 0;
    let mut highest_score = 0;
    let mut begin = 0;
    let mut i = 0;
    for c in input.bytes(){
        if c == b' '{
            if score>highest_score{
                highest_score=score;
                pos[0] = begin;
                pos[1] = i;
            }
            score=0;
            begin = i+1;
        }
        else{
            score += c - b'a' + 1;
        }
        i += 1;
    }
    if score>highest_score{
        pos[0] = begin;
        pos[1] = i;

    }
    &input[pos[0]..pos[1]]
}

fn main() {
    let str = "what time are we climbing up the volcano";
    println!("{}", high(str));
}
