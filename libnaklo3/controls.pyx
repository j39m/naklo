import libnaklo3.controls_util as controls_util
import libnaklo3.title_merge_block as title_merge_block

cdef set VALID_TAGS = set((
    "album",
    "albumartist",
    "albumsort",
    "arranger",
    "artist",
    "composer",
    "conductor",
    "date",
    "discnumber",
    "discsubtitle",
    "disctotal",
    "genre",
    "location",
    "lyricist",
    "performer",
    "title",
    "tracknumber",
    "tracktotal",
))

cdef unnest_dictionary(dict nested, int level, callback):
    """
    Descends into `nested` until `level` is reached, invoking
    `callback` on the accrued keys and the lowest-level value.
    """
    assert level >= 0, "BUG: unnest_dictionary() called with level < 0"
    if level == 0:
        for (key, value) in nested.items():
            callback(key, value)
        return
    for (key, value) in nested.items():
        unnest_dictionary(
                value, level - 1, lambda *args: callback(key, *args))

cdef map_tags(tuple songs, int num_songs, str tag_name, span_spec, value):
    if tag_name not in VALID_TAGS:
        raise ValueError("invalid tag name: ``{}''".format(tag_name))
    span = controls_util.parse_span(span_spec, num_songs)
    for i in span:
        song = songs[i-1]

        # Note: `listify_tag_values()` returns a new list, never a
        # reference to the original `value`. This is important, because
        # it prevents multiple songs from referring to the same data
        # that holds tag values.
        values_copy = controls_util.listify_tag_values(value)
        try:
            song[tag_name].extend(values_copy)
        except KeyError:
            song[tag_name] = values_copy

cdef tuple process_classic_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    # The hierarchy is span, tag name, tag value.
    unnest_dictionary(yaml_dictionary, 1,
            lambda s, t, v: map_tags(result, num_songs, t, s, v))
    return result

# Has identical return type to process_classic_tag_block() but is cast
# from an inverted tag block instead.
cdef tuple process_inverted_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    # The hierarchy is tag name, span, tag value.
    unnest_dictionary(yaml_dictionary, 1,
            lambda t, s, v: map_tags(result, num_songs, t, s, v))
    return result


# Applies a view of tags to a view of songs.
# Mutates |songs|.
cdef apply_template(tuple template, list songs):
    assert len(template) == len(songs), \
        "BUG: len(template) [{}] != len(songs) [{}]".format(
            len(template), len(songs))

    # Enforces the structure returned by the process_\w\+tag_block()
    # functions above.
    for (tag_values_pairs, song) in zip(template, songs):
        assert isinstance(tag_values_pairs, dict), \
            "BUG: expected a dict here"

        for (tag_name, values) in tag_values_pairs.items():
            assert isinstance(values, list), \
                "BUG: expected a list here"

            for value in values:
                song.add_tag(tag_name, value)

# Applies track numbers and totals.
# Mutates |songs|.
cdef apply_track_numbers(list songs):
    for (index, song) in enumerate(songs):
        song.add_tag("tracknumber", str(index + 1))
        song.add_tag("tracktotal", str(len(songs)))


cdef class NakloController:
    cdef list songs
    # Tuple of tag-value mappings:
    # (
    #   {
    #       tag1: [value1, value2, ...],
    #       tag2: [value1, ...],
    #   },
    #   {
    #       ...
    #   },
    #   ...
    # )
    cdef list processed_tag_blocks

    def __init__(self, list songs):
        self.songs = songs
        self.processed_tag_blocks = list()

    def apply_tags(self):
        """Applies all templates to all songs."""
        for template in self.processed_tag_blocks:
            apply_template(template, self.songs)
        apply_track_numbers(self.songs)

    def clear(self):
        for song in self.songs:
            song.clear()

    def enact(self):
        for song in self.songs:
            song.enact()

    def add_tag_blocks(self, dict tag_blocks):
        """
        |tag_blocks| contains at most 2 items,
        1.  a classic tag block or
        2.  an inverted tag block.
        """
        for (block_identifier, block) in tag_blocks.items():
            if block_identifier == "classic-tag-block":
                self.processed_tag_blocks.append(process_classic_tag_block(
                    block, len(self.songs)))
            elif block_identifier == "inverted-tag-block":
                self.processed_tag_blocks.append(process_inverted_tag_block(
                    block, len(self.songs)))
            elif block_identifier == "title-merge-block":
                self.processed_tag_blocks.append(
                        title_merge_block.process(
                            block, len(self.songs)))
            else:
                raise ValueError(
                    "unrecognized block identifier: ``{}''".format(
                        block_identifier))
