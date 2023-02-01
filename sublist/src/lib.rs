#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();
    match first_len.cmp(&second_len) {
        std::cmp::Ordering::Equal => {
            match is_equal(_first_list, _second_list, 0, 0) {
                true => return Comparison::Equal,
                false => return Comparison::Unequal,
            }
        }
        std::cmp::Ordering::Less => {
            for x in 0..=second_len-first_len {
                match is_equal(_first_list, _second_list, 0, x) {
                    true => return Comparison::Sublist,
                    false => continue,
                }
            }
            Comparison::Unequal
        },
        std::cmp::Ordering::Greater => {
            for x in 0..=first_len-second_len {
                match is_equal(_first_list, _second_list, x, 0) {
                    true => return Comparison::Superlist,
                    false => continue,
                }
            }
            Comparison::Unequal
        }
    }
}

fn is_equal<T: PartialEq>(_first_list: &[T], _second_list: &[T], skip_first: usize, skip_second: usize) -> bool {
    for (i, j) in _first_list.iter().skip(skip_first).zip(_second_list.iter().skip(skip_second)) {
        match i.eq(j) {
            false => return false,
            true => continue,
        }
    }
    return true
}
