#![allow(dead_code, unused_variables)]
extern crate mml;

use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct A<T> where T: Debug {
    a: T,
}

impl<T> A<T> where T: Debug {
    fn a(a: T) -> Self {
        A {
            a: a,
        }
    }
}

impl<T> B<T> for A<T> where T: Debug {
    fn a(&self) -> Option<T> {
        None
    }
}

trait B<T>: Debug where T: Debug {
    fn a(&self) -> Option<T>;
}

impl<T> B<T> {
    fn a(&self) -> Option<T> {
        None
    }
}

#[test]
fn test_realization() {
    let vec = mml::rs2dot("tests/realization.rs").unwrap();
    let target = r#"digraph ml {
    ndA[label="{&lt;&lt;&lt;Structure&gt;&gt;&gt;\nA|- a: T|- a(a : T) -&gt; Self}"][shape="record"];
    ndB[label="{&lt;&lt;&lt;Trait&gt;&gt;&gt;\nB|a(&amp; Self) -&gt; Option &lt; T &gt;|- a(&amp; self) -&gt; Option &lt; T &gt;}"][shape="record"];
    ndB -> ndA[label=""][style="dashed"][arrowhead="onormal"];
}
"#;

    let path = "target/test/realization";
    if let Err(_) = std::fs::create_dir_all(path) {
        if !Path::new(path).is_dir() {
            panic!("Could not create directory!")
        }
    }

    File::create("target/test/realization/realization.svg").unwrap().write(&mml::content2svg(vec.clone()).unwrap()).unwrap();
    File::create("target/test/realization/realization_target.svg").unwrap().write(&mml::content2svg(target.to_string().into_bytes()).unwrap()).unwrap();

    assert_eq!(String::from_utf8(vec).unwrap(), target);
}
