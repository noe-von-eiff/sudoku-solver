use std::collections::HashSet;

pub fn has_duplicates(arr: &[u8]) -> bool {
    let mut set: HashSet<u8> = HashSet::new();

    for &element in arr {
        if set.contains(&element) {
            return true; // Found a duplicate
        }
        set.insert(element);
    }

    false // No duplicates found
}