#!/usr/bin/env python


import subprocess as sp
import os
import glob
from contextlib import contextmanager


def fetch_git_tags():
    cmd = ['git', 'tag']
    output = sp.check_output(cmd)
    return [line for line in output.split("\n")
            if line]


def checkout_commit(tag):
    cmd = ['git', 'checkout', tag]
    sp.check_call(cmd, stdout=sp.PIPE)


@contextmanager
def reset_commit():
    cmd = ['git', 'rev-parse', 'HEAD']
    commit = sp.check_output(cmd).strip()
    try:
        yield
    finally:
        checkout_commit(commit)


class ModelGenError(RuntimeError):
    pass


class NoSpecScript(ModelGenError):
    pass


def generate_output():
    if not os.path.isfile("scripts/spec.py"):
        raise NoSpecScript("no spec script")

    cmd = ["python", "scripts/spec.py"]
    return sp.check_output(cmd)


def reset_output_dir(dirname):
    files = glob.iglob(os.path.join(dirname, "*.json"))
    for filename in files:
        os.remove(filename)


def main():
    output_dir = os.path.realpath(
        os.path.join(os.path.dirname(__file__), "..", "model_descriptions")
    )
    reset_output_dir(output_dir)
    os.chdir("bokeh-src")
    tags = fetch_git_tags()

    with reset_commit():
        for tag in tags:
            print(tag)
            output_filename = os.path.join(output_dir, "{}.json".format(tag))
            checkout_commit(tag)
            
            try:
                output = generate_output()
                with open(output_filename, "w") as outfile:
                    outfile.write(output)
            except NoSpecScript:
                pass
            except sp.CalledProcessError as e:
                print("Error: {}".format(e))


if __name__ == "__main__":
    main()
