#!/usr/bin/env python


import json
from collections import namedtuple


class Node(namedtuple("NodeBase", ["id", "name"])):
    def __hash__(self):
        return self.id

    def neo_node(self):
        return py2neo.data.Node("Model", name=self.name, id=self.id)


Link = namedtuple("Link", ["src", "dest"])  # ids


def main(args):
    nodes = set()
    links = set()

    doc = json.load(args.filename)

    root_id = int(doc["roots"]["root_ids"][0])
    nodes.add(Node(id=root_id, name="Plot"))

    for obj in doc["roots"]["references"]:
        obj_id = int(obj["id"])
        nodes.add(Node(id=obj_id, name=obj["type"]))
        links.add(Link(src=root_id, dest=int(obj["id"])))

        for _, subobj in obj["attributes"].items():
            if isinstance(subobj, dict):
                if "type" in subobj and "id" in subobj:
                    nodes.add(Node(id=int(subobj["id"]), name=subobj["type"]))
                    links.add(Link(src=obj_id, dest=int(subobj["id"])))
            elif isinstance(subobj, list):
                for subobj in subobj:
                    if isinstance(subobj, dict):
                        if "type" in subobj and "id" in subobj:
                            nodes.add(Node(id=int(subobj["id"]), name=subobj["type"]))
                            links.add(Link(src=obj_id, dest=int(subobj["id"])))

    # Render graphviz file

    # Header
    args.output.write("digraph G {\n")

    # Nodes
    for node in nodes:
        name = f"node_{node.id}"
        args.output.write(f'    {name} [label="{node.name}"];\n')

    args.output.write("\n")

    # Links
    for link in links:
        src_name = f"node_{link.src}"
        dest_name = f"node_{link.dest}"
        args.output.write(f"    {src_name} -> {dest_name}\n")

    # Footer
    args.output.write("}\n")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("filename", type=argparse.FileType("r"))
    parser.add_argument(
        "-o", "--output", required=False, type=argparse.FileType("w"), default="-"
    )
    main(parser.parse_args())
