# coding: utf-8
"""Test doctest contained tests in every file of the module.
"""

import os
import sys
import doctest
import shutil
import re
import warnings
import types

import fastobo


def _load_tests_from_module(tests, module, globs, setUp=None, tearDown=None):
    """Load tests from module, iterating through submodules"""
    for attr in (getattr(module, x) for x in dir(module) if not x.startswith('_')):
        if isinstance(attr, types.ModuleType):
            attr.__test__ = {x: getattr(attr, x) for x in attr.__all__}
            tests.addTests(doctest.DocTestSuite(
                attr,
                globs=globs,
                setUp=setUp,
                tearDown=tearDown,
                optionflags=doctest.ELLIPSIS,
            ))
    return tests


def load_tests(loader, tests, ignore):
    """load_test function used by unittest to find the doctests"""

    globs = {
        "fastobo": fastobo,
    }

    if not sys.argv[0].endswith('green'):
        tests = _load_tests_from_module(tests, fastobo, globs)
    return tests


def setUpModule():
    warnings.simplefilter('ignore')


def tearDownModule():
    warnings.simplefilter(warnings.defaultaction)
