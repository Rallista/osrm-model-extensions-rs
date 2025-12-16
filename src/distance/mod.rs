use spoken_unit::{SpokenFoot, SpokenKilometer, SpokenMeter, SpokenMile, SpokenUnit};

pub mod spoken_distance;
pub mod spoken_numbers;
pub mod spoken_unit;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Meters,
    Kilometers,
    Miles,
    Feet,
}

impl Unit {
    pub fn spoken_form(&self, count: f64) -> String {
        match self {
            Unit::Meters => SpokenMeter::get_form(count),
            Unit::Kilometers => SpokenKilometer::get_form(count),
            Unit::Miles => SpokenMile::get_form(count),
            Unit::Feet => SpokenFoot::get_form(count),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    value: f64,
    unit: Unit,
}

impl Distance {
    pub fn new(value: f64, unit: Unit) -> Self {
        Distance { value, unit }
    }

    pub fn from_meters(value: f64) -> Self {
        Distance::new(value, Unit::Meters)
    }

    pub fn from_kilometers(value: f64) -> Self {
        Distance::new(value, Unit::Kilometers)
    }

    pub fn from_miles(value: f64) -> Self {
        Distance::new(value, Unit::Miles)
    }

    pub fn from_feet(value: f64) -> Self {
        Distance::new(value, Unit::Feet)
    }

    pub fn to(&self, target_unit: Unit) -> Distance {
        // First convert to meters as base unit
        let meters = match self.unit {
            Unit::Meters => self.value,
            Unit::Kilometers => self.value * 1000.0,
            Unit::Miles => self.value * 1609.344,
            Unit::Feet => self.value * 0.3048,
        };

        // Then convert from meters to target unit and return new Distance
        let new_value = match target_unit {
            Unit::Meters => meters,
            Unit::Kilometers => meters / 1000.0,
            Unit::Miles => meters / 1609.344,
            Unit::Feet => meters / 0.3048,
        };

        Distance {
            value: new_value,
            unit: target_unit,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }

    pub fn is_metric(&self) -> bool {
        matches!(self.unit, Unit::Meters | Unit::Kilometers)
    }

    pub fn spoken_unit(&self) -> String {
        self.unit.spoken_form(self.value)
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.to(Unit::Meters).value == other.to(Unit::Meters).value
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Convert both to meters before comparing
        let self_in_meters = self.to(Unit::Meters).value;
        let other_in_meters = other.to(Unit::Meters).value;
        self_in_meters.partial_cmp(&other_in_meters)
    }
}

impl Distance {
    pub fn min(self, other: Distance) -> Distance {
        if self <= other { self } else { other }
    }

    pub fn max(self, other: Distance) -> Distance {
        if self >= other { self } else { other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_i18n::t;

    #[test]
    fn test_basic_conversions() {
        let distance = Distance::new(1.0, Unit::Kilometers);

        assert_eq!(distance.to(Unit::Meters).value, 1000.0);
        assert!((distance.to(Unit::Miles).value - 0.621371).abs() < 0.000001);
        assert!((distance.to(Unit::Feet).value - 3280.84).abs() < 0.01);
    }

    #[test]
    fn test_same_unit() {
        let distance = Distance::new(5.0, Unit::Meters);
        assert_eq!(distance.to(Unit::Meters).value, 5.0);
    }

    #[test]
    fn test_spoken_form() {
        let single_meter = Distance::new(1.0, Unit::Meters);
        let multiple_meters = Distance::new(2.0, Unit::Meters);

        assert_eq!(single_meter.spoken_unit(), t!("units.singular.meter"));
        assert_eq!(multiple_meters.spoken_unit(), t!("units.plural.meters"));
    }
}
