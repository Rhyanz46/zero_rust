struct Test{
    panjang: u32,
    lebar: u32
}

impl Test{
    fn to_string(&self){
        println!("panjang : {}, lebar : {}", self.panjang, self.lebar);
    }
}

trait Belajar{
    fn defisinikan(panjang: &u32) -> u32;

}



fn main(){
    let data = Test{panjang : 123, lebar : 34};
    data.to_string();
}