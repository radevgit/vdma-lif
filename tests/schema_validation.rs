//! Validates the official LIF examples (extracted from `spec/examples/*.md`) against the
//! vendored schema `spec/LIF.schema`.

use serde_json::Value;

fn read(path: &str) -> Value {
    let text = std::fs::read_to_string(format!("{}/{path}", env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("{path}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn validator() -> jsonschema::Validator {
    jsonschema::options()
        .should_validate_formats(true)
        .build(&read("spec/LIF.schema"))
        .unwrap_or_else(|e| panic!("schema: {e}"))
}

fn fixtures() -> Vec<String> {
    let dir = format!("{}/tests/examples", env!("CARGO_MANIFEST_DIR"));
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .filter(|n| n.ends_with(".json"))
        .collect();
    names.sort();
    names
}

#[test]
fn all_official_examples_validate_against_the_schema() {
    let v = validator();
    let names = fixtures();
    assert_eq!(names.len(), 28, "expected the 28 full LIF examples");
    for n in names {
        let errors: Vec<String> = v
            .iter_errors(&read(&format!("tests/examples/{n}")))
            .map(|e| format!("{e} at {}", e.instance_path()))
            .collect();
        assert!(
            errors.is_empty(),
            "{n} is not valid:\n{}",
            errors.join("\n")
        );
    }
}

/// The validator must really reject things, otherwise the test above proves nothing.
#[test]
fn validator_rejects_bad_layouts() {
    let v = validator();
    let good = read("tests/examples/01-forward-edge.json");
    assert!(v.is_valid(&good));

    let mut bad = good.clone();
    bad["metaInformation"]["lifVersion"] = "two".into();
    assert!(!v.is_valid(&bad), "bad lifVersion pattern accepted");

    let mut bad = good.clone();
    bad["origins"] = serde_json::json!([]);
    assert!(!v.is_valid(&bad), "empty origins accepted");

    let mut bad = good.clone();
    bad["metaInformation"]["exportTimestamp"] = "yesterday".into();
    assert!(!v.is_valid(&bad), "bad date-time accepted");

    let mut bad = good;
    bad["origins"][0]["layouts"][0]["edges"][0]["mobileRobotTypeEdgeProperties"][0]
        ["rotationAtStartNodeAllowed"] = "SIDEWAYS".into();
    assert!(!v.is_valid(&bad), "bad enum accepted");
}

/// Numbers compare by value, so `1` equals `1.0` (the official files cannot be edited).
fn same_json(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same_json(p, q))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| same_json(v, w)))
        }
        _ => a == b,
    }
}

/// Every official example: parse into the types, write back, same JSON, and still valid.
#[test]
fn all_official_examples_round_trip_through_the_types() {
    let v = validator();
    for n in fixtures() {
        let original = read(&format!("tests/examples/{n}"));
        let typed: vdma_lif::Lif = serde_json::from_value(original.clone())
            .unwrap_or_else(|e| panic!("{n}: does not parse: {e}"));
        let back = serde_json::to_value(&typed).unwrap();
        assert!(same_json(&back, &original), "{n}: round trip differs");
        assert!(v.is_valid(&back), "{n}: serialised output not valid");
    }
}

#[test]
fn same_json_is_not_fooled() {
    assert!(same_json(
        &serde_json::json!({"a": 1}),
        &serde_json::json!({"a": 1.0})
    ));
    assert!(!same_json(
        &serde_json::json!({"a": 1}),
        &serde_json::json!({"a": 2})
    ));
    assert!(!same_json(
        &serde_json::json!({"a": 1}),
        &serde_json::json!({"a": 1, "b": 1})
    ));
}
