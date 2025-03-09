#!/usr/bin/env python3
# vim: set fileencoding=utf-8 :
# vim: set et ts=4 sw=4:
"""
  mdbook-pdf-outline
  An outline (Table of Content) generator for mdBook-pdf.

  Author:  Hollow Man <hollowman@opensuse.org>
  License: GPL-3.0

  Copyright © 2022-2025 Hollow Man (@HollowMan6). All rights reserved.

  This document is free software; you can redistribute it and/or modify it under the terms of the GNU General
  Public License as published by the Free Software Foundation; either version 3 of the License, or (at your option)
  any later version.

  This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
  warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

  You should have received a copy of the GNU General Public License along with this program.
  If not, see <http://www.gnu.org/licenses/>.
"""


from pypdf import PdfReader, PdfWriter
from pypdf.generic import Fit
import urllib.parse
import lxml.html
import re
import json
import sys
import os


buffer = []


def get_parent(level, parent_dict):
    if level > 1:
        temp = parent_dict
        for _ in range(1, level - 1):
            if temp["child"] and temp["child"]["node"]:
                temp = temp["child"]
        return temp["node"]
    return None


def update_parent_dict(
    parent_dict, level, writer, text, page, fit=None, handle_buffer=False
):
    temp = parent_dict
    for _ in range(1, level):
        if not temp["child"]:
            temp["child"] = {"node": None, "child": {}}
        temp = temp["child"]

    if not handle_buffer:
        if page is None:
            buffer.append((level, text))
            return
        else:
            # Flush buffer so that ToC items without page destinations are
            # added to the outline with the next page destination
            for item in buffer:
                update_parent_dict(
                    parent_dict, item[0], writer, item[1], page, fit, True
                )
            buffer.clear()

    temp["node"] = writer.add_outline_item(
        text, page, get_parent(level, parent_dict), fit=fit
    )
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
            dest = reader.named_destinations["/{}".format(
                urllib.parse.quote(id))]

            page = None
            fit = None
            if dest.get("/Type") != "/Fit":
                page = reader.get_destination_page_number(dest)
                fit = Fit(
                    dest.get("/Type"),
                    (dest.get("/Left"), dest.get("/Top"), dest.get("/Zoom")),
                )

            update_parent_dict(
                parent_dict, level, writer, results.text_content(), page, fit
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
            if "href" in target_element.attrib:
                for content in target_element.attrib["href"].split("#"):
                    dest_name += content.removesuffix(
                        ".html").replace("/", "-") + "-"
                dest_name = dest_name.rstrip("-")
                dest_name = "/{}".format(urllib.parse.quote(dest_name.lower()))

                if dest_name in reader.named_destinations:
                    dest = reader.named_destinations[dest_name]
                else:
                    print("Warning: Destination not found: {}".format(dest_name))
                    for d in reader.named_destinations.items():
                        if d[0].startswith(dest_name):
                            dest = d[1]
                            break

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
                parent_dict, level, writer, head.text_content(), page, fit
            )


def lxml_parse(data):
    root = lxml.html.fromstring(data)
    return root.find_class("chapter")


def add_toc_outline(html_page, reader, writer):
    parent_dict = {"node": None, "child": {}}
    found_toc = False
    with open(html_page, "r", encoding="utf8") as f:
        data = f.read()
        # Table of contents
        for result in lxml_parse(data):
            parse_toc(result, reader, writer, parent_dict)
            found_toc = True
            break

    if found_toc:
        return

    # Parse print.html to find the actual "toc-xxxx.js" file
    root = lxml.html.fromstring(data)
    scripts = root.xpath('//script[starts-with(@src, "toc-")]/@src')
    if not scripts:
        scripts = ["toc.js"]

    for script in scripts:
        try:
            with open(os.path.join(os.path.dirname(html_page), script), "r", encoding="utf8") as f:
                match = re.compile(
                    r"this\.innerHTML\s*=\s*'([^']*)';").search(f.read())
                if match:
                    for result in lxml_parse(match.group(1)):
                        parse_toc(result, reader, writer, parent_dict)
                        # Once a ToC is found, we can return
                        return
        except FileNotFoundError:
            pass


def main():
    sys.stdin.reconfigure(encoding="utf8")
    context = json.loads(sys.stdin.read())

    conf = context["config"]["output"]["pdf-outline"]

    reader = PdfReader("../pdf/output.pdf")

    writer = PdfWriter()
    writer.append(reader)

    if "like-wkhtmltopdf" in conf and conf["like-wkhtmltopdf"]:
        add_wkhtmltopdf_like_outline("../html/print.html", reader, writer)
    else:
        add_toc_outline("../html/print.html", reader, writer)

    meta = context["config"]["book"]
    try:
        writer.add_metadata(
            {
                "/DisplayDocTitle": True,
                "/Title": meta.get("title") or "",
                "/Author": ", ".join(meta["authors"]),
                "/Subject": meta.get("description") or "",
                "/CreationDate": reader.metadata["/CreationDate"],
                "/ModDate": reader.metadata["/ModDate"],
                "/Creator": "mdBook-pdf",
                "/Lang": meta.get("language") or "",
            }
        )
    except Exception:
        pass

    with open("output.pdf", "wb") as f:
        writer.write(f)


if __name__ == "__main__":
    main()
