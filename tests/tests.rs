#[cfg(test)]
use assert_json_diff::*;
use octopt::*;
use reqwest::*;
use serde_json::*;

/// Deserializes the options set by Octo for a new game.
#[test]
fn default_octo_options() {
    let octo_defaults = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":0,"loadStoreQuirks":0,"vfOrderQuirks":0,"clipQuirks":1,"vBlankQuirks":1,"jumpQuirks":0,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":1,"fontStyle":"octo"});
    let octo_defaults_bool = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"});
    let deserialized_defaults: OctoOptions = octo_defaults.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

#[test]
fn deserialize_default_octo_options() {
    let octo_defaults = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":0,"loadStoreQuirks":0,"vfOrderQuirks":0,"clipQuirks":1,"vBlankQuirks":1,"jumpQuirks":0,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":1,"fontStyle":"octo"});
    let octo_defaults_bool = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"});
    let deserialized_defaults: OctoOptions = octo_defaults.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

/// Deserializes the options set by Octo for a new game.
#[test]
fn default_octo_options_bool() {
    let octo_defaults_bool = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"});
    let deserialized_defaults: OctoOptions = octo_defaults_bool.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

/// Deserializes the empty option set
#[test]
fn empty_options() {
    let empty = json!({});
    let deserialized_empty: OctoOptions = empty.to_string().parse().unwrap();
    println!("{}", deserialized_empty);
}

/// Downloads the CHIP-8 Community Archive programs.json and tries to parse every single one
#[test]
fn chip8_archive() {
    let body = blocking::get(
        "https://raw.githubusercontent.com/JohnEarnest/chip8Archive/master/programs.json",
    )
    .unwrap()
    .text()
    .unwrap();
    let programs: Value = body.parse().unwrap();
    for (_, program) in programs.as_object().unwrap() {
        let _: OctoOptions = program["options"].to_string().parse().unwrap();
    }
}
