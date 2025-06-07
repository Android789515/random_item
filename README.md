# Random Item

Get a random item out of a sequential collection.

## Getting A Reference

```rs
use random_item::random_item;

let numbers = (0..99).collect::<Vec<u8>>();

let random_number = random_item(&numbers);
```

## Getting An Owned Value (Cloned)

```rs
use random_item::random_item;

let messages = Vec::from([
   String::from("Hello, "),
   String::from("fellow "),
   String::from("Rustaceans!"),
]);

let random_message = random_owned_item(&messages);
```

```rs
use random_item::random_item;

#[derive(PartialEq, Clone)]
struct Shape {
   sides: u32,
}

let fruits = [
   Shape { sides: 1 },
   Shape { sides: 1 },
   Shape { sides: 2 },
   Shape { sides: 3 },
   Shape { sides: 5 },
   Shape { sides: 8 },
   Shape { sides: 13 },
   Shape { sides: 21 },
];

let random_fruit = random_owned_item(&fruits);
```
