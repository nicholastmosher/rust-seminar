
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn next_light(light: &TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Red => TrafficLight::Green,
    }
}

impl TrafficLight {
    fn next_light(&self) -> TrafficLight {
        match self {
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Red => TrafficLight::Green,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light() {
        let red = TrafficLight::Red;
        let the_next_light = red.next_light();
        assert_eq!(the_next_light, TrafficLight::Green);
    }
}
