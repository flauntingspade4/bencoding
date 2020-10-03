use bencoding::from_str;

#[test]
fn decode_string() {
    let encoded = "4:spam";
    let decoded: String = from_str(encoded).unwrap();

    assert_eq!("spam".to_string(), decoded);

    let encoded = "0:";
    let decoded: String = from_str(encoded).unwrap();

    assert_eq!("".to_string(), decoded);

    let encoded = "4:eggs";
    let decoded: String = from_str(encoded).unwrap();

    assert_eq!("eggs".to_string(), decoded);
}

#[test]
fn decode_int() {
    let encoded = "i52e";

    assert_eq!(52, from_str(encoded).unwrap());

    let encoded = "i-52e";

    assert_eq!(-52, from_str(encoded).unwrap());

    let encoded = "i0e";

    assert_eq!(0, from_str(encoded).unwrap());

    let encoded = "i-0e";

    assert_eq!(0, from_str(encoded).unwrap());
}

#[test]
fn decode_vec() {
    let encoded = "le";
    let decoded: Vec<String> = from_str(encoded).unwrap();

    assert_eq!(Vec::<String>::new(), decoded);

    let encoded = "l4:spam4:eggse";
    let decoded: Vec<String> = from_str(encoded).unwrap();

    assert_eq!(vec!["spam".to_string(), "eggs".to_string()], decoded);

    let encoded = "li5ei-15ei25ee";
    let decoded: Vec<i64> = from_str(encoded).unwrap();

    assert_eq!(vec![5, -15, 25], decoded,);

    let encoded = "lli7ei2eeli12ei53eee";

    let decoded: Vec<Vec<i64>> = from_str(encoded).unwrap();

    assert_eq!(vec![vec![7, 2], vec![12, 53]], decoded);
}

#[test]
fn decode_dict() {
    use bencoding::{dict, Dict};

    let dict = dict!(
        "cow".to_string(),
        "moo".to_string(),
        "spam".to_string(),
        "eggs".to_string()
    );

    assert_eq!(dict, from_str("d3:cow3:moo4:spam4:eggse").unwrap());

    let dict = dict!("spam".to_string(), vec!["a".to_string(), "b".to_string()]);

    assert_eq!(dict, from_str("d4:spaml1:a1:bee").unwrap());

    let dict = dict!(
        "publisher".to_string(),
        "bob".to_string(),
        "publisher-webpage".to_string(),
        "www.example.com".to_string(),
        "publisher.location".to_string(),
        "home".to_string()
    );

    assert_eq!(
        dict,
        from_str(
            "d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.location4:homee"
        )
        .unwrap()
    );
}
