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

cdef str UNSIGNED_SHORT_ARRAY_TYPE = "H"

cdef str OVERWIDE_SPAN_EXCEPTION_FORMAT = "overwide span: ``{}''"
cdef str BAD_TAG_NAME_EXCEPTION_FORMAT = "invalid tag name: ``{}''"
cdef str BAD_TAG_VALUE_EXCEPTION_FORMAT = \
    "unexpected tag value ``{}'' for ``{}''"

cdef str CLASSIC_TAG_BLOCK_ID = "classic-tag-block"
cdef str INVERTED_TAG_BLOCK_ID = "inverted-tag-block"

cdef array.array span_from(str span_spec):
    cdef array.array span = array.array(UNSIGNED_SHORT_ARRAY_TYPE)
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
cdef array.array make_span(span_spec):
    if isinstance(span_spec, int):
        return array.array(UNSIGNED_SHORT_ARRAY_TYPE, [span_spec,])
    elif isinstance(span_spec, str):
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


cdef classic_apply_single(dict tags_and_values, dict song):
    for (tag_name, tag_values) in tags_and_values.items():
        if tag_name not in VALID_TAGS:
            raise ValueError(BAD_TAG_NAME_EXCEPTION_FORMAT.format(tag_name))
        if tag_name not in song:
            song[tag_name] = list()

        if isinstance(tag_values, str):
            song[tag_name].append(tag_values)
        elif isinstance(tag_values, list):
            assert_tag_values_are_strings(tag_values)
            song[tag_name].extend(tag_values)
        else:
            raise ValueError(
                BAD_TAG_VALUE_EXCEPTION_FORMAT.format(
                    type(tag_values), tag_name))


cdef classic_apply(dict tags_and_values, list spanned_songs):
    for song in spanned_songs:
        classic_apply_single(tags_and_values, song)


# Creates a tuple of tag-value mappings.
# (
#   {
#       tag1: (value1, value2, ...),
#       tag2: (value1, ...),
#   },
#   {
#       ...
#   },
#   ...
# )
cdef tuple process_classic_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (span_spec, tags_and_values) in yaml_dictionary.items():
        span_now = make_span(span_spec)

        if not span_is_well_bounded(span_now, num_songs):
            raise ValueError(OVERWIDE_SPAN_EXCEPTION_FORMAT.format(span_spec))
        classic_apply(tags_and_values, [result[i-1] for i in span_now])

    return result


cdef inverted_apply_single(str tag_name, tag_values, song):
    if tag_name not in song:
        song[tag_name] = list()

    if isinstance(tag_values, str):
        song[tag_name].append(tag_values)
    elif isinstance(tag_values, list):
        assert_tag_values_are_strings(tag_values)
        song[tag_name].extend(tag_values)
    else:
        raise ValueError(
            BAD_TAG_VALUE_EXCEPTION_FORMAT.format(type(tag_values), tag_name))
    

cdef inverted_apply(
        span_spec, str tag_name, tag_values, int num_songs, tuple all_songs):
    span_now = make_span(span_spec)
    if not span_is_well_bounded(span_now, num_songs):
        raise ValueError(OVERWIDE_SPAN_EXCEPTION_FORMAT.format(span_spec))

    for song in [all_songs[i-1] for i in span_now]:
        inverted_apply_single(tag_name, tag_values, song)


# Has identical return type to process_classic_tag_block() but is cast
# from an inverted tag block instead.
cdef tuple process_inverted_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (tag_name, spans_and_values) in yaml_dictionary.items():
        if tag_name not in VALID_TAGS:
            raise ValueError(BAD_TAG_NAME_EXCEPTION_FORMAT.format(tag_name))

        for (span_spec, tag_values) in spans_and_values.items():
            inverted_apply(span_spec, tag_name, tag_values, num_songs, result)

    return result


class NakloController:

    def __init__(self, list songs):
        self.songs = songs
        self.processed_tag_blocks = list()

    def __apply_template(self, tuple template):
        """Applies a single template to all songs."""
        assert len(template) == len(self.songs), \
            "BUG: len(template) [{}] != len(self.songs) [{}]".format(
                len(template), len(self.songs))

        # Enforces the structure returned by the process_\w\+tag_block()
        # functions above.
        for (tag_values_pairs, song) in zip(template, self.songs):
            assert isinstance(tag_values_pairs, dict), \
                "BUG: expected a dict here"

            for (tag_name, values) in tag_values_pairs.items():
                assert isinstance(values, tuple), \
                    "BUG: expected a tuple here"

                for value in values:
                    song.add_tag(tag_name, value)

    def apply_tags(self):
        """Applies all templates to all songs."""
        for template in self.processed_tag_blocks:
            self.__apply_template(template)

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
            if block_identifier == CLASSIC_TAG_BLOCK_ID:
                self.processed_tag_blocks.append(process_classic_tag_block(
                    block, len(self.songs)))
            elif block_identifier == INVERTED_TAG_BLOCK_ID:
                self.processed_tag_blocks.append(process_inverted_tag_block(
                    block, len(self.songs)))
            else:
                raise ValueError(
                    "unrecognized block identifier: ``{}''".format(
                        block_identifier))
