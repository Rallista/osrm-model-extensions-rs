use round::round;

use crate::distance::spoken_numbers::number_to_words;
use crate::distance::{Distance, Unit};
use std::fmt;

pub struct SpokenDistance {
    distance: Distance,
    is_metric: bool,
}

impl SpokenDistance {
    pub fn from_distance(distance: Distance) -> Self {
        SpokenDistance {
            distance,
            is_metric: distance.is_metric(),
        }
    }

    pub fn from_meters(value: f64, metric: bool) -> Self {
        SpokenDistance {
            distance: Distance::from_meters(value),
            is_metric: metric,
        }
    }

    fn converted(&self) -> Distance {
        if self.is_metric {
            let km = self.distance.to(Unit::Kilometers);
            if km.value() < 0.25 {
                self.distance.to(Unit::Meters)
            } else {
                km
            }
        } else {
            let mi = self.distance.to(Unit::Miles);
            if mi.value() < 0.25 {
                self.distance.to(Unit::Feet)
            } else {
                mi
            }
        }
    }

    pub fn spoken(&self) -> String {
        let distance = self.converted();
        let spoken_unit = distance.spoken_unit();
        let value = distance.value();
        let rounded = match distance.unit() {
            Unit::Feet | Unit::Meters => {
                if value < 1.0 {
                    round(value, 1)
                } else if value < 10.0 {
                    value.round()
                } else if value < 50.0 {
                    (value / 5.0).round() * 5.0
                } else {
                    (value / 50.0).round() * 50.0
                }
            }
            _ => {
                if value < 1.0 {
                    round(value, 1)
                } else {
                    value.round()
                }
            }
        };
        let spoken_number = number_to_words(rounded);
        format!("{} {}", spoken_number, spoken_unit)
    }
}

impl fmt::Display for SpokenDistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.spoken())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_under_250m() {
        let distance = SpokenDistance::from_meters(200.0, true);
        assert_eq!(distance.spoken(), "two hundred meters");
    }

    #[test]
    fn test_metric_over_250m() {
        let distance = SpokenDistance::from_meters(3000.0, true);
        assert_eq!(distance.spoken(), "three kilometers");
    }

    #[test]
    fn test_imperial_under_quarter_mile() {
        // 400 meters ≈ 1312 feet (under 0.25 miles)
        let distance = SpokenDistance::from_meters(400.0, false);
        assert_eq!(distance.spoken(), "one thousand three hundred feet");
    }

    #[test]
    fn test_imperial_over_quarter_mile() {
        // 5000 meters ≈ 3.1 miles
        let distance = SpokenDistance::from_meters(5000.0, false);
        assert_eq!(distance.spoken(), "three miles");
    }

    #[test]
    fn test_small_metric_distance() {
        // 0.7 meters
        let distance = SpokenDistance::from_meters(0.7, true);
        assert_eq!(distance.spoken(), "three quarter meters");
    }

    #[test]
    fn test_smallest_imperial_distance() {
        // 0.1 meters ≈ 0.3 feet
        let distance = SpokenDistance::from_meters(0.1, false);
        assert_eq!(distance.spoken(), "one quarter feet");
    }

    #[test]
    fn test_smaller_imperial_distance() {
        // 0.7 meters ≈ 2.3 feet
        let distance = SpokenDistance::from_meters(0.7, false);
        assert_eq!(distance.spoken(), "two feet");
    }

    #[test]
    fn test_small_imperial_distance() {
        // 2 meters ≈ 6.6 feet
        let distance = SpokenDistance::from_meters(2.0, false);
        assert_eq!(distance.spoken(), "seven feet");
    }

    #[test]
    fn test_fractional_kilometer() {
        let distance = SpokenDistance::from_meters(750.0, true);
        assert_eq!(distance.spoken(), "three quarter kilometers");
    }

    #[test]
    fn test_fractional_mile() {
        // 1207 meters ≈ 0.75 miles
        let distance = SpokenDistance::from_meters(1207.0, false);
        assert_eq!(distance.spoken(), "three quarter mile");
    }
}
