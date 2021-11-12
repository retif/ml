# autogen tools

This directory contains some PHP cli scripts for automating generation of diagrams.

disclaimer: these were a quick/dirty hack and are nothing fancy.

|Script|Function|
|------|--------|
|gen_diagrams.php|processes a list of local crate paths and generates bare,compact,full diagrams for each|
|gen_diagrams_remote.php|generates diagrams for remote crates and makes a nice html index page|
|gen_and_push_to_github.php|publishes diagrams to github.|
|gen_diagrams_for_deps.php|generates diagrams for current crate and all dependencies|

### Provided json files. (input to gen_diagrams_remote.php)

|File | Description|
|---- | -----------|
|remote_crates.json|default file, used if no argument supplied. These are MaidSafe related crates|

### JSON format

Basic usage looks like:

```
{
    "bls_dkg": {"git": "https://github.com/maidsafe/bls_dkg"}, 
    "blsttc": {"git": "https://github.com/maidsafe/blsttc"}
}
```

Above we simply define a crate name and a git url where we can clone it from.

Sometimes however, a github repo may contain a workspace with multiple crates and/or
multiple 'src' roots.  In such cases, use the following syntax:


```
"sn_api": {"git": "https://github.com/maidsafe/safe_network", "path": "sn_api", "crate": "safe_network"},
"sn_cli": {"git": "https://github.com/maidsafe/safe_network", "path": "sn_cli", "crate": "safe_network"},
```

Above we have specified the same git url, but we provide a unique path within the repo.  The
path must contain a `src` directory.  

Also, we provide a `crate` name, in this case `safe_network`.  This is used as the "real"
crate name, as published to crates.io, docs.rs, etc.