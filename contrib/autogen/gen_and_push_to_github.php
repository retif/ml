#!/usr/bin/php
<?php

$repo_url = @$argv[1];
$crates_file = @$argv[2];

if(!$repo_url) {
    die("missing repo url");
}

$workdir = '/tmp/gen_and_push_to_github';

$cmd = sprintf('rm -rf %1$s && mkdir -p %1$s', escapeshellarg($workdir));
shell($cmd) && die("failed initializing $workdir\n");
chdir($workdir) || die("failed chdir");

$gitdir = sprintf('%s/diagrams', $workdir);
$cmd = sprintf('git clone --depth=1 %s %s', escapeshellarg($repo_url), escapeshellarg($gitdir));
shell($cmd) && die("git clone failed\n");
chdir($gitdir) || die("failed chdir");

$workdir_gen = sprintf('%s/gen', $workdir);

$cmd = sprintf("%s/gen_diagrams_remote.php %s %s",
                __DIR__,
                escapeshellarg($crates_file),
                escapeshellarg($workdir_gen));
shell($cmd) && die("");

$workdir_gen_diagrams = sprintf('%s/diagrams', $workdir_gen);

$cmd = sprintf('cp %s/* %s', escapeshellarg($workdir_gen_diagrams), escapeshellarg($gitdir));
shell($cmd) && die("failed copying diagrams to $gitdir\n");

$cmd = 'git add *.svg index.html && git commit -m "auto-generated diagrams" -a';
shell($cmd) && die("failed git add/commit\n");

$cmd = 'git push';
shell($cmd) && die("failed git push\n");


function shell($cmd) {
    passthru($cmd, $rc);
    return $rc;
}

?>