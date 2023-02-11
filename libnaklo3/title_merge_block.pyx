import libnaklo3.controls_util as controls_util

"""
title_merge_block is separated into its own module because it is
sufficiently different from classic-tag-block and inverted-tag-block to
the point where code reuse is minimal.
"""

cdef map_prefixes(span_spec, str prefix, list preprocess, int num_songs):
    span = controls_util.parse_span(span_spec, num_songs)
    for i in span:
        preprocess[i-1].append(prefix)

cdef join_titles(list preprocess):
    result = list()
    for song_titles in preprocess:
        joined_title = " ".join(song_titles)
        value = {"title": [joined_title,]} if joined_title else dict()
        result.append(value)
    return tuple(result)

def process(dict block, int num_songs):
    """Returns a type appropriate for `NakloController`."""
    preprocess = [list() for _ in range(num_songs)]
    for (span_spec, prefix) in block.items():
        map_prefixes(span_spec, prefix, preprocess, num_songs)
    return join_titles(preprocess)
