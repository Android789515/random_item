//! # Random Item
//!
//! `random_item` lets you get a random item from a sequential collection.

/// Gets a random item from a sequential collection.
/// # Example
/// ```
/// use random_item::random_item;
/// 
/// let numbers = (0..99).collect::<Vec<u8>>();
///
/// assert!(numbers.contains(random_item(&numbers)));
/// ```
pub fn random_item<Type>(collection: &[Type]) -> &Type {
    &collection[rand::random_range(0..collection.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_item_from_vec() {
        let numbers = (0..99).collect::<Vec<u8>>();

        assert!(numbers.contains(random_item(&numbers)));
    }

    #[test]
    fn gets_item_from_array() {
        let letters = ['a', 'b', 'c'];

        assert!(letters.contains(random_item(&letters)));
    }
}
