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
        self.errs = []

    def _slurp(self):
        c = self.ofile.readlines()
        #c = [ontent.strip() for ontent in c if ontent.strip()]
        c = [ontent.strip() for ontent in c]
        return c

    def checkSanity(self):
        return False

    def showInsanity(self):
        return []

class titleFile(pyaFile):
    """
    A titleFile is a file full of titles.
    """

    def checkSanity(self):
        clenerr = None
        if len(self.contents) != self.numFiles:
            clenerr = str("titleFile length != numfiles (%d != %d)"
                          % (len(self.contents), self.numFiles))
        errlist = [clenerr,]
        self.errs = [e for e in errlist if e]
        if self.errs:
            return 1
        return 0

    def showInsanity(self):
        return self.errs

    def enumerateTitles(self):
        return self.contents


class controlFile(pyaFile):
    """
    A controlFile is a file containing directives to pyaklon.
    """

    def __init__(self, fname, numFiles):
        super(controlFile, self).__init__(fname, numFiles)
        self.hwmFile = 1    # high water mark file
        self.lwmFile = 1    # low water mark file
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
                                   "discsubtitle",))
        (self.lines, self.lnrs) = self._parse()

    def divulgeState(self):
        """
        for debugging use: print out the token results of slurping
        and parsing.
        """
        for pair in zip(self.lnrs, self.lines):
            print("%d: %s" %(pair[0], str(pair[1])))
        return 0

    def _parse(self):
        lines = []
        lnrs = []
        i = 1
        for l in self.contents:
            l = self._unComment(l)
            if l:
                lines.append(self._tokenize(l))
                lnrs.append(i)
            i += 1
        return (lines, lnrs)

    def _unComment(self, line):
        line = line.split("//", 1)[0]
        line = line.split("#", 1)[0]
        return line.strip()

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
            (lhts, tag) = self._lhandTokenize(sp[0])
            return (lhts, tag, sp[1])
        except (IndexError, TypeError): # case None
            return None

    def _lhandTokenize(self, lhs):
        """
        Extract exploded numbering scheme and artist from a given string
        of left-hand tokens. Returns None on malformed left-hand tokens.
        Returns a list of extracted tokens otherwise.
        """
        try:
            tkns = lhs.split()
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

        if not nbrg:
            nbrg = list(range(1, self.numFiles+1))
        else:
            nbrg.sort()
        if nbrg[-1] > self.hwmFile:
            self.hwmFile = nbrg[-1]
        if nbrg[0] < self.lwmFile:
            self.lwmFile = nbrg[0]
        if tag in self.acceptableTags:
            return (nbrg, tag)
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

    def checkSanity(self):
        errors = []
        for entry in zip(self.lnrs, self.lines):
            if entry[1] is None:
                errors.append(entry)
                continue
            lhts = entry[1][0]
            if (lhts[0] < 1) or (lhts[-1] > self.numFiles):
                errors.append(entry)
                continue
        self.errs = errors
        if self.errs:
            return 1
        return 0

    def showInsanity(self):
        errlist = []
        for err in self.errs:
            lnr = err[0]
            line = self.contents[lnr-1]
            fmtd = str("%02d: %s" % (lnr, line))
            errlist.append(fmtd)
        return errlist

    def enumerateDirectives(self):
        """
        returns an ordered tuple of
        (lhts, tag, value)
        where lhts is a misnomer for the numbers only (not including
        the tag, which is its own separate value) for which ``tag''
        should be applied with ``value.''
        """
        return self.lines

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

    def checkSanity(self):
        """
        checkSanity: basic sanity check on the files to operate upon.
        This method is especially unsafe and is theoretically
        inconsistent in the face of a determined and swift-moving
        attacker. So is the rest of this program, really, so...
        """
        notFiles = [f for f in self.contents if not os.path.isfile(f)]
        self.errs = notFiles
        if notFiles:
            return 1
        return 0

    def showInsanity(self):
        if self.errs:
            errlist = [f + " is not a file" for f in self.errs]
            return errlist