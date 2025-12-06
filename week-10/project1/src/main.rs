struct Laptop {
    hp:u32,
    ibm:u32,
    toshiba:u32,
    dell:u32,
}

fn main() {
    let price = Laptop {
        hp:650_000,
        ibm:755_000,
        toshiba:550_000,
        dell:850_000,

    };

    println!("your total cost for 3 of each laptop is N{} have a nice day  ", price.total());
}
impl Laptop {
    fn total(&self)->u32 {
        (self.hp * 3) + (self.ibm * 3) + (self.toshiba * 3) + (self.dell * 3)
    }
}

