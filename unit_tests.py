#!/usr/bin/python

import unittest
from unittest.mock import Mock, call
import yaml

import libnaklo3.controls_util
from libnaklo3.controls import NakloController


def control_dict_for_testing(contents):
    return yaml.safe_load(contents)


class MockSong:
    def __init__(self):
        self.add_tag = Mock(return_value=None)

    def assert_exact_tags_added(self, call_list, any_order=False):
        self.add_tag.assert_has_calls(call_list, any_order=any_order)
        assert self.add_tag.call_count == len(call_list), \
            "{} != {}".format(self.add_tag.call_count, len(call_list))


class TestParseSpan(unittest.TestCase):
    def test_integer_span(self):
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               libnaklo3.controls_util.parse_span, "", 1)
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               libnaklo3.controls_util.parse_span, 2, 1)
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               libnaklo3.controls_util.parse_span, "2", 1)

    def test_ranged_span(self):
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               libnaklo3.controls_util.parse_span, "1-2", 1)
        self.assertSequenceEqual(
            libnaklo3.controls_util.parse_span("1-5", 5),
            range(1, 6))

    def test_mixed_span(self):
        self.assertSequenceEqual(
            libnaklo3.controls_util.parse_span("1 3 5-8 11", 11),
            (1, 3, 5, 6, 7, 8, 11))


class TestTitleMergeBlock(unittest.TestCase):
    def test_simple_merge(self):
        songs = [MockSong() for _ in range(6)]
        controller = NakloController(songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            title-merge-block:
                "*": Unit Test Cantata -
                "1 3 5": Part One -
                "2 4 6": Part Two -
                1: The Beginning
                "2-4": The Middle
                6: The End
            """
        ))
        controller.apply_tags()

        songs[1-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part One - The Beginning"),
            call("tracknumber", "1"),
            call("tracktotal", "6"),
        ])
        songs[2-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part Two - The Middle"),
            call("tracknumber", "2"),
            call("tracktotal", "6"),
        ])
        songs[3-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part One - The Middle"),
            call("tracknumber", "3"),
            call("tracktotal", "6"),
        ])
        songs[4-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part Two - The Middle"),
            call("tracknumber", "4"),
            call("tracktotal", "6"),
        ])
        songs[5-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part One -"),
            call("tracknumber", "5"),
            call("tracktotal", "6"),
        ])
        songs[6-1].assert_exact_tags_added([
            call("title", "Unit Test Cantata - Part Two - The End"),
            call("tracknumber", "6"),
            call("tracktotal", "6"),
        ])

    def test_merge_with_inverted_block(self):
        songs = [MockSong() for _ in range(2)]
        controller = NakloController(songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            inverted-tag-block:
                title:
                    01: Hello there!
                    02: General Kenobi!

            title-merge-block:
                "*": Is that legal?
                01: Don't ask stupid questions.
                02: I will make it legal.
            """
        ))
        controller.apply_tags()

        songs[1-1].assert_exact_tags_added([
            call("title", "Hello there!"),
            call("title", "Is that legal? Don't ask stupid questions."),
            call("tracknumber", "1"),
            call("tracktotal", "2"),
        ])
        songs[2-1].assert_exact_tags_added([
            call("title", "General Kenobi!"),
            call("title", "Is that legal? I will make it legal."),
            call("tracknumber", "2"),
            call("tracktotal", "2"),
        ])

    # `title-merge-block` must not add empty titles to songs.
    def test_mix_with_other_block(self):
        songs = [MockSong() for _ in range(2)]
        controller = NakloController(songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            # 01 has no title.
            title-merge-block:
                02: I have a title, unlike track 1.
            """
        ))
        controller.apply_tags()

        songs[1-1].assert_exact_tags_added([
            call("tracknumber", "1"),
            call("tracktotal", "2"),
        ])
        songs[2-1].assert_exact_tags_added([
            call("title", "I have a title, unlike track 1."),
            call("tracknumber", "2"),
            call("tracktotal", "2"),
        ])

# Some of these tests rely on the apparent behavior of yaml.safe_load()
# observing the underlying YAML insertion order. Python dictionaries are
# (today) observant of insertion order, but there's no guarantee that
# the yaml module is...


class TestBasicTagBlockAddition(unittest.TestCase):
    """Chiefly tests NakloController.add_tag_blocks()."""

    def test_trivial_construction(self):
        """
        For testing purposes, we may initialize NakloController
        instances with lists of NoneType. This verifies that we can
        get away with doing so (as long as we don't attempt to either
        apply_tags() or to enact()).
        """
        # An empty list shall be fine.
        controller = NakloController(list())

        # A list of NoneType shall be fine.
        controller = NakloController([None] * 13)

    def test_span_overrun_exception(self):
        """
        In an album of zero songs, attempting to tag the first song
        shall raise a span error.
        """
        control_data = control_dict_for_testing(
            """
            classic-tag-block:
                1:
                    title: "Hello there!"
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               controller.add_tag_blocks, control_data)

        control_data = control_dict_for_testing(
            """
            inverted-tag-block:
                title:
                    1: "General Kenobi!"
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(ValueError, "^bad span:.+$",
                               controller.add_tag_blocks, control_data)

    def test_invalid_span_exception(self):
        """
        NakloController.add_tag_blocks() shall raise an exception if
        it ingests an invalid span.

        TODO(j39m): better exception handling would be nice down in the
        span-making functions
        """
        control_data = control_dict_for_testing(
            """
            classic-tag-block:
                not-a-span:
                    artist: Some Artist
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(
            ValueError, "^too many values to unpack.+$",
            controller.add_tag_blocks, control_data)

        control_data = control_dict_for_testing(
            """
            inverted-tag-block:
                title:
                    not-a-span: "Hello there!"
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(
            ValueError, "^too many values to unpack.+$",
            controller.add_tag_blocks, control_data)

    def test_invalid_tag_name(self):
        """
        NakloController.add_tag_blocks() shall raise an exception if
        it ingests an invalid tag name.
        """
        control_data = control_dict_for_testing(
            """
            classic-tag-block:
                1-13:
                    artist: "Hello there!"
                    general: kenobi
            """
        )
        controller = NakloController([None] * 13)
        self.assertRaisesRegex(
            ValueError, "^invalid tag name: ``general''$",
            controller.add_tag_blocks, control_data)

        control_data = control_dict_for_testing(
            """
            inverted-tag-block:
                artist:
                    1-13: "Hello there!"
                General:
                    1-13: "Kenobi"
            """
        )
        controller = NakloController([None] * 13)
        self.assertRaisesRegex(
            ValueError, "^invalid tag name: ``General''$",
            controller.add_tag_blocks, control_data)

    def test_invalid_tag_value(self):
        """
        NakloController.add_tag_blocks() shall raise an exception if
        it the tag block leaf is not a string or list of strings.
        """
        control_data = control_dict_for_testing(
            """
            classic-tag-block:
                1-13:
                    title: "Hello there!"
            inverted-tag-block:
                artist:
                    1-13:
                        General: "Kenobi!"
            """
        )
        controller = NakloController([None] * 13)
        self.assertRaisesRegex(
            ValueError, "^unexpected dict value.+$",
            controller.add_tag_blocks, control_data)

        control_data = control_dict_for_testing(
            """
            inverted-tag-block:
                artist:
                    1-13: "Hello there!"
            classic-tag-block:
                1-13:
                    title:
                        General: "Kenobi!"
            """
        )
        controller = NakloController([None] * 13)
        self.assertRaisesRegex(
            ValueError, "^unexpected dict value.+$",
            controller.add_tag_blocks, control_data)

    def test_invalid_control_struct(self):
        """
        NakloController.add_tag_blocks() shall raise an exception if
        it doesn't recognize a top-level block identifier.
        """
        control_data = control_dict_for_testing(
            """
            unknowable-tag-block: "Hello there!"
            other-unknowable-tag-block:
                -   "These entries ought not trigger failure,"
                -   "Since these tests rely in part on the "
                -   "apparent insert-order-observant behavior "
            final-unknowable-tag-block:
                -   "of yaml.safe_load()."
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(
            ValueError, "^unrecognized block name: ``unknowable.+$",
            controller.add_tag_blocks, control_data)


# These tests rely more heavily on the ordered insertion behavior of
# yaml.safe_load().
#
# Maintainer's note: observe the call grouping when calling
# MockSong.assert_exact_tags_added(). The controller will have processed
# all tags in the classic-tag-block and then all tags in the
# inverted-tag-block. Within each tag block, calls are grouped by order
# of the tag name's first appearance.
class TestTagApplication(unittest.TestCase):
    """Chiefly tests NakloController.apply_tags()."""

    def test_classic_tag_block_application(self):
        mock_songs = [MockSong(),]
        controller = NakloController(mock_songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            classic-tag-block:
                1:
                    composer: Fryderyk Chopin
                    location: Warsaw Philharmonic
                    date: "2005-00-00"
            """
        ))

        controller.apply_tags()

        mock_songs[1-1].assert_exact_tags_added([
            call("composer", "Fryderyk Chopin"),
            call("location", "Warsaw Philharmonic"),
            call("date", "2005-00-00"),
            call("tracknumber", "1"),
            call("tracktotal", "1"),
        ])

    def test_inverted_tag_block_application(self):
        mock_songs = [MockSong(),]
        controller = NakloController(mock_songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            inverted-tag-block:
                location:
                    1: Warsaw Philharmonic
                date:
                    1: "2005-00-00"
                composer:
                    1: Claude Debussy
            """
        ))

        controller.apply_tags()

        mock_songs[1-1].assert_exact_tags_added([
            call("location", "Warsaw Philharmonic"),
            call("date", "2005-00-00"),
            call("composer", "Claude Debussy"),
            call("tracknumber", "1"),
            call("tracktotal", "1"),
        ])

    def test_multiple_tag_block_application(self):
        mock_songs = [MockSong() for _ in range(8)]
        controller = NakloController(mock_songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            classic-tag-block:
                1-8:
                    artist: Rafał Blechacz
                    album: Hypothetical Rachmaninoff Album
                1-6:
                    artist: Royal Concertgebouw Amsterdam
                    conductor: Jerzy Semkow
            inverted-tag-block:
                composer:
                    1-6: Sergei Rachmaninoff
                    "7 8": Maurice Ravel
                title:
                    1: Concerto no. 4 in G minor op. 40 - 1. Allegro vivace
                    2: Concerto no. 4 in G minor op. 40 - 2. Largo
                    3: Concerto no. 4 in G minor op. 40 - 3. Allegro vivace
                    4: Concerto no. 2 in C minor op. 18 - 1. Moderato
                    5: Concerto no. 2 in C minor op. 18 - 2. Allegro sostenuto
                    6: Concerto no. 2 in C minor op. 18 - 3. Allegro scherzando
                    7: Menuet sur le nom d'Haydn M. 58
                    8: La Valse M. 72b
            """
        ))

        controller.apply_tags()

        # Verifies the Rachmaninoff parts of the album.
        common_rach_tags = [
            call("artist", "Rafał Blechacz"),
            call("artist", "Royal Concertgebouw Amsterdam"),
            call("album", "Hypothetical Rachmaninoff Album"),
            call("conductor", "Jerzy Semkow"),
            call("composer", "Sergei Rachmaninoff"),
        ]
        title_names = [
            "Concerto no. 4 in G minor op. 40 - 1. Allegro vivace",
            "Concerto no. 4 in G minor op. 40 - 2. Largo",
            "Concerto no. 4 in G minor op. 40 - 3. Allegro vivace",
            "Concerto no. 2 in C minor op. 18 - 1. Moderato",
            "Concerto no. 2 in C minor op. 18 - 2. Allegro sostenuto",
            "Concerto no. 2 in C minor op. 18 - 3. Allegro scherzando",
        ]

        for (index, title) in enumerate(title_names):
            mock_songs[index].assert_exact_tags_added([
                *common_rach_tags,
                call("title", title),
                call("tracknumber", str(index + 1)),
                call("tracktotal", str(len(mock_songs))),
            ])

        # Verifies the Ravel parts of the album.
        common_ravel_tags = [
            call("artist", "Rafał Blechacz"),
            call("album", "Hypothetical Rachmaninoff Album"),
            call("composer", "Maurice Ravel"),
        ]
        mock_songs[7-1].assert_exact_tags_added([
            *common_ravel_tags,
            call("title", "Menuet sur le nom d'Haydn M. 58"),
            call("tracknumber", "7"),
            call("tracktotal", str(len(mock_songs))),
        ])
        mock_songs[8-1].assert_exact_tags_added([
            *common_ravel_tags,
            call("title", "La Valse M. 72b"),
            call("tracknumber", "8"),
            call("tracktotal", str(len(mock_songs))),
        ])

    def test_tag_block_aliases(self):
        mock_songs = [MockSong() for _ in range(1)]
        controller = NakloController(mock_songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            # alias for "classic-tag-block"
            span-tag-block:
                01:
                    artist: Rafał Blechacz
            # alias for "inverted-tag-block"
            tag-span-block:
                composer:
                    01: Francis Poulenc
            """
        ))

        controller.apply_tags()

        mock_songs[1-1].assert_exact_tags_added([
            call("artist", "Rafał Blechacz"),
            call("composer", "Francis Poulenc"),
            call("tracknumber", "1"),
            call("tracktotal", str(len(mock_songs))),
        ])

    def test_wildcard_span(self):
        mock_songs = [MockSong() for _ in range(2)]
        controller = NakloController(mock_songs)
        controller.add_tag_blocks(control_dict_for_testing(
            """
            classic-tag-block:
                1:
                    artist: Hello there!
                    composer: Obi-wan Kenobi
                2:
                    artist: General Kenobi!
                    composer: General Grievous
                "*":
                    albumartist: Sheev Palpatine
            inverted-tag-block:
                album:
                    "*": Prequel Memes vol. 1
            """
        ))

        controller.apply_tags()

        mock_songs[1-1].assert_exact_tags_added([
            call("artist", "Hello there!"),
            call("composer", "Obi-wan Kenobi"),
            call("albumartist", "Sheev Palpatine"),
            call("album", "Prequel Memes vol. 1"),
            call("tracknumber", "1"),
            call("tracktotal", "2"),
        ])

        mock_songs[2-1].assert_exact_tags_added([
            call("artist", "General Kenobi!"),
            call("composer", "General Grievous"),
            call("albumartist", "Sheev Palpatine"),
            call("album", "Prequel Memes vol. 1"),
            call("tracknumber", "2"),
            call("tracktotal", "2"),
        ])


if __name__ == "__main__":
    unittest.main()
