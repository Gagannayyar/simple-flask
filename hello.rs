struct Car {
    mpg: i32,
    color: &'static str,
    top_speed: f32
}

impl Car {
    fn set_mpg(&mut self, new_mpg: i32) {
        self.mpg = new_mpg
    }

    fn set_color(&mut self, new_color: &'static str) {
        self.color = new_color
    }

    fn set_top_speed(&mut self, new_top_speed: f32) {
        self.top_speed = new_top_speed
    }
}

fn main () {
    let mut car = Car {
        mpg: 10,
        color: "White",
        top_speed: 100.01
    };
    println!("{}",car.mpg);
    println!("{}",car.color);
    println!("{}",car.top_speed);

    car.set_mpg(15);
    car.set_color("Blue");
    car.set_top_speed(40.01);

    println!("{}",car.mpg);
    println!("{}",car.color);
    println!("{}",car.top_speed);
}