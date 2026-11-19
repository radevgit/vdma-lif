//! Meta information, origins and layouts.

use serde::{Deserialize, Serialize};

use crate::edge::Edge;
use crate::node::Node;
use crate::station::Station;

/// Meta information about the LIF file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInformation {
    /// Unique identifier of the LIF file describing a specific facility. Multiple exports for the
    /// same facility should share it; tell them apart with `export_timestamp`.
    pub lif_id: String,
    /// Creator of the LIF file (company or person).
    pub creator: String,
    /// When the file was created/updated, ISO 8601 UTC (`YYYY-MM-DDTHH:mm:ss.fffZ`).
    pub export_timestamp: String,
    /// Version of LIF, `[Major].[Minor].[Patch]`, e.g. `2.0.0`.
    pub lif_version: String,
}

/// An origin: the common reference for several associated layouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    /// Unique identifier of the origin.
    pub origin_id: String,
    /// User-defined, human-readable name (e.g. "Hall B: Floors 1, 2, and 3"). Not for logic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_descriptor: Option<String>,
    /// Layouts within the facility, all sharing this origin (at least one).
    pub layouts: Vec<Layout>,
}

/// A layout for order generation and routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    /// Unique identifier of the layout.
    pub layout_id: String,
    /// User-defined, human-readable name. Not for logical purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_descriptor: Option<String>,
    /// Version of the layout. Suggested: an integer as a string, incremented with each change,
    /// starting at `"1"`.
    pub layout_version: String,
    /// Which level or floor the layout represents, where there are several.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_level_id: Option<String>,
    /// All nodes of the layout.
    pub nodes: Vec<Node>,
    /// All edges of the layout.
    pub edges: Vec<Edge>,
    /// All stations of the layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stations: Option<Vec<Station>>,
}
