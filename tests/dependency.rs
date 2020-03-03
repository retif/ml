#![allow(dead_code, unused_variables)]
extern crate mml;

use std::fs::File;
use std::path::Path;
use std::io::Write;

struct A {}

impl A {
    fn b(b: &B) {}
}

struct B {}

#[test]
fn test_dependency() {
    let vec = mml::rs2dot("tests/dependency.rs").unwrap();
    let target = r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- b(b : &amp; B)}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB}"][shape="record"];
    ndB -> ndA[label=""][style="dashed"][arrowhead="vee"];
}
"#;

    let path = "target/test/dependency";
    if let Err(_) = std::fs::create_dir_all(path) {
        if !Path::new(path).is_dir() {
            panic!("Could not create directory!")
        }
    }

    File::create("target/test/dependency/dependency.svg").unwrap().write(&mml::content2svg(vec.clone()).unwrap()).unwrap();
    File::create("target/test/dependency/dependency_target.svg").unwrap().write(&mml::content2svg(target.to_string().into_bytes()).unwrap()).unwrap();

    assert_eq!(String::from_utf8(vec).unwrap(), target);
}
