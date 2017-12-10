"""
This file implements the abstraction of a single song to be tagged.
"""

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
        del(self.tags[key])

    def do_tag(self):
        raise NotImplementedError("XXX j39m call metaflac!")
