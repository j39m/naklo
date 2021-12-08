from cpython cimport bool
from cpython cimport array
import array

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

cdef array.array span_from(str span_spec):
    cdef array.array span = array.array("H")
    for token in span_spec.split():
        try:
            span.append(int(token))
            continue
        except ValueError:
            # Pop out of this exception handler
            pass
        (lower_bound, upper_bound) = token.split("-")
        span.extend(list(range(int(lower_bound), int(upper_bound) + 1)))
    return span


# Creates a span from a string (or an int).
# Raises ValueError if this cannot be done.
cdef array.array make_span(span_spec, int total_num_songs):
    if isinstance(span_spec, int):
        return array.array("H", [span_spec,])
    elif isinstance(span_spec, str):
        if span_spec == "*":
            span_spec = "1-{}".format(total_num_songs)
        return span_from(span_spec)
    raise ValueError("expected span spec, got ``{}''".format(span_spec))


cdef bool span_is_well_bounded(array.array span, int num_songs):
    for value in span:
        if value > num_songs:
            return False
    return True


cdef assert_tag_values_are_strings(list tag_values):
    for value in tag_values:
        if not isinstance(value, str):
            raise ValueError(
                "unexpected non-str detected: ``{}''".format(str(tag_values)))


cdef list listify_tag_values(str tag_name, raw_tag_values):
    if (isinstance(raw_tag_values, list)):
        assert_tag_values_are_strings(raw_tag_values)
        return raw_tag_values
    elif (isinstance(raw_tag_values, dict)):
        raise ValueError(
            "unexpected tag value {} for ``{}''".format(
                str(raw_tag_values), tag_name))
    return [str(raw_tag_values),]


# Applies |tag_name| having |tag_values| to |spanned_songs|.
# Caller is responsible for having dealt creating |spanned_songs|
# from a previously specified span.
cdef apply_to_view(str tag_name, list tag_values, list spanned_songs):
    assert_tag_values_are_strings(tag_values)
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
        apply_to_view(tag_name, listify_tag_values(tag_name, tag_values), spanned_songs)


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
        span_now = make_span(span_spec, num_songs)

        if not span_is_well_bounded(span_now, num_songs):
            raise ValueError("overwide span: {}".format(span_spec))
        classic_apply(tags_and_values, [result[i-1] for i in span_now])

    return result


# Has identical return type to process_classic_tag_block() but is cast
# from an inverted tag block instead.
cdef tuple process_inverted_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (tag_name, spans_and_values) in yaml_dictionary.items():
        for (span_spec, tag_values) in spans_and_values.items():
            span_now = make_span(span_spec, num_songs)

            if not span_is_well_bounded(span_now, num_songs):
                raise ValueError("overwide span: {}".format(span_spec))
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
