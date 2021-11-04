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
    let deserialized_defaults: Options = octo_defaults.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

#[test]
fn deserialize_default_octo_options() {
    let octo_defaults = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":0,"loadStoreQuirks":0,"vfOrderQuirks":0,"clipQuirks":1,"vBlankQuirks":1,"jumpQuirks":0,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":1,"fontStyle":"octo"});
    let octo_defaults_bool = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"});
    let deserialized_defaults: Options = octo_defaults.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

/// Deserializes the options set by Octo for a new game.
#[test]
fn default_octo_options_bool() {
    let octo_defaults_bool = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":false,"loadStoreQuirks":false,"vfOrderQuirks":false,"clipQuirks":true,"vBlankQuirks":true,"jumpQuirks":false,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":true,"fontStyle":"octo"});
    let deserialized_defaults: Options = octo_defaults_bool.to_string().parse().unwrap();
    assert_json_eq!(octo_defaults_bool, deserialized_defaults);
}

/// Deserializes the empty option set
#[test]
fn empty_options() {
    let empty = json!({});
    let deserialized_empty: Options = empty.to_string().parse().unwrap();
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
        let _: Options = program["options"].to_string().parse().unwrap();
    }
}

/// Downloads the default .octo.rc from the C-Octo repo and parses it
#[test]
fn octo_rc_deserialize_default() {
    let body = blocking::get("https://raw.githubusercontent.com/JohnEarnest/c-octo/main/octo.rc")
        .unwrap()
        .text()
        .unwrap();
    let _ = Options::from_ini(&body);
}

#[test]
fn octo_rc_deserialize() {
    let octo_defaults = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":0,"loadStoreQuirks":0,"vfOrderQuirks":0,"clipQuirks":1,"vBlankQuirks":1,"jumpQuirks":0,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":1,"fontStyle":"octo"});
    let deserialized_defaults: Options = octo_defaults.to_string().parse().unwrap();
    let ini_defaults = "core.tickrate = 20\ncore.max_rom=3215\r\ncore.rotation=0\r\ncore.font=octo\r\ncore.touch_mode=none\r\ncolors.plane0=#996600\r\ncolors.plane1=FFCC00\r\ncolors.plane2=FF6600\r\ncolors.plane3=662200\r\ncolors.background=000000\r\ncolors.sound=FFAA00\r\nquirks.shift=0\r\nquirks.loadstore=0\r\nquirks.clip=1\r\nquirks.vblank=1\r\nquirks.jump0=0\r\nquirks.logic=1\r\nquirks.vforder=0\r\n";
    let deserialized_ini_defaults = Options::from_ini(ini_defaults).unwrap();
    assert_json_eq!(
        json!(deserialized_defaults),
        json!(deserialized_ini_defaults)
    );
}

#[test]
fn octo_rc_serialize() {
    let octo_defaults = json!({"tickrate":20,"fillColor":"#FFCC00","fillColor2":"#FF6600","blendColor":"#662200","backgroundColor":"#996600","buzzColor":"#FFAA00","quietColor":"#000000","shiftQuirks":0,"loadStoreQuirks":0,"vfOrderQuirks":0,"clipQuirks":1,"vBlankQuirks":1,"jumpQuirks":0,"screenRotation":0,"maxSize":3215,"touchInputMode":"none","logicQuirks":1,"fontStyle":"octo"});
    let deserialized_defaults: Options = octo_defaults.to_string().parse().unwrap();
    let ini_defaults = "core.tickrate=20\r\ncore.max_rom=3215\r\ncore.rotation=0\r\ncore.font=octo\r\ncore.touch_mode=none\r\ncolors.plane1=FFCC00\r\ncolors.plane2=FF6600\r\ncolors.plane3=662200\r\ncolors.plane0=996600\r\ncolors.sound=FFAA00\r\ncolors.background=000000\r\nquirks.shift=0\r\nquirks.loadstore=0\r\nquirks.jump0=0\r\nquirks.logic=1\r\nquirks.clip=1\r\nquirks.vblank=1\r\nquirks.vforder=0\r\n";
    let ini_defaults_deserialized = Options::to_ini(deserialized_defaults);
    assert_eq!(ini_defaults, ini_defaults_deserialized);
}
