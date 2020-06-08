from cpython cimport bool
import mutagen
import mutagen.easyid3
import mutagen.flac
import mutagen.oggopus

cdef class BaseMutagenSong:

    cdef str path
    cdef dict tags
    cdef object mutagen_object

    def __init__(self, str path, object mutagen_class):
        self.path = path
        self.tags = dict()
        self.mutagen_object = mutagen_class(path)

    def __str__(self):
        result = list()
        result.append(self.path)
        for (tag_name, tag_values) in self.tags.items():
            if len(tag_values) == 1:
                result.append("  {}: {}".format(tag_name, tag_values[0]))
            else:
                result.append("  {}:\n    {}".format(
                    tag_name, "\n    ".join(tag_values)
                ))
        return "\n".join(result)

    def add_tag(self, str tag, str value):
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


cdef song_from(str path):
    if path.endswith(".flac"):
        return FlacSong(path)
    elif path.endswith(".mp3"):
        return Mp3Song(path)
    elif path.endswith(".opus"):
        return OpusSong(path)
    raise RuntimeError("no handler for ``{}''".format(path))


def songs_from(list paths):
    """Returns a list of songs given |paths|."""
    result = list()
    for path in paths:
        result.append(song_from(path))

    return result
