# Random Item

Get a random item out of a sequential collection.

## Getting A Reference

```rs
use random_item::random_item;

let messages = Vec::from([
   String::from("Hello, "),
   String::from("fellow "),
   String::from("Rustaceans!"),
]);

let random_message = random_owned_item(&messages);
```

## Getting An Owned Value (Cloned)

```rs
use random_item::random_item;

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
```
