# -*- coding: utf-8 -*-
"""
Configuration for the package.
"""
import os

try:
    DISABLE_CACHE = int(os.environ.get("DISABLE_CACHE", 0))
except ValueError:
    DISABLE_CACHE = 0
