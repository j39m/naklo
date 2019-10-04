"""
This file implements the abstraction of naklo control blocks.
"""

import yaml

__all__ = [
    "controls_to_array",
    "number_tracks",
    "Control",
]

SPAN = "naklo-span"
SPAN_WC = "all"


def controls_to_array(fname):
    """
    Given a control file, parse the file as a series of naklo control blocks
    and return the resulting array of Control objects.
    """
    with open(fname, "r") as cfp:
        return [Control(yam) for yam in yaml.safe_load(cfp)]


def number_tracks(tr_arr):
    """
    Given the array of all tracks (as Track objects), apply the tracknumber
    and tracktotal tags to each track.
    """
    total = len(tr_arr)
    for (index, track) in enumerate(tr_arr, 1):
        track["tracktotal"] = str(total)
        track["tracknumber"] = str(index)

def __range_or_int(span_token):
    """Expands span tokens, which are either bare ints or int ranges."""
    try:
        return [int(span_token),]
    except ValueError:
        (low, high) = span_token.split("-")
        return list(range(int(low), int(high) + 1))

def process_span(span_spec):
    """
    Given a free-form naklo span spec, return the list of the exact set
    described by said span spec.
    """
    # We might get a bare int, so be sure to handle that case up front.
    try:
        if int(span_spec) == span_spec:
            return [span_spec,]
    except ValueError:
        pass

    # Barring the single-int case, go through the usual parsing.
    all_spans = list()
    for each_span in span_spec.split():
        all_spans.extend(__range_or_int(each_span))
    return all_spans


class Control(object):
    """
    A Control object represents a naklo control block.
    """
    def __init__(self, kvd):
        self.kvstore = dict(kvd)
        span_spec = self.kvstore.pop(SPAN, SPAN_WC)
        self.span = (
            process_span(span_spec) if span_spec != SPAN_WC
            else SPAN_WC
        )

    def __getitem__(self, key):
        return self.kvstore[key]

    def __str__(self):
        basis = "Control (span {}): ".format(self.span)
        return basis + str(self.kvstore)

    def keys(self):
        """Pass through to my kvstore.keys."""
        return self.kvstore.keys()

    def apply_to_song(self, song):
        """
        Given a single song object, apply all my tags.
        """
        def value_to_list(raw_val):
            """Multiply-valued tags are rep'd as newl-separated strings."""
            value_list = list()
            for single_ in str(raw_val).split("\n"):
                single = single_.strip()
                if single:
                    value_list.append(single)
            return value_list

        for (tag, value) in self.kvstore.items():
            value_list = value_to_list(value)
            for val_single in value_list:
                song[tag] = val_single

    def apply_to_songs(self, songs_arr):
        """
        Given a array of all the songs (in consecutive order), apply all
        my tags to the relevant songs.
        """
        span = (
            self.span if self.span != SPAN_WC
            else range(1, len(songs_arr) + 1)
        )
        selection = [songs_arr[i - 1] for i in span]
        for song in selection:
            self.apply_to_song(song)
