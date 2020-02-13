#!/usr/bin/python

from distutils.core import setup
from Cython.Build import cythonize

setup(
    ext_modules=cythonize(
        "libnaklo3/*.pyx",
        compiler_directives={
            "language_level": "3",
            "warn.undeclared": True,
            "warn.maybe_uninitialized": True,
            "warn.unused": True,
            "warn.unused_arg": True,
            "warn.unused_result": True,
        }
    ),
)
