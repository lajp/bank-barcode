//! Build and print a Finnish virtual bank barcode.
//!
//! Run with: `cargo run --example basic`

use bank_barcode::{Barcode, BarcodeVersion};

fn main() {
    let barcode = Barcode::builder()
        .version(BarcodeVersion::V5)
        .account_number("FI79 4405 2020 0360 82")
        .reference("RF09868516259619897")
        .calendar_due_date(2010, 6, 12)
        .sum(488_315)
        .build()
        .expect("valid barcode inputs");

    println!("{barcode}");
}
