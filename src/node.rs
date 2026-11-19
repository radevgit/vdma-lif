//! Nodes and their per-robot-type properties.

use serde::{Deserialize, Serialize};

use crate::action::{Action, MobileRobotType};

/// A node of a layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Unique identifier of the node across all layouts of this LIF file. Different LIF files may
    /// contain duplicate node IDs; mapping them is up to fleet control.
    pub node_id: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_descriptor: Option<String>,
    /// Unique identification of the map in which the node or its position is referenced.
    pub map_id: String,
    /// Geometric location of the node.
    pub node_position: NodePosition,
    /// Properties per mobile robot type (at least one). A robot type without an entry may not use
    /// the node. At most one entry per robot type.
    pub mobile_robot_type_node_properties: Vec<MobileRobotTypeNodeProperty>,
}

/// Geometric location of a node, in m, relative to the origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodePosition {
    /// X-coordinate on the layout in m.
    pub x: f64,
    /// Y-coordinate on the layout in m.
    pub y: f64,
}

/// Properties of a node for the listed mobile robot types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotTypeNodeProperty {
    /// The robot types these properties apply to (at least one).
    pub mobile_robot_types: Vec<MobileRobotType>,
    /// Absolute orientation of the robot on the node in rad (`-π..=π`), relative to the origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// How far the robot may deviate from the node position to traverse it smoothly.
    #[serde(
        rename = "allowedDeviationXY",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_deviation_xy: Option<AllowedDeviationXy>,
    /// Actions that shall always be included on this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<Action>>,
    /// Actions on this node whose validity depends on outside factors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feasible_actions: Option<Vec<Action>>,
    /// Action types strictly forbidden on this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forbidden_action_types: Option<Vec<String>>,
    /// Load restriction on this node. If absent, unloaded and loaded robots (any load set) may use it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_restriction: Option<LoadRestriction>,
}

/// An ellipse around the node position within which the robot's control point may deviate. The
/// node position is the ellipse's center. All fields optional; see LIF.md §8.3.8 for the rules
/// on minimum and maximum values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedDeviationXy {
    /// Minimum semi-major axis in m (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a_minimum: Option<f64>,
    /// Maximum semi-major axis in m (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a_maximum: Option<f64>,
    /// Minimum semi-minor axis in m (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b_minimum: Option<f64>,
    /// Maximum semi-minor axis in m (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b_maximum: Option<f64>,
    /// Rotation angle of the major axis in rad (`-π/2..=π/2`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
}

/// Which loads a node or edge may be traversed with.
///
/// Note: `LIF.md` §8.3.11 describes a `restrictionType` enum instead of the booleans; this crate
/// follows the schema and the official examples (see `spec/SOURCE.md`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadRestriction {
    /// `true`: the robot may traverse unloaded.
    pub unloaded: bool,
    /// `true`: the robot may traverse loaded.
    pub loaded: bool,
    /// Load sets that may be transported (names as in the robot's factsheet). Only evaluated if
    /// `loaded`. If absent or empty, all load sets the robot supports are allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_set_names: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const NODE: &str = r#"{"nodeId":"n1","nodeDescriptor":"start","mapId":"m1","nodePosition":{"x":1.5,"y":-2.0},"mobileRobotTypeNodeProperties":[{"mobileRobotTypes":[{"manufacturer":"ACME","seriesName":"X1"}],"theta":1.5,"allowedDeviationXY":{"aMinimum":0.1,"aMaximum":0.5,"bMinimum":0.1,"bMaximum":0.3,"theta":0.2},"requiredActions":[{"actionType":"beep","blockingType":"NONE"}],"feasibleActions":[{"actionType":"pick","blockingType":"HARD"}],"forbiddenActionTypes":["drop"],"loadRestriction":{"unloaded":true,"loaded":true,"loadSetNames":["EUR"]}}]}"#;

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
    fn node_full() {
        let n: Node = round_trip(NODE);
        let p = &n.mobile_robot_type_node_properties[0];
        assert_eq!(
            p.allowed_deviation_xy.as_ref().unwrap().a_maximum,
            Some(0.5)
        );
    }

    #[test]
    fn node_minimal_and_empty_deviation() {
        let n: Node = round_trip(
            r#"{"nodeId":"n","mapId":"m","nodePosition":{"x":0.0,"y":0.0},"mobileRobotTypeNodeProperties":[{"mobileRobotTypes":[{"manufacturer":"A","seriesName":"S"}],"allowedDeviationXY":{}}]}"#,
        );
        let p = &n.mobile_robot_type_node_properties[0];
        assert_eq!(p.allowed_deviation_xy, Some(AllowedDeviationXy::default()));
        assert_eq!(p.theta, None);
    }

    #[test]
    fn json_name_is_allowed_deviation_xy_uppercase() {
        let p = MobileRobotTypeNodeProperty {
            mobile_robot_types: vec![],
            theta: None,
            allowed_deviation_xy: Some(AllowedDeviationXy::default()),
            required_actions: None,
            feasible_actions: None,
            forbidden_action_types: None,
            load_restriction: None,
        };
        let s = serde_json::to_string(&p).unwrap();
        assert!(s.contains("\"allowedDeviationXY\""), "{s}");
    }

    #[test]
    fn load_restriction_follows_booleans() {
        let r: LoadRestriction = round_trip(r#"{"unloaded":false,"loaded":true}"#);
        assert!(r.loaded && !r.unloaded);
        assert!(serde_json::from_str::<LoadRestriction>(r#"{"restrictionType":"BOTH"}"#).is_err());
        assert!(serde_json::from_str::<LoadRestriction>(r#"{"unloaded":true}"#).is_err());
    }

    #[test]
    fn missing_required_fields_are_rejected() {
        assert!(serde_json::from_str::<Node>(r#"{"nodeId":"n","mapId":"m"}"#).is_err());
        assert!(serde_json::from_str::<NodePosition>(r#"{"x":1.0}"#).is_err());
    }
}
