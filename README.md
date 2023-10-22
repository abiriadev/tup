# `tup!()` macro

Generate a tuple or tuple type by repeating value or type.

```rs
use tup::tup;

// make tuple of length 6 filled with 0
let new_tuple = tup!(0; 6);

// define a function accepting a tuple consisting of four `usize`
fn draw(box: tup!(usize; 4)) {
    // ...
}
```
