"""
This file implements the abstraction of a single song to be tagged.
"""

__all__ = [
    "songs_to_array",
    "Song",
]

VALID_TAGS = set((
    "artist",
    "album",
    "albumartist",
    "albumsort",
    "performer",
    "conductor",
    "composer",
    "arranger",
    "lyricist",
    "title",
    "location",
    "date",
    "genre",
    "discnumber",
    "disctotal",
    "discsubtitle",
))


def songs_to_array(*fnames):
    """
    Given one or more paths to songs, return an array consisting of each
    path initialized as a song object.
    """
    return [Song(fname) for fname in fnames]


class Song(object):
    """
    Abstraction of a song object containing tags.
    """
    def __init__(self, path):
        self.path = path
        self.tags = dict()

    def __setitem__(self, key, value):
        if key not in VALID_TAGS:
            raise ValueError("invalid key ``{}.''".format(key))
        try:
            self.tags[key].append(value)
        except KeyError:
            self.tags[key] = [value,]

    def __delitem__(self, key):
        del self.tags[key]

    def build_metaflac_stdin(self):
        """
        Return a list of the form KEY=VAL for each tag-value pair in me.
        """
        retv = list()
        for (tag, value_list) in self.tags.items():
            retv.extend(["{}={}".format(tag, val) for val in value_list])
        return retv

    def do_tag(self, dry_run=True):
        """
        Perform the metaflac call that will tag this song.
        If dry_run is True, print the same (and do not actually do it).
        """
        if dry_run:
            print(self.path)
            metaflac_in = self.build_metaflac_stdin()
            metaflac_in.insert(0, "")
            print("\n  ".join(metaflac_in))
        else:
            raise NotImplementedError("XXX j39m call metaflac!")
