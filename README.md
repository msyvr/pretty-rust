# made with &#x2661;, written in Rust

## julia fractal
```rust
let mut i = 0;
while i < 255 && z.norm() <= 2 {
    z = z * z + c;
    i += 1;
}
```

![julia fractal <= 2.0](static/fractal_lt2.png)

```rust
let mut i = 0;
while i < 255 && z.norm() <= 1.1 {
    z = z * z + c;
    i += 1;
}
```
![julia fractal <= 1.1](static/fractal_lt1p1.png)

```rust
let columns = 1500;
let rows = 1500;

let k_cols = 3.0 / columns as f32;
let k_rows = 3.0 / rows as f32;
```
![julia fractal square](static/fractal_square.png)