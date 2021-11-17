#!/usr/bin/php
<?php

$workdir = @$argv[2] ?: '/tmp/gen_diagrams_remote';
$diagrams_dir = "$workdir/diagrams";

$cmd = sprintf('rm -rf %1$s && mkdir -p %1$s', escapeshellarg($workdir));
shell($cmd) && die("failed initializing $workdir\n");

$cratefile = @$argv[1] ?: sprintf('%s/%s', __DIR__, 'remote_crates.json');
($text = @file_get_contents($cratefile)) || die("failed reading file $cratefile\n");
($json = @json_decode($text, true)) || die("failed parsing json in $cratefile\n");

chdir($workdir) || die("failed chdir");

$cratepaths = [];
foreach($json as $crate => $meta) {
    $url = @$meta['git'];
    $cmd = sprintf('git clone --depth=1 %s %s', escapeshellarg($url), escapeshellarg($crate));
    shell($cmd) && die("git clone failed\n");
    $cratedir = $crate . '/' . @$meta['path'];
    $src_url_mask = "";
    if(strstr($url, "github.com")) {
        $src_url_mask = sprintf("%s/blob/master/{file}", $url);
    }
    $cratepaths[] = sprintf("%s[::]%s",
        escapeshellarg(sprintf("%s/%s", $workdir, $cratedir)),
        escapeshellarg($src_url_mask));

    echo "\n";
}

$cmd = sprintf("%s/gen_diagrams.php %s %s", 
                escapeshellcmd(__DIR__),
                escapeshellarg($diagrams_dir),
                implode(' ', $cratepaths));

shell($cmd) && die("");

gen_html_index($diagrams_dir, $json);

echo "\nFinished. Diagrams are in $diagrams_dir\n";

exit(0);

function shell($cmd) {
    passthru($cmd, $rc);
    return $rc;
}

function gen_html_index($diagrams_dir, $json) {
  
    $buf = <<< END
<html><head><style>
body { font-family: arial, helvetica, sans-serif; }
.diagrams{ 
    border-collapse: separate;
    border-spacing: 10px;
    background-color: #eeeeee;
}
.diagrams td {
    border: 2px solid #aaa;
    padding: 5px;
}
.even {
    background-color: #bbbbbb;
}
.odd{ background-color: #dddddd;
}
thead td {
    text-align: center;
}
</style></head>
<body>
<table class='diagrams'>
<thead>
<tr><td>Crate</td><td colspan="3">Diagrams</td><td colspan="3">References</td></tr>
</thead>
END;

    $cnt = 0;
    foreach($json as $crate => $meta) {
        $url = @$meta['git'];
        $realcrate = @$meta['crate'] ?: $crate;
        $crate_io_url = sprintf("https://crates.io/crates/%s", $realcrate);
        $docs_rs_url = sprintf("https://docs.rs/%s/", $realcrate);
        $bare = sprintf('%s-bare.svg', $crate);
        $compact = sprintf('%s-compact.svg', $crate);
        $full = sprintf('%s.svg', $crate);
        $class = $cnt % 2 == 0 ? 'even' : 'odd';

        $buf .= sprintf('<tr class="%s"><td>%s</td><td><a href="%s">bare</a></td><td><a href="%s">compact</a></td><td><a href="%s">full</a></td><td><a href="%s">repo</a></td><td><a href="%s">crates.io</a></td><td><a href="%s">docs.rs</a></td></tr>', $class, $crate, $bare, $compact, $full, $url, $crate_io_url, $docs_rs_url) . "\n";
        $cnt ++;
    }
    $buf .= "</table></body></html>";
    $path = sprintf('%s/%s', $diagrams_dir, 'index.html');
    file_put_contents($path, $buf) || die("could not write index.html");
}
?>