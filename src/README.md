# BigFixed

A BigFixed is a fixed point number designed for arithmetic with rational numbers. BigFixed is a complementary positional arithmetic system with base `Digit` being one of the native unsigned integer types `u*` (except the maximal `u128`). For an overview of the theory behind BigFixed see the included document.

Indexing is done using the Index type. It combines isize and usize for unified indexing. Technically it just uses isize, rejecting usize values which are too large and asserting nonnegativity when converting to usize.

The components of a BigFixed are

```
- head: Digit
- body: Vec<Digit>
- position: Index
- precision: Index
```

*Do not construct a BigFixed directly* -- Use `BigFixed::construct` or any of the `BigFixed::from` converters if possible. There is a proper format (no redundant data in body, pure zero has no position, etc.) which some operations rely upon and which the constuctors ensure. The converters are self explanatory and `BigFixed::construct` simply constructs directly and then formats the result before releasing it.

Together head and body represent a sequence of digits in little endian order with the head repeating to positive infinite weight. Position and precision relate to the radix weight of the number and the tailing 0s after the body. Specifically `position` is the weight of the least body entry (or the least copy of `head` if the body is empty) and precision declares how many 0s are specified after this position. A precision of `None` means infinite precision (pure number) and a position of `None` only applies to pure zero.

Though not actually implemented by this code, for the sake of example take DIGIT to be modular arithmetic base ten. Then ALLONES is the number 9 and the resulting positional arithmetic is the familiar Hindu-Arabian decimal system. In this example the pure integer 1 is represented by

```
BigFixed {
    head: 0,
    body: [1],
    position: 0,
    precision: None
}
```

The number -150.00 is

```
BigFixed {
    head: 9,
    body: [8, 5],
    position: 1,
    precision: Some(3)
}
```

The number zero is special: The head is 0 and the body is empty. Position only makes sense if there is a precision; either both are present or both are not.
