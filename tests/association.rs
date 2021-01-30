#![allow(dead_code)]
extern crate mml;

use std::fs::File;
use std::io::Write;
use std::path::Path;

struct A {}

impl A {
    fn b() -> B {
        B {}
    }
}

struct Ab {}

impl Ab {
    fn b() -> B {
        B {}
    }
}

struct B {}

impl B {
    fn a() -> Ab {
        Ab {}
    }
}

#[test]
fn test_association() {
    let vec = mml::rs2dot("tests/association.rs").unwrap();
    let target = r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- b() -&gt; B}"][shape="record"];
    ndAb[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAb|- b() -&gt; B}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB|- a() -&gt; Ab}"][shape="record"];
    ndAb -> ndB[label=""][arrowhead="none"];
    ndB -> ndA[label=""][arrowhead="vee"];
}
"#;

    let path = "target/test/association";
    if let Err(_) = std::fs::create_dir_all(path) {
        if !Path::new(path).is_dir() {
            panic!("Could not create directory!")
        }
    }

    File::create("target/test/association/association.svg")
        .unwrap()
        .write(&mml::content2svg(vec.clone()).unwrap())
        .unwrap();
    File::create("target/test/association/association_target.svg")
        .unwrap()
        .write(&mml::content2svg(target.to_string().into_bytes()).unwrap())
        .unwrap();

    assert_eq!(String::from_utf8(vec).unwrap(), target);
}
