# -*- coding: utf-8 -*-
import argparse

from . import _benchmark

parser = argparse.ArgumentParser(description="Rust Prime Utilities.")

parser.add_argument(
    "--benchmark",
    action="store_true",
    help="Run a primitive benchmark suite.",
    default=None,
)

args = parser.parse_args()

if args.benchmark:
    # Benchmarking utility
    print("Starting benchmark...")

    _benchmark.run()
