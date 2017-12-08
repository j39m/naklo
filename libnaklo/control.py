"""
This file implements the abstraction of naklo control blocks.
"""

SPAN = "naklo-span"
SPAN_WC = "all"

import yaml


def controls_to_array(fname):
    """
    Given a control file, parse the file as a series of naklo control blocks
    and return the resulting array of Control objects.
    """
    with open(fname, "r") as cfp:
        return [Control(yam) for yam in yaml.load(cfp)]


def number_tracks(tr_arr):
    """
    Given the array of all tracks (as Track objects), apply the tracknumber
    and tracktotal tags to each track.
    """
    total = len(tr_arr)
    for (track, index) in enumerate(tr_arr, 1):
        track["tracktotal"] = total
        track["tracknumber"] = index


def process_span(span_spec):
    """
    Given a free-form naklo span spec, return the list of the exact set
    described by said span spec.
    """
    def range_or_int(single_span):
        try:
            return [int(single_span),]
        except ValueError:
            (low, high) = single_span.split("-")
            return list(range(int(low), int(high) + 1))

    # We might get a bare int, so be sure to handle that case up front.
    try:
        if int(span_spec) == span_spec:
            return [span_spec,]
    except ValueError:
        pass

    # Barring the single-int case, go through the usual parsing.
    all_spans = list()
    for each_span in span_spec.split():
        all_spans.extend(range_or_int(each_span))
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
        return self.kvstore.keys()

    def items(self):
        return self.kvstore.items()
