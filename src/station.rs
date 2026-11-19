//! Stations: logical places where a mobile robot can interact with the environment.

use serde::{Deserialize, Serialize};

/// A station: any logical place where a mobile robot can explicitly interact with the
/// environment, including but not limited to physical interactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    /// Unique identifier of the station across all layouts of this LIF file. Recommended to
    /// match between all LIFs, WMSs and physical labelling.
    pub station_id: String,
    /// Name of the station. May be forwarded to the robot in a VDA 5050 order (e.g. pick/drop).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_name: Option<String>,
    /// User-defined, human-readable name. Not to be used for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_descriptor: Option<String>,
    /// Type of the station. May be forwarded to the robot in a VDA 5050 order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_type: Option<String>,
    /// Physical height in m (at least 0) of the base of the load on the station when picked up or
    /// dropped off. If absent, it must not be assumed to be zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_height: Option<f64>,
    /// Nodes at which interaction with the station takes place (at least one). Which one is used
    /// is up to fleet control.
    pub interaction_node_ids: Vec<String>,
    /// Center point and orientation of the station. For visualization only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_position: Option<StationPosition>,
}

/// Center point and orientation of a station, relative to the origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationPosition {
    /// X-coordinate in m.
    pub x: f64,
    /// Y-coordinate in m.
    pub y: f64,
    /// Absolute orientation in rad (`-π..=π`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(json: &str) -> Station {
        let typed: Station = serde_json::from_str(json).unwrap();
        let back = serde_json::to_value(&typed).unwrap();
        assert_eq!(
            back,
            serde_json::from_str::<serde_json::Value>(json).unwrap()
        );
        typed
    }

    #[test]
    fn full_station() {
        let s = round_trip(
            r#"{"stationId":"s1","stationName":"Dock 1","stationDescriptor":"d","stationType":"RACK","stationHeight":0.8,"interactionNodeIds":["n1","n2"],"stationPosition":{"x":1.0,"y":2.0,"theta":1.5}}"#,
        );
        assert_eq!(s.interaction_node_ids.len(), 2);
        assert_eq!(s.station_position.unwrap().theta, Some(1.5));
    }

    #[test]
    fn minimal_station_and_position_without_theta() {
        let s = round_trip(r#"{"stationId":"s","interactionNodeIds":["n"]}"#);
        assert_eq!(s.station_height, None);
        let s = round_trip(
            r#"{"stationId":"s","interactionNodeIds":["n"],"stationPosition":{"x":0.0,"y":0.0}}"#,
        );
        assert_eq!(s.station_position.unwrap().theta, None);
    }

    #[test]
    fn missing_required_fields_are_rejected() {
        assert!(serde_json::from_str::<Station>(r#"{"stationId":"s"}"#).is_err());
        assert!(serde_json::from_str::<Station>(r#"{"interactionNodeIds":["n"]}"#).is_err());
        assert!(serde_json::from_str::<StationPosition>(r#"{"x":1.0}"#).is_err());
    }
}
