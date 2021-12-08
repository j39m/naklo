import libnaklo3.controls_util as controls_util

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

# Applies |tag_name| having |tag_values| to |spanned_songs|.
# Caller is responsible for having dealt creating |spanned_songs|
# from a previously specified span.
cdef apply_to_view(str tag_name, list tag_values, list spanned_songs):
    for song in spanned_songs:
        try:
            song[tag_name].extend(tag_values)
        except KeyError:
            # This copy is important; the song is taking ownership of a
            # separate instance of |tag_values|.
            song[tag_name] = list(tag_values)


cdef classic_apply(dict tags_and_values, list spanned_songs):
    for (tag_name, tag_values) in tags_and_values.items():
        if tag_name not in VALID_TAGS:
            raise ValueError("invalid tag name: ``{}''".format(tag_name))
        apply_to_view(tag_name,
                controls_util.listify_tag_values(tag_name, tag_values),
                spanned_songs)


# Creates a tuple of tag-value mappings.
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
cdef tuple process_classic_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (span_spec, tags_and_values) in yaml_dictionary.items():
        span_now = controls_util.parse_span(span_spec, num_songs)
        classic_apply(tags_and_values, [result[i-1] for i in span_now])

    return result


# Has identical return type to process_classic_tag_block() but is cast
# from an inverted tag block instead.
cdef tuple process_inverted_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (tag_name, spans_and_values) in yaml_dictionary.items():
        for (span_spec, tag_values) in spans_and_values.items():
            span_now = controls_util.parse_span(span_spec, num_songs)
            classic_apply({tag_name: tag_values},
                          [result[i-1] for i in span_now])
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
            else:
                raise ValueError(
                    "unrecognized block identifier: ``{}''".format(
                        block_identifier))
