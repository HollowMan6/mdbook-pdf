#!/usr/bin/env python3
# vim: set fileencoding=utf-8 :
# vim: set et ts=4 sw=4:
'''
  mdbook-pdf-outline
  Author:  Hollow Man <hollowman@opensuse.org>

  Copyright © 2022 Hollow Man(@HollowMan6). All rights reserved.

  This document is free software; you can redistribute it and/or modify it under the terms of the GNU General
  Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option)
  any later version.
'''


from pypdf import PdfReader, PdfMerger
from pypdf.generic import Fit
import lxml.html
import re

def update_parent_dict(parent_dict, level, node):
    temp = parent_dict
    for _ in range(1, level):
        if not temp["child"]:
            temp["child"] = {"node": None, "child": {}}
        temp = temp["child"]
    temp["node"] = node
    temp["child"] = {}


def add_outline(html_page, reader, merger):
    parent_dict = {"node": None, "child": {}}
    with open(html_page, "r", encoding="utf8") as f:
        data = f.read()
        root = lxml.html.fromstring(data)
        for id in re.findall(r'<a class="header" href="#(.+?)">', data):
            try:
                results = root.get_element_by_id(id)
            except KeyError:
                continue
            if results is None:
                continue
            if not results.tag[1:].isdigit():
                continue
            level = int(results.tag[1:])
            dest = reader.named_destinations["/{}".format(id)]
            parent = None
            if level > 1:
                temp = parent_dict
                for i in range(1, level - 1):
                    if temp["child"] and temp["child"]["node"]:
                        temp = temp["child"]
                parent = temp["node"]

            if dest.get('/Type') == '/Fit':
                update_parent_dict(parent_dict, level, merger.add_outline_item(
                    results.text_content(), 0, parent))
                continue
            update_parent_dict(parent_dict, level, merger.add_outline_item(
                results.text_content(), reader.get_destination_page_number(
                    dest), parent, fit=Fit(
                        dest.get('/Type'), (dest.get('/Left'), dest.get('/Top'), dest.get('/Zoom')))))


def main():
    input()

    reader = PdfReader("../pdf/output.pdf")

    merger = PdfMerger()
    merger.append(reader)

    add_outline("../html/print.html", reader, merger)

    with open("output.pdf", "wb") as f:
        merger.write(f)


if __name__ == "__main__":
    main()
