# board-repr-perft

Performance testing various board representations in Dellacherie's algorithm.

- `naive` represents the board as a 10x40 row-major array of `bool`s.
- `row bits` represents the board as row-major bitboard; an array of 40 `u16`s.
- `col bits` represents the board as a column-major bitboard; an array of 10 `u64`s.
- The column height variations augment the board with an array of column height values.
- The `pext` version of `col bits` uses the x86_64 instruction `pext` to compute
  line clears, which is not available on all platforms.

![](results.svg)

Measured on an AMD Ryzen 7 3700X. Compiled with `-C target-cpu=native`. Note
that Zen 2 (such as my 3700X) and earlier AMD CPUs have slow implementations of
`pext` - `col bits pext` should be even faster than `col bits` on Intel CPUS and
Zen 3 or later AMD CPUs.
