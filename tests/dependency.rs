#![allow(dead_code, unused_variables)]
extern crate rust2uml;

struct A {
}

impl A {
    fn b(b: &B) {
    }
}

struct B {
}

#[test]
fn test_dependency() {
    assert_eq!(
        String::from_utf8(rust2uml::rs2dot("tests/dependency.rs").unwrap()).unwrap(),
        r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- b(b: &amp;B)}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB}"][shape="record"];
    ndB -> ndA[label=""][style="dashed"][arrowhead="vee"];
}
"#);
}
