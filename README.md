![travis](https://travis-ci.org/A1-Triard/phantom-type.svg?branch=master)

# phantom-type

A `PhantomData` analog which prevents "parameter is never used" error,
but does not produce any restrictions in contrast with `PhantomData`.

## Optional `no_std`

If your crate has `std` feature, use the following method to specify `phantom-type` dependency:

```toml
[features]
default = ["std"]
std = ["phantom-type/std"]

[dependencies]
phantom-type = { version = "0.3.0", default-features = false }
```
