# BigFixed

A BigFixed is an arbitrary precision fixed point number designed for arbitrary precision arithmetic. BigFixed is a two's complement positional arithmetic system with base `Digit` being one of the native unsigned integer types `u*` (except the maximal `u128`). Little endian positional order is used throughout. For an overview of the theory behind BigFixed see the included document.

The components of a single BigFixed are

```
- data: Vec<Digit>
- position: isize
```

*Do not construct a BigFixed directly* -- Use `BigFixed::construct` or any of the `BigFixed::from` converters instead. Some operations such as equality do not work if data is not trimmed.

Data contains the nontrivial coefficients of the positional expansion stored in little endian order. Trivial coefficients are 0s in the tail and either 0s or -1s (as Digit) in the head depending on the sign of the BigFixed. Position is the weight of the least significant datum `data[0]`.

The fraction part of a BigFixed is all the coefficients with negative weight. The integer part is all the coefficients with nonnegative weight. Either part can be cast to a BigInt. The integer part is equivalent to the floor. Alternatively data can itself be cast to a BigInt, though this does not consider position.

BigFixeds can be constructed from any pair `(i,d)` where `i` is a BigInt and `d` is a BigUint. Integral BigFixeds can be constructed from any of the native integer types `u*`, `i*`. Any native floating point type `f*` can be converted to a BigFixed.

BigFixed supports most of the arithmetic operations in `std::ops` with some caveats:

- Division must be given a precision parameter
