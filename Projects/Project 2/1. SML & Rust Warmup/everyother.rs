fn everyother(input: &[i32]) -> Vec<i32> {
    match input {
        [] => Vec::new(),
        [_] => Vec::new(),
        [_, l, rest @ ..] => {
            let mut result = Vec::with_capacity(input.len() / 2);
            result.push(*l);
            result.extend(everyother(rest));
            result
        }
    }
}

fn main() {
    let input = vec![3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];
    let result = everyother(&input);
    println!("{:?}", result);

    assert_eq!(result, vec![5, 11, 17, 29, 41], " Test failed");
}
