fn main(){
    let mut kata_string = String::from("Hello, world!");
    // hello.push('wwq'); //error, push hanya untuk menyisip char
    kata_string.push('w'); 
    kata_string.push_str("input string"); //push_str untuk mengimport string atau kata
    println!("{}", kata_string);

    //bagaimana dengan string biasa ??

    let mut kata_biasa = "hahai";
    // kata_biasa.push('w'); //tidak bisa di lakukan
    println!("{}", kata_biasa);
    kata_biasa = "agar tidak warning";
    println!("{}", kata_biasa);
    
    println!("jumlah char kata string = {}", kata_string.len());
    //string biasa ?
    println!("jumlah char kata biasa = {}", kata_biasa.len());//bisa

    println!("kata string is empty?= {}", kata_string.is_empty());
    println!("kata biasa is empty?= {}", kata_biasa.is_empty());//bisa

    println!("", );
    for a in kata_string.split_whitespace(){
        println!("{}", a)
    }

    println!("", );

    for a in kata_biasa.split_whitespace(){//biasa
        println!("{}", a)
    }

    println!("apakah kata string punya kata 'Hello' ? : {} ", kata_string.contains("Hello"));
    println!("apakah kata string punya kata 'hello' ? : {} ", kata_string.contains("hello"));

    //string biasa?

    println!("apakah kata biasa punya kata 'agar' ? : {} ", kata_biasa.contains("agar"));
    println!("apakah kata biasa punya kata 'Agar' ? : {} ", kata_biasa.contains("Agar"));
    
}