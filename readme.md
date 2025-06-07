# Random Item

Get a random item out of a sequential collection.

## Example

```rs
let numbers = (0..99).collect::<Vec<u8>>();

assert!(numbers.contains(random_item(&numbers)));
```
