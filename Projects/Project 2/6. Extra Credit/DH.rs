#[derive(Debug, Clone, Copy, PartialEq)]
enum Store {
    Leftorium,
    SprawlMart,
    TryNSave,
    KingToots,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gift {
    GreenDress,
    SaxophoneBook,
    Slingshot,
    Pacifier,
}

impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for Gift {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let stores = vec![
        Store::Leftorium,
        Store::SprawlMart,
        Store::TryNSave,
        Store::KingToots,
    ];

    let gifts = vec![
        Gift::GreenDress,
        Gift::SaxophoneBook,
        Gift::Slingshot,
        Gift::Pacifier,
    ];

    // Generate all permutations
    let store_perms = generate_permutations(stores);
    let gift_perms = generate_permutations(gifts);

    solve_recursively(&store_perms, &gift_perms, 0, 0);
}

fn solve_recursively(
    s_perms: &Vec<Vec<Store>>,
    g_perms: &Vec<Vec<Gift>>,
    s_idx: usize,
    g_idx: usize,
) {
    // Base case: Checked all store permutations
    if s_idx >= s_perms.len() {
        println!("D'oh! No solution found.");
        return;
    }

    // Base case: Checked all gift permutations for this store perm -> move to next store perm
    if g_idx >= g_perms.len() {
        solve_recursively(s_perms, g_perms, s_idx + 1, 0);
        return;
    }

    // Check current combination
    if is_valid_solution(&s_perms[s_idx], &g_perms[g_idx]) {
        print_solution_recursive(&s_perms[s_idx], &g_perms[g_idx], 0);
        return;
    }

    // Recurse to next gift permutation
    solve_recursively(s_perms, g_perms, s_idx, g_idx + 1);
}

// Checks if a  combination of stores and gifts meets all criteria
fn is_valid_solution(stores: &Vec<Store>, gifts: &Vec<Gift>) -> bool {
    //the Leftorium was his second stop (index 1)
    if stores[1] != Store::Leftorium {
        return false;
    }

    // : He bought the saxophone book at King Toots
    let king_toots_index = find_index_recursive(stores, &Store::KingToots, 0);
    if let Some(idx) = king_toots_index {
        if gifts[idx] != Gift::SaxophoneBook {
            return false;
        }
    } else {
        return false;
    }

    // Two stops after leaving Try-N-Save
    let try_n_save_index = find_index_recursive(stores, &Store::TryNSave, 0);
    if let Some(idx) = try_n_save_index {
        if idx + 2 >= stores.len() {
            return false;
        }
        if gifts[idx + 2] != Gift::Pacifier {
            return false;
        }
    } else {
        return false;
    }

    //  The store he visited immediately after buying the slingshot was not Sprawl-Mart
    let slingshot_index = find_index_recursive(gifts, &Gift::Slingshot, 0);
    if let Some(idx) = slingshot_index {
        if idx + 1 >= stores.len() {
            return false;
        }
        if stores[idx + 1] == Store::SprawlMart {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn find_index_recursive<T: PartialEq>(items: &Vec<T>, target: &T, index: usize) -> Option<usize> {
    if index >= items.len() {
        return None;
    }
    if &items[index] == target {
        return Some(index);
    }
    find_index_recursive(items, target, index + 1)
}

fn print_solution_recursive(stores: &Vec<Store>, gifts: &Vec<Gift>, index: usize) {
    if index == 0 {
        println!("Found the solution:\n");
    }
    if index >= stores.len() {
        return;
    }
    println!(
        "Stop {}: Store = {:<15} | Gift = {}",
        index + 1,
        stores[index],
        gifts[index]
    );
    print_solution_recursive(stores, gifts, index + 1);
}

fn generate_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut items_clone = items.clone();
    let len = items.len();
    permute_recursive(&mut items_clone, 0, len, &mut result);
    result
}

// The outer recursive function for Heaps algorithm
fn permute_recursive<T: Clone>(
    items: &mut Vec<T>,
    start: usize,
    len: usize,
    result: &mut Vec<Vec<T>>,
) {
    if start >= len {
        result.push(items.clone());
        return;
    }

    permute_loop(items, start, start, len, result);
}

fn permute_loop<T: Clone>(
    items: &mut Vec<T>,
    start: usize,
    i: usize,
    len: usize,
    result: &mut Vec<Vec<T>>,
) {
    if i >= len {
        return;
    }

    items.swap(start, i);
    permute_recursive(items, start + 1, len, result);
    items.swap(start, i); // Backtrack

    // Continue the "loop"
    permute_loop(items, start, i + 1, len, result);
}
