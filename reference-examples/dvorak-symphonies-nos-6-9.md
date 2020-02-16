# Unified naklo control file

Given that all songs cohabit the same directory as this file and are
named `dd-tt.flac`, I opened this file in `vim` and issued

```sh
naklo -c <(sed -n 19,30p %) -c <(sed -n 36,47p %) 01-??.flac
naklo -c <(sed -n 19,30p %) -c <(sed -n 53,72p %) 02-??.flac
naklo -c <(sed -n 19,30p %) -c <(sed -n 78,90p %) 03-??.flac
```

I see three separate places in the booklet where a tempo marking is
given "Allego." I'm quite sure this is a typo and have corrected it
for myself.

## Common tags

```yaml
classic-tag-block:
    "*":
        album: Dvořák - Symphonies Nos 6-9
        albumartist: Sir Colin Davis
        artist:
            - Sir Colin Davis
            - London Symphony Orchestra
        conductor: Sir Colin Davis
        composer: Antonin Dvořák
        location: The Barbican, London
        genre: Classical
        disctotal: 3
```

## Disk 1

```yaml
classic-tag-block:
    "*":
        discnumber: 1
        date:
            - "2004-09-29"
            - "2004-09-30"
inverted-tag-block:
    title:
        1: "Symphony No 6 in D major, Op 60 - Allegro non tanto"
        2: "Symphony No 6 in D major, Op 60 - Adagio"
        3: "Symphony No 6 in D major, Op 60 - Scherzo (Furiant) - Presto"
        4: "Symphony No 6 in D major, Op 60 - Finale - Allegro con spirito"
```

## Disk 2

```yaml
classic-tag-block:
    "*":
        discnumber: 2
inverted-tag-block:
    date:
        "1-4": "2001-03-21"
        "5-8":
            - "1999-10-03"
            - "1999-10-04"
    title:
        1: "Symphony No 7 in D minor, Op 70 - Allegro maestoso"
        2: "Symphony No 7 in D minor, Op 70 - Poco adagio"
        3: "Symphony No 7 in D minor, Op 70 - Scherzo: Vivace - Poco meno mosso"
        # typo: "Allego"
        4: "Symphony No 7 in D minor, Op 70 - Finale: Allegro"
        5: "Symphony No 8 in G major, Op 88 - Allegro con brio"
        6: "Symphony No 8 in G major, Op 88 - Adagio"
        7: "Symphony No 8 in G major, Op 88 - Allegretto grazioso - Molto vivace"
        # typo: "Allego"
        8: "Symphony No 8 in G major, Op 88 - Allegro ma non troppo"
```

## Disk 3

```yaml
classic-tag-block:
    "*":
        discnumber: 3
        date:
            - "1999-09-29"
            - "1999-09-30"
inverted-tag-block:
    title:
        1: "Symphony No 9 in E minor, 'From the New World', Op 95 - Adagio - Allegro molto"
        2: "Symphony No 9 in E minor, 'From the New World', Op 95 - Largo"
        3: "Symphony No 9 in E minor, 'From the New World', Op 95 - Scherzo: Molto vivace - Poco sostenuto"
        # typo: "Allego"
        4: "Symphony No 9 in E minor, 'From the New World', Op 95 - Allegro con fuoco"
```
