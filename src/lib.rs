#![doc = include_str!("../README.md")]

pub mod action;
pub mod edge;
pub mod layout;
pub mod node;
pub mod station;

use serde::{Deserialize, Serialize};

pub use layout::{Layout, MetaInformation, Origin};

/// A LIF file: meta information and one or more origins with their layouts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lif {
    /// Meta information about the file.
    pub meta_information: MetaInformation,
    /// Origins (at least one), each with its layouts.
    pub origins: Vec<Origin>,
}
