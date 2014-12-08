# naklo

... is a Perl script that calls metaflac to write FLAC metadata in batches. It operates by following a user-provided control file instructing it on how to tag things. A reference tag file that contains many examples of how to tag should be included with this script. 

Be warned: naklo is neither patient nor kind (nor is it boastful, envious, or proud). It will ravenously go through all FLAC files (if not specified by user, assumed to be *all* FLAC files in the working directory --- it will ask for confirmation about that case) and tag everything. There are no dry runs, no previews. Use caution, and remember that "metaflac --remove-all-tags" is your friend. 

naklo was conceived in C, but that was a bad idea. It's now taking the Perl road. 

~~naklo is a program written in C that leverages the FLAC C API (and by extension libFLAC) to write FLAC metadata in batches. naklo is not designed to EDIT metadata: only to write it, e.g. immediately after encoding a fresh rip.~~

~~naklo's chief modus operandi involves reading a file (which you, the user, create) that specifies the tags to be written to the files. An example file with the requisite formatting is probably provided; if you cannot find it, you can find one from documentation elsewhere. The tags file should be named "tags," "Tags," or "TAGS." Adding the extension ".txt" to any of these is also fine. But exotic things like "tAgS" or "T@G5" simply won't work. The main specialty~~

~~naklo depends upon libFLAC (the FLAC development libraries --- on Fedora 20 they are found in the package "flac-devel") and on a working GCC.~~