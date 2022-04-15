# BigFixed

A BigFixed is a data format designed for arithmetic with rational numbers, specifically with those numbers which have truncating binary expansion. The distinction between fixed point and floating point is somewhat arbitrary in this context, yet we exclusively call it fixed point for reasons which are not very important at the moment.

 BigFixed is a complementary positional arithmetic system with base `Digit` being one of the native unsigned integer types `u*` (except the maximal `u128`, Digit uses a DoubleDigit for calculations). For an overview of the theory behind BigFixed see the included document.

Positional indexing is done using the `crate::Index` type. It combines isize and usize for unified indexing. Technically it just uses isize, rejecting usize values which are too large and asserting nonnegativity when converting to usize. Many Index operations are overloaded to work with isize and usize directly.

The components of a BigFixed are

```
- head: Digit
- body: Vec<Digit>
- position: Index
```

*Do not construct a BigFixed directly* -- Use `BigFixed::construct` or any of the `BigFixed::from` converters if possible. There is a proper format (no redundant data in body, pure zero's position, etc.) which some operations rely upon and which the constuctors ensure. The converters are self explanatory and `BigFixed::construct` simply constructs directly and then formats the result before releasing it.

Together head and body represent a sequence of Digits (little endian) with the head repeating to positive infinite weight. The head can be only 0 or ALLONES (the maximal nonzero Digit, it has binary expansion 111...111) as per complementary arithmetic. Position is the weight of the least significant body entry, i.e. the position of the radix point. The expansion is taken to be all 0s after this point.

Coefficients in the expansion can be accessed by Index and MutIndex, using the `crate::Index` type (apologies for the name collision). Accessing a reference via Index gives the appropriate reference value (head, body internal, or tail) without changing the BigFixed or incurring reallocation. Taking a mutable reference first expands the body to the position if necessary, possibly reallocating, to guarantee that any mutation occurs inside the body. Mutable references break the format so make sure to reformat afterwards. If a contigous block of coefficients is to be altered, reallocation can be kept to a minimum by first calling `ensure_valid_range` to bring the whole block space into mutable scope at once.

_________________________

Though not actually implemented by this code, for the sake of example take Digit to be modular arithmetic base ten. Then ALLONES is the number 9 and the resulting positional arithmetic is the familiar Hindu-Arabian decimal system. In this example the integer 1, with decimal expansion `...0001.000...`, is represented by

```
BigFixed {
    head: 0,
    body: vec![1],
    position: 0
}
```

The number -150.00, a negative number, gets its expansion from the complementary calculation
```
...0000150.00000... =  150
...9999849.99999... = -150 direct opposite
...9999850.00000... = -150 geometric series equivalent
```

Therefore if we want `x` to be the BigFixed representing -150 we take

```
let x = BigFixed {
    head: 9,
    body: vec![5, 8], // little endian; vecs have index 0 on the left
    position: 1
}
```

Taking `x[0]` gives (a reference to the Digit) 0 (as does `x[-1], x[-2],...`), while `x[1] == 5`, `x[2] == 8`, and `x[3] == x[4] == ... == 9`.

We can turn `x` into -149 by setting `x[0] = 1`, or into -151 by setting `x[1] = 5` and `x[0] = 9`. Both of these resize (and potentially reallocate) the body to length 3. Of course we could have alternately done `x (+/-)= BigFixed::from(1)` and let BigFixed figure out the details, at the minor but noteworthy cost of constructing a BigFixed (1) on top of resizing the body of `x`.

The number zero is special: The head is 0 and the body is empty. Position (the weight of the least nonzero coefficient) is meaningless in this case, so as a convention formatting zero always sets its position to 0. We could have chosen to sets its position to some kind of infinity or maximal value but it is generally better to avoid those if possible. And having a position of 0 corresponds nicely with being a small nonnegative integer -- all integers between 1 and ALLONES have a position of 0 as well.
