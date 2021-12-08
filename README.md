# naklo

... is batch music tagger. You use it by specifying tags for a given
album in one or more files and applying them in one shot with a naklo
invocation. Through `mutagen`, naklo supports tagging FLAC, MP3 (to a
limited extent), and Opus files.

> **NOTE**: This project is free software, a personal project by j39m.
> However, Google LLC owns the copyright on commits
> `d7144ae933c48733abf818308e8153786b4d23a6` and newer. This does not
> impact your ability to use and to hack at this free software; I
> provide this notice only for attribution purposes.

## Control files: format

I encourage the gentle reader to browse
[the reference-examples/ directory](reference-examples/) to get a feel
for how control files work; the formal definition is a little stuffy.

A naklo control file contains up to two types of blocks. In general,
the format goes:

```
# A map of spans to tags to values. Useful when the tag-value pairs are
# strongly grouped together in the same span.
#
# Note that the leaf node need not be a single string value; it can be a
# list, enabling you to lay down several values at once for a given tag
# in a given span.
classic-tag-block:
    <span>:
        <tag>: <value>
        <tag>:
            - <value>
            - <value>
    <span>:
        ...

# A map of tags to spans to values. Useful when tag values are not
# strongly grouped together in the same span.
#
# As with the classic-tag-block, note that the leaf node need not be a
# string value.
inverted-tag-block:
    <tag>:
        <span>: <value>
        <span>:
            - <value>
            - <value>
    <tag>:
        ...
```

A *span* is a numerical specification consisting of any of

*   a single numerical token,
*   a set of tracks denoted by `<lower>-<upper>` (inclusive), or
*   the wildcard token, denoted by a literal star `*`.

## Control files: known caveats

1.  Wildcard spans (`*`) must be quoted, lest the YAML fail to parse.
1.  While most leaf values need not be enclosed in quotes, you will need
    to do something about leaf values containing colon characters (`:`).
    If quote enclosure is not feasible, I prefer to use YAML's `>-`
    notation.

## Build Notes

The first time you build, you will need to manually symlink the shared
objects spawned under `build/` into `libnaklo3/`. You'll also need to
repeat this whenever you upgrade Python.

After the symlink step, you should be able to

```
./setup.py build_ext && ./unit_tests.py
```
