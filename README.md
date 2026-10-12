# bank-barcode

Generate Finnish virtual bank barcodes (*pankkiviivakoodi*) in Rust.

This crate produces the digit string for both version 4 and version 5 of the
Finnish virtual bank barcode as defined in the
[Finanssiala specification][spec]. Bank barcodes may only be printed for IBAN
accounts in the `FI` country.

[spec]: https://www.finanssiala.fi/wp-content/uploads/2021/03/Pankkiviivakoodi-opas.pdf

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
bank-barcode = "0.2"
```

Then build a barcode through `BarcodeBuilder` and format it via its
`Display` implementation:

```rust
// The reference doesn't start with "RF", so a V4 barcode is produced
let barcode = bank_barcode::Barcode::builder()
    .account_number("FI16 8000 1400 0502 67")
    .reference(12345)
    .calendar_due_date(2025, 2, 26)
    .euros(123)
    .cents(45)
    .build()
    .expect("Builder failed");

println!("{barcode}");
```

Unless set explicitly with `.version(...)`, the barcode version is detected
from the reference: references starting with `RF` produce a V5 barcode and other
references a V4 barcode. Without a reference, V5 is used.

A runnable end-to-end example lives in [`examples/basic.rs`](examples/basic.rs)
and can be invoked with:

```sh
cargo run --example basic
```

## Minimum supported Rust version

`bank-barcode` builds on Rust **1.88** and later.

## License

Licensed under the [MIT License](LICENSE).
