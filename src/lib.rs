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

/// Gets a random owned item (by cloning) from a sequential collection.
/// # Example
/// ```
/// use random_item::random_owned_item;
/// 
/// let messages = Vec::from([
///    String::from("Hello, "),
///    String::from("fellow "),
///    String::from("Rustaceans!"),
/// ]);
/// 
/// let random_message = random_owned_item(&messages);
/// assert!(messages.contains(&random_message));
/// ```
pub fn random_owned_item<Type: Clone>(collection: &[Type]) -> Type {
    random_item(collection).clone()
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

    #[test]
    fn gets_owned_item_from_vec() {
        let names = Vec::from([
            String::from("Aiko"),
            String::from("Amir"),
            String::from("Nia"),
            String::from("Luca"),
            String::from("Zara"),
            String::from("Ravi"),
            String::from("Suki"),
            String::from("Omar"),
            String::from("Lila"),
            String::from("Kian"),
        ]);

        let random_name = random_owned_item(&names);
        assert!(names.contains(&random_name));
    }

    #[test]
    fn gets_owned_item_from_array() {
        #[derive(PartialEq, Clone)]
        struct Shape {
            sides: u32,
        }

        let shapes = [
            Shape { sides: 1 },
            Shape { sides: 1 },
            Shape { sides: 2 },
            Shape { sides: 3 },
            Shape { sides: 5 },
            Shape { sides: 8 },
            Shape { sides: 13 },
            Shape { sides: 21 },
        ];

        let random_shape = random_owned_item(&shapes);
        assert!(shapes.contains(&random_shape));
    }
}
