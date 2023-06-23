#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.len() < second_list.len() && check(first_list, second_list) {
        return Comparison::Sublist;
    };
    if first_list.len() > second_list.len() && check(second_list, first_list) {
        return Comparison::Superlist;
    };
    Comparison::Unequal
}

fn check<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|candidate| candidate == first_list)
}
