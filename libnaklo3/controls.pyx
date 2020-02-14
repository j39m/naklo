from cpython cimport bool
from cpython cimport array
import array

VALID_TAGS = (
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
)

UNSIGNED_SHORT_ARRAY_TYPE = "H"


cdef array.array span_from(str span_spec):
    cdef array.array span = array.array(UNSIGNED_SHORT_ARRAY_TYPE)
    for token in span_spec.split():
        try:
            span.append(int(token))
        except ValueError:
            (lower_bound, upper_bound) = token.split("-")
            span.extend(list(range(int(lower_bound), int(upper_bound) + 1)))
    return span


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


cdef void assert_tag_values_are_strings(list tag_values):
    for value in tag_values:
        if not isinstance(value, str):
            raise ValueError(
                "unexpected non-str detected: ``{}''".format(str(tag_values)))


cdef void classic_apply_single(dict tags_and_values, dict song):
    for (tag_name, tag_values) in tags_and_values.items():
        if tag_name not in VALID_TAGS:
            raise ValueError(
                    "invalid tag name: ``{}''".format(tag_name))
        if tag_name not in song:
            song[tag_name] = list()

        if isinstance(tag_values, str):
            song[tag_name].append(tag_values)
        elif isinstance(tag_values, list):
            assert_tag_values_are_strings(tag_values)
            song[tag_name].extend(tag_values)
        else:
            raise ValueError(
                "unexpected tag value ``{}'' for ``{}''".format(
                    type(tag_values), tag_name))


cdef void classic_apply(dict tags_and_values, list spanned_songs):
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
            raise ValueError("bad span: ``{}''".format(span_spec))
        classic_apply(tags_and_values, [result[i-1] for i in span_now])

    return result


cdef void inverted_apply_single(str tag_name, tag_values, song):
    if tag_name not in song:
        song[tag_name] = list()

    if isinstance(tag_values, str):
        song[tag_name].append(tag_values)
    elif isinstance(tag_values, list):
        assert_tag_values_are_strings(tag_values)
        song[tag_name].extend(tag_values)
    else:
        raise ValueError(
            "unexpected tag value ``{}'' for ``{}''".format(
                type(tag_values), tag_name))
    

cdef void inverted_apply(
        span_spec, str tag_name, tag_values, int num_songs, tuple all_songs):
    span_now = make_span(span_spec)
    if not span_is_well_bounded(span_now, num_songs):
        raise ValueError("bad span: ``{}''".format(span_spec))

    for song in [all_songs[i-1] for i in span_now]:
        inverted_apply_single(tag_name, tag_values, song)


# Has identical return type to process_classic_tag_block() but is cast
# from an inverted tag block instead.
cdef tuple process_inverted_tag_block(dict yaml_dictionary, int num_songs):
    cdef tuple result = tuple(dict() for _ in range(num_songs))

    for (tag_name, spans_and_values) in yaml_dictionary.items():
        if tag_name not in VALID_TAGS:
            raise ValueError(
                    "invalid tag name: ``{}''".format(tag_name))

        for (span_spec, tag_values) in spans_and_values.items():
            inverted_apply(span_spec, tag_name, tag_values, num_songs, result)

    return result
