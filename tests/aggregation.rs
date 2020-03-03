#![allow(dead_code)]
extern crate mml;

use std::fs::File;
use std::io::Write;
use std::path::Path;

struct Amut {
    b: *mut B,
}

struct Aconst {
    b: *const B,
}

struct B {}

#[test]
fn test_aggregation() {
    let vec = mml::rs2dot("tests/aggregation.rs").unwrap();
    let target = r#"digraph ml {
    ndAmut[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAmut|- b: * mut B}"][shape="record"];
    ndAconst[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAconst|- b: * const B}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB}"][shape="record"];
    ndB -> ndAmut[label=""][arrowhead="odiamond"];
    ndB -> ndAconst[label=""][arrowhead="odiamond"];
}
"#;

    let path = "target/test/aggregation";
    if let Err(_) = std::fs::create_dir_all(path) {
        if !Path::new(path).is_dir() {
            panic!("Could not create directory!")
        }
    }

    File::create("target/test/aggregation/aggregation.svg").unwrap().write(&mml::content2svg(vec.clone()).unwrap()).unwrap();
    File::create("target/test/aggregation/aggregation_target.svg").unwrap().write(&mml::content2svg(target.to_string().into_bytes()).unwrap()).unwrap();

    assert_eq!(String::from_utf8(vec).unwrap(), target);
}
