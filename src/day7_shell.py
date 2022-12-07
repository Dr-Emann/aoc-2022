#!/usr/bin/env python3

import os
import shutil
import tempfile
from pathlib import Path
import subprocess
import argparse
from argparse import ArgumentParser

def main():
    parser = ArgumentParser()
    parser.add_argument("input", action="store", nargs="?", type=argparse.FileType("r"))
    args = parser.parse_args()
    if not args.input:
        args.input = open(Path(__file__).parent.parent / "input/2022/day7.txt", "r")

    root = Path(os.path.realpath(tempfile.mkdtemp()))
    d = Path(root)
    try:
        for line in args.input:
            line = line.strip()
            if line == "$ cd /":
                pass
            elif line == "$ ls":
                pass
            elif line == "$ cd ..":
                d = d.parent
            elif line.startswith("dir "):
                dir_name = line[4:]
                (d / dir_name).mkdir()
            elif line.startswith("$ cd "):
                dir_name = line[5:]
                d = d / dir_name
            else:
                size, _, name = line.partition(' ')
                subprocess.run(["truncate", "-s", size, str(d / name)])
        print()
        print(f"About to drop you into a pretend shell (you're actually under {root})")
        print(f"run `exit` when you're done")
        print()

        subprocess.run(["bash", "-i"], cwd=str(root), env={
            **os.environ,
            "PS1": fr"$(pwd | sed s%^{root}%% | sed s_^\$_/_) \$ ",
        })
    finally:
        shutil.rmtree(root)


if __name__ == '__main__':
    main()