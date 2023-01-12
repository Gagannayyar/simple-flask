struct Square {
    width: i32,
    height: i32
}

impl Square {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn change_width(&mut self, new_width: i32) {
        self.width = new_width
    }
}

fn main() {
    let mut sq = Square {width: 5, height: 5};
    sq.change_width(10);
    println!("{}",sq.area());

}