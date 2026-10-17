use std::collections::HashMap;
use std::sync::LazyLock;

struct TagDetails {}
type StaticTagMap = HashMap<&'static str, TagDetails>;

fn homogeneous_tags() -> &'static StaticTagMap {
    static HOM: LazyLock<StaticTagMap> = LazyLock::new(|| {
        HashMap::from([
            ("album", TagDetails {}),
            ("albumartist", TagDetails {}),
            ("albumsort", TagDetails {}),
            ("discnumber", TagDetails {}),
            ("discsubtitle", TagDetails {}),
            ("disctotal", TagDetails {}),
            ("genre", TagDetails {}),
            ("tracktotal", TagDetails {}),
            ("musicbrainz_albumid", TagDetails {}),
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
    heterogeneous_tags().contains_key(tag) || homogeneous_tags().contains_key(tag)
}

pub fn tag_is_homogeneous(tag: &str) -> bool {
    homogeneous_tags().contains_key(tag)
}
