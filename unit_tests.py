#!/usr/bin/python

import unittest
import unittest.mock
import yaml

from libnaklo3.controls import NakloController


def control_dict_for_testing(contents):
    return yaml.safe_load(contents)


class MockSong:

    def __init__(self):
        self.add_tag = unittest.mock.Mock(return_value=None)

    def assert_tags_added(self, call_list):
        self.add_tag.assert_has_calls(call_list)
        assert self.add_tag.call_count == len(call_list)

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
        self.assertRaisesRegex(ValueError, "^overwide span:.+$",
            controller.add_tag_blocks, control_data)

        control_data = control_dict_for_testing(
            """
            inverted-tag-block:
                title:
                    1: "General Kenobi!"
            """
        )
        controller = NakloController(list())
        self.assertRaisesRegex(ValueError, "^overwide span:.+$",
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
                    tag: value
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
            ValueError, "^unexpected tag value.+for ``artist''$",
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
            ValueError, "^unexpected tag value.+for ``title''$",
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
            ValueError, "^unrecognized block identifier: ``unknowable.+$",
            controller.add_tag_blocks, control_data)


# These tests rely more heavily on the ordered insertion behavior of
# yaml.safe_load().
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

        mock_songs[0].assert_tags_added([
            unittest.mock.call("composer", "Fryderyk Chopin"),
            unittest.mock.call("location", "Warsaw Philharmonic"),
            unittest.mock.call("date", "2005-00-00"),
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

        mock_songs[0].assert_tags_added([
            unittest.mock.call("location", "Warsaw Philharmonic"),
            unittest.mock.call("date", "2005-00-00"),
            unittest.mock.call("composer", "Claude Debussy"),
        ])

    def test_multiple_tag_block_application(self):
        raise NotImplementedError("TODO(j39m)")

if __name__ == "__main__":
    unittest.main()
