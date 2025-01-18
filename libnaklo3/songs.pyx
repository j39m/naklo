from cpython cimport bool

import logging
logger = logging.getLogger(__name__)
import re

import mutagen
import mutagen.easyid3
import mutagen.flac
import mutagen.oggopus
import mutagen.oggvorbis
import mutagen.wavpack

cdef class BaseMutagenSong:

    cdef str path
    cdef dict tags
    cdef object mutagen_object
    __tag_sort_dict = {
        "artist": "00",
        "performer": "01",
        "conductor": "02",
        "composer": "03",
    }
    warnworthy_matcher = re.compile('(["\']| - )')

    def __init__(self, str path, object mutagen_class):
        self.path = path
        self.tags = dict()
        self.mutagen_object = mutagen_class(path)

    @classmethod
    def __tag_sorting_key(self, tag_name):
        return self.__tag_sort_dict.get(tag_name, tag_name)

    def __sorted_keys(self):
        return sorted(self.tags.keys(), key=self.__tag_sorting_key)

    def __str__(self):
        result = []
        result.append(f"{self.path}: “{self.__get_title()}”")
        for tag_name in self.__sorted_keys():
            tag_values = self.tags[tag_name]
            if tag_name == "title":
                continue
            if len(tag_values) == 1:
                result.append(f"    {tag_name}: {tag_values[0]}")
            else:
                result.append(f"    {tag_name}:")
                result.extend([f"        {val}" for val in tag_values])
        return "\n".join(result)

    def __get_title(self):
        try:
            return self.tags["title"][0]
        except KeyError:
            return "NO TITLE"

    def get_tag_keys(self):
        return self.tags.keys()

    def get_path(self):
        return self.path

    def add_tag(self, str tag, str value):
        matched = self.warnworthy_matcher.search(value)
        if matched:
            logger.warning("concerning character found")
            logger.warning(f'    {value}')
            logger.warning(f'{" " * (matched.start() + 4)}^ here')
        try:
            self.tags[tag].append(value)
        except KeyError:
            self.tags[tag] = [value,]

    def clear(self):
        self.mutagen_object.clear()

    def enact(self):
        for (tag_name, tag_values) in self.tags.items():
            self.mutagen_object[tag_name] = tag_values
        self.mutagen_object.save()

    def items(self):
        return self.tags.items()


class FlacSong(BaseMutagenSong):

    def __init__(self, path):
        super().__init__(path, mutagen.flac.FLAC)


class Mp3Song(BaseMutagenSong):

    def __init__(self, path):
        super().__init__(path, mutagen.easyid3.EasyID3)

    # Note that "albumartist" appears to map to "performer" and
    # "performer" to nothing at all.
    def add_tag(self, str tag, str value):
        if tag == "tracktotal":
            return
        return super().add_tag(tag, value)


class OpusSong(BaseMutagenSong):

    def __init__(self, path):
        super().__init__(path, mutagen.oggopus.OggOpus)


class WavPackSong(BaseMutagenSong):

    def __init__(self, path):
        super().__init__(path, mutagen.wavpack.WavPack)


class VorbisSong(BaseMutagenSong):

    def __init__(self, path):
        super().__init__(path, mutagen.oggvorbis.OggVorbis)


cdef song_from(str path):
    if path.endswith(".flac"):
        return FlacSong(path)
    elif path.endswith(".mp3"):
        return Mp3Song(path)
    elif path.endswith(".opus"):
        return OpusSong(path)
    elif path.endswith(".wv"):
        return WavPackSong(path)
    elif path.endswith(".ogg"):
        return VorbisSong(path)
    raise RuntimeError("no handler for ``{}''".format(path))


def songs_from(list paths):
    """Returns a list of songs given |paths|."""
    result = list()
    for path in paths:
        result.append(song_from(path))

    return result


# Override EasyID3's default handling of the "albumartist" tag, which
# manifests as the "performer" tag in Quod Libet. This appears to be
# addressed in mutagen#252.
mutagen.easyid3.EasyID3.RegisterTXXXKey(
    "albumartist", "QuodLibet::albumartist")
