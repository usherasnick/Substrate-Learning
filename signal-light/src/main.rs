enum TrafficLight {
    Green,
    Yellow,
    Red,
}

pub trait LightOn {
    fn duration(&self) -> u8;
}

impl LightOn for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Green => {
                20
            },
            TrafficLight::Yellow => {
                5
            },
            TrafficLight::Red => {
                30
            },
        }
    }
}

fn main() {
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    let red = TrafficLight::Red;
    println!("green time {}", green.duration());
    println!("yellow time {}", yellow.duration());
    println!("red time {}", red.duration());
}


