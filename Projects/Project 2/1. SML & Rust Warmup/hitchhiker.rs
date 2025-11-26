fn main() {
    println!("{}", hitchhiker(&[1, 2, 3, 4, 5, 42, 6, 7, 8, 9, 10]));
    println!("{}", hitchhiker(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    assert_eq!(hitchhiker(&[]), false);
    assert_eq!(hitchhiker(&[42]), true);
    assert_eq!(hitchhiker(&[1, 2, 3, 4, 5, 42, 6, 7, 8, 9, 10]), true);
    assert_eq!(hitchhiker(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), false);
}

fn hitchhiker(list: &[i32]) -> bool {
    match list {
        [] => false,
        [42, ..] => true,
        [_head, tail @ ..] => hitchhiker(tail),
    }
}
