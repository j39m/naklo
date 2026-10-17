use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

struct TagDetails {}
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
            ("arranger", TagDetails {}),
            ("artist", TagDetails {}),
            ("composer", TagDetails {}),
            ("conductor", TagDetails {}),
            ("date", TagDetails {}),
            ("location", TagDetails {}),
            ("lyricist", TagDetails {}),
            ("performer", TagDetails {}),
            ("title", TagDetails {}),
            ("tracknumber", TagDetails {}),
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
