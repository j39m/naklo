"""
This file implements the abstraction of a single song to be tagged.
"""

import sys
import subprocess
import os.path
import mutagen
import mutagen.flac
import mutagen.easyid3

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


class Song():
    """
    A Song object applies to one file and can hold its tags. You
    initialize Song objects (and its subclasses) with the path to
    the file being tagged.

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

        This method should return nothing on success and raise an IOError
        on failure.
        """
        raise NotImplementedError("Song.do_tag()")

class MetaflacFlacSong(Song):
    """
    A MetaflacFlacSong is a Song that calls metaflac to tag itself.
    """
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
            return

        meta = subprocess.Popen(
            args=("metaflac", "--import-tags-from=-", self.path),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        stdin_str = "\n".join(self.build_metaflac_stdin())
        byt = stdin_str.encode(encoding="UTF-8")
        (_, stderr) = meta.communicate(input=byt)
        if meta.wait():
            efmt = "tag failed on ``{}:'' {}"
            raise IOError(efmt.format(self.path, stderr))

class MutagenFlacSong(Song):
    """
    A MutagenFlacSong is a Song that identifies as FLAC and which uses
    mutagen to tag itself.
    """
    def __init__(self, path):
        super().__init__(path)
        self.mutagen_obj = mutagen.flac.FLAC(path)

    def do_tag(self, dry_run=False):
        raise NotImplementedError("MutagenFlacSong.do_tag()")

class MutagenMp3Song(Song):
    """
    A MutagenMp3Song is a Song that identifies as mp3 and which uses
    mutagen to tag itself.
    """
    def __init__(self, path):
        super().__init__(path)
        self.mutagen_obj = mutagen.easyid3.EasyID3(path)

    def do_tag(self, dry_run=False):
        raise NotImplementedError("MutagenMp3Song.do_tag()")

EXT_MUTAGEN_MAP = {
    ".flac": MutagenFlacSong,
    ".mp3": MutagenMp3Song,
}

def song_to_mutagen_file(fname):
    """Given a filename, return a Mutagen file object of that file."""
    (_, ext) = os.path.splitext(fname)
    try:
        fclass = EXT_MUTAGEN_MAP[ext]
    except KeyError:
        raise IOError("Bad extension: ``{}''".format(ext))
    return fclass(fname)

def songs_to_array(*fnames, use_metaflac=True):
    """
    Given one or more paths to songs, return an array consisting of each
    path initialized as a song object.
    """
    if use_metaflac:
        return [MetaflacFlacSong(fname) for fname in fnames]

    retv = list()
    for fname in fnames:
        retv.append(song_to_mutagen_file(fname))
    return retv
