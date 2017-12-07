# naklo

... is a Python 3 script that calls metaflac to write FLAC metadata
in batches. Little has changed since the days of Perl naklo,
except mainly the name, "pyaklon," which distinguishes it from the
today-deprecated (???) name of "naklo."
("pynaklo" was used in an earlier burner script, so I couldn't recycle
that.)

One significant change is that it is no longer possible to use the
directive "all" before tag/value indicators to apply to all files. By
default, omitting an explicit numeric specification will apply a
tag-and-value pair globally. This was unnecessary functionality that
I didn't think through; therefore it is removed to marginally simplify
parsing in pyaklon.

I must say I'm much happier with this Python 3 port. It's easier to read,
easier to maintain, and does things with a tad more formal rigor than
the Perl version.

The old README picks up below.

... was a Perl script that calls metaflac to write FLAC metadata in batches.
It operates by following a user-provided control file instructing it on how
to tag things. A reference tag file that contains many examples of how to
tag should be included with this script. Two references files in particular
give examples of how to format metadata directives ("control" and "titles,"
which are respectively to be used with the "-c" and "-t" options).

Be warned: naklo is neither patient nor kind (nor is it boastful, envious,
or proud). It will ravenously go through all FLAC files (if not specified by
user, assumed to be *all* FLAC files in the working directory --- it will
ask for confirmation about that case) and tag everything. There are no dry
runs, no previews. Use caution, and remember that
"metaflac --remove-all-tags" is your friend.

naklo was conceived in C, but that was a bad idea. It's now taking the Perl
road.

~~naklo is a program written in C that leverages the FLAC C API (and by
extension libFLAC) to write FLAC metadata in batches. naklo is not designed
to EDIT metadata: only to write it, e.g. immediately after encoding a fresh
rip.~~

~~naklo's chief modus operandi involves reading a file (which you, the user,
create) that specifies the tags to be written to the files. An example file
with the requisite formatting is probably provided; if you cannot find it,
you can find one from documentation elsewhere. The tags file should be named
"tags," "Tags," or "TAGS." Adding the extension ".txt" to any of these is
also fine. But exotic things like "tAgS" or "T@G5" simply won't work. The
main specialty~~

~~naklo depends upon libFLAC (the FLAC development libraries --- on Fedora
20 they are found in the package "flac-devel") and on a working GCC.~~
