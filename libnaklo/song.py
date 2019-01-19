"""
This file implements the abstraction of a single song to be tagged.
"""

import sys
import subprocess

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
    "tracknumber",
    "tracktotal",
    "discnumber",
    "disctotal",
    "discsubtitle",
))


def songs_to_array(*fnames, use_metaflac=True):
    """
    Given one or more paths to songs, return an array consisting of each
    path initialized as a song object.
    """
    if use_metaflac:
        return [MetaflacFlacSong(fname) for fname in fnames]
    raise NotImplementedError("TODO(j39m): use mutagen.")


class Song():
    """
    A Song object applies to one file and can hold its tags.

    Do not use this object by itself. Subclass it appropriately.
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

    def do_tag(self, dry_run=False):
        """
        Act now and tag the song to which I correspond.
        """
        raise NotImplementedError("Song.do_tag()")

class MetaflacFlacSong(Song):
    """
    A MetaflacFlacSong is a Song that calls metaflac to tag itself.
    """
    def __init__(self, path):
        super().__init__(path)
        self.last_stdout = None
        self.last_stderr = None

    def build_metaflac_stdin(self):
        """
        Return a list of the form KEY=VAL for each tag-value pair in me.
        """
        retv = list()
        for (tag, value_list) in sorted(self.tags.items()):
            retv.extend(["{}={}".format(tag, val) for val in value_list])
        retv.append("")
        return retv

    def do_tag(self, dry_run=False):
        """
        Perform the metaflac call that will tag this song.
        If dry_run is True, print the same (and do not actually do it).
        """
        if dry_run:
            sys.stdout.write(self.path)
            metaflac_in = self.build_metaflac_stdin()
            metaflac_in.insert(0, "")
            print("\n  ".join(metaflac_in))
            return 0

        meta = subprocess.Popen(
            args=("metaflac", "--import-tags-from=-", self.path),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        stdin_str = "\n".join(self.build_metaflac_stdin())
        byt = stdin_str.encode(encoding="UTF-8")
        (stdout, stderr) = meta.communicate(input=byt)
        self.last_stdout = stdout
        self.last_stderr = stderr
        return meta.wait()
