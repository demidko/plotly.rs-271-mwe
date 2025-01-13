# MWE for [plotly.rs/issues/271](https://github.com/plotly/plotly.rs/issues/271)

Full example in [`src/main.rs`](src/main.rs). You can reproduce the error as follows

```shell
cargo build
```

## Brief

The problem is that code using `pub enum GroupNorm` does not compile. The following code examples do not work

```rust
use plotly::GroupNorm;
```

```
error[E0432]: unresolved import `plotly::GroupNorm`
 --> src/main.rs:1:14
  |
1 | use plotly::{GroupNorm, Plot, Scatter};
  |              ^^^^^^^^^ no `GroupNorm` in the root
```

**Although `enum GroupNorm` is declared as `pub`, its parent module `scatter` is private**

```rust
use plotly::traces::scatter::GroupNorm;
```

```
error[E0603]: module `scatter` is private
  --> src/main.rs:1:21
   |
1  | use plotly::traces::scatter::GroupNorm;
   |                     ^^^^^^^  --------- enum `GroupNorm` is not publicly re-exported
   |                     |
   |                     private module
```
