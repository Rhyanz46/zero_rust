//implement struct

struct Data{
    width : u32,
    height : u32
}

impl Data{
    fn printkan(&self) {
        println!("Data : width = {}, height = {}", self.width, self.height);
    }

    fn sama(&self) -> bool{
        let kebenaran = self.width == self.height;
        return kebenaran
    }
}


fn main(){
    let data_baru = Data {width : 12, height: 30};
    data_baru.printkan();
    print!("nilainya sama atau tidak ?  : {}", data_baru.sama());
}