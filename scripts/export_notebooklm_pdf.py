#!/usr/bin/env python3
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Repository to Google NotebookLM Chapter-Based PDF Pipeline )
# ==============================================================================
# SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
#   - Exodus 31:18: "written with the finger of God"
#     Inscription of eternal moral truth upon stone.
#   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself"
#     Systematic modularization and chapter-based order.
#   - Proverbs 22:28: "Remove not the ancient landmark"
#     Preservation of invariant boundaries across all tracks.
#
# ARCHITECTURAL STRATEGY:
#   Partitions the entire repository into discrete, thematic Chapters:
#     - Chapter 01: Core Architecture, Manifest & Decalogue Bedrock
#     - Chapter 02: Track 0 - Kernel Substrate & Bare Metal
#     - Chapter 03: Track 1 - Abstract Syntax Tree & Parser
#     - Chapter 04: Track 1 - Core Types & Process Tape Ribbon
#     - Chapter 05: Track 1 - AI Cortex & Neural Learning Engine
#     - Chapter 06: Track 1 - Runtime Environment & Evaluator
#     - Chapter 07: Track 1 - Bytecode Assembler & Lowering
#     - Chapter 08: Track 1 - Diagnostics & 4-Block Linter
#     - Chapter 09: Track 1 - Terminal, REPL & Hover Inspection
#     - Chapter 10: Track 1 - Verification & Functional Test Suites
#     - Chapter 11: Track 2 - Lexicon: Character Primitive Type & Bridges
#     - Chapter 12: Track 2 - Lexicon: Number Primitive Type & Bridges
#     - Chapter 13: Track 2 - Lexicon: State, Container, Site & Text
#     - Chapter 14: Track 2 - Lexicon: Systems, Stewards & Physical Places
#     - Chapter 15: Track 2 - Lexicon: Verbs, Pronouns & Operators
#     - Chapter 16: Track 2 - Lexicon: Modifiers, Conjunctions & Traps
#     - Chapter 17: Track 2 - Standard Library Modules
#     - Chapter 18: Track 3 - Applications, C/Lua Bridges & Studio
#     - Chapter 19: Track 4 & 5 - Immutable Data & Infrastructure
# ==============================================================================

import os
import sys
import html
import json
import time
import shutil
import argparse
import subprocess
from datetime import datetime
from pathlib import Path

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Configuration Constants, Binary Filters & Chapter Definitions )
# ==============================================================================

REPO_ROOT = Path("/home/professorseanex/OmniCode").resolve()
OUTPUT_DIR = REPO_ROOT / "docs" / "notebooklm"

EXCLUDE_DIRS = {
    ".git", "target", ".vscode", ".gemini", "node_modules",
    "__pycache__", ".cargo", "dist", "notebooklm", "scrolls"
}

BINARY_EXTENSIONS = {
    ".pdf", ".img", ".elf", ".efi", ".bin", ".iso", ".png", ".jpg", ".jpeg",
    ".gif", ".ico", ".woff", ".woff2", ".ttf", ".eot", ".tar", ".gz",
    ".zip", ".pyc", ".o", ".a", ".so", ".dylib", ".rlib"
}

def is_binary_file(filepath: Path) -> bool:
    """Detects whether a file is binary via extension or byte inspection."""
    if filepath.suffix.lower() in BINARY_EXTENSIONS:
        return True
    try:
        with open(filepath, "rb") as f:
            chunk = f.read(1024)
            if b"\x00" in chunk:
                return True
    except Exception:
        return True
    return False

CHAPTER_DEFINITIONS = [
    {
        "id": "CH 01",
        "slug": "Ch01_Foundations_and_Manifest",
        "filename": "OmniCode_Ch01_Foundations_and_Manifest.pdf",
        "title": "Chapter 1: Foundations, Manifest & Decalogue Bedrock",
        "subtitle": "Moral Bedrock, Exodus 20 Decalogue, Manifest & Packaging",
        "prompt": "How does the Exodus 20 Decalogue govern the AI cortex?",
    },
    {
        "id": "CH 02",
        "slug": "Ch02_Track0_Kernel_Substrate",
        "filename": "OmniCode_Ch02_Track0_Kernel_Substrate.pdf",
        "title": "Chapter 2: Track 0 - Kernel Substrate & Bare Metal",
        "subtitle": "Bare-Metal x86 Boot, GDT, IDT, MMIO, VGA & Serial Ports",
        "prompt": "Trace how Genesis Boot initializes hardware MMIO registers.",
    },
    {
        "id": "CH 03",
        "slug": "Ch03_Track1_Parser_and_AST",
        "filename": "OmniCode_Ch03_Track1_Parser_and_AST.pdf",
        "title": "Chapter 3: Track 1 - Abstract Syntax Tree & Parser",
        "subtitle": "Lexical Tokens, Grammar Rules, AST Entities & Asides",
        "prompt": "Explain how the AST preserves first-class intent asides.",
    },
    {
        "id": "CH 04",
        "slug": "Ch04_Track1_Core_and_Tape_Engine",
        "filename": "OmniCode_Ch04_Track1_Core_and_Tape_Engine.pdf",
        "title": "Chapter 4: Track 1 - Core Types & Process Tape Ribbon",
        "subtitle": "TapeHead 5-Ws Ribbon, Cell, Word, Reels & Operators",
        "prompt": "How does the TapeHead coordinate the 5 Ws and 1 H in time?",
    },
    {
        "id": "CH 05",
        "slug": "Ch05_Track1_AI_Cortex_and_Neural_Engine",
        "filename": "OmniCode_Ch05_Track1_AI_Cortex_and_Neural_Engine.pdf",
        "title": "Chapter 5: Track 1 - AI Cortex & Neural Learning Engine",
        "subtitle": "Cognitive Loop, Synapses, Episodic Learner & Transducer",
        "prompt": "How does the AI Cortex reconcile deliberations with truth?",
    },
    {
        "id": "CH 06",
        "slug": "Ch06_Track1_Runtime_and_Evaluator",
        "filename": "OmniCode_Ch06_Track1_Runtime_and_Evaluator.pdf",
        "title": "Chapter 6: Track 1 - Runtime Environment & Evaluator",
        "subtitle": "SVO Verbs, Causal Clock, Transactions & Rollback",
        "prompt": "How does runtime guarantee forward time in rollback?",
    },
    {
        "id": "CH 07",
        "slug": "Ch07_Track1_Bytecode_Assembler",
        "filename": "OmniCode_Ch07_Track1_Bytecode_Assembler.pdf",
        "title": "Chapter 7: Track 1 - Bytecode Assembler & Lowering",
        "subtitle": "Instruction Encodings, Lowering Lexicon Tapes to Words",
        "prompt": "Trace lowering pipeline from tape rolls to machine words.",
    },
    {
        "id": "CH 08",
        "slug": "Ch08_Track1_Diagnostics_Linter",
        "filename": "OmniCode_Ch08_Track1_Diagnostics_Linter.pdf",
        "title": "Chapter 8: Track 1 - Diagnostics & 4-Block Linter",
        "subtitle": "Spatial Ruler, Morphological Bounds & Block Validation",
        "prompt": "How does the 4-block linter enforce spatial line geometry?",
    },
    {
        "id": "CH 09",
        "slug": "Ch09_Track1_Terminal_and_Hover",
        "filename": "OmniCode_Ch09_Track1_Terminal_and_Hover.pdf",
        "title": "Chapter 9: Track 1 - Terminal, REPL & Hover Inspection",
        "subtitle": "Interactive REPL, Task Scheduler & 3-Tier Hover Cards",
        "prompt": "How does hover engine bridge tape rolls to omni sources?",
    },
    {
        "id": "CH 10",
        "slug": "Ch10_Track1_Verification_Tests",
        "filename": "OmniCode_Ch10_Track1_Verification_Tests.pdf",
        "title": "Chapter 10: Track 1 - Verification & Functional Test Suites",
        "subtitle": "Integrated Tripwires, Boot Tests & Functional Execution",
        "prompt": "How do tripwire tests prove zero-disk-mutation on dry run?",
    },
    {
        "id": "CH 11",
        "slug": "Ch11_Track2_Lexicon_Character_Type",
        "filename": "OmniCode_Ch11_Track2_Lexicon_Character_Type.pdf",
        "title": "Chapter 11: Track 2 - Lexicon: Character Primitive Type",
        "subtitle": "Single-Cell Enclosure, 5-Trit / 8-Bit ASCII Invariance",
        "prompt": "Explain the bijective mapping of character 'A' to 5 trits.",
    },
    {
        "id": "CH 12",
        "slug": "Ch12_Track2_Lexicon_Number_Type",
        "filename": "OmniCode_Ch12_Track2_Lexicon_Number_Type.pdf",
        "title": "Chapter 12: Track 2 - Lexicon: Number Primitive Type",
        "subtitle": "Dual-Rail Polar Planes, 32-Trit Word & Zero-Sum ALU",
        "prompt": "How does dual-rail balance enable O(1) rail-swap negation?",
    },
    {
        "id": "CH 13",
        "slug": "Ch13_Track2_Lexicon_State_and_Containers",
        "filename": "OmniCode_Ch13_Track2_Lexicon_State_and_Containers.pdf",
        "title": "Chapter 13: Track 2 - Lexicon: State, Container, Site & Text",
        "subtitle": "Quad-State T4 Clock, Memory Envelopes & Bounded Sites",
        "prompt": "How do IT, BOUND, and TURN govern canonical loop cadence?",
    },
    {
        "id": "CH 14",
        "slug": "Ch14_Track2_Lexicon_Nouns_and_Entities",
        "filename": "OmniCode_Ch14_Track2_Lexicon_Nouns_and_Entities.pdf",
        "title": "Chapter 14: Track 2 - Lexicon: Systems, Places & Vessels",
        "subtitle": "Operators, Supervisors, Storehouses, Partitions & Vaults",
        "prompt": "Trace how Covenant Storehouse partitions via v--.",
    },
    {
        "id": "CH 15",
        "slug": "Ch15_Track2_Lexicon_Verbs_and_Operators",
        "filename": "OmniCode_Ch15_Track2_Lexicon_Verbs_and_Operators.pdf",
        "title": "Chapter 15: Track 2 - Lexicon: Verbs, Pronouns & Operators",
        "subtitle": "Directional SVO Verbs, Scale Vectors & Polarity",
        "prompt": (
            "Explain difference between ascension (/-^) and descension (\\-v)."
        ),
    },
    {
        "id": "CH 16",
        "slug": "Ch16_Track2_Lexicon_Modifiers_and_Connectives",
        "filename": (
            "OmniCode_Ch16_Track2_Lexicon_Modifiers_and_Connectives.pdf"
        ),
        "title": "Chapter 16: Track 2 - Modifiers, Conjunctions & Traps",
        "subtitle": "Invariants, Quad-States, Cadence, Pipelines & Caveats",
        "prompt": "How does the adversative caveat operator (!) guard truth?",
    },
    {
        "id": "CH 17",
        "slug": "Ch17_Track2_Standard_Library",
        "filename": "OmniCode_Ch17_Track2_Standard_Library.pdf",
        "title": "Chapter 17: Track 2 - Standard Library Modules",
        "subtitle": "Core Logic, Vectors, Arithmetic & Memory Management",
        "prompt": "How do standard library modules export pure invariants?",
    },
    {
        "id": "CH 18",
        "slug": "Ch18_Track3_Applications_and_Studio",
        "filename": "OmniCode_Ch18_Track3_Applications_and_Studio.pdf",
        "title": "Chapter 18: Track 3 - Applications, Bridges & Studio",
        "subtitle": "C99 Bare-Metal, Lua Coroutines, Boundary Demos & HUD",
        "prompt": "How does Lua coroutine engine run tape rolls as fibers?",
    },
    {
        "id": "CH 19",
        "slug": "Ch19_Track4_and_5_Storage_and_Infra",
        "filename": "OmniCode_Ch19_Track4_and_5_Storage_and_Infra.pdf",
        "title": "Chapter 19: Track 4 & 5 - Immutable Data & Infrastructure",
        "subtitle": "Canonical Bibles, OCI Scripts, Toolchain & CI/CD",
        "prompt": (
            "How does infrastructure preserve immutable scriptural anchors?"
        ),
    },
]

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( File Discovery, Chapter Partitioning, HTML Rendering & Chrome Lowering )
# ==============================================================================

CSS_STYLES = """
@page {
    size: letter portrait;
    margin: 1.5cm 1.2cm 1.5cm 1.2cm;
    @top-center {
        content: "OmniCode AI OS & Substrate | Google NotebookLM Curriculum";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        font-size: 7pt;
        color: #888888;
    }
    @bottom-left {
        content: "Witnesses: Exodus 31:18, Proverbs 24:27, Proverbs 22:28";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        font-size: 7pt;
        color: #888888;
    }
    @bottom-right {
        content: "Page " counter(page);
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        font-size: 7pt;
        color: #555555;
    }
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    color: #1a1a1a;
    background: #ffffff;
    line-height: 1.45;
    font-size: 8.5pt;
    margin: 0;
    padding: 0;
}

.page-break {
    page-break-before: always;
    break-before: page;
}

.cover-page {
    min-height: 90vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    border: 3px double #1e3a8a;
    padding: 35px;
    background: linear-gradient(135deg, #f8fafc 0%, #edf2f7 100%);
    box-sizing: border-box;
}

.cover-title {
    font-size: 20pt;
    font-weight: 800;
    color: #0f172a;
    margin-bottom: 6px;
    letter-spacing: -0.5px;
    border-bottom: 3px solid #3b82f6;
    padding-bottom: 10px;
}

.cover-subtitle {
    font-size: 11pt;
    color: #334155;
    font-weight: 600;
    margin-bottom: 20px;
}

.cover-metadata-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    background: #ffffff;
    border: 1px solid #cbd5e1;
    border-radius: 6px;
    padding: 14px;
    margin-bottom: 20px;
}

.metadata-item {
    font-size: 8pt;
}

.metadata-label {
    font-weight: bold;
    color: #475569;
    text-transform: uppercase;
    font-size: 7pt;
    display: block;
}

.metadata-value {
    color: #0f172a;
    font-family: monospace;
    font-size: 8pt;
}

.scripture-card {
    background: #f0fdf4;
    border-left: 4px solid #16a34a;
    padding: 10px 14px;
    border-radius: 4px;
    margin-bottom: 18px;
}

.scripture-title {
    font-weight: bold;
    color: #166534;
    font-size: 8pt;
    text-transform: uppercase;
    margin-bottom: 4px;
}

.scripture-verse {
    font-style: italic;
    color: #14532d;
    font-size: 7.5pt;
    margin: 2px 0;
}

.guide-card {
    background: #eff6ff;
    border-left: 4px solid #2563eb;
    padding: 12px 16px;
    border-radius: 4px;
    margin-bottom: 18px;
}

.guide-title {
    font-weight: bold;
    color: #1e40af;
    font-size: 8.5pt;
    text-transform: uppercase;
    margin-bottom: 5px;
}

.guide-text {
    font-size: 7.5pt;
    color: #1e3a8a;
    line-height: 1.35;
    margin: 3px 0;
}

.toc-container {
    margin: 15px 0;
}

.toc-header {
    font-size: 13pt;
    font-weight: bold;
    color: #0f172a;
    border-bottom: 2px solid #cbd5e1;
    padding-bottom: 5px;
    margin-bottom: 10px;
    text-transform: uppercase;
}

.toc-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 7.5pt;
}

.toc-table th {
    background: #f1f5f9;
    color: #334155;
    text-align: left;
    padding: 5px 8px;
    border-bottom: 2px solid #94a3b8;
    text-transform: uppercase;
    font-size: 7pt;
}

.toc-table td {
    padding: 4px 8px;
    border-bottom: 1px solid #e2e8f0;
}

.toc-table tr:nth-child(even) {
    background: #f8fafc;
}

.file-card {
    margin-top: 12px;
    margin-bottom: 20px;
}

.file-header-box {
    background: #0f172a;
    color: #ffffff;
    padding: 7px 10px;
    border-radius: 4px 4px 0 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    page-break-after: avoid;
    break-after: avoid;
}

.file-path {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", monospace;
    font-size: 8pt;
    font-weight: 700;
    color: #38bdf8;
    word-break: break-all;
}

.file-meta-badges {
    display: flex;
    gap: 6px;
}

.badge {
    background: #1e293b;
    border: 1px solid #334155;
    padding: 1px 5px;
    border-radius: 3px;
    font-size: 6.5pt;
    color: #cbd5e1;
    font-family: monospace;
    text-transform: uppercase;
}

.file-content-box {
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-top: none;
    padding: 8px;
    border-radius: 0 0 4px 4px;
    overflow-x: auto;
}

pre.code-block {
    margin: 0;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", monospace;
    font-size: 6.8pt;
    line-height: 1.32;
    color: #0f172a;
    tab-size: 4;
    white-space: pre-wrap;
    word-break: break-word;
}

.line-num {
    color: #94a3b8;
    user-select: none;
    display: inline-block;
    width: 32px;
    text-align: right;
    margin-right: 8px;
    border-right: 1px solid #cbd5e1;
    padding-right: 5px;
}
"""

def categorize_file(rel_path: str) -> str:
    """Categorizes a file into one of the 19 canonical chapter slugs."""
    p = rel_path.lower()
    
    # Chapter 1: Root manifests and foundational documents
    if '/' not in rel_path or any(
        rel_path.startswith(x)
        for x in ['README', 'manifest', 'Cargo', '.env']
    ):
        return "Ch01_Foundations_and_Manifest"
        
    # Chapter 2: Track 0
    if 'track 0' in p:
        return "Ch02_Track0_Kernel_Substrate"
        
    # Chapters 3 - 10: Track 1 Host Engine
    if 'track 1' in p:
        if '/tests/' in p:
            return "Ch10_Track1_Verification_Tests"
        if '/src/parser/' in p or '/src/ast/' in p:
            return "Ch03_Track1_Parser_and_AST"
        if '/src/core/' in p or '/src/cogs/' in p or rel_path.endswith(
            ('main.rs', 'lib.rs')
        ):
            return "Ch04_Track1_Core_and_Tape_Engine"
        if '/src/ai/' in p:
            return "Ch05_Track1_AI_Cortex_and_Neural_Engine"
        if '/src/runtime/environment/' in p or '/src/runtime/evaluator/' in p:
            return "Ch06_Track1_Runtime_and_Evaluator"
        if '/src/runtime/assembler/' in p:
            return "Ch07_Track1_Bytecode_Assembler"
        if '/src/diagnostics/linter/' in p:
            return "Ch08_Track1_Diagnostics_Linter"
        if '/src/terminal/' in p or '/src/cli/' in p or 'hover' in p:
            return "Ch09_Track1_Terminal_and_Hover"
        return "Ch04_Track1_Core_and_Tape_Engine"
        
    # Chapters 11 - 17: Track 2 Language & Standard Library
    if 'track 2' in p:
        if '01_nouns/ideas/types/character' in p:
            return "Ch11_Track2_Lexicon_Character_Type"
        if '01_nouns/ideas/types/number' in p:
            return "Ch12_Track2_Lexicon_Number_Type"
        if '01_nouns/ideas/types' in p:
            return "Ch13_Track2_Lexicon_State_and_Containers"
        if '01_nouns' in p:
            return "Ch14_Track2_Lexicon_Nouns_and_Entities"
        if '02_pronouns' in p or '03_verbs' in p or '/operators/' in p:
            return "Ch15_Track2_Lexicon_Verbs_and_Operators"
        if any(
            x in p for x in [
                '04_adjectives', '05_adverbs', '06_prepositions',
                '07_conjunctions', '08_interjections'
            ]
        ):
            return "Ch16_Track2_Lexicon_Modifiers_and_Connectives"
        if '/lib/' in p:
            return "Ch17_Track2_Standard_Library"
        return "Ch16_Track2_Lexicon_Modifiers_and_Connectives"
        
    # Chapter 18: Track 3 Applications
    if 'track 3' in p:
        return "Ch18_Track3_Applications_and_Studio"
        
    # Chapter 19: Track 4 & 5 Storage & Infrastructure
    if 'track 4' in p or 'track 5' in p or 'scripts' in p:
        return "Ch19_Track4_and_5_Storage_and_Infra"
        
    return "Ch01_Foundations_and_Manifest"

def discover_repository_files():
    """Scans and categorizes text and manifest files across the repository."""
    all_files = []
    
    for root, dirs, files in os.walk(REPO_ROOT):
        dirs[:] = [
            d for d in dirs
            if d not in EXCLUDE_DIRS and not d.startswith(".")
        ]
        
        rel_root = Path(root).relative_to(REPO_ROOT)
        if any(x in rel_root.parts for x in ["notebooklm", "scrolls"]):
            continue
            
        top_component = rel_root.parts[0] if rel_root.parts else ""
        
        for file in sorted(files):
            if (
                file.startswith(".")
                or file.endswith("~")
                or file.endswith(".tmp")
                or file.endswith(".tmp.html")
            ):
                continue
                
            full_path = Path(root) / file
            rel_path = full_path.relative_to(REPO_ROOT)
            rel_str = str(rel_path)
            
            if any(x in rel_path.parts for x in ["notebooklm", "scrolls"]):
                continue
                
            is_bin = is_binary_file(full_path)
            file_bytes = full_path.stat().st_size
            lines_count = 0
            words_count = 0
            content = ""
            
            if not is_bin:
                try:
                    with open(
                        full_path, "r", encoding="utf-8", errors="replace"
                    ) as f:
                        content = f.read()
                        lines_count = (
                            content.count("\n") + (1 if content else 0)
                        )
                        words_count = len(content.split())
                except Exception as e:
                    content = f"[Error reading file: {e}]"
            else:
                if full_path.suffix.lower() == ".pdf":
                    size_mb = file_bytes / (1024 * 1024)
                    content = (
                        f"[Immutable Scripture Reference: "
                        f"{file} ({size_mb:.2f} MB)]"
                    )
                else:
                    size_kb = file_bytes / 1024.0
                    content = f"[Binary Artifact: {file} ({size_kb:.1f} KB)]"
                lines_count = 1
                words_count = len(content.split())
                
            entry = {
                "rel_path": rel_str,
                "full_path": str(full_path),
                "filename": file,
                "extension": full_path.suffix.lower(),
                "top_dir": top_component,
                "chapter_slug": categorize_file(rel_str),
                "is_binary": is_bin,
                "lines": lines_count,
                "words": words_count,
                "bytes": file_bytes,
                "content": content,
            }
            all_files.append(entry)
            
    return all_files

def render_file_card_html(entry):
    """Renders a single file into a publication card with line numbers."""
    rel_path = entry["rel_path"]
    ext = entry["extension"]
    is_bin = entry["is_binary"]
    lines = entry["lines"]
    words = entry["words"]
    size_kb = entry["bytes"] / 1024.0
    content = entry["content"]
    
    badge_type = ext.replace(".", "").upper() if ext else "TEXT"
    if is_bin:
        badge_type = "DOC/BINARY"
        
    out = []
    out.append('<div class="file-card page-break">')
    out.append('  <div class="file-header-box">')
    out.append(f'    <div class="file-path">{html.escape(rel_path)}</div>')
    out.append('    <div class="file-meta-badges">')
    out.append(f'      <span class="badge">{badge_type}</span>')
    out.append(f'      <span class="badge">{lines} Lines</span>')
    out.append(f'      <span class="badge">{words} Words</span>')
    out.append(f'      <span class="badge">{size_kb:.1f} KB</span>')
    out.append('    </div>')
    out.append('  </div>')
    out.append('  <div class="file-content-box">')
    
    if is_bin:
        out.append(
            '    <p style="font-style:italic; color:#475569; '
            'margin:8px 0; font-size:8pt;">'
        )
        out.append(f'      {html.escape(content)}')
        out.append('    </p>')
    else:
        out.append('    <pre class="code-block">')
        raw_lines = content.splitlines()
        for i, l in enumerate(raw_lines, 1):
            escaped = html.escape(l)
            out.append(f'<span class="line-num">{i}</span>{escaped}')
        out.append('</pre>')
        
    out.append('  </div>')
    out.append('</div>\n')
    return "\n".join(out)

def build_chapter_html(chapter_def, files):
    """Assembles a self-contained publication document for a single chapter."""
    ch_id = chapter_def["id"]
    ch_title = chapter_def["title"]
    ch_sub = chapter_def["subtitle"]
    ch_prompt = chapter_def["prompt"]
    
    ch_lines = sum(f["lines"] for f in files)
    ch_words = sum(f["words"] for f in files)
    gen_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    html_parts = []
    html_parts.append('<!DOCTYPE html>')
    html_parts.append('<html lang="en">')
    html_parts.append('<head>')
    html_parts.append('  <meta charset="UTF-8">')
    html_parts.append(f'  <title>{html.escape(ch_title)}</title>')
    html_parts.append(f'  <style>{CSS_STYLES}</style>')
    html_parts.append('</head>')
    html_parts.append('<body>')
    
    # 1. Cover Page
    html_parts.append('<div class="cover-page">')
    html_parts.append(
        f'  <div class="cover-title">OmniCode: {html.escape(ch_title)}</div>'
    )
    html_parts.append(
        f'  <div class="cover-subtitle">{html.escape(ch_sub)}</div>'
    )
    
    html_parts.append('  <div class="cover-metadata-grid">')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append(
        '      <span class="metadata-label">Chapter Identifier</span>'
    )
    html_parts.append(
        f'      <span class="metadata-value">{html.escape(ch_id)}</span>'
    )
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append(
        '      <span class="metadata-label">NotebookLM Word Count</span>'
    )
    html_parts.append(
        f'      <span class="metadata-value">{ch_words:,} words</span>'
    )
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append(
        '      <span class="metadata-label">Cataloged Chapter Files</span>'
    )
    html_parts.append(
        f'      <span class="metadata-value">{len(files)} files '
        f'({ch_lines:,} lines)</span>'
    )
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append(
        '      <span class="metadata-label">Generation Date</span>'
    )
    html_parts.append(f'      <span class="metadata-value">{gen_time}</span>')
    html_parts.append('    </div>')
    html_parts.append('  </div>')
    
    # Scripture Witness Card
    html_parts.append('  <div class="scripture-card">')
    html_parts.append(
        '    <div class="scripture-title">'
        'Epistemic Covenant Bedrock & Immutable Landmarks</div>'
    )
    html_parts.append(
        '    <div class="scripture-verse">"And he gave unto Moses... two '
        'tables of testimony, tables of stone, written with the finger of '
        'God." &mdash; Exodus 31:18</div>'
    )
    html_parts.append(
        '    <div class="scripture-verse">"Prepare thy work without, and '
        'make it fit for thyself in the field; and afterwards build thine '
        'house." &mdash; Proverbs 24:27</div>'
    )
    html_parts.append(
        '    <div class="scripture-verse">"Remove not the ancient landmark, '
        'which thy fathers have set." &mdash; Proverbs 22:28</div>'
    )
    html_parts.append('  </div>')
    
    # Grounding Prompt Card
    html_parts.append('  <div class="guide-card">')
    html_parts.append(
        '    <div class="guide-title">'
        'Google NotebookLM Grounding Focus & Learning Prompt</div>'
    )
    html_parts.append(
        f'    <div class="guide-text">&bull; <strong>Chapter Focus:</strong> '
        f'<em>"{html.escape(ch_prompt)}"</em></div>'
    )
    html_parts.append(
        '    <div class="guide-text">'
        '&bull; <strong>Architectural Rule:</strong> '
        'All definitions conform to 4-block structure & spatial ruler.</div>'
    )
    html_parts.append('  </div>')
    html_parts.append('</div>')
    
    # 2. Table of Contents
    html_parts.append('<div class="toc-container page-break">')
    html_parts.append(
        f'  <div class="toc-header">Chapter Manifest & Index '
        f'({len(files)} files)</div>'
    )
    html_parts.append('  <table class="toc-table">')
    html_parts.append('    <thead>')
    html_parts.append('      <tr>')
    html_parts.append('        <th>#</th>')
    html_parts.append('        <th>File Path</th>')
    html_parts.append('        <th>Type</th>')
    html_parts.append('        <th>Lines</th>')
    html_parts.append('        <th>Words</th>')
    html_parts.append('        <th>Size (KB)</th>')
    html_parts.append('      </tr>')
    html_parts.append('    </thead>')
    html_parts.append('    <tbody>')
    
    for i, f in enumerate(files, 1):
        ext_clean = (
            f["extension"].replace(".", "").upper()
            if f["extension"] else "TEXT"
        )
        size_kb = f["bytes"] / 1024.0
        html_parts.append('      <tr>')
        html_parts.append(f'        <td>{i}</td>')
        html_parts.append(
            f'        <td><strong>{html.escape(f["rel_path"])}</strong></td>'
        )
        html_parts.append(f'        <td>{ext_clean}</td>')
        html_parts.append(f'        <td>{f["lines"]:,}</td>')
        html_parts.append(f'        <td>{f["words"]:,}</td>')
        html_parts.append(f'        <td>{size_kb:.1f}</td>')
        html_parts.append('      </tr>')
        
    html_parts.append('    </tbody>')
    html_parts.append('  </table>')
    html_parts.append('</div>')
    
    # 3. File Cards
    for f in files:
        html_parts.append(render_file_card_html(f))
        
    html_parts.append('</body>')
    html_parts.append('</html>')
    
    return "\n".join(html_parts)

def convert_html_to_pdf(html_content, output_pdf_path):
    """Renders HTML directly to PDF using Google Chrome headless print."""
    tmp_html = output_pdf_path.with_suffix(".tmp.html")
    with open(tmp_html, "w", encoding="utf-8") as f:
        f.write(html_content)
        
    browser_bin = shutil.which("google-chrome") or shutil.which("chromium")
    if not browser_bin:
        raise RuntimeError("No headless Chrome browser found in PATH.")
        
    cmd = [
        browser_bin,
        "--headless",
        "--disable-gpu",
        "--no-sandbox",
        "--hide-scrollbars",
        f"--print-to-pdf={output_pdf_path}",
        str(tmp_html)
    ]
    
    t0 = time.time()
    res = subprocess.run(
            cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
    t1 = time.time()
    
    if tmp_html.exists():
        tmp_html.unlink()
        
    if res.returncode != 0:
        err = res.stderr.decode('utf-8', errors='ignore')
        raise RuntimeError(
            f"Chrome PDF generation failed (code {res.returncode}): {err}"
        )
        
    pdf_size_kb = output_pdf_path.stat().st_size / 1024.0
    return t1 - t0, pdf_size_kb

def unite_pdfs(input_pdfs, output_pdf_path):
    """Assembles multiple PDF chapter pieces into a unified Master PDF."""
    t0 = time.time()
    if shutil.which("pdfunite"):
        cmd = (
            ["pdfunite"] + [str(p) for p in input_pdfs] +
            [str(output_pdf_path)]
        )
        res = subprocess.run(
            cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        if res.returncode == 0:
            t1 = time.time()
            return t1 - t0, output_pdf_path.stat().st_size / 1024.0
            
    if shutil.which("gs"):
        cmd = [
            "gs", "-dBATCH", "-dNOPAUSE", "-q",
            "-sDEVICE=pdfwrite",
            f"-sOutputFile={output_pdf_path}"
        ] + [str(p) for p in input_pdfs]
        res = subprocess.run(
            cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        if res.returncode == 0:
            t1 = time.time()
            return t1 - t0, output_pdf_path.stat().st_size / 1024.0
            
    raise RuntimeError("Neither pdfunite nor gs was able to unite PDF volumes.")

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Orchestration, Multi-Chapter Execution & Ingestion Manifest )
# ==============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Export OmniCode repository into NotebookLM Chapter PDFs."
    )
    parser.add_argument(
        "--output-dir",
        default=str(OUTPUT_DIR),
        help="Target directory for generated PDF files."
    )
    parser.add_argument(
        "--chapter",
        default=None,
        help="Optional single chapter slug or ID to generate."
    )
    parser.add_argument(
        "--no-master",
        action="store_true",
        help="Skip assembling the complete unified Master PDF."
    )
    
    args = parser.parse_args()
    target_dir = Path(args.output_dir).resolve()
    target_dir.mkdir(parents=True, exist_ok=True)
    
    print("=" * 78)
    print("OMNICODE NOTEBOOKLM CHAPTER EXPORT PIPELINE (19 CANONICAL CHAPTERS)")
    print(f"Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Target Directory: {target_dir}")
    print("=" * 78)
    
    print("\n[1/4] Discovering repository files across all tracks...")
    all_files = discover_repository_files()
    
    total_files = len(all_files)
    total_lines = sum(f["lines"] for f in all_files)
    total_words = sum(f["words"] for f in all_files)
    total_mb = sum(f["bytes"] for f in all_files) / (1024 * 1024)
    
    print(f"  • Total Cataloged Files: {total_files}")
    print(f"  • Total Source Lines:   {total_lines:,}")
    print(f"  • Total Word Count:     {total_words:,} words")
    print(f"  • Total Raw Volume:     {total_mb:.2f} MB")
    
    # Map files by chapter slug
    files_by_slug = {}
    for f in all_files:
        files_by_slug.setdefault(f["chapter_slug"], []).append(f)
        
    # Filter chapters if user requested a specific one
    active_chapters = CHAPTER_DEFINITIONS
    if args.chapter:
        query = args.chapter.lower()
        active_chapters = [
            c for c in CHAPTER_DEFINITIONS
            if query in c["slug"].lower() or query in c["id"].lower()
        ]
        if not active_chapters:
            print(f"[-] Error: No chapter matched query '{args.chapter}'.")
            sys.exit(1)
            
    print(f"\n[2/4] Rendering {len(active_chapters)} discrete Chapter PDFs...")
    generation_manifest = []
    generated_chapter_pdfs = []
    
    for idx, c in enumerate(active_chapters, 1):
        c_id = c["id"]
        c_slug = c["slug"]
        c_file = c["filename"]
        c_title = c["title"]
        c_sub = c["subtitle"]
        out_pdf = target_dir / c_file
        
        c_files = files_by_slug.get(c_slug, [])
        if not c_files:
            continue
            
        words = sum(f["words"] for f in c_files)
        lines = sum(f["lines"] for f in c_files)
        
        print(f"\n--- [{idx:02d}/{len(active_chapters):02d}] {c_file} ---")
        print(f"  • Files: {len(c_files)} | Lines: {lines:,} "
              f"| Words: {words:,}")
        
        html_doc = build_chapter_html(c, c_files)
        duration, size_kb = convert_html_to_pdf(html_doc, out_pdf)
        generated_chapter_pdfs.append(out_pdf)
        
        size_mb = size_kb / 1024.0
        print(
            f"  • Rendered in {duration:.2f}s | "
            f"Output Size: {size_kb:.1f} KB ({size_mb:.2f} MB)"
        )
        
        generation_manifest.append({
            "chapter_id": c_id,
            "slug": c_slug,
            "filename": c_file,
            "path": str(out_pdf),
            "title": c_title,
            "subtitle": c_sub,
            "prompt": c["prompt"],
            "file_count": len(c_files),
            "line_count": lines,
            "word_count": words,
            "size_kb": size_kb,
            "render_seconds": round(duration, 2),
        })
        
    # Optional Master Compilation from Chapter pieces
    if not args.no_master and not args.chapter and generated_chapter_pdfs:
        print("\n--- Assembling Unified Master PDF (All Chapters Combined) ---")
        master_pdf = target_dir / "OmniCode_Complete_Repository_Master.pdf"
        try:
            u_sec, u_size = unite_pdfs(generated_chapter_pdfs, master_pdf)
            print(
                f"  • United {len(generated_chapter_pdfs)} chapters in "
                f"{u_sec:.2f}s | Size: {u_size/1024.0:.2f} MB"
            )
            generation_manifest.insert(0, {
                "chapter_id": "MASTER",
                "slug": "Complete_Repository_Master",
                "filename": "OmniCode_Complete_Repository_Master.pdf",
                "path": str(master_pdf),
                "title": "Complete Repository Master Edition",
                "subtitle": (
                    "Unified Epistemic Corpus: All 19 Canonical Chapters"
                ),
                "prompt": "Synthesize full architecture across all tracks.",
                "file_count": total_files,
                "line_count": total_lines,
                "word_count": total_words,
                "size_kb": u_size,
                "render_seconds": round(u_sec, 2),
            })
        except Exception as e:
            print(f"  [!] Master assembly warning: {e}")
            
    print("\n[3/4] Writing Ingestion Manifests for Google NotebookLM...")
    manifest_md = target_dir / "NOTEBOOKLM_INGESTION_MANIFEST.md"
    manifest_json = target_dir / "notebooklm_manifest.json"
    
    with open(manifest_json, "w", encoding="utf-8") as f:
        json.dump(generation_manifest, f, indent=2)
        
    with open(manifest_md, "w", encoding="utf-8") as f:
        f.write("# OmniCode: Google NotebookLM Chapter Ingestion Manifest\n\n")
        gen_str = datetime.now().strftime('%Y-%m-%d %H:%M:%S')
        f.write(f"**Generated:** {gen_str}  \n")
        f.write(f"**Repository:** `{REPO_ROOT}`  \n")
        f.write(
            f"**Total Cataloged Files:** {total_files} | "
            f"**Lines:** {total_lines:,} | **Words:** {total_words:,}  \n\n"
        )
        
        f.write("## Scripture Witnesses & Invariants\n")
        f.write("- **Exodus 31:18**: *\"written with the finger of God\"*\n")
        f.write(
            "- **Proverbs 24:27**: *\"Prepare thy work without, and make "
            "it fit for thyself in the field; and afterwards build thine "
            "house.\"*\n"
        )
        f.write(
            "- **Proverbs 22:28**: *\"Remove not the ancient landmark, "
            "which thy fathers have set.\"*\n\n"
        )
        
        f.write("## Chapter Curriculum Map for AI Model Training\n\n")
        f.write(
            "| Ch | PDF Filename | Files | Lines | Words | Size | Prompt |\n"
        )
        f.write(
            "|---|---|---|---|---|---|---|\n"
        )
        for m in generation_manifest:
            size_str = f"{m['size_kb']/1024.0:.2f} MB"
            f.write(
                f"| **{m['chapter_id']}** | `{m['filename']}` | "
                f"{m['file_count']} | {m['line_count']:,} | "
                f"{m['word_count']:,} | {size_str} | {m['prompt']} |\n"
            )
            
        f.write("\n## How to Ingest into Google NotebookLM\n\n")
        f.write(
            "1. Open **[NotebookLM](https://notebooklm.google.com)** "
            "with your Google AI account.\n"
        )
        f.write(
            "2. Create a new notebook named "
            "**`OmniCode Operating System & Language`**.\n"
        )
        f.write(
            "3. **Modular Chapter Ingestion (Recommended):** Upload the "
            "individual Chapter PDFs (`OmniCode_Ch01_*.pdf` through "
            "`OmniCode_Ch19_*.pdf`). Each chapter is bounded (10-35 pages), "
            "enabling high-resolution grounding and precise citation without "
            "needle-in-a-haystack token dilution.\n"
        )
        f.write(
            "4. **Unified Master Upload:** Alternatively, upload "
            "`OmniCode_Complete_Repository_Master.pdf` for cross-cutting "
            "repository-wide queries.\n"
        )
        
    print(f"  • Markdown Manifest: {manifest_md}")
    print(f"  • JSON Manifest:     {manifest_json}")
    
    print("\n[4/4] Chapter Pipeline Completed Successfully!")
    print("=" * 78)

if __name__ == "__main__":
    main()
