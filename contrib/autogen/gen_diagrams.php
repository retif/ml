#!/usr/bin/php
<?php

$diagrams_dir = @$argv[1];
$crates_args = @array_slice($argv, 2);

if(!$diagrams_dir || !@count($crates_args) || $diagrams_dir == '--help') {
    die(<<<END
missing arguments.

Usage:
    gen_diagrams <diagram-dir> <crate-arg> [crate-arg] ..

crate-arg may take either form:
    1. path
    2. path[::]src-url-mask

path is the absolute path to crate root.

src-url-mask is a string template for url to the source code. eg:

    file:///path/to/crate/{file}

    -- or --

    https:///github.com/username/project/blob/master/{file}

END
    );
}

$crates = [];
foreach($crates_args as $ca) {
    @list($crate, $src_url_mask) = @explode("[::]", $ca);
    $crates[] = ['crate' => $crate, 'src_url_mask' => $src_url_mask];
}

$cmd = sprintf('rm -rf %1$s && mkdir -p %1$s', escapeshellarg($diagrams_dir));
shell($cmd) &&die("failed initializing diagrams directory\n");

echo "Initializing ml...\n";
chdir(__DIR__ .'/../../') || die("chdir failed\n");
shell('cargo run --example ml -- help > /dev/null 2> /dev/null');

$bin_paths = [
    sprintf('%s/../../target/release/examples/ml', __DIR__),
    sprintf('%s/../../target/debug/examples/ml', __DIR__),
];

$bin_cmd = '';
foreach($bin_paths as $bp) {
    if(file_exists($bp)) {
        $bin_cmd = realpath($bp);
        break;
    }
}
if(!$bin_cmd) {
    die("ml command not found\n");
}

foreach($crates as $crate) {
    $bin_cmd_iter = $bin_cmd;
    if($src_url_mask) {
        $bin_cmd_iter .= sprintf(" --src_url_mask %s", $crate['src_url_mask']);
    }
    gen_diagram($bin_cmd_iter, $diagrams_dir, $crate['crate']);
}

function gen_diagram($bin_cmd, $diagrams_dir, $crate_root) {    
    $crate = basename($crate_root);
    echo "Processing $crate ($crate_root)\n";
    @chdir($crate_root) || die("failed to chdir to $crate_root");
    $name = escapeshellarg(basename($crate_root));
    $pathbase = sprintf("%s/$name", escapeshellarg($diagrams_dir));
    shell("$bin_cmd && cp target/doc/mml/ml.svg $pathbase.svg") && die("failed generating $pathbase.svg\n");
    shell("$bin_cmd --include_methods false --include_implems false && cp target/doc/mml/ml.svg $pathbase-compact.svg") && die("failed generating $pathbase-compact.svg\n");
    shell("$bin_cmd --include_fields false --include_methods false --include_implems false && cp target/doc/mml/ml.svg $pathbase-bare.svg") && die("failed generating pathbase-bare.svg\n");
}

exit(0);

function shell($cmd) {
    passthru($cmd, $rc);
    return $rc;
}
?>