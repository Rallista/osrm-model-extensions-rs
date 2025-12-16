use osrm_openapi_models::models::{Annotation, RouteStep};

use crate::geo::utilities::get_coordinate_index;
use crate::osrm::RouteStepExt;

pub(crate) fn step_maneuver_name(step: RouteStep) -> String {
    step.name
        .clone()
        .filter(|s| !s.is_empty())
        .or_else(|| step.r#ref.clone().filter(|s| !s.is_empty()))
        .or_else(
            || match (step.exits.is_none(), step.destinations.is_none()) {
                (true, false) => step
                    .destinations_string()
                    .ok()
                    .flatten()
                    .filter(|s| !s.is_empty()),
                (false, false) => {
                    let exits = step.exits_string().ok().flatten().unwrap_or_default();
                    let destinations = step
                        .destinations_string()
                        .ok()
                        .flatten()
                        .unwrap_or_default();
                    Some(format!("{} {}", exits, destinations))
                }
                _ => None,
            },
        )
        .unwrap_or_default()
}

pub(crate) fn speed_at_distance(
    geometry: String,
    annotations: Annotation,
    distance: f64,
    polyline_precision: u32,
) -> Option<f64> {
    let index = get_coordinate_index(geometry.as_str(), polyline_precision, distance)?;
    annotations.speed?.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_step_maneuver_label_with_name() {
        let step = RouteStep {
            name: Some("Main Street".to_string()),
            r#ref: None,
            exits: None,
            destinations: None,
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "Main Street");
    }

    #[test]
    fn test_step_maneuver_label_with_ref() {
        let step = RouteStep {
            name: None,
            r#ref: Some("I-95".to_string()),
            exits: None,
            destinations: None,
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "I-95");
    }

    #[test]
    fn test_step_maneuver_label_with_destinations_only() {
        let step = RouteStep {
            name: None,
            r#ref: None,
            exits: None,
            destinations: Some(json!("Boston")),
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "Boston");
    }

    #[test]
    fn test_step_maneuver_label_with_exits_and_destinations() {
        let step = RouteStep {
            name: None,
            r#ref: None,
            exits: Some(json!("Exit 23")),
            destinations: Some(json!("New York")),
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "Exit 23 New York");
    }

    #[test]
    fn test_step_maneuver_label_with_all_none() {
        let step = RouteStep {
            name: None,
            r#ref: None,
            exits: None,
            destinations: None,
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "");
    }

    #[test]
    fn test_step_maneuver_label_precedence() {
        // Testing that name takes precedence over ref and destinations
        let step = RouteStep {
            name: Some("Main Street".to_string()),
            r#ref: Some("I-95".to_string()),
            exits: Some(json!("Exit 1")),
            destinations: Some(json!("Boston")),
            ..Default::default()
        };

        assert_eq!(step_maneuver_name(step), "Main Street");
    }
}
