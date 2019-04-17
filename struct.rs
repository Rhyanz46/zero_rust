//struct

struct Data{
    panjang : u32,
    lebar : u32
}

fn main(){
    let hasil = Data {panjang : 123, lebar : 222};

    println!("panjang : {}", hasil.panjang);
    println!("lebar : {}", hasil.lebar);
