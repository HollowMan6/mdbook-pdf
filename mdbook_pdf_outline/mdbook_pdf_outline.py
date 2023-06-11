#!/usr/bin/env python3
# vim: set fileencoding=utf-8 :
# vim: set et ts=4 sw=4:
"""
  mdbook-pdf-outline
  Author:  Hollow Man <hollowman@opensuse.org>

  Copyright © 2022 Hollow Man(@HollowMan6). All rights reserved.

  This document is free software; you can redistribute it and/or modify it under the terms of the GNU General
  Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option)
  any later version.
"""


from pypdf import PdfReader, PdfWriter
from pypdf.generic import Fit
import urllib.parse
import lxml.html
import re
import json


buffer = []


def update_parent_dict(
    parent_dict, level, writer, text, page, parent, fit=None, handle_buffer=False
):
    if not handle_buffer:
        if page is None:
            buffer.append((level, text, parent))
            return
        else:
            for item in buffer:
                update_parent_dict(
                    parent_dict, item[0], writer, item[1], page, item[2], fit, True
                )
            buffer.clear()
    temp = parent_dict
    for _ in range(1, level):
        if not temp["child"]:
            temp["child"] = {"node": None, "child": {}}
        temp = temp["child"]
    temp["node"] = writer.add_outline_item(text, page, parent, fit=fit)
    temp["child"] = {}


def add_wkhtmltopdf_like_outline(html_page, reader, writer):
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
            dest = reader.named_destinations["/{}".format(urllib.parse.quote(id))]
            parent = None
            if level > 1:
                temp = parent_dict
                for _ in range(1, level - 1):
                    if temp["child"] and temp["child"]["node"]:
                        temp = temp["child"]
                parent = temp["node"]

            page = None
            fit = None
            if dest.get("/Type") != "/Fit":
                page = reader.get_destination_page_number(dest)
                fit = Fit(
                    dest.get("/Type"),
                    (dest.get("/Left"), dest.get("/Top"), dest.get("/Zoom")),
                )

            update_parent_dict(
                parent_dict, level, writer, results.text_content(), page, parent, fit
            )


def parse_toc(toc, reader, writer, parent_dict, level=1):
    for head in toc:
        section = head.find_class("section")
        if len(section) > 0:
            parse_toc(section[0], reader, writer, parent_dict, level + 1)
        else:
            dest_name = ""
            target_element = None
            for element in head.iter():
                if (
                    element.tag == "a"
                    or element.tag == "div"
                    or element.find_class("part-title")
                ):
                    target_element = element
                    break
            to_remove = head.find_class("toggle")
            for element in to_remove:
                element.getparent().remove(element)
            if target_element is None:
                continue
            dest = None
            parent = None
            if "href" in target_element.attrib:
                for content in target_element.attrib["href"].split("#"):
                    dest_name += content.removesuffix(".html").replace("/", "-") + "-"
                dest_name = dest_name.rstrip("-")
                dest_name = "/{}".format(urllib.parse.quote(dest_name.lower()))
                dest_name = dest_name.replace(".", "")

                if dest_name in reader.named_destinations:
                    dest = reader.named_destinations[dest_name]
                else:
                    for d in reader.named_destinations.items():
                        if d[0].startswith(dest_name):
                            dest = d[1]
                            break
            if level > 1:
                temp = parent_dict
                for _ in range(1, level - 1):
                    if temp["child"] and temp["child"]["node"]:
                        temp = temp["child"]
                parent = temp["node"]

            page = None
            fit = None
            if dest:
                if dest.get("/Type") != "/Fit":
                    page = reader.get_destination_page_number(dest)
                    fit = Fit(
                        dest.get("/Type"),
                        (dest.get("/Left"), dest.get("/Top"), dest.get("/Zoom")),
                    )

            update_parent_dict(
                parent_dict, level, writer, head.text_content(), page, parent, fit
            )


def add_toc_outline(html_page, reader, writer):
    parent_dict = {"node": None, "child": {}}
    with open(html_page, "r", encoding="utf8") as f:
        data = f.read()
        root = lxml.html.fromstring(data)
        results = root.find_class("chapter")
        # Table of contents
        for result in results:
            parse_toc(result, reader, writer, parent_dict)
            break


def main():
    conf = json.loads(input())["config"]["output"]["pdf-outline"]

    reader = PdfReader("../pdf/output.pdf")

    writer = PdfWriter()
    writer.append(reader)

    if "like-wkhtmltopdf" in conf and conf["like-wkhtmltopdf"]:
        add_wkhtmltopdf_like_outline("../html/print.html", reader, writer)
    else:
        add_toc_outline("../html/print.html", reader, writer)

    with open("output.pdf", "wb") as f:
        writer.write(f)


if __name__ == "__main__":
    main()
