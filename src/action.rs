//! Actions and mobile robot types, shared by node and edge properties.

use serde::{Deserialize, Serialize};

/// A mobile robot type: manufacturer and series, as in the robot's factsheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotType {
    /// Name of the manufacturer. Should correspond to the manufacturer field of the MQTT header
    /// and topic level.
    pub manufacturer: String,
    /// Unique name of the robot series in the context of the manufacturer. Should correspond to
    /// `factsheet.typeSpecification.seriesName`.
    pub series_name: String,
}

/// An action on a node or edge. Same meaning as the VDA 5050 action, but without an `actionId`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    /// Name of the action as in the VDA 5050 specification. Manufacturer-specific actions are
    /// allowed if agreed with the fleet control system.
    pub action_type: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_descriptor: Option<String>,
    /// Whether the action may run in parallel with others and with movement.
    pub blocking_type: BlockingType,
    /// Parameters with statically defined values sent along with the action. Other parameters
    /// with dynamic values may be added by fleet control (see the robot's factsheet).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_parameters: Option<Vec<ActionParameter>>,
}

/// Blocking type of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockingType {
    /// Can run in parallel with other actions and movement.
    None,
    /// Can run in parallel with other actions, but movement is stopped.
    Soft,
    /// Can run in parallel with movement, but no other action.
    Single,
    /// Neither other actions nor movement may happen at the same time.
    Hard,
}

/// A key/value parameter of an action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionParameter {
    /// The key of the parameter.
    pub key: String,
    /// The value: any JSON value (array, boolean, number, string or object). The data type is
    /// defined in the robot's VDA 5050 factsheet.
    pub value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip<T: Serialize + for<'a> Deserialize<'a>>(json: &str) -> T {
        let typed: T = serde_json::from_str(json).unwrap();
        let back = serde_json::to_value(&typed).unwrap();
        assert_eq!(
            back,
            serde_json::from_str::<serde_json::Value>(json).unwrap()
        );
        typed
    }

    #[test]
    fn robot_type() {
        let t: MobileRobotType = round_trip(r#"{"manufacturer":"ACME","seriesName":"X1"}"#);
        assert_eq!(t.series_name, "X1");
    }

    #[test]
    fn action_full_and_minimal() {
        let a: Action = round_trip(
            r#"{"actionType":"beep","actionDescriptor":"warn","blockingType":"SINGLE","actionParameters":[{"key":"n","value":3},{"key":"on","value":true},{"key":"l","value":["a"]},{"key":"o","value":{"x":1}},{"key":"s","value":"left"}]}"#,
        );
        assert_eq!(a.blocking_type, BlockingType::Single);
        let m: Action = round_trip(r#"{"actionType":"beep","blockingType":"NONE"}"#);
        assert_eq!(m.action_parameters, None);
    }

    #[test]
    fn all_blocking_types_and_bad_ones() {
        for (s, b) in [
            ("NONE", BlockingType::None),
            ("SOFT", BlockingType::Soft),
            ("SINGLE", BlockingType::Single),
            ("HARD", BlockingType::Hard),
        ] {
            let v: BlockingType = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(v, b);
        }
        assert!(serde_json::from_str::<BlockingType>("\"MEDIUM\"").is_err());
    }

    #[test]
    fn missing_required_fields_are_rejected() {
        assert!(serde_json::from_str::<Action>(r#"{"actionType":"beep"}"#).is_err());
        assert!(serde_json::from_str::<MobileRobotType>(r#"{"manufacturer":"ACME"}"#).is_err());
        assert!(serde_json::from_str::<ActionParameter>(r#"{"key":"k"}"#).is_err());
    }
}
