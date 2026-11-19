//! Edges and their per-robot-type properties, with trajectory, corridor and line guidance.

use serde::{Deserialize, Serialize};

use crate::action::{Action, MobileRobotType};
pub use crate::node::LoadRestriction;

/// An edge of a layout. The LIF contains only edges usable by at least one mobile robot type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    /// Unique identifier of the edge across all layouts of this LIF file.
    pub edge_id: String,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_descriptor: Option<String>,
    /// Id of the start node. Always part of the current layout.
    pub start_node_id: String,
    /// Id of the end node. May be in another layout (a transition between layouts).
    pub end_node_id: String,
    /// Properties per mobile robot type (at least one). One element per allowed robot type.
    pub mobile_robot_type_edge_properties: Vec<MobileRobotTypeEdgeProperty>,
}

/// Properties of an edge for the listed mobile robot types.
///
/// The spec says `trajectory` and `physicalLineGuidedProperty` shall not be used together
/// (the schema forbids both at once). This is not checked by the types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileRobotTypeEdgeProperty {
    /// The robot types these properties apply to (at least one). Each must also be listed in the
    /// node properties of the start and end node.
    pub mobile_robot_types: Vec<MobileRobotType>,
    /// How orientations are interpreted. Default if absent: `TANGENTIAL`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation_type: Option<OrientationType>,
    /// Only for omni-directional robots. `true` (default): reach the orientation before entering
    /// the edge; `false`: rotate on the edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reach_orientation_before_entering: Option<bool>,
    /// All possible orientations of the robot on the edge, in rad. Fleet control selects one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_robot_orientations: Option<Vec<f64>>,
    /// Allowed rotation directions at the start node. Default if absent: `BOTH`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_at_start_node_allowed: Option<RotationAllowed>,
    /// Allowed rotation directions at the end node. Default if absent: `BOTH`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_at_end_node_allowed: Option<RotationAllowed>,
    /// Permitted maximum speed in m/s (at least 0). No limit if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_speed: Option<f64>,
    /// Maximum rotation speed in rad/s (at least 0). No limit if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_rotation_speed: Option<f64>,
    /// Permitted minimal height of the load handling device in m. No limit if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_load_handling_device_height: Option<f64>,
    /// Permitted maximum height of the robot including the load, in m (at least 0).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_mobile_robot_height: Option<f64>,
    /// `true` (default): robots may enter automatic management by fleet control while on this edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reentry_allowed: Option<bool>,
    /// Actions that shall always be included on this edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<Action>>,
    /// Actions on this edge whose validity depends on outside factors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feasible_actions: Option<Vec<Action>>,
    /// Action types strictly forbidden on this edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forbidden_action_types: Option<Vec<String>>,
    /// Load restriction on this edge. If absent, unloaded and loaded robots (any load set) may use it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_restriction: Option<LoadRestriction>,
    /// The curve the robot moves on between the nodes, as a NURBS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<Trajectory>,
    /// For simple robots (line- or wire-guided) that cannot process trajectories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_line_guided_property: Option<PhysicalLineGuidedProperty>,
    /// Boundaries within which the robot may deviate from its trajectory. No corridor if absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corridor: Option<Corridor>,
}

/// How the orientations of an edge are interpreted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrientationType {
    /// Relative to the global project-specific map coordinate system.
    Global,
    /// Tangential to the edge (0.0 forwards, π backwards).
    Tangential,
}

/// Allowed directions of rotation at a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RotationAllowed {
    /// Rotation not allowed.
    None,
    /// Counter-clockwise (positive).
    Ccw,
    /// Clockwise (negative).
    Cw,
    /// Both directions.
    Both,
}

/// A NURBS trajectory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trajectory {
    /// Degree of the curve (at least 1). Default if absent: 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<u32>,
    /// Knot values, each in `0.0..=1.0`. If absent: uniform clamped knots.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knot_vector: Option<Vec<f64>>,
    /// Control points including start and end point (at least 2, and at least `degree` + 1).
    pub control_points: Vec<ControlPoint>,
}

/// A control point of a NURBS, in m relative to the origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlPoint {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Weight of the point on the curve (greater than 0). Default if absent: 1.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

/// Information for robots that cannot process trajectories. Both fields optional.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PhysicalLineGuidedProperty {
    /// Direction identifier of the edge at junctions (line- or wire-guided robots).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Length of the edge in m, for robots that need it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
}

/// Boundaries in which a robot can deviate from its trajectory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Corridor {
    /// Maximum corridor margin to the left of the edge in m (at least 0).
    pub maximum_left_width: f64,
    /// Maximum corridor margin to the right of the edge in m (at least 0).
    pub maximum_right_width: f64,
    /// Whether the boundaries apply to the kinematic center (default) or the contour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corridor_reference_point: Option<CorridorReferencePoint>,
}

/// Point of the robot the corridor boundaries apply to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CorridorReferencePoint {
    /// The kinematic center of the robot.
    KinematicCenter,
    /// The contour of the robot.
    Contour,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROBOT: &str = r#"{"manufacturer":"ACME","seriesName":"X1"}"#;

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
    fn edge_with_every_property() {
        let json = format!(
            r#"{{"edgeId":"e1","edgeDescriptor":"aisle","startNodeId":"n1","endNodeId":"n2","mobileRobotTypeEdgeProperties":[{{"mobileRobotTypes":[{ROBOT}],"orientationType":"GLOBAL","reachOrientationBeforeEntering":false,"mobileRobotOrientations":[0.0,3.14],"rotationAtStartNodeAllowed":"CCW","rotationAtEndNodeAllowed":"NONE","maximumSpeed":1.5,"maximumRotationSpeed":0.5,"minimumLoadHandlingDeviceHeight":0.1,"maximumMobileRobotHeight":2.0,"reentryAllowed":true,"requiredActions":[{{"actionType":"a","blockingType":"NONE"}}],"feasibleActions":[{{"actionType":"b","blockingType":"SOFT"}}],"forbiddenActionTypes":["c"],"loadRestriction":{{"unloaded":true,"loaded":false}},"trajectory":{{"degree":2,"knotVector":[0.0,0.0,0.0,1.0,1.0,1.0],"controlPoints":[{{"x":0.0,"y":0.0}},{{"x":1.0,"y":1.0,"weight":2.0}},{{"x":2.0,"y":0.0}}]}},"physicalLineGuidedProperty":{{"direction":"left","length":5.0}},"corridor":{{"maximumLeftWidth":0.5,"maximumRightWidth":0.7,"corridorReferencePoint":"CONTOUR"}}}}]}}"#
        );
        let e: Edge = round_trip(&json);
        let p = &e.mobile_robot_type_edge_properties[0];
        // both present at once is accepted (decision Q250)
        assert!(p.trajectory.is_some() && p.physical_line_guided_property.is_some());
        assert_eq!(p.rotation_at_start_node_allowed, Some(RotationAllowed::Ccw));
    }

    #[test]
    fn minimal_edge_and_empty_line_guided() {
        let json = format!(
            r#"{{"edgeId":"e","startNodeId":"a","endNodeId":"b","mobileRobotTypeEdgeProperties":[{{"mobileRobotTypes":[{ROBOT}],"physicalLineGuidedProperty":{{}}}}]}}"#
        );
        let e: Edge = round_trip(&json);
        let p = &e.mobile_robot_type_edge_properties[0];
        assert_eq!(
            p.physical_line_guided_property,
            Some(PhysicalLineGuidedProperty::default())
        );
        assert_eq!(p.trajectory, None);
    }

    #[test]
    fn enums_accept_all_values_and_reject_others() {
        for s in ["NONE", "CCW", "CW", "BOTH"] {
            let v: RotationAllowed = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(serde_json::to_string(&v).unwrap(), format!("\"{s}\""));
        }
        for s in ["GLOBAL", "TANGENTIAL"] {
            let v: OrientationType = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(serde_json::to_string(&v).unwrap(), format!("\"{s}\""));
        }
        for s in ["KINEMATIC_CENTER", "CONTOUR"] {
            let v: CorridorReferencePoint = serde_json::from_str(&format!("\"{s}\"")).unwrap();
            assert_eq!(serde_json::to_string(&v).unwrap(), format!("\"{s}\""));
        }
        assert!(serde_json::from_str::<RotationAllowed>("\"SIDEWAYS\"").is_err());
        assert!(serde_json::from_str::<CorridorReferencePoint>("\"KINEMATICCENTER\"").is_err());
    }

    #[test]
    fn missing_required_fields_are_rejected() {
        assert!(serde_json::from_str::<Corridor>(r#"{"maximumLeftWidth":1.0}"#).is_err());
        assert!(serde_json::from_str::<Trajectory>(r#"{"degree":1}"#).is_err());
        assert!(serde_json::from_str::<ControlPoint>(r#"{"x":1.0}"#).is_err());
        assert!(serde_json::from_str::<Edge>(
            r#"{"edgeId":"e","startNodeId":"a","endNodeId":"b"}"#
        )
        .is_err());
    }
}
