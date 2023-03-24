fn main() {
    let green = Green{};
    let yellow = Yellow{};
    let red = Red{};
    println!("green light on for {} second", green.duration());
    println!("yellow light on for {} second", yellow.duration());
    println!("red light on for {} second", red.duration());
}

pub trait LightOn {
    fn duration(&self) -> u8;
}

struct Green {}

impl LightOn for Green {
    fn duration(&self) -> u8 {
        30
    }
}

struct Yellow {}

impl LightOn for Yellow {
    fn duration(&self) -> u8 {
        5
    }
}

struct Red {}

impl LightOn for Red {
    fn duration(&self) -> u8 {
        20
    }
}



