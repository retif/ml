#![feature(rustc_private)]
#![feature(box_patterns)]
#![allow(dead_code)]
extern crate rust2uml;

struct Amut {
    b: *mut B,
}

struct Aconst {
    b: *const B,
}

struct B {
}

#[test]
fn test_aggregation() {
    rust2uml::Config::set_global(rust2uml::Config::default());
    
    assert_eq!(
        String::from_utf8(rust2uml::rs2dot("tests/aggregation.rs").unwrap()).unwrap(),
        r#"digraph ml {
    ndAmut[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAmut|- b: *mut B}"][shape="record"];
    ndAconst[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAconst|- b: *const B}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB}"][shape="record"];
    ndB -> ndAmut[label=""][arrowhead="odiamond"];
    ndB -> ndAconst[label=""][arrowhead="odiamond"];
}
"#);
}
