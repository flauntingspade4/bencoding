mod structs;

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

#[test]
fn person_encode() {
    use structs::{Person, Publisher, StructContainingVec};

    let person = Person::new("test_01".to_string(), "Male".to_string(), 50);

    assert_eq!(
        "d4:name7:test_016:gender4:Male3:agei50ee",
        bencoding::to_string(&person).unwrap()
    );

    let pulisher = Publisher::new(
        "bob".to_string(),
        "www.example.com".to_string(),
        "home".to_string(),
    );

    assert_eq!(
        "d4:name3:bob17:publisher_webpage15:www.example.com18:publisher_location4:homee",
        bencoding::to_string(&pulisher).unwrap()
    );

    let vstruct = StructContainingVec::from(vec![5, 24, 16, 178]);

    assert_eq!(
        "d3:vecli5ei24ei16ei178eee",
        bencoding::to_string(&vstruct).unwrap()
    );
}
