# BigFixed Numbers

A BigFixed is a data format designed for arithmetic with arbitrarily precise binary-represented numbers. The distinction between fixed point and floating point is ultimately meaningless when precision is arbitrary but we choose to call them fixed point numbers for specific reasons which are not particularly important.

Floating point numbers are great -- everyone wants to use them and not worry about the details and for the most part that's fine -- but implementations are generally architecture-dependent (even if only slightly) and hence non-deterministic. The primary goal of BigFixed is to build a platform-independent floating point equivalent but with control over the various precision choices which go into creating a floating point number system.

**[The current stage of development is to build the background machinery to enable this goal. There are a few technical parameters exposed to a user at the moment. The ultimate plan is to, once the bulk of the capabilities have been hammered out, build a configuration scheme which handles the technical details and exposes a small set of parameters to the end user. They will choose a few values relating to how precise they need their numbers to be and the setup will handle all the details to get it to Just Work™ in the same way that familiar floating point numbers Just Work™]**

BigFixed is a positional arithmetic system with base `Digit` being one of the native unsigned integer types `u*` (except the maximal `u128`). **[During development Digit is u16 but in the final product the choice of Digit will be either u32, u64, or configurable with some setup scheme.]** Each BigFixed number has an infinite binary representation where all but finitely many bits are trivial. Digit partitions this sequence into chunks:

```
... 00000000 00101101 01001101.10100111 01000000 00000000 ...
// if Digit is u8
```

Many operations have extreme cases where overflow/other errors (mostly from index limitations) are possible. These are typically avoidable but still most operations return a `Result` type just in case.

Following Rust's native integer conventions, two's complement (aka complementary arithmetic) is used for everything except division. Division uses sign-magnitude.

## Indexes

Positional indexing is done using the `crate::Index` type. Index is itself an enum split into `Bit(isize)` and `Position(isize)`. Positional refers to the Digits which make up a BigFixed number and Bit refers to the Bit. If the BigFixed `x` has the binary representation given above (with Digit u8) then the number with `Index::Position(-1)` is `10100111` and the bit at `Index::Bit(10)` is this one:

```
... 00000000 00101[1]01 01001101.10100111 01000000 00000000 ...
```

Indexes can be added, multiplied, compared, and cast between enum versions. Casting from Bit to Position is essentially integer division and as such is a lossy operation.

The integer type `isize`, though vast, is finite. As such there are technically a large number of `Index::Position` values which are out of range of the corresponding `Index::Bit` variant. The `IndexError` is an error designed to represent these phenomena. Most index operations have extreme cases where an `IndexError` is possible so most operations return the `Result` type instead of direct results.

## The BigFixed Struct

The components of a BigFixed are

```
- head: Digit
- body: Vec<Digit>
- position: Index
```

Together head and body represent a sequence of Digits (little endian) with the head repeating to positive infinite positions. The head can be only 0 or ALLONES (the maximal Digit, it has binary expansion 111...111) as per complementary arithmetic. Position is the weight of the least significant body entry, i.e. the position of the radix point. The binary expansion is taken to be all 0s below this point.

There is a proper format for BigFixeds which, if broken, leads to algorithmic inefficiency or incompatibility. The position must be of type `Index::Position` (not Bit) and the body must not contain trivial data; data is trivial if it can be absorbed into the head or tail. Any BigFixed can be properly formatted by calling `self.format()`.

Coefficients in the expansion can be accessed by the standard operations Index and MutIndex using the `crate::Index` type (apologies for the name collision). Accessing a reference via Index gives the appropriate reference value (head, body internal, or tail) without changing the BigFixed or incurring reallocation. BigFixed numbers can also be indexed by an `isize` -- its value is coerced into an `Index::Position` first.

Taking a mutable reference first expands the body to the position if necessary, possibly reallocating, to guarantee that any mutation occurs inside the body. Taking mutable references can break the BigFixed format so make sure to reformat afterwards. If a contigous block of coefficients is to be altered then reallocation can be kept to a minimum by first calling `self.ensure_valid_range(...)` to bring the whole block space into mutable scope at once.

The number zero is special: The head is 0 and the body is empty. Position (the weight of the least nontrivial coefficient) is meaningless in this case so as a convention formatting zero always sets its position to 0. We could have chosen to set its position to some kind of infinity or maximal value but we didn't. Having a position of 0 corresponds nicely with being a small integer -- all other integers between -ALLONES and ALLONES have a position of 0 as well.

## Conversions

All of Rust's native integer types can be converted to and from BigFixed using `std::convert` syntax. Converting from an integer to a BigFixed is always lossless.

Obtaining an integer from a BigFixed is not lossless. For unsigned integer types like `u32::from(&x)` where x is a BigFixed is a direct bit cast for the corresponding bit range. On the other hand, taking an unsigned integer is a saturating operation. If the value of `x` is too big (positive or negative) then the result is the maximal appropriate `i*` value. **[The rounding scheme for fractional values is not currently explicitly defined but will eventually agree with that of Rust's native *float_type*::round approach.]**

BigFixed also has a general floating point conversion. The international standard for floating point format IEEE 754 contains three parameters for a bitwise floating point data format: exponent width, exponent bias, and significand width. Any floating point number following this standard can be converted back and forth with BigFixed with a simple conversion based on the `self.float_from_bits` and `self.float_to_bits` methods. This covers the Rust native f32 and f64 types as well as floating point types of other crates.

## Operations

Most of the operations in `std::ops` are implemented for BigFixed. All but division are lossless operations; division is special and has only a precisioned implementation.

Of the standard (non-division) operations, all of them are based on `OpAssign`. That is, taking `&x + &y` first constructs a new BigFixed `c = x.clone()` and then calls `c += &y`, returning `c`. Whenever possible use the `OpAssign` version to minimize allocations. The `OpAssign` implementations are done in place, resizing (if necessary) instead of reconstructing the Vec of the body.

**[Division is possible at the moment using `BigFixed::combined_div(&mut num, &denom, places)`. Long division is used to compute the quotient and remainder simultaneously (hence the name `combined`) out to the specified number of places. This particular structure was chosen to minimize reallocations in the computation -- it is intended to be the machinery behind more user-friendly division and remainder operation API access points in the future. The numerator is modified during execution of the algorithm and at the end it contians the value of the remainder; the quotient is constructed and returned.]**

## Cutoffs

Although pure operations on BigFixed numbers are lossless, typically in practice we actually do not want the full precision of the result. Our numbers can quickly become unimaginably precisely defined (thousands or millions of digits), slowing down our code in order to retain precision levels which we ultimately do not care about anyway. This problem exists for all the standard operations but multiplication is the primary culprit.

The way we handle this is with cutoffs. Cutoffs and the Cutoff struct are substantial enough to warrant their own documentation **[in the works]** but the main idea is to specify how much precision we require of our result and to only compute out to that level of precision.

All of the standard operations have cutoff variants which do precisely this. Calling them is as easy as feeding a pair `(&b, c)` where we would have fed `&b` for the lossless version, i.e.

```
// a and b are BigFixeds and c is a Cutoff
let full_product = &a * &b;
let cutoff_product = &a * (&b, c);
```

**[The cutoff machinery is in place but its end-user API has not been finalized. The plan is to have a configuration object which contains two global Cutoffs -- one for internal computations and one for final results. The former encodes higher precision than the latter. See https://stackoverflow.com/questions/612507/what-are-the-applications-benefits-of-an-80-bit-extended-precision-data-type -- using 80 bit numbers for computations of 64 bit numbers]**

# Examples

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
...9999850.00000... = -150 equivalent value
```

Therefore if we want `x` to be the BigFixed representing -150 we take

```
let x = BigFixed {
    head: 9,
    body: vec![5, 8], // little endian
    position: 1
}
```

Taking `x[0]` gives (a reference to the Digit) 0 (as does `x[-1], x[-2],...`), while `x[1] == 5`, `x[2] == 8`, and `x[3] == x[4] == ... == 9`.

We can turn `x` into -149 by setting `x[0] = 1`, or into -151 by setting `x[1] = 5` and `x[0] = 9`. Both of these resize (and potentially reallocate) the body to length 3. Of course we could have alternately done `x (+/-)= BigFixed::from(1)` and let BigFixed figure out the details. This approach incurs the minor but noteworthy cost of constructing a BigFixed (1) on top of resizing the body of `x`.
