#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // Inverting verify_sublist I can verify if the first list is a superlist of the second list
    fn verify_sublist<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
        if needle.len() == 0 {
            return true;
        }
        while !haystack.is_empty() {
            if haystack.starts_with(needle) {
                return true;
            }
            haystack = &haystack[1..];
        }
        false
    }

    match (_first_list, _second_list) {
        (first_list, second_list) if first_list == second_list => Comparison::Equal,
        (first_list, second_list) if verify_sublist(second_list, first_list) => Comparison::Sublist,
        (first_list, second_list) if verify_sublist(first_list, second_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
