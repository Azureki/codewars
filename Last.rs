fn main() {}

fn last<T: Clone>(slice: &[T]) -> T {
    let length = slice.len();
    slice[length - 1].clone()
}
