#![allow(dead_code)]

extern crate mml;

use std::fs::File;
use std::io::Write;
use std::path::Path;

struct A {
    b: B,
}

struct B {}

#[test]
fn test_composition() {
    let vec = mml::rs2dot("tests/composition.rs").unwrap();
    let target = r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- b: B}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB}"][shape="record"];
    ndB -> ndA[label=""][arrowhead="diamond"];
}
"#;

    let path = "target/test/composition/";
    if let Err(_) = std::fs::create_dir_all(path) {
        if !Path::new(path).is_dir() {
            panic!("Could not create directory!")
        }
    }

    File::create("target/test/composition/composition.svg").unwrap().write(&mml::content2svg(vec.clone()).unwrap()).unwrap();
    File::create("target/test/composition/composition_target.svg").unwrap().write(&mml::content2svg(target.to_string().into_bytes()).unwrap()).unwrap();

    assert_eq!(String::from_utf8(vec).unwrap(), target);
}
