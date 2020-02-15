#!/usr/bin/python

import unittest
import unittest.mock
import yaml

from libnaklo3.controls import NakloController


def control_dict_for_testing(contents):
    return yaml.safe_load(contents)


class TestNakloControllerBasic(unittest.TestCase):
    """Chiefly tests NakloController.add_tag_blocks()."""

    def test_trivial_construction(self):
        """
        For testing purposes, we may initialize NakloController
        instances with lists of NoneType. This verifies that we can
        get away with doing so (as long as we don't attempt to enact()).
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

if __name__ == "__main__":
    unittest.main()
