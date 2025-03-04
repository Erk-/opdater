# Opdater

Small trait based approch to implement updating of structs.

The main goal of this crate is to allow updating a struct continously
without writing a lot of boilerplate code.

## Example

```rust
use opdater::Opdater;

#[derive(Debug, PartialEq, Opdater)]
struct Bla {
    a: Option<i32>,
    b: Option<f32>,
}

let mut bla = Bla { a: None, b: None };
let bla_op = Bla {
    a: Some(10),
    b: Some(13.37),
};

bla.update(bla_op);

assert_eq!(
    bla,
    Bla {
        a: Some(10),
        b: Some(13.37)
    }
);

let bla_op2 = Bla {
    a: Some(5),
    b: None,
};

bla.update(bla_op2);

assert_eq!(
    bla,
    Bla {
        a: Some(5),
        b: Some(13.37)
    }
);
```

## Etmylogy

Opdater means to update in Danish.

## License
This is licensed under the ISC License
