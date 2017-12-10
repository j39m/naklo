"""
This file implements a few lazy functions that make processing tile files
a little easier.

There is no real title object in naklo - the Control object supersedes
and encompasses that functionality. This module exists to enable the
shorthand, non-YAML style title slurp.
"""

__all__ = [
    "titles_to_array",
    "apply_titles_to_songs",
]


def titles_to_array(fname):
    """Return a titles file as a list."""
    with open(fname, "r") as tfp:
        return [title.strip() for title in tfp]


def apply_titles_to_songs(titles_arr, songs_arr):
    """
    Given an array of titles and an array of songs, apply all entries in
    the former to the latter.
    """
    for (title, song) in zip(titles_arr, songs_arr):
        song["title"] = title
