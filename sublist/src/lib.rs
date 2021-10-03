#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Option<Comparison> {
    for i in 0..second_list.len() {
        if i + first_list.len() > second_list.len() {
            return None;
        }
        if first_list == &second_list[i..first_list.len() + i] {
            return Some(Comparison::Sublist)
        }
    }

    None
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.eq(second_list) {
        return Comparison::Equal;
    } else {
        if is_sublist(first_list, second_list) == Some(Comparison::Sublist) {
            return Comparison::Sublist;
        } if is_sublist(second_list, first_list) == Some(Comparison::Sublist) {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}
