#!/usr/bin/env python3

"""
Helper classes for pyaklon
"""

import os


class pyaFile(object):
    """
    Generic class of file
    """

    def __init__(self, fname, numFiles):
        self.fname = fname
        self.numFiles = numFiles
        self.ofile = open(fname, "r")
        self.contents = self._slurp()

    def _slurp(self):
        c = self.ofile.readlines()
        c = [ontent.strip() for ontent in c if ontent.strip()]
        return c

    def checkSanity(self):
        return False


class titleFile(pyaFile):
    """
    A titleFile is a file full of titles.
    """


class controlFile(pyaFile):
    """
    A controlFile is a file containing directives to pyaklon.
    """

    def __init__(self, fname, numFiles):
        super(controlFile, self).__init__(fname, numFiles)
        (self.lines, self.lnrs) = self._parse()
        self.acceptableTags = set(("artist",
                                   "album",
                                   "albumartist",
                                   "albumsort",
                                   "performer",
                                   "conductor",
                                   "composer",
                                   "arranger",
                                   "lyricist",
                                   "title",
                                   "location",
                                   "date",
                                   "genre",
                                   "discnumber",
                                   "disctotal",
                                   "discsubtitle,"))

    def _parse(self):
        lines = []
        lnrs = []
        i = 1
        for l in self.contents:
            if not self._isComment(l):
                lines.append(self._tokenize(l))
                lnrs.append(i)
            i += 1
        return (lines, lnrs)

    def _isComment(self, line):
        try:
            return (line[:2] == "//" or line[0] == "#")
        except IndexError:
            return False

    def _tokenize(self, line):
        """
        Returns a tuple of tokens from a given line.
        Returns None on a malformed line.
        """
        try:
            sp = [t.strip() for t in line.split("=", 1)]
            lhts = self._lhandTokenize(sp[0])
            tokens = lhts.append(sp[1])
            return tuple(tokens)
        except (IndexError, AttributeError): # bad split, None.append
            return None

    def _lhandTokenize(self, lhs):
        """
        Extract exploded numbering scheme and artist from a given string
        of left-hand tokens. Returns None on malformed left-hand tokens.
        Returns a list of extracted tokens otherwise.
        """
        try:
            tkns = lhts.split()
            tag = tkns[-1]
            nbrg_raw = tkns[:-1]
            nbrg = []

            for n in nbrg_raw:
                try:
                    nbrg.append(int(n))
                except ValueError:
                    ranger = self._numRangeSplit(n)
                    if ranger:
                        nbrg += ranger
        except IndexError:
            return None

        nbrg.sort()
        if tag in self.acceptableTags:
            lhts = nbrg + [tag]
            return lhts
        return None

    def _numRangeSplit(self, nr):
        try:
            lims = nr.split("-")
            if len(lims) != 2:
                return None
            lwr = int(lims[0])
            upr = int(lims[1])
            if upr <= lwr or lwr < 1:
                return None
            return list(range(lwr, upr+1))
        except ValueError:
            return None

    def checkSanity(self, demonstrate=False):
        """
        checkSanity: if "demonstrate" argument is not False, the return
        type changes from True/False to empty/nonempty list!
        """
        return False


class listingFile(pyaFile):
    """
    A listingFile enumerates the files to be tagged.
    This file is the only one that should have "None" passed as an
    argument for numFiles. Then this number determines the numFiles
    argument for the other files.
    """

    def __init__(self, fname, numFiles=None):
        super(listingFile, self).__init__(fname, numFiles)
        self.numFiles = self.countTagees()

    def countTagees(self):
        return len(self.contents)

    def enumerateTagees(self):
        return self.contents

    def checkSanity(self, demonstrate=False):
        """
        checkSanity: if "demonstrate" argument is not False, the return
        type changes from True/False to empty/nonempty list!
        This method is especially unsafe and is theoretically
        inconsistent in the face of a determined and swift-moving
        attacker. So is the rest of this program, really, so...
        """
        notFiles = [f for f in self.contents if os.path.isfile(f)]
        if demonstrate:
            return notFiles
        if notFiles:
            return False
        return True
