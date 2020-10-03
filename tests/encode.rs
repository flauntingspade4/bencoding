use bencoding::BenCodeAble;

#[test]
fn encode_string() {
    let string = "spam".to_string();

    assert_eq!("4:spam", string.bencode());

    let string = "".to_string();

    assert_eq!("0:", string.bencode());

    let string = "eggs".to_string();

    assert_eq!("4:eggs", string.bencode());
}

#[test]
fn encode_int() {
    let integer = 52;

    assert_eq!("i52e", integer.bencode());

    let integer = -52;

    assert_eq!("i-52e", integer.bencode());

    let integer = 0;

    assert_eq!("i0e", integer.bencode());

    let integer = -0;

    assert_eq!("i0e", integer.bencode());
}

#[test]
fn encode_vec() {
    let vec = vec!["spam".to_string(), "eggs".to_string()];

    assert_eq!("l4:spam4:eggse", vec.bencode());

    let vec: Vec<i64> = vec![15, 6];

    assert_eq!("li15ei6ee", vec.bencode());

    let vec: Vec<Vec<i64>> = vec![vec![16, 3], vec![12, 25]];

    assert_eq!("lli16ei3eeli12ei25eee", vec.bencode());
}

#[test]
fn encode_dict() {
    use bencoding::{dict, Dict};

    let dict = dict!(
        "cow".to_string(),
        "moo".to_string(),
        "spam".to_string(),
        "eggs".to_string()
    );

    assert_eq!("d3:cow3:moo4:spam4:eggse".to_string(), dict.bencode());

    let dict = dict!("spam".to_string(), vec!["a".to_string(), "b".to_string()]);

    assert_eq!("d4:spaml1:a1:bee".to_string(), dict.bencode());

    let dict = dict!(
        "publisher".to_string(),
        "bob".to_string(),
        "publisher-webpage".to_string(),
        "www.example.com".to_string(),
        "publisher.location".to_string(),
        "home".to_string()
    );

    assert_eq!(
        "d9:publisher3:bob17:publisher-webpage15:www.example.com18:publisher.location4:homee"
            .to_string(),
        dict.bencode()
    );
}
