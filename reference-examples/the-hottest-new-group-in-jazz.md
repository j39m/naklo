## Unified naklo control file

Most of this metadata is taken
[from AllMusic](https://www.allmusic.com/album/the-hottest-new-group-in-jazz-compilation-mw0000079757).
It's not clear to me whether the slash-credited personnel per song are
in fact composers or arrangers.

I gleaned the fine-grained personnel arrangements from the tags provided
by Presto Music.

To write tags, I opened this file in `vim` (with folds to max for
readability). I then issued

```sh
naklo -c <(sed -n 24,33p %) -c <(sed -n 39,101p %) 01-??.flac
naklo -c <(sed -n 24,33p %) -c <(sed -n 107,162p %) 02-??.flac
```

assuming files were present in cwd named like `dd-tt.flac`.

## Common tags

```yaml
classic-tag-block:
    "*":
        album: The Hottest New Group in Jazz
        artist: Lambert, Hendricks, & Ross
        genre: Jazz
        performer:
            - Dave Lambert
            - Jon Hendricks
            - Annie Ross
        disctotal: 2
```

## Disk 1

```yaml
classic-tag-block:
    "*":
        discnumber: 1

inverted-tag-block:
    title:
        1: Charleston Alley
        2: Moanin'
        3: Twisted
        4: Bijou
        5: Cloudburst
        6: Centerpiece
        7: Gimme That Wine
        8: Sermonette
        9: Summertime
        10: Everybody's Boppin'
        11: Cotton Tail
        12: All Too Soon
        13: Happy Anatomy
        14: Rocks in my Bed
        15: Main Stem
        16: I Don't Know What Kind of Blues I Got
        17: Things Ain't What They Used to Be
        18: Midnight Indigo
        19: What Am I Here For?
        20: In a Mellow Tone
        21: Caravan
    performer:
        2-10:
            - Harry "Sweets" Edison
            - Gildo Mahones
            - Ike Isaacs
            - Walter Bolden
        11-21:
            - Gildo Mahones
            - Ike Isaacs
            - Jimmy Wormsworth
    composer:
        1: Horace Henderson
        2: Bobby Timmons
        3:
            - Wardell Gray
            - Annie Ross
        4: Ralph Burns
        5: Jimmy Harris
        6: Harry "Sweets" Edison
        8: Cannonball Adderley
        9:
            - George Gershwin
            - Ira Gershwin
            - DuBose Heyward
        "11-16 18-21": Duke Ellington
        12: Carl Sigman
        17:
            - Mercer Ellington
            - Ted Persons
        19: Frankie Laine
        20: Milt Gabler
        21:
            - Irving Mills
            - Juan Tizol
        "1 4 6 7 8 10": Jon Hendricks
        "1 5": Leroy Kirkland
```

## Disk 2

```yaml
classic-tag-block:
    "*":
        discnumber: 2

inverted-tag-block:
    title:
        1: Come on Home
        2: The New ABC
        3: Farmer's Market
        4: Cookin' at the Continental
        5: With Malice Toward None
        6: Hi-Fly
        7: Home Cookin'
        8: Halloween Spooks
        9: Popity Pop
        10: Blue
        11: Mr. P.C.
        12: Walkin'
        13: This Here
        14: Swingin' Till the Girls Come Home
        15: Twist City
        16: Just a Little Bit of Twist
        17: A Night in Tunisia
        18: A Night in Tunisia
    performer:
        1-11:
            - Gildo Mahones
            - Ike Isaacs
            - Jimmy Wormsworth
        12-18:
            - Pony Poindexter
            - Gildo Mahones
            - Ron Carter
            - Stu Martin
        15-18:
            - W. Yancy
    composer:
        "1 4 7": Horace Silver
        "2 8": Dave Lambert
        3:
            - Art Farmer
            - Annie Ross
        "4 5 13": Jon Hendricks
        5: Tom McIntosh
        6: Randy Weston
        9: Slim Gaillard
        10: Gildo Mahones
        11: John Coltrane
        12: Richard Carpenter
        13: Bobby Timmons
        14: Oscar Pettiford
        15: Matthew Gee
        16: Don Covay
        "17 18":
            - Dizzy Gillespie
            - Frank Paparelli
```

## Detailed personnel info

*   Gildo Mahones,          piano
*   Harry "Sweets" Edison,  trumpet
*   Ike Isaacs,             bass
*   Walter Bolden,          drums
*   Jimmy Wormsworth,       drums
*   Pony Poindexter,        alto saxophone
*   Ron Carter,             bass
*   Stu Martin,             drums
*   W. Yancy,               bass
