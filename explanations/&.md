# The `&` operator

In Rust, the "&" operator is the "bitwise AND" operator, which performs an "AND" operation on the bits of two integers. This means that for each bit, the result will be 1 if the corresponding bits are both 1, otherwise the result will be 0.

When you perform the 10 & 8 operation in Rust, the numbers are first converted to binary, which gives `1010` and `1000`, respectively. Then, the "&" operation is applied to each pair of bits, which gives `1000`, which is the binary representation of 8.

Thus, the result of the 10 & 8 operation is 8, because only the bit corresponding to the value 8 (the fourth bit from the right) is both 1 in 10 and 8. The other bits are different and therefore valued at 0 in the result.
