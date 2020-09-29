use bencoding::BenCodeAble;

#[test]
fn decode_string() {
    let mut encoded = "4:spam".to_string();

    assert_eq!(
        "spam".to_string(),
        String::de_bencode(&mut encoded).unwrap().0
    );

    let mut encoded = "0:".to_string();

    assert_eq!("".to_string(), String::de_bencode(&mut encoded).unwrap().0);

    let mut encoded = "4:eggs".to_string();

    assert_eq!(
        "eggs".to_string(),
        String::de_bencode(&mut encoded).unwrap().0
    );
}

#[test]
fn decode_int() {
    let mut encoded = "i52e".to_string();

    assert_eq!(52, i32::de_bencode(&mut encoded).unwrap().0);

    let mut encoded = "i-52e".to_string();

    assert_eq!(-52, i32::de_bencode(&mut encoded).unwrap().0);

    let mut encoded = "i0e".to_string();

    assert_eq!(0, i32::de_bencode(&mut encoded).unwrap().0);

    let mut encoded = "i-0e".to_string();

    assert_eq!(0, i32::de_bencode(&mut encoded).unwrap().0);
}

#[test]
fn decode_vec() {
    /*let mut encoded = "le".to_string();

    assert_eq!(
        Vec::<String>::new(),
        Vec::<String>::de_bencode(&mut encoded).unwrap().0
    );

    let mut encoded = "l4:spam4:eggse".to_string();

    assert_eq!(
        vec!["spam".to_string(), "eggs".to_string()],
        Vec::<String>::de_bencode(&mut encoded).unwrap().0
    );

    let mut encoded = "li5ei-15ei25ee".to_string();

    assert_eq!(
        vec![5, -15, 25],
        Vec::<i64>::de_bencode(&mut encoded).unwrap().0
    );*/

    // let mut encoded = "llie7ie2iile12ie53ii".to_string();

    let vec: Vec<Vec<i64>> = vec![vec![7, 2], vec![12, 53]];

    let mut encoded = vec.bencode();

    println!("{}", encoded);

    assert_eq!(
        vec![vec![7, 2], vec![12, 53]],
        Vec::<Vec<i64>>::de_bencode(&mut encoded).unwrap().0
    );
}
