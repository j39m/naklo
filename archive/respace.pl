#!/usr/bin/perl

# double all leading whitespaces in the file.

use strict;
use warnings;

open(my $fh, "<", "./naklo")
    or die "Couldn't open naklo for reading!";

for my $line (<$fh>) {
    $line =~ m/^(\s*)[^\s]/;
    print "$1$line";
}
