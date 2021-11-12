#!/usr/bin/php
<?php

if(!file_exists("./src")) die("src not present.  Please run from crate root\n");

$workdir = @$argv[2] ?: '/tmp/gen_diagrams_for_deps';
$diagrams_dir = "$workdir/diagrams";

$cmd = sprintf('rm -rf %1$s && mkdir -p %1$s', escapeshellarg($workdir));
shell($cmd) && die("failed initializing $workdir\n");

$cmd = sprintf("cargo metadata --format-version 1");
exec($cmd, $output, $rc);  if( $rc ) die("cargo metadata failed\n");
$json = implode("\n", $output);

$meta = @json_decode($json, true);
if( !$meta ) die("invalid json\n");

$root = @$meta['resolve']['root'];
if( !$root ) die("root not found\n");
list($rootname, $rootversion) = @explode(' ', $root);

$crates = [];
foreach( @$meta['resolve']['nodes'] as $node ) {
    list($name, $version) = @explode(' ', @$node['id']);
    if($name == $rootname) {
        continue;
    }

    $crates[$name] = $version;
}

$cratepaths = [];
// $home = getenv('HOME');
foreach($crates as $name => $version) {
   unset($lines);
   echo "finding dep: $name-$version\n";
   $cmd = sprintf('find  ~/.cargo/registry/src -name %s-%s', escapeshellarg($name), $version);
   exec($cmd, $lines, $rc);  if( $rc ) die("find failed");
   $path = @$lines[0];
   if(!$path || !file_exists($path)) die("dep path '$path' does not exist for crate $name\n");
   $cratepaths[] = $path;
}

$cratepaths[] = getcwd();  // current crate.

$cmd = sprintf("%s/gen_diagrams.php %s %s", 
                escapeshellcmd(__DIR__),
                escapeshellarg($diagrams_dir),
                implode(' ', $cratepaths));
shell($cmd) && die("");

echo "\nFinished. Diagrams are in $diagrams_dir\n";

exit(0);

function shell($cmd) {
    passthru($cmd, $rc);
    return $rc;
}