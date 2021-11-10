# ML -Modeling Language-

[![Crate][crate-badge]][crate] [![travis-badge][]][travis] [![appveyor-badge]][appveyor] [![dependencyci-badge]][dependencyci]

A library (and cli tool) to generating UML language from Rust's project into graphiz/dot file.

## QuickStart

### Obtain mml

```
$ git clone https://github.com/dan-da/ml.git
$ cd ml
```

### Install Dependencies

The `dot` binary from graphviz package must exist in your path.
```
$ apt install graphviz
```
(or do the equivalent for your OS)


Nightly rustc is required.  Run this from **within** ml crate.
```
$ rustup install rustc llvm-tools-preview
```

### Build and run mml
```
$ cargo run --example ml
```

important: ml looks for files beneath ./src, so you should always cd to crate root before running it.

### View generated diagram
```
$ firefox target/doc/ml.svg
```

`inkscape` also works well as an svg viewer.


## Usage

```
$ ./target/debug/examples/ml --help Usage: ml [OPTIONS]

Parses rust source code and generates UML diagram

Arguments:
--include_fields [bool] include fields/variants in diagram
--include_implems [bool] include trait implementation methods in diagram
--include_methods [bool] include methods in diagram
--struct_header_bgcolor [str] header background color for structs
--struct_fields_bgcolor [str] fields background color for structs
--struct_method_bgcolor [str] methods background color for structs
--struct_implem_bgcolor [str] implems background color for structs
--enum_header_bgcolor [str] header background color for enums
--enum_fields_bgcolor [str] fields background color for enums
--enum_method_bgcolor [str] methods background color for enums
--enum_implem_bgcolor [str] implems background color for enums
--trait_header_bgcolor [str] header background color for traits
--trait_method_bgcolor [str] methods background color for traits
--font [str] Font name
```

Output is always under target/doc/mml/

You can add `ml` binary to your path and then you should be able to run
it for any rust crate.

--------------------------- Old, possibly obsolete -----------------

## Usage
This repo is provided as a [Cargo package](http://doc.crates.io/manifest.html) and a [build script](http://doc.crates.io/build-script.html).

1. adjust your `Cargo.toml` to include.
```toml
build = "build.rs"

[build-dependencies.mml]
version = "0.1"
```

2. And your `build.rs` to generate your uml [graph/viz](http://www.graphviz.org/doc/info/lang.html) and Structured Vector Graphics at `target/dot/$CARGO_PKG_NAME.{dot,svg}`.
```rust
extern crate mml;

fn main() {
    let dest: String = concat!("target/doc/", env!("CARGO_PKG_NAME")).to_string();

    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
}
```

3. (Facultative) From your entry point library file, you can add the generated vectorized graph.
```rust
//! ![uml](ml.svg)
```

4. (Facultative) With the [travis-cargo](https://github.com/huonw/travis-cargo)'s instructions, you can prepare your *graphviz*'s dependency like with this example.
```yaml
addons:
  apt:
    packages:
      - graphviz
before_script:
  - if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then brew update           ; fi
  - if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then brew install graphviz ; fi
...
script:
  - |
      travis-cargo build &&
...
```

## Features
Consider this list of fonctionalities like unstandard-uml.
* implem -- add a column to show the functions from a implementation. 
* fn-emilgardis -- the function fields are preceded by *fn* keyword (Asked by [Emilgardis](https://github.com/Emilgardis)).

## Knowledge
This is a reading list of material relevant to *Ml*. It includes prior research that has - at one time or another - influenced the design of *Ml*, as well as publications about *Ml*.
* [Supporting Tool Reuse with Model Transformation](http://www.yusun.io/papers/sede-2009.pdf)
* [Unified Modeling Language Version 2.5](http://www.omg.org/spec/UML/2.5)

## License

`ml` is primarily distributed under the terms of both the [MIT license](https://opensource.org/licenses/MIT) and the [Apache License (Version 2.0)](https://www.apache.org/licenses/LICENSE-2.0), with portions covered by various BSD-like licenses.

See [LICENSE-APACHE](LICENSE-APACHE), and [LICENSE-MIT](LICENSE-MIT) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[crate-badge]: https://img.shields.io/badge/crates.io-v0.1-orange.svg?style=flat-square
[crate]: https://crates.io/crates/mml
[travis-badge]: https://travis-ci.org/adjivas/ml.svg?branch=master&style=flat-square
[travis]: https://travis-ci.org/adjivas/ml
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/7nvg286cq11f5l7l?svg=true
[appveyor]: https://ci.appveyor.com/project/adjivas/ml/branch/master
[dependencyci-badge]: https://dependencyci.com/github/adjivas/ml/badge
[dependencyci]: https://dependencyci.com/github/adjivas/ml
