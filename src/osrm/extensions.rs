use osrm_openapi_models::models::{RouteStep, StepManeuver};
use serde_json::Value;

/// Extension trait for RouteStep to safely extract strings from Value fields
pub trait RouteStepExt {
    /// Extract geometry as a string, returning an error for unsupported object types
    fn geometry_string(&self) -> Result<Option<String>, String>;

    /// Extract destinations as a string, returning an error for unsupported object types
    fn destinations_string(&self) -> Result<Option<String>, String>;

    /// Extract exits as a string, returning an error for unsupported object types
    fn exits_string(&self) -> Result<Option<String>, String>;
}

/// Extension trait for StepManeuver to safely extract strings from Value fields
pub trait StepManeuverExt {
    /// Extract instruction as a string, returning an error for unsupported object types
    fn instruction_string(&self) -> Result<Option<String>, String>;
}

impl RouteStepExt for RouteStep {
    fn geometry_string(&self) -> Result<Option<String>, String> {
        value_to_string(self.geometry.as_ref(), "geometry")
    }

    fn destinations_string(&self) -> Result<Option<String>, String> {
        value_to_string(self.destinations.as_ref(), "destinations")
    }

    fn exits_string(&self) -> Result<Option<String>, String> {
        value_to_string(self.exits.as_ref(), "exits")
    }
}

impl StepManeuverExt for StepManeuver {
    fn instruction_string(&self) -> Result<Option<String>, String> {
        // instruction is already a String, not a Value
        Ok(self.instruction.clone())
    }
}

/// Helper function to safely convert a Value to a String
fn value_to_string(value: Option<&Value>, field_name: &str) -> Result<Option<String>, String> {
    match value {
        None => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(Value::Null) => Ok(None),
        Some(Value::Object(_)) => Err(format!(
            "TODO: {} field contains a JSON object, which is not yet supported. \
                 Please file an issue with your use case.",
            field_name
        )),
        Some(Value::Array(_)) => Err(format!(
            "TODO: {} field contains a JSON array, which is not yet supported. \
                 Please file an issue with your use case.",
            field_name
        )),
        Some(Value::Number(n)) => Ok(Some(n.to_string())),
        Some(Value::Bool(b)) => Ok(Some(b.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_geometry_string_with_string() {
        let step = RouteStep {
            geometry: Some(json!("abc123polyline")),
            ..Default::default()
        };
        assert_eq!(
            step.geometry_string().unwrap(),
            Some("abc123polyline".to_string())
        );
    }

    #[test]
    fn test_geometry_string_with_null() {
        let step = RouteStep {
            geometry: Some(Value::Null),
            ..Default::default()
        };
        assert_eq!(step.geometry_string().unwrap(), None);
    }

    #[test]
    fn test_geometry_string_with_none() {
        let step = RouteStep {
            geometry: None,
            ..Default::default()
        };
        assert_eq!(step.geometry_string().unwrap(), None);
    }

    #[test]
    fn test_geometry_string_with_object_returns_error() {
        let step = RouteStep {
            geometry: Some(json!({"type": "LineString", "coordinates": [[0, 0]]})),
            ..Default::default()
        };
        assert!(step.geometry_string().is_err());
        assert!(
            step.geometry_string()
                .unwrap_err()
                .contains("not yet supported")
        );
    }

    #[test]
    fn test_geometry_string_with_array_returns_error() {
        let step = RouteStep {
            geometry: Some(json!(["point1", "point2"])),
            ..Default::default()
        };
        assert!(step.geometry_string().is_err());
    }

    #[test]
    fn test_destinations_string_with_string() {
        let step = RouteStep {
            destinations: Some(json!("New York")),
            ..Default::default()
        };
        assert_eq!(
            step.destinations_string().unwrap(),
            Some("New York".to_string())
        );
    }

    #[test]
    fn test_exits_string_with_string() {
        let step = RouteStep {
            exits: Some(json!("Exit 23")),
            ..Default::default()
        };
        assert_eq!(step.exits_string().unwrap(), Some("Exit 23".to_string()));
    }

    #[test]
    fn test_geometry_string_with_number() {
        let step = RouteStep {
            geometry: Some(json!(42)),
            ..Default::default()
        };
        assert_eq!(step.geometry_string().unwrap(), Some("42".to_string()));
    }

    #[test]
    fn test_geometry_string_with_bool() {
        let step = RouteStep {
            geometry: Some(json!(true)),
            ..Default::default()
        };
        assert_eq!(step.geometry_string().unwrap(), Some("true".to_string()));
    }
}
