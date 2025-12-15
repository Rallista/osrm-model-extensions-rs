use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum SpeedClass {
    Slow,
    Medium,
    Fast,
}

impl SpeedClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpeedClass::Slow => "slow",
            SpeedClass::Medium => "medium",
            SpeedClass::Fast => "fast",
        }
    }

    pub fn from_meters_per_second(meters_per_second: Option<f64>) -> Option<SpeedClass> {
        let speed = meters_per_second?;

        // Using match with f64 comparison
        match speed.partial_cmp(&0.0)? {
            Ordering::Less => None,
            Ordering::Greater | Ordering::Equal => {
                if speed < 14.0 {
                    Some(SpeedClass::Slow)
                } else if speed < 22.0 {
                    Some(SpeedClass::Medium)
                } else {
                    Some(SpeedClass::Fast)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(SpeedClass::Slow.as_str(), "slow");
        assert_eq!(SpeedClass::Medium.as_str(), "medium");
        assert_eq!(SpeedClass::Fast.as_str(), "fast");
    }

    #[test]
    fn test_from_meters_per_second_none() {
        assert_eq!(SpeedClass::from_meters_per_second(None), None);
    }

    #[test]
    fn test_from_meters_per_second_negative() {
        assert_eq!(SpeedClass::from_meters_per_second(Some(-1.0)), None);
    }

    #[test]
    fn test_from_meters_per_second_zero() {
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(0.0)),
            Some(SpeedClass::Slow)
        );
    }

    #[test]
    fn test_from_meters_per_second_slow() {
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(10.0)),
            Some(SpeedClass::Slow)
        );
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(13.9)),
            Some(SpeedClass::Slow)
        );
    }

    #[test]
    fn test_from_meters_per_second_medium() {
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(14.0)),
            Some(SpeedClass::Medium)
        );
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(18.0)),
            Some(SpeedClass::Medium)
        );
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(21.9)),
            Some(SpeedClass::Medium)
        );
    }

    #[test]
    fn test_from_meters_per_second_fast() {
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(22.0)),
            Some(SpeedClass::Fast)
        );
        assert_eq!(
            SpeedClass::from_meters_per_second(Some(30.0)),
            Some(SpeedClass::Fast)
        );
    }

    #[test]
    fn test_speed_class_equality() {
        assert_eq!(SpeedClass::Slow, SpeedClass::Slow);
        assert_eq!(SpeedClass::Medium, SpeedClass::Medium);
        assert_eq!(SpeedClass::Fast, SpeedClass::Fast);

        assert_ne!(SpeedClass::Slow, SpeedClass::Medium);
        assert_ne!(SpeedClass::Medium, SpeedClass::Fast);
        assert_ne!(SpeedClass::Slow, SpeedClass::Fast);
    }
}
