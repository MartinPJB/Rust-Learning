# The `|` operator
In Rust, the "|" operator is the "bitwise OR" operator, which performs an "OR" operation on the bits of two integers. This means that for each bit, the result will be 1 if either or both of the corresponding bits are 1, otherwise the result will be 0.

When you perform the 2 | 9 operation in Rust, the numbers are first converted to binary, which gives `0010` and `1001`, respectively. Then, the "|" operation is applied to each pair of bits, which gives `1011`, which is the binary representation of 11.

Thus, the result of the 2 | 9 operation is 11, because the bits corresponding to the values 2 and 9 (the second and fourth bits from the right) are both 0 in one number and 1 in the other. The other bits are the same and therefore valued at 1 in the result.
