#[test]
fn encode_utf8_string() {
    let string = "ϚРΑϺ".to_string();

    assert_eq!("8:ϚРΑϺ", bencoding::to_string(&string).unwrap());

    let string = "".to_string();

    assert_eq!("0:", bencoding::to_string(&string).unwrap());

    let string = "ত࠷ۆஓ".to_string();

    assert_eq!("11:ত࠷ۆஓ", bencoding::to_string(&string).unwrap());
}
