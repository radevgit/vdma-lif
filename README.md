# vdma-lif

Rust types for the **LIF (Layout Interchange Format) version 2.0.0** of the VDMA: track layouts (nodes, edges, stations, per robot type) as JSON. LIF is the sister format of VDA 5050: it carries a site's layout from the vehicle integrator to a master control system.

- All types of the LIF 2.0.0 schema: `Lif`, meta information, origins, layouts, nodes, edges, stations, per-robot-type properties, actions, trajectories (NURBS), corridors, load restrictions.
- Field names and enum strings match the format exactly (including `allowedDeviationXY`).
- Optional fields are `Option` and are left out of the JSON when `None`.
- Only `serde` and `serde_json` as dependencies.
- Tests check that all 28 official examples parse into the types, serialise back to the same JSON, and validate against the official JSON schema (vendored in `spec/`, see `spec/SOURCE.md`).

Not included: geometry processing, layout validation beyond what the types express, graph algorithms, conversion to or from VDA 5050.

## Example

```rust
use vdma_lif::Lif;

fn main() -> Result<(), serde_json::Error> {
    let json = r#"{
        "metaInformation": {
            "lifId": "demo", "creator": "me",
            "exportTimestamp": "2026-11-19T10:00:00.000Z", "lifVersion": "2.0.0"
        },
        "origins": [{
            "originId": "o1",
            "layouts": [{
                "layoutId": "l1", "layoutVersion": "1",
                "nodes": [
                    {"nodeId": "A", "mapId": "m", "nodePosition": {"x": 0.0, "y": 0.0},
                     "mobileRobotTypeNodeProperties": [
                        {"mobileRobotTypes": [{"manufacturer": "ACME", "seriesName": "X1"}]}]},
                    {"nodeId": "B", "mapId": "m", "nodePosition": {"x": 10.0, "y": 0.0},
                     "mobileRobotTypeNodeProperties": [
                        {"mobileRobotTypes": [{"manufacturer": "ACME", "seriesName": "X1"}]}]}
                ],
                "edges": [{
                    "edgeId": "A-B", "startNodeId": "A", "endNodeId": "B",
                    "mobileRobotTypeEdgeProperties": [{
                        "mobileRobotTypes": [{"manufacturer": "ACME", "seriesName": "X1"}],
                        "maximumSpeed": 1.5
                    }]
                }]
            }]
        }]
    }"#;

    let lif: Lif = serde_json::from_str(json)?;
    let layout = &lif.origins[0].layouts[0];
    for edge in &layout.edges {
        let speed = edge.mobile_robot_type_edge_properties[0].maximum_speed;
        println!("{} -> {}: max speed {:?}", edge.start_node_id, edge.end_node_id, speed);
        assert_eq!(speed, Some(1.5));
    }

    // Types serialise back to the exact JSON field names.
    assert!(serde_json::to_string(&lif)?.contains("\"mobileRobotTypeEdgeProperties\""));
    Ok(())
}
```

## Notes on the format

- **No limits enforced.** Ranges (e.g. `theta` within ±π, `degree` at least 1) and `minItems` rules of the schema are in the doc comments only; the types accept what a real file contains. The spec's defaults (e.g. `orientationType` `TANGENTIAL`) are not applied either: an absent field is `None`.
- **`trajectory` and `physicalLineGuidedProperty`** shall not be used together on one edge; the types accept both.
- **`loadRestriction`:** `LIF.md` §8.3.11 describes a `restrictionType` enum, but the schema and all official examples use the booleans `unloaded` and `loaded`. This crate follows the schema and the examples. See `spec/SOURCE.md`.

## Version and source

Follows LIF **2.0.0** (September 2026; branch `dev/V2` of the [official repository](https://github.com/Intralogistics-2X-LIF/Layout-Interchange-Format), commit `d689808`, 2026-10-27). The specification, schema and examples are vendored in `spec/`.

LIF - Layout Interchange Format, © VDMA e. V. Reprinting and any other form of reproduction is permitted only if the source is acknowledged; the source is the repository above.

## Licence

MIT. The vendored specification in `spec/` is MIT-licensed by VDMA e. V. (`spec/LICENSE-VDMA`).
