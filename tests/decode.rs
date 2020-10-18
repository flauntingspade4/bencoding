mod structs;

#[test]
fn decode_string() {
    let encoded = "4:spam";
    let decoded: String = bencoding::from_str(encoded).unwrap();

    assert_eq!("spam".to_string(), decoded);

    let encoded = "0:";
    let decoded: String = bencoding::from_str(encoded).unwrap();

    assert_eq!("".to_string(), decoded);

    let encoded = "4:eggs";
    let decoded: String = bencoding::from_str(encoded).unwrap();

    assert_eq!("eggs".to_string(), decoded);
}

#[test]
fn decode_int() {
    let encoded = "i52e";

    assert_eq!(52, bencoding::from_str(encoded).unwrap());

    let encoded = "i-52e";

    assert_eq!(-52, bencoding::from_str(encoded).unwrap());

    let encoded = "i0e";

    assert_eq!(0, bencoding::from_str(encoded).unwrap());

    let encoded = "i-0e";

    assert_eq!(0, bencoding::from_str(encoded).unwrap());
}

#[test]
fn decode_vec() {
    let encoded = "le";
    let decoded: Vec<String> = bencoding::from_str(encoded).unwrap();

    assert_eq!(Vec::<String>::new(), decoded);

    let encoded = "l4:spam4:eggse";
    let decoded: Vec<String> = bencoding::from_str(encoded).unwrap();

    assert_eq!(vec!["spam".to_string(), "eggs".to_string()], decoded);

    let encoded = "li5ei-15ei25ee";
    let decoded: Vec<i64> = bencoding::from_str(encoded).unwrap();

    assert_eq!(vec![5, -15, 25], decoded,);

    let encoded = "lli7ei2eeli12ei53eee";

    let decoded: Vec<Vec<i64>> = bencoding::from_str(encoded).unwrap();

    assert_eq!(vec![vec![7, 2], vec![12, 53]], decoded);
}

#[test]
fn person_decode() {
    use structs::{Person, Publisher, StructContainingVec};

    let encoded = "d4:name7:test_016:gender4:Male3:agei50ee";
    let decoded = Person::new("test_01".to_string(), "Male".to_string(), 50);

    assert_eq!(decoded, bencoding::from_str(encoded).unwrap());

    let encoded = "d4:name3:bob17:publisher_webpage15:www.example.com18:publisher_location4:homee";
    let decoded = Publisher::new(
        "bob".to_string(),
        "www.example.com".to_string(),
        "home".to_string(),
    );

    assert_eq!(decoded, bencoding::from_str(encoded).unwrap());

    let encoded = "d3:vecli5ei24ei16ei178eee";
    let decoded = StructContainingVec::from(vec![5, 24, 16, 178]);

    assert_eq!(decoded, bencoding::from_str(encoded).unwrap());
}
