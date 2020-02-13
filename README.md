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

## Control file format

> **TODO(j39m):** dogfood this and copy something into
> reference-examples/.

A naklo control file contains up to three types of blocks. In general,
the format goes:

```
# A map of text substitutions that naklo will blindly make to tag values
# wherever they are encountered. SCREAMING_SNAKE_CASE is not mandatory
# but is reasonably useful for detecting typos.
naklo-substitutions:
    "ONE_NAME": "some value"
    "ANOTHER_NAME": "another value"

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

You can find further reading

1.  in [the controls module](libnaklo3/controls.pyx) and
1.  in [the reference-examples/ directory](reference-examples/).
