fn main(){
    assert_eq!(true, true);// jika tidak sama maka akan menghasilkan error
    // assert_eq!(true, false);//akan error

    //kita akan mencoba melakukan error

    let sparkle_heart = vec![240, 159, 146, 150];
    // let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    println!("{:?}", sparkle_heart);
}