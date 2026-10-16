#!/usr/bin/env python3
"""Build a print-friendly, bookmarkable PDF from the Guidebook function pages."""

from __future__ import annotations

import html
import re
import textwrap
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT
from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    HRFlowable,
    PageBreak,
    Paragraph,
    Preformatted,
    SimpleDocTemplate,
    Spacer,
)

from generate_function_pages import collect_functions

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "docs/book/src/function-pages.md"
OUTPUT = ROOT / "output/pdf/GTE-Function-Appendix.pdf"
FONT_DIR = Path("/System/Library/Fonts")
BODY_FONT = FONT_DIR / "Supplemental/Arial Unicode.ttf"
CODE_FONT = FONT_DIR / "Menlo.ttc"

GREEN = colors.HexColor("#81b64c")
INK = colors.HexColor("#20231e")
MUTED = colors.HexColor("#60665c")
RULE = colors.HexColor("#d8ded1")
CODE_BG = colors.HexColor("#f4f6f2")
FUNCTION_COUNT = 0


def register_fonts() -> None:
    pdfmetrics.registerFont(TTFont("GTEBody", str(BODY_FONT)))
    pdfmetrics.registerFont(TTFont("GTECode", str(CODE_FONT)))


def extract_pages(markdown: str) -> list[tuple[str, str]]:
    pattern = re.compile(
        r"^<!-- GTE_FUNCTION_PAGE: ([^|\n]+)\|([^\n]+) -->\n(.*?)^<!-- END_GTE_FUNCTION_PAGE -->$",
        re.S | re.M,
    )
    pages = [(slug.strip(), body.strip()) for slug, _title, body in pattern.findall(markdown)]
    if len(pages) < 92:
        raise ValueError(f"Expected at least 92 function pages, found {len(pages)}")
    return pages


def inline_markup(value: str) -> str:
    escaped = html.escape(value, quote=False)
    escaped = re.sub(
        r"`([^`]+)`",
        lambda match: f'<font name="GTECode" size="8">{match.group(1)}</font>',
        escaped,
    )
    escaped = re.sub(r"\*\*(.+?)\*\*", r"<b>\1</b>", escaped)
    escaped = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"<u>\1</u>", escaped)
    return escaped


def wrap_code(source: str, width: int = 118) -> str:
    wrapped_lines: list[str] = []
    for line in source.splitlines():
        if not line:
            wrapped_lines.append("")
            continue
        indentation = len(line) - len(line.lstrip(" "))
        continuation = " " * min(indentation + 2, 24)
        parts = textwrap.wrap(
            line,
            width=width,
            initial_indent="",
            subsequent_indent=continuation,
            break_long_words=True,
            break_on_hyphens=False,
            replace_whitespace=False,
            drop_whitespace=False,
        )
        wrapped_lines.extend(parts or [line])
    return "\n".join(wrapped_lines)


class FunctionTitle(Paragraph):
    def __init__(self, text: str, slug: str, style: ParagraphStyle):
        super().__init__(text, style)
        self.bookmark_slug = slug


class GtePdf(SimpleDocTemplate):
    def afterFlowable(self, flowable):
        if isinstance(flowable, FunctionTitle):
            key = flowable.bookmark_slug
            self.canv.bookmarkPage(key)
            self.canv.addOutlineEntry(flowable.getPlainText(), key, level=0, closed=True)


def draw_page(canvas, document) -> None:
    canvas.saveState()
    width, height = letter
    canvas.setStrokeColor(GREEN)
    canvas.setLineWidth(1.2)
    canvas.line(document.leftMargin, height - 39, width - document.rightMargin, height - 39)
    canvas.setFillColor(INK)
    canvas.setFont("GTEBody", 8)
    canvas.drawString(document.leftMargin, height - 30, "GTE  /  FUNCTION APPENDIX")
    canvas.setFillColor(MUTED)
    canvas.drawRightString(width - document.rightMargin, height - 30, f"{FUNCTION_COUNT} production Rust functions")
    canvas.setStrokeColor(RULE)
    canvas.setLineWidth(0.6)
    canvas.line(document.leftMargin, 39, width - document.rightMargin, 39)
    canvas.setFillColor(MUTED)
    canvas.setFont("GTEBody", 8)
    canvas.drawString(document.leftMargin, 26, "The GTE Book  |  Source paths refer to the Rust application")
    canvas.drawRightString(width - document.rightMargin, 26, f"{document.page}")
    canvas.restoreState()


def styles() -> dict[str, ParagraphStyle]:
    defaults = getSampleStyleSheet()
    return {
        "cover_title": ParagraphStyle(
            "GTECoverTitle", parent=defaults["Title"], fontName="GTEBody", fontSize=29,
            leading=36, textColor=INK, alignment=TA_LEFT, spaceAfter=15,
        ),
        "cover_subtitle": ParagraphStyle(
            "GTECoverSubtitle", fontName="GTEBody", fontSize=13, leading=19,
            textColor=MUTED, spaceAfter=14,
        ),
        "cover_heading": ParagraphStyle(
            "GTECoverHeading", fontName="GTEBody", fontSize=11, leading=15,
            textColor=INK, spaceBefore=17, spaceAfter=7,
        ),
        "title": ParagraphStyle(
            "GTEFunctionTitle", fontName="GTEBody", fontSize=20, leading=25,
            textColor=INK, spaceAfter=9, keepWithNext=True,
        ),
        "h2": ParagraphStyle(
            "GTEH2", fontName="GTEBody", fontSize=12, leading=16,
            textColor=colors.HexColor("#46652d"), spaceBefore=14, spaceAfter=5,
            keepWithNext=True,
        ),
        "h3": ParagraphStyle(
            "GTEH3", fontName="GTEBody", fontSize=10, leading=13,
            textColor=INK, spaceBefore=10, spaceAfter=4, keepWithNext=True,
        ),
        "body": ParagraphStyle(
            "GTEBodyText", fontName="GTEBody", fontSize=9, leading=13,
            textColor=INK, spaceAfter=7, alignment=TA_LEFT,
        ),
        "meta": ParagraphStyle(
            "GTEMeta", fontName="GTEBody", fontSize=8.2, leading=12,
            textColor=MUTED, spaceAfter=6,
        ),
        "bullet": ParagraphStyle(
            "GTEBullet", fontName="GTEBody", fontSize=9, leading=13,
            textColor=INK, leftIndent=13, firstLineIndent=-9, spaceAfter=3,
        ),
        "code": ParagraphStyle(
            "GTECodeBlock", fontName="GTECode", fontSize=6.3, leading=8,
            textColor=INK, backColor=CODE_BG, borderColor=RULE, borderWidth=0.55,
            borderPadding=7, leftIndent=0, rightIndent=0, spaceBefore=4,
            spaceAfter=9, splitLongWords=1,
        ),
    }


def append_markdown_body(story: list, body: str, slug: str, style: dict[str, ParagraphStyle]) -> None:
    lines = body.splitlines()
    code: list[str] = []
    in_code = False
    paragraph: list[str] = []
    title_added = False

    def flush_paragraph() -> None:
        if paragraph:
            story.append(Paragraph(inline_markup(" ".join(paragraph)), style["body"]))
            paragraph.clear()

    index = 0
    while index < len(lines):
        line = lines[index]
        if line.startswith("```"):
            flush_paragraph()
            if in_code:
                story.append(Preformatted(wrap_code("\n".join(code)), style["code"], maxLineLength=118))
                code.clear()
                in_code = False
            else:
                in_code = True
            index += 1
            continue
        if in_code:
            code.append(line)
            index += 1
            continue
        if not line.strip():
            flush_paragraph()
            index += 1
            continue
        if line.startswith("# "):
            flush_paragraph()
            if not title_added:
                title_added = True
                title_text = inline_markup(line[2:])
                story.append(FunctionTitle(title_text, slug, style["title"]))
            index += 1
            continue
        if line.startswith("## "):
            flush_paragraph()
            story.append(Paragraph(inline_markup(line[3:]), style["h2"]))
            index += 1
            continue
        if line.startswith("### "):
            flush_paragraph()
            story.append(Paragraph(inline_markup(line[4:]), style["h3"]))
            index += 1
            continue
        if line.startswith("**Source:**"):
            flush_paragraph()
            story.append(Paragraph(inline_markup(line), style["meta"]))
            index += 1
            continue
        if line.startswith("- "):
            flush_paragraph()
            story.append(Paragraph("- &nbsp;" + inline_markup(line[2:]), style["bullet"]))
            index += 1
            continue
        paragraph.append(line.strip())
        index += 1
    flush_paragraph()
    if in_code and code:
        story.append(Preformatted(wrap_code("\n".join(code)), style["code"], maxLineLength=118))


def build() -> None:
    global FUNCTION_COUNT
    register_fonts()
    pages = extract_pages(SOURCE.read_text(encoding="utf-8"))
    functions = collect_functions()
    if [slug for slug, _body in pages] != [function["slug"] for function in functions]:
        raise ValueError("Function inventory has changed; run tools/generate_function_pages.py first")
    for function, (_slug, body) in zip(functions, pages):
        location = f"**Source:** `{function['source_path']}:{function['line']}`"
        if location not in body or function["snippet"] not in body:
            raise ValueError(f"Stale function page for {function['qualified']}; regenerate the function pages first")
    FUNCTION_COUNT = len(pages)
    module_count = len({function["module"] for function in functions})
    style = styles()
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    document = GtePdf(
        str(OUTPUT), pagesize=letter, title="GTE Function Appendix",
        author="GTE Book", subject="Function-by-function reference for the Rust chess app",
        leftMargin=0.7 * inch, rightMargin=0.7 * inch,
        topMargin=0.78 * inch, bottomMargin=0.72 * inch,
    )
    story = [Spacer(1, 0.92 * inch), Paragraph("GTE Function Appendix", style["cover_title"]),
        HRFlowable(width="100%", thickness=3, color=GREEN, spaceBefore=1, spaceAfter=17),
        Paragraph("A readable, function-by-function guide to the Rust application behind Guess the Elo.", style["cover_subtitle"]),
        Paragraph(f"{FUNCTION_COUNT} functions · {module_count} focused Rust modules · one navigable PDF", style["cover_subtitle"]),
        Spacer(1, 0.22 * inch), Paragraph("How to use this appendix", style["cover_heading"]),
        Paragraph("Each function begins on a fresh page and has a PDF bookmark. Open the document outline in your PDF reader to jump directly to a function. Each entry includes its source location, responsibility, signature, implementation excerpt, and a note about how it fits into the larger codebase.", style["body"]),
        Paragraph("Coverage", style["cover_heading"]),
        Paragraph("main.rs selects an entry point. The api/ modules handle Chess.com archives; chess/ owns PGN, positions, and scoring; cli/ separates terminal input, screens, progress, and exercise checks; lessons/ defines the 24-exercise catalog; and web/ handles HTTP, sessions, game pages, and the book reader. HTML templates and stylesheets live under assets/.", style["body"]),
        Paragraph("The code examples are taken from the current application source. The companion browser book provides the same function pages with chapter navigation and the surrounding project walkthrough.", style["body"]),
        PageBreak()]
    for page_index, (slug, body) in enumerate(pages):
        if page_index:
            story.append(PageBreak())
        append_markdown_body(story, body, slug, style)
    document.build(story, onFirstPage=draw_page, onLaterPages=draw_page)
    print(f"Created {OUTPUT} with {len(pages)} function entries")


if __name__ == "__main__":
    build()
