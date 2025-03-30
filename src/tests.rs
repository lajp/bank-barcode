use time::{Date, Month};

use crate::{Barcode, BarcodeBuilder, BarcodeVersion};

#[test]
fn v4_1() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI79 4405 2020 0360 82")
        .euros(4883)
        .cents(15)
        .reference("868516259619897")
        .due_date(Date::from_calendar_date(2010, Month::June, 12).unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "479440520200360820048831500000000868516259619897100612",
        code.to_string(),
    );
}

#[test]
fn v4_2() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI58 1017 1000 0001 22")
        .sum(48299)
        .reference("559582243294671")
        .due_date(Date::from_calendar_date(2012, Month::January, 31).unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "458101710000001220004829900000000559582243294671120131",
        code.to_string(),
    );
}

#[test]
fn v4_3() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI02 5000 4640 0013 02")
        .sum(69380)
        .reference("69875672083435364")
        .due_date(Date::from_calendar_date(2011, Month::July, 24).unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "402500046400013020006938000000069875672083435364110724",
        code.to_string(),
    );
}

#[test]
fn v4_4() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI15 6601 0001 5306 41")
        .euros(7444)
        .cents(54)
        .reference("7758474790647489")
        .due_date(Date::from_calendar_date(2019, Month::December, 19).unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "415660100015306410074445400000007758474790647489191219",
        code.to_string(),
    );
}

#[test]
fn v4_5() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI16 8000 1400 0502 67")
        .euros(935)
        .cents(85)
        .reference("78777679656628687")
        .build()
        .unwrap();

    assert_eq!(
        "416800014000502670009358500000078777679656628687000000",
        code.to_string(),
    );
}

#[test]
fn v4_6() {
    let code = Barcode::builder()
        .version(BarcodeVersion::V4)
        .account_number("FI73 3131 3001 0000 58")
        .reference("868624")
        .due_date(Date::from_calendar_date(2013, Month::August, 9).unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "473313130010000580000000000000000000000000868624130809",
        code.to_string(),
    );
}

#[test]
fn v4_7() {
    let code = BarcodeBuilder::v4()
        .account_number("FI83 3301 0001 1007 75")
        .reference("92125374252539897737")
        .calendar_due_date(2016, 5, 25)
        .sum(15000020)
        .build()
        .unwrap();

    assert_eq!(
        "483330100011007751500002000092125374252539897737160525",
        code.to_string(),
    );
}

#[test]
fn v4_8() {
    let code = BarcodeBuilder::v4()
        .account_number("FI39 3636 3002 0924 92")
        .sum(103)
        .calendar_due_date(2023, 3, 11)
        .reference("590738390")
        .build()
        .unwrap();

    assert_eq!(
        "439363630020924920000010300000000000000590738390230311",
        code.to_string(),
    );
}

#[test]
fn v4_9() {
    let code = BarcodeBuilder::v4()
        .account_number("FI92 3939 0001 0033 91")
        .cents(2)
        .reference("1357914")
        .calendar_due_date(2099, 12, 24)
        .build()
        .unwrap();

    assert_eq!(
        "492393900010033910000000200000000000000001357914991224",
        code.to_string(),
    );
}

#[test]
fn v5_1() {
    let code = Barcode::builder()
        .account_number("FI79 4405 2020 0360 82")
        .sum(488315)
        .reference("RF09868516259619897")
        .calendar_due_date(2010, 6, 12)
        .build()
        .unwrap();

    assert_eq!(
        "579440520200360820048831509000000868516259619897100612",
        code.to_string()
    )
}

#[test]
fn v5_2() {
    let code = Barcode::builder()
        .account_number("FI58 1017 1000 0001 22")
        .euros(482)
        .cents(99)
        .reference("RF06559582243294671")
        .calendar_due_date(2010, 1, 31)
        .build()
        .unwrap();

    assert_eq!(
        "558101710000001220004829906000000559582243294671100131",
        code.to_string()
    )
}

#[test]
fn v5_3() {
    let code = Barcode::builder()
        .account_number("FI02 5000 4640 0013 02")
        .euros(693)
        .cents(80)
        .reference("RF61698756720839")
        .calendar_due_date(2011, 7, 24)
        .build()
        .unwrap();

    assert_eq!(
        "502500046400013020006938061000000000698756720839110724",
        code.to_string()
    )
}

#[test]
fn v5_4() {
    let code = Barcode::builder()
        .account_number("FI15 6601 0001 5306 41")
        .sum(744454)
        .reference("RF847758474790647489")
        .calendar_due_date(2019, 12, 19)
        .build()
        .unwrap();

    assert_eq!(
        "515660100015306410074445484000007758474790647489191219",
        code.to_string()
    )
}

#[test]
fn v5_5() {
    let code = Barcode::builder()
        .account_number("FI16 8000 1400 0502 67")
        .sum(93585)
        .reference("RF6078777679656628687")
        .build()
        .unwrap();

    assert_eq!(
        "516800014000502670009358560000078777679656628687000000",
        code.to_string()
    )
}

#[test]
fn v5_6() {
    let code = Barcode::builder()
        .account_number("FI73 3131 3001 0000 58")
        .sum(0)
        .reference("RF10868624")
        .calendar_due_date(2013, 8, 9)
        .build()
        .unwrap();

    assert_eq!(
        "573313130010000580000000010000000000000000868624130809",
        code.to_string()
    )
}

#[test]
fn v5_7() {
    let code = Barcode::builder()
        .account_number("FI83 3301 0001 1007 75")
        .euros(150000)
        .cents(20)
        .reference("RF7192125374252539897737")
        .calendar_due_date(2016, 5, 25)
        .build()
        .unwrap();

    assert_eq!(
        "583330100011007751500002071092125374252539897737160525",
        code.to_string()
    )
}

#[test]
fn v5_8() {
    let code = Barcode::builder()
        .account_number("FI39 3636 3002 0924 92")
        .euros(1)
        .cents(3)
        .reference("RF66590738390")
        .calendar_due_date(2023, 3, 11)
        .build()
        .unwrap();

    assert_eq!(
        "539363630020924920000010366000000000000590738390230311",
        code.to_string()
    )
}

#[test]
fn v5_9() {
    let code = Barcode::builder()
        .account_number("FI92 3939 0001 0033 91")
        .cents(2)
        .reference("RF951357914")
        .calendar_due_date(2099, 12, 24)
        .build()
        .unwrap();

    assert_eq!(
        "592393900010033910000000295000000000000001357914991224",
        code.to_string()
    )
}
