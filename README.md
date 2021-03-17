[![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]

# schwifty

A simple IBAN validation library inspired by Python's `schwifty`.

## Sample Usage
```rust
    assert!(schwifty::validate("GB82 WEST 1234 5698 7654 32").is_ok());
```

[ci]: https://github.com/Elinvynia/schwifty/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/schwifty/Rust/master?style=flat-square
[docs]: https://docs.rs/schwifty
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
[crate-link]: https://crates.io/crates/schwifty
[crate-version]: https://img.shields.io/crates/v/schwifty.svg?style=flat-square
