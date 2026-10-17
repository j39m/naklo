use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub struct TagDetails {
    pub sort_order: Option<&'static str>,
}
type StaticTagMap = HashMap<&'static str, TagDetails>;

fn homogeneous_tags() -> &'static HashSet<&'static str> {
    static HOM: LazyLock<HashSet<&str>> = LazyLock::new(|| {
        HashSet::from([
            "album",
            "albumartist",
            "albumsort",
            "discnumber",
            "discsubtitle",
            "disctotal",
            "genre",
            "tracktotal",
            "musicbrainz_albumid",
        ])
    });
    &*HOM
}

fn heterogeneous_tags() -> &'static StaticTagMap {
    static HETEROGENEOUS: LazyLock<StaticTagMap> = LazyLock::new(|| {
        HashMap::from([
            ("arranger", TagDetails { sort_order: None }),
            (
                "artist",
                TagDetails {
                    sort_order: Some("00"),
                },
            ),
            (
                "composer",
                TagDetails {
                    sort_order: Some("01"),
                },
            ),
            ("conductor", TagDetails { sort_order: None }),
            (
                "date",
                TagDetails {
                    sort_order: Some("zb"),
                },
            ),
            (
                "location",
                TagDetails {
                    sort_order: Some("za"),
                },
            ),
            ("lyricist", TagDetails { sort_order: None }),
            ("performer", TagDetails { sort_order: None }),
            ("title", TagDetails { sort_order: None }),
            ("tracknumber", TagDetails { sort_order: None }),
        ])
    });
    &*HETEROGENEOUS
}

pub fn tag_is_valid(tag: &str) -> bool {
    heterogeneous_tags().contains_key(tag) || homogeneous_tags().contains(tag)
}

pub fn tag_is_homogeneous(tag: &str) -> bool {
    homogeneous_tags().contains(tag)
}

pub fn tag_details(tag: &str) -> &TagDetails {
    &heterogeneous_tags()[tag]
}
