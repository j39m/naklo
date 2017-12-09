# naklo

... is a script that tags FLAC files. I am motivated to implement it because
I didn't quickly find a batch tagger that frees me from graphical UIs and
mouse usage.

## Intended data model

naklo lets you write your tags down in one or more text files s.t. they can
always live independently of the FLACs themselves. It's a batch computation
that always enables you to nuke your metadata and invoke naklo anew to retag
all your files. Typically, you feed naklo two types of files.

### Control file

A control file is a file containing YAML blocks that can specify many tags.

### Title file

A title file is a plaintext file containing titles, one per line.
