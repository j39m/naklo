# naklo

... is batch music tagger. You use it by specifying tags for a given
album in one or more files and applying them in one shot with a naklo
invocation. Through `mutagen`, naklo supports tagging:

*   FLAC
*   MP3 (to a limited extent)
*   Opus
*   WavPack

> **NOTE**: This project is free software, a personal project by j39m.
> However, Google LLC owns the copyright on commits
> `d7144ae933c48733abf818308e8153786b4d23a6` and newer. This does not
> impact your ability to use and to hack at this free software; I
> provide this notice only for attribution purposes.

## Control Files: Example

Before consulting the formal definition of the `naklo` control blocks,
consider this example that tags the "Ghibli Jazz" album:

```yaml
span-tag-block:
  "*":
    album: Ghibli Jazz
    albumartist: All That Jazz
    arranger: Tomoo Nogami
    genre:
      - Anime
      - Jazz

tag-span-block:
  composer:
    01 02 04-06 08 10 12: Joe Hisaishi
    03: Yumi Arai
    07:
      - Bill Danoff
      - Taffy Nivert
      - John Denver
    09: Haruomi Hosono
    11: Tokiko Kato
  artist:
    01 03 05 07 09 11: Yuriko Kuwahara
    "*": All That Jazz

title-merge-block:
  01: 君をのせて
  02: 海の見える街
  03: やさしさに包まれたなら
  04: 風の通り道
  05: となりのトトロ
  06: 人生のメリーゴーランド
  07: カントリーロード
  08: もののけ姫
  09: 風の谷のナウシカ
  10: ナウシカ・レクイエム
  11: 時には昔の話を
  12: 崖の上のポニョ
```

## Control Files: Formal Definition

`naklo` recognizes three types of control blocks:

1.  A `span-tag-block` has a hierarchy of spans, tags, and values.
    The most obvious use case is album-wide common tags (using the
    wildcard span, `*`).
1.  A `tag-span-block` has a hierarchy of tags, spans, and values.
1.  A `title-merge-block` builds up titles from span-defined fragments
    and joins them with spaces. This is not terribly interesting when
    all works in the block have disjoint tiles, but it greatly reduces
    repetition in multi-part works, e.g. operas. Note that you're free
    to indicate a `title` tag (and map values to spans) in a
    `tag-span-block`. In the "totally disjoint titles" case, these
    constructs are pretty much congruent.

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
