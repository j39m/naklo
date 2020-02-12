# naklo

... is batch music tagger. You use it by specifying tags for a given
album in one or more files and applying them in one shot with a naklo
invocation. Through `mutagen`, naklo supports tagging FLAC, MP3 (to a
limited extent), and Opus files.

> **NOTE**: Google LLC owns the copyright on commits
> `d7144ae933c48733abf818308e8153786b4d23a6` and newer.

## Intended data model

Typically, you feed naklo two types of files.

### Control file

A control file is a file containing YAML blocks that can specify many tags.

### Title file

A title file is a plaintext file containing titles, one per line.
