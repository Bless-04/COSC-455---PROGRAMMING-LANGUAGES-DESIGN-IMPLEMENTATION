/// everything but 0 is true
fn int_to_bool(xs: &[i32]) -> Vec<bool> {
    match xs {
        [] => vec![],
        [0, rest @ ..] => {
            let mut r = vec![false];
            r.extend(int_to_bool(rest));
            r
        }
        [_, rest @ ..] => {
            let mut r = vec![true];
            r.extend(int_to_bool(rest));
            r
        }
    }
}

// boolToInt: reverse of intToBool
fn bool_to_int(xs: &[bool]) -> Vec<i32> {
    match xs {
        [] => vec![],
        [false, rest @ ..] => {
            let mut r = vec![0];
            r.extend(bool_to_int(rest));
            r
        }
        [true, rest @ ..] => {
            let mut r = vec![1];
            r.extend(bool_to_int(rest));
            r
        }
    }
}

// padZeros: prepend n zeros
fn pad_zeros(n: usize, xs: &[i32]) -> Vec<i32> {
    if n == 0 {
        xs.to_vec()
    } else {
        let mut r = vec![0];
        r.extend(pad_zeros(n - 1, xs));
        r
    }
}

// alignLists: pad shorter list to equal length
fn align_lists(l1: &[i32], l2: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let len1 = l1.len();
    let len2 = l2.len();

    if len1 > len2 {
        (l1.to_vec(), pad_zeros(len1 - len2, l2))
    } else if len2 > len1 {
        (pad_zeros(len2 - len1, l1), l2.to_vec())
    } else {
        (l1.to_vec(), l2.to_vec())
    }
}

// doBinaryAddition: add two bool lists with carry
fn do_binary_addition(l1: &[bool], l2: &[bool], carry: bool) -> Vec<bool> {
    match (l1, l2) {
        ([], []) => {
            if carry {
                vec![true]
            } else {
                vec![]
            }
        }

        ([], _) => do_binary_addition(&[false][..], l2, carry),

        (_, []) => do_binary_addition(l1, &[false][..], carry),

        ([x, xs @ ..], [y, ys @ ..]) => {
            let sum_val = (*x as i32) + (*y as i32) + (carry as i32);

            let result_bit = (sum_val % 2) != 0;
            let new_carry = sum_val >= 2;

            let mut r = vec![result_bit];
            r.extend(do_binary_addition(xs, ys, new_carry));
            r
        }
    }
}

// binaryAddition: convert → bool, reverse, add, reverse → int
fn binary_addition(l1: &[i32], l2: &[i32]) -> Vec<i32> {
    let b1 = int_to_bool(l1);
    let b2 = int_to_bool(l2);

    let r1_bool = reverse_bool(&b1);
    let r2_bool = reverse_bool(&b2);

    let sum_rev = do_binary_addition(&r1_bool, &r2_bool, false);
    let sum = reverse_bool(&sum_rev);

    bool_to_int(&sum)
}

// helper to reverse bool list
fn reverse_bool(xs: &[bool]) -> Vec<bool> {
    match xs {
        [] => vec![],
        [x, rest @ ..] => {
            let mut r = reverse_bool(rest);
            r.push(*x);
            r
        }
    }
}

fn trim_zeros(xs: &[i32]) -> Vec<i32> {
    match xs {
        [] => vec![0],
        [0] => vec![0],
        [0, rest @ ..] => trim_zeros(rest),
        _ => xs.to_vec(),
    }
}

fn binary_subtraction(l1: &[i32], l2: &[i32]) -> Vec<i32> {
    let (al1, al2) = align_lists(l1, l2);
    let target_len = al1.len();

    // 1's complement of B
    let b2_bool = int_to_bool(&al2);
    let b2_not: Vec<bool> = bool_map_not(&b2_bool);
    let b2_inv = bool_to_int(&b2_not);

    // 2's complement
    let twos_comp = binary_addition(&b2_inv, &[1]);

    // raw subtraction = A + 2's complement
    let raw = binary_addition(&al1, &twos_comp);

    // if result longer, drop first bit
    if raw.len() > target_len {
        trim_zeros(&raw[1..])
    } else {
        raw
    }
}

fn bool_map_not(xs: &[bool]) -> Vec<bool> {
    match xs {
        [] => vec![],
        [x, rest @ ..] => {
            let mut r = vec![!*x];
            r.extend(bool_map_not(rest));
            r
        }
    }
}

fn main() {
    assert_eq!(binary_addition(&[1, 0], &[1, 0]), vec![1, 0, 0]);
    assert_eq!(
        binary_addition(&[1, 1, 1, 1, 0], &[1, 0, 1, 1]),
        vec![1, 0, 1, 0, 0, 1]
    );
    assert_eq!(
        binary_addition(&[1, 0, 0, 1, 1, 0, 1], &[0, 0, 1, 0, 0, 1, 0]),
        vec![1, 0, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        binary_addition(&[1, 0, 0, 1, 0, 0, 1], &[0, 0, 1, 1, 0, 0, 1]),
        vec![1, 1, 0, 0, 0, 1, 0]
    );
    assert_eq!(
        binary_addition(&[1, 0, 0, 0, 1, 1, 1], &[1, 0, 1, 1, 0]),
        vec![1, 0, 1, 1, 1, 0, 1]
    );

    assert_eq!(binary_subtraction(&[1, 0, 0, 1], &[1, 0, 1]), vec![1, 0, 0]);
    assert_eq!(binary_subtraction(&[1, 0, 0, 1], &[1, 0, 0]), vec![1, 0, 1]);

    println!("Tests passing");
}
