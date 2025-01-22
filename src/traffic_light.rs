#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn action(&self) -> &str {
        match self {
            TrafficLight::Red => "Stop",
            TrafficLight::Yellow => "Slow Down",
            TrafficLight::Green => "Go",
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Red Light: {}", red_light.action());
    println!("Yellow Light: {}", yellow_light.action());
    println!("Green Light: {}", green_light.action());
}
