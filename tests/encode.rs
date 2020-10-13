#[test]
fn encode_string() {
    let string = "spam".to_string();

    assert_eq!("4:spam", bencoding::to_string(&string).unwrap());

    let string = "".to_string();

    assert_eq!("0:", bencoding::to_string(&string).unwrap());

    let string = "eggs".to_string();

    assert_eq!("4:eggs", bencoding::to_string(&string).unwrap());
}

#[test]
fn encode_int() {
    let integer = 52;

    assert_eq!("i52e", bencoding::to_string(&integer).unwrap());

    let integer = -52;

    assert_eq!("i-52e", bencoding::to_string(&integer).unwrap());

    let integer = 0;

    assert_eq!("i0e", bencoding::to_string(&integer).unwrap());

    let integer = -0;

    assert_eq!("i0e", bencoding::to_string(&integer).unwrap());
}

#[test]
fn encode_vec() {
    let vec = vec!["spam".to_string(), "eggs".to_string()];

    assert_eq!("l4:spam4:eggse", bencoding::to_string(&vec).unwrap());

    let vec: Vec<i64> = vec![15, 6];

    assert_eq!("li15ei6ee", bencoding::to_string(&vec).unwrap());

    let vec: Vec<Vec<i64>> = vec![vec![16, 3], vec![12, 25]];

    assert_eq!("lli16ei3eeli12ei25eee", bencoding::to_string(&vec).unwrap());
}
/*
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

    let dict = dict!(
        "open".to_string(),
        vec![24, 34, 44],
        "close".to_string(),
        vec![42, 43, 44]
    );

    assert_eq!(
        "d5:closeli42ei43ei44ee4:openli24ei34ei44eee".to_string(),
        dict.bencode()
    );
}
*/
