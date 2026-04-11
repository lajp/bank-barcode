//! Generate Finnish virtual bank barcodes (*pankkiviivakoodi*).
//!
//! This crate produces the digit string for both version 4 and version 5 of the
//! Finnish virtual bank barcode as defined in the
//! [Finanssiala specification](https://www.finanssiala.fi/wp-content/uploads/2021/03/Pankkiviivakoodi-opas.pdf).
//!
//! Build a [`Barcode`] through [`BarcodeBuilder`] and format it via its
//! [`Display`](std::fmt::Display) implementation.

use std::num::ParseIntError;

use either::Either;
use iban::Iban;
use time::{Date, Month};

#[cfg(test)]
mod tests;

/// A representation on different versions of the bank barcode
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Default)]
pub enum BarcodeVersion {
    /// The version 4 of the bank barcode
    ///
    /// Only difference to `BarcodeVersion::V5` is that the reference number is max 20 digits long
    V4,
    /// The version 5 of the bank barcode
    ///
    /// The reference number is max 23 digits long
    #[default]
    V5,
}

/// This struct represents a [bank barcode](https://www.finanssiala.fi/wp-content/uploads/2021/03/Pankkiviivakoodi-opas.pdf)(pankkiviivakoodi).
/// It stores information about the IBAN, the sum (max 999999.99€),
/// a reference number (20 digits for V4, 23 digits for V5) and the due date.
///
/// Bank barcodes may only be printed for FI IBAN numbers.
///
/// For information about constructing it, see `BarcodeBuilder`
///
/// # Usage
///
/// ```rust
/// let barcode = bank_barcode::Barcode::builder()
///     .account_number("FI73 3131 3001 0000 58")
///     .build()
///     .unwrap();
///
/// assert_eq!("573313130010000580000000000000000000000000000000000000", barcode.to_string());
/// ```
#[derive(Debug, Clone)]
pub struct Barcode {
    version: BarcodeVersion,
    account_number: iban::Iban,
    euros: u32,
    cents: u8,
    reference: String,
    due_date: Option<Date>,
}

/// This struct is used to construct a `Barcode`
///
/// # Usage
/// ```rust
/// let barcode = bank_barcode::Barcode::builder()
///     .version(bank_barcode::BarcodeVersion::V4)
///     .account_number("FI16 8000 1400 0502 67")
///     .reference(12345)
///     .calendar_due_date(2025, 2, 26)
///     .euros(123)
///     .cents(45)
///     .build()
///     .expect("Builder failed");
/// ```
#[derive(Debug, Default)]
pub struct BarcodeBuilder {
    version: BarcodeVersion,
    account_number: Option<Either<Iban, String>>,
    euros: u32,
    cents: u8,
    reference: Option<String>,
    due_date: Option<Either<Date, (i32, u8, u8)>>,
}

/// Errors that may occur while building a [`Barcode`].
#[derive(Debug, Clone, thiserror::Error)]
pub enum BuilderError {
    /// No account number was supplied to the builder.
    #[error("No account number specified")]
    NoAccount,
    /// The provided account number string could not be parsed as an IBAN.
    #[error("Failed to parse account number {0}")]
    InvalidAccount(#[from] iban::ParseError),
    /// The IBAN was valid but its country code is not `FI`.
    #[error(
        "A non FI IBAN provided. Bank barcode may only be printed for IBAN accounts starting with FI"
    )]
    AccountNotFinnish,
    /// The total sum exceeds the maximum the barcode format can encode (999 999,99 €).
    #[error("The total sum is too large (over 999 999,99)")]
    SumTooLarge,
    /// The cents value was outside the valid range `0..=99`.
    #[error("The amount of cents is invalid (not between 0..=99)")]
    InvalidCents,
    /// The reference number is longer than the version allows
    /// (20 digits for V4, 23 digits for V5).
    #[error(
        "The reference number is too large (the limit is 20 digits for V4 and 23 digits for V5)"
    )]
    ReferenceTooLarge,
    /// The reference number could not be parsed as digits.
    #[error("Invalid reference")]
    InvalidReference(#[from] ParseIntError),
    /// A V5 reference number does not start with the required `RF` prefix.
    #[error("Malformed reference: The reference for V5 has to start with 'RF'")]
    MalformedReference,
    /// The supplied calendar date components do not form a valid date.
    #[error("Invalid date provided {0}")]
    InvalidDate(#[from] time::error::ComponentRange),
}

impl BarcodeBuilder {
    /// Completes the builder and returns the [`Barcode`].
    ///
    /// # Errors
    ///
    /// Returns a [`BuilderError`] if any field is missing or invalid:
    /// no account number, an unparseable or non-Finnish IBAN, an out-of-range
    /// sum or cents, an oversized or malformed reference number, or an
    /// invalid due date.
    pub fn build(self) -> Result<Barcode, BuilderError> {
        let account_number = match self.account_number {
            Some(Either::Left(iban)) => iban,
            Some(Either::Right(number)) => number.parse()?,
            None => return Err(BuilderError::NoAccount),
        };

        if account_number.country_code() != "FI" {
            return Err(BuilderError::AccountNotFinnish);
        }

        if self.cents >= 100 {
            return Err(BuilderError::InvalidCents);
        }

        if self.euros >= 999_999 {
            return Err(BuilderError::SumTooLarge);
        }

        let reference = match self.version {
            BarcodeVersion::V4 => {
                if let Some(rn) = self.reference.as_ref() {
                    let _: u128 = rn.parse()?;
                }

                let reference = self.reference.unwrap_or("0".into());
                if reference.len() > 20 {
                    Err(BuilderError::ReferenceTooLarge)
                } else {
                    Ok(reference)
                }
            }
            BarcodeVersion::V5 => {
                let reference_rf = self.reference.unwrap_or("RF00".into());
                if !reference_rf.starts_with("RF") {
                    return Err(BuilderError::MalformedReference);
                }
                let _: u128 = reference_rf[2..].parse()?;
                let reference = reference_rf[2..].to_string();

                if reference.len() > 23 {
                    Err(BuilderError::ReferenceTooLarge)
                } else {
                    Ok(reference)
                }
            }
        }?;

        let due_date = match self.due_date {
            Some(Either::Left(date)) => Some(date),
            Some(Either::Right((year, month, day))) => Some(Date::from_calendar_date(
                year,
                Month::try_from(month)?,
                day,
            )?),
            None => None,
        };

        Ok(Barcode {
            version: self.version,
            account_number,
            euros: self.euros,
            cents: self.cents,
            reference,
            due_date,
        })
    }

    /// Construct a `BarcodeBuilder` with `BarcodeVersion::V4`
    #[must_use]
    pub fn v4() -> Self {
        Self::default().version(BarcodeVersion::V4)
    }

    /// Construct a `BarcodeBuilder` with `BarcodeVersion::V5`
    ///
    /// NOTE: This is also the default value for the `version`
    #[must_use]
    pub fn v5() -> Self {
        Self::default().version(BarcodeVersion::V5)
    }

    /// Set the `BarcodeVersion` of the barcode. Defaults to `BarcodeVersion::V5`.
    #[must_use]
    pub fn version(self, version: BarcodeVersion) -> Self {
        Self { version, ..self }
    }

    /// Specify the account number. The account number is the only *mandatory* field.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // public API: keep `impl ToString` to accept any displayable value
    pub fn account_number(self, account: impl ToString) -> Self {
        Self {
            account_number: Some(Either::Right(account.to_string())),
            ..self
        }
    }

    /// Specify the account number as an `iban::Iban` value. The account number is the only *mandatory* field.
    #[must_use]
    pub fn account_number_iban(self, account: iban::Iban) -> Self {
        Self {
            account_number: Some(Either::Left(account)),
            ..self
        }
    }

    /// Specify the amount of euros, default values is 0
    #[must_use]
    pub fn euros(self, euros: u32) -> Self {
        Self { euros, ..self }
    }

    /// Specify the amount of cents, default value is 0.
    /// For `BarcodeBuilder::build` to succeed, value must bet between 0 and 99.
    ///
    /// *NOTE*: if you want to specify the total amount of cents, use `BarcodeBuilder::sum`
    /// instead.
    #[must_use]
    pub fn cents(self, cents: u8) -> Self {
        Self { cents, ..self }
    }

    /// Specify the total amount of cents in the sum. The default value is 0.
    #[must_use]
    pub fn sum(self, sum: u32) -> Self {
        Self {
            euros: sum / 100,
            cents: (sum % 100) as u8,
            ..self
        }
    }

    /// Specify the reference number. The default value is 0.
    /// The reference number is a maximum of 20 digits for `BarcodeVersion::V4` and 23 digits for
    /// `BarcodeVersion::V5`. The `BarcodeVersion::V5` reference number starts with the string "RF"
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // public API: keep `impl ToString` to accept any displayable value
    pub fn reference(self, reference: impl ToString) -> Self {
        Self {
            reference: Some(reference.to_string()),
            ..self
        }
    }

    /// Specify the due date as a `time::date`. Default value: no due date.
    #[must_use]
    pub fn due_date(self, due_date: Date) -> Self {
        Self {
            due_date: Some(Either::Left(due_date)),
            ..self
        }
    }

    /// Specify the due date as year, month and day. Default value: no due date.
    /// If invalid date is specified, the call to `BarcodeBuilder::build` will fail.
    #[must_use]
    pub fn calendar_due_date(self, year: i32, month: u8, day: u8) -> Self {
        Self {
            due_date: Some(Either::Right((year, month, day))),
            ..self
        }
    }
}

impl Barcode {
    /// Construct a builder, for more information see `BarcodeBuilder`.
    #[must_use]
    pub fn builder() -> BarcodeBuilder {
        BarcodeBuilder::default()
    }
}

impl std::fmt::Display for Barcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use time::macros::format_description;

        match self.version {
            BarcodeVersion::V4 => {
                write!(
                    f,
                    "4{}{:0>6}{:0>2}000{:0>20}{}",
                    &self.account_number.as_str()[2..],
                    self.euros,
                    self.cents,
                    self.reference,
                    self.due_date.map_or("000000".into(), |d| d
                        .format(format_description!(
                            version = 2,
                            "[year repr:last_two][month][day]"
                        ))
                        .expect("bug: formatting failed"))
                )
            }
            BarcodeVersion::V5 => {
                let ref_str = self.reference.clone();
                let ref_nro = if ref_str.len() < 2 {
                    format!("{:0<2}", self.reference)
                } else {
                    ref_str
                };

                write!(
                    f,
                    "5{}{:0>6}{:0>2}{}{:0>21}{}",
                    &self.account_number.as_str()[2..],
                    self.euros,
                    self.cents,
                    &ref_nro[..2],
                    &ref_nro[2..],
                    self.due_date.map_or("000000".into(), |d| d
                        .format(format_description!(
                            version = 2,
                            "[year repr:last_two][month][day]"
                        ))
                        .expect("bug: formatting failed"))
                )
            }
        }
    }
}
