#![feature(rustc_private)]
#![feature(box_patterns)]
#![allow(dead_code)]
extern crate rust2uml;

struct A {
}

impl A {
    fn b() -> B {
        B {
        }
    }
}

struct Ab {
}

impl Ab {
    fn b() -> B {
        B {
        }
    }
}

struct B {
}

impl B {
    fn a() -> Ab {
        Ab {
        }
    }
}

#[test]
fn test_association() {
    assert_eq!(
        String::from_utf8(rust2uml::rs2dot("tests/association.rs").unwrap()).unwrap(),
        r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- b() -&gt; B}"][shape="record"];
    ndAb[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nAb|- b() -&gt; B}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nB|- a() -&gt; Ab}"][shape="record"];
    ndAb -> ndB[label=""][arrowhead="none"];
    ndB -> ndA[label=""][arrowhead="vee"];
}
"#);
}
