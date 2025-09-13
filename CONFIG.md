# Advanced Configuration

This doc tackles advanced configuration options, it assumed you've read the main [README.md](https://github.com/tnballo/scapegoat/blob/master/README.md).

## Runtime Configuration

### Tuning the the tree's `a` factor

The [original scapegoat tree paper's](https://people.csail.mit.edu/rivest/pubs/GR93.pdf) alpha, `a`, can be chosen in the range `0.5 <= a < 1.0`.
`a` tunes how "aggressively" the data structure self-balances.
It controls the trade-off between total rebuild time and maximum height guarantees.

* As `a` approaches `0.5`, the tree will rebalance more often. Ths means slower insertions, but faster lookups and deletions.
	* An `a` equal to `0.5` means a tree that always maintains a perfect balance (e.g."complete" binary tree, at all times).

* As `a` approaches `1.0`, the tree will rebalance less often. This means quicker insertions, but slower lookups and deletions.
	* If `a` reached `1.0`, it'd mean a tree that never rebalances.

We choose 2/3, e.g. `a = 0.666...`, by default.

* The original paper does not recommend any specific value for `a`, but Figure 4 shows comparative results for values in the range `0.55 <= a < 0.75`. The authors recommend tuning `a` for your expected workload.

* This is the same  default used in the [recursive] [Open Data Structures textbook implementation](https://opendatastructures.org/ods-java/8_Scapegoat_Trees.html) of a scapegoat tree.

Note our default is almost exactly in the middle of the paper's range, suggesting it's a balanced choice (pun intended).

`a` can be changed at runtime via the API `set_rebal_param(alpha_num: U12F20, alpha_denom: U12F20)`.
The library's performance characteristics can be tuned on-the-fly, without recompiling.
For example, manually setting the default 2/3 would be:

```rust
use scapegoat::SgMap;
let mut map: SgMap<isize, isize, 10> = SgMap::new();
assert!(map.set_rebal_param(2.0, 3.0).is_ok());
```

## Features for Compile-time Configuration

> **WARNING:** Please do *NOT* enable any of the below optional or experimental features if publishing your project on [crates.io](https://crates.io/).
> Features are additive. Suppose an upstream project that uses your project as a dependency also uses another downstream dependency that uses this library (e.g. 2+ transitive dependencies on `scapegoat` in a single build).
> If you enabled a feature: all code would compile, *but* would not have the runtime performance characteristic expected!

### The `low_mem_insert` feature (Optional)

If this feature is enabled, the internal arena doesn't maintain a free list.
Removing this metadata saves stack space (lower memory footprint) but significantly slows down insertion (higher runtime).

* **Memory gain if enabled:** save up to `self.capacity() * core::mem::size_of<u16>()` per instance of set/map.

* **Runtime penalty if enabled:** `insert` becomes `O(n)` instead of `O(log n)`. The larger the arena, the more that matters (algorithmic complexity downgrade). `get` and `remove` remain unchanged.

### The `fast_rebalance` feature (Optional)

If this feature is enabled, every node stores an additional piece of internal metadata: subtree size.
This metadata increases stack space usage (higher memory footprint) but significantly speeds up rebalancing operations (faster runtime).

* **Memory penalty if enabled:** costs up to `self.capacity() * core::mem::size_of<u16>()` per instance of set/map.

* **Runtime gain if enabled:** does not change algorithmic complexity, but `insert` becomes faster. `get` remains unchanged. Due to extra book keeping needed to keep subtree size caches updated following node removal, `remove` slows down for the average case but may improve for the worst case.

### The `alt_impl` feature (Experimental)

By default, this library uses the algorithms proposed in the original paper ([Galperin and Rivest, 1993](https://people.csail.mit.edu/rivest/pubs/GR93.pdf)).
The `alt_impl` feature enables optimizations proposed in the subsequent PhD thesis ([Galperin, 1996](https://dspace.mit.edu/handle/1721.1/10639)).

> **Warning:** This feature is currently a work in progress, it's not finished or guaranteed to be an improvement (e.g. the implementation may be incorrect). But risk is low - this feature only affects performance.
>
> The feature-gate means we can compare the two modes before potentially setting a new default in a future version.
> Beyond that point the non-default is only worth supporting if it's measurably superior for some usecase.

The main optimization is eliminating recursion.
This library already does that, but likely in a way inferior to the "official" algorithm (implemented prior to find/reading the thesis). Please see thesis pages 95 and 97 for the algorithm's pseudo code (needs translation to Rust!).

