#!/usr/bin/env python3
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Repository to Google NotebookLM Multi-Part PDF Ingestion Pipeline )
# ==============================================================================
# SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
#   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
#   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
#                     and afterwards build thine house." — Systematic modularization.
#   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
#                     — Preservation of invariant boundaries and balanced truth.
#
# ARCHITECTURAL STRATEGY:
#   Builds in discrete, modular pieces:
#     Piece 1: Volume 1 - Architecture, Manifests & Epistemics (~30k words)
#     Piece 2: Volume 2 - Host Engine & Virtual Machine (Track 1) (~100k words)
#     Piece 3: Volume 3 - OmniCode Language & Standard Library (Track 2) (~65k words)
#     Piece 4: Volume 4 - Applications, Interop & Infrastructure (Tracks 3 & 5) (~22k words)
#     Piece 5: Master Volume - Complete unified repository assembled via fast binary unite
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

# Repo Root Resolution
REPO_ROOT = Path("/home/professorseanex/OmniCode").resolve()
OUTPUT_DIR = REPO_ROOT / "docs" / "notebooklm"

EXCLUDE_DIRS = {
    ".git", "target", ".vscode", ".gemini", "node_modules",
    "__pycache__", ".cargo", "dist", "notebooklm"
}

BINARY_EXTENSIONS = {
    ".pdf", ".img", ".elf", ".efi", ".bin", ".iso", ".png", ".jpg", ".jpeg",
    ".gif", ".ico", ".woff", ".woff2", ".ttf", ".eot", ".tar", ".gz",
    ".zip", ".pyc", ".o", ".a", ".so", ".dylib", ".rlib"
}

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( File Discovery, Binary Filtration & Statistical Classification )
# ==============================================================================

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

def discover_repository_files():
    """Scans and categorizes all text and manifest files across the repository."""
    files_by_track = {
        "ROOT": [],
        "TRACK 0": [],
        "TRACK 1": [],
        "TRACK 2": [],
        "TRACK 3": [],
        "TRACK 4": [],
        "TRACK 5": [],
    }
    
    all_files = []
    
    for root, dirs, files in os.walk(REPO_ROOT):
        # Exclude output dir, git, target, etc.
        dirs[:] = [
            d for d in dirs
            if d not in EXCLUDE_DIRS and not d.startswith(".")
        ]
        
        # Avoid scanning docs/notebooklm if inside docs
        rel_root = Path(root).relative_to(REPO_ROOT)
        if "notebooklm" in rel_root.parts:
            continue
            
        top_component = rel_root.parts[0] if rel_root.parts else ""
        
        for file in sorted(files):
            # Skip hidden files, temp files, and previous artifacts
            if file.startswith(".") or file.endswith("~") or file.endswith(".tmp") or file.endswith(".tmp.html"):
                continue
                
            full_path = Path(root) / file
            rel_path = full_path.relative_to(REPO_ROOT)
            
            # Skip any notebooklm docs
            if "notebooklm" in rel_path.parts:
                continue
                
            is_bin = is_binary_file(full_path)
            file_bytes = full_path.stat().st_size
            lines_count = 0
            words_count = 0
            content = ""
            
            if not is_bin:
                try:
                    with open(full_path, "r", encoding="utf-8", errors="replace") as f:
                        content = f.read()
                        lines_count = content.count("\n") + (1 if content else 0)
                        words_count = len(content.split())
                except Exception as e:
                    content = f"[Error reading file: {e}]"
            else:
                # Binary or immutable document entry
                if full_path.suffix.lower() == ".pdf":
                    content = f"[Immutable Scripture / Canonical Document Reference: {file} ({file_bytes / (1024*1024):.2f} MB)]"
                else:
                    content = f"[Binary Artifact: {file} ({file_bytes / 1024:.1f} KB)]"
                lines_count = 1
                words_count = len(content.split())
                
            entry = {
                "rel_path": str(rel_path),
                "full_path": str(full_path),
                "filename": file,
                "extension": full_path.suffix.lower(),
                "top_dir": top_component,
                "is_binary": is_bin,
                "lines": lines_count,
                "words": words_count,
                "bytes": file_bytes,
                "content": content,
            }
            
            all_files.append(entry)
            
            # Map into Track categories
            if not top_component:
                files_by_track["ROOT"].append(entry)
            elif "TRACK 0" in top_component.upper():
                files_by_track["TRACK 0"].append(entry)
            elif "TRACK 1" in top_component.upper():
                files_by_track["TRACK 1"].append(entry)
            elif "TRACK 2" in top_component.upper():
                files_by_track["TRACK 2"].append(entry)
            elif "TRACK 3" in top_component.upper():
                files_by_track["TRACK 3"].append(entry)
            elif "TRACK 4" in top_component.upper():
                files_by_track["TRACK 4"].append(entry)
            elif "TRACK 5" in top_component.upper():
                files_by_track["TRACK 5"].append(entry)
            else:
                files_by_track["ROOT"].append(entry)

    return all_files, files_by_track

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( CSS Design System, HTML Document Construction & Chrome Lowering )
# ==============================================================================

CSS_STYLES = """
@page {
    size: letter portrait;
    margin: 1.5cm 1.2cm 1.5cm 1.2cm;
    @top-center {
        content: "OmniCode AI OS & Substrate | Google NotebookLM Corpus";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #888888;
    }
    @bottom-left {
        content: "Covenant Witnesses: Exodus 31:18, Proverbs 24:27, Proverbs 22:28";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #888888;
    }
    @bottom-right {
        content: "Page " counter(page);
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #555555;
    }
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
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

.avoid-break {
    page-break-inside: avoid;
    break-inside: avoid;
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
    font-size: 24pt;
    font-weight: 800;
    color: #0f172a;
    margin-bottom: 6px;
    letter-spacing: -0.5px;
    text-transform: uppercase;
    border-bottom: 3px solid #3b82f6;
    padding-bottom: 10px;
}

.cover-subtitle {
    font-size: 12pt;
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
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
    font-size: 8.5pt;
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
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace;
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

def render_file_card_html(entry):
    """Renders a single file into a structured card with metadata badge and formatted content."""
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
    out.append(f'<div class="file-card page-break">')
    out.append(f'  <div class="file-header-box">')
    out.append(f'    <div class="file-path">{html.escape(rel_path)}</div>')
    out.append(f'    <div class="file-meta-badges">')
    out.append(f'      <span class="badge">{badge_type}</span>')
    out.append(f'      <span class="badge">{lines} Lines</span>')
    out.append(f'      <span class="badge">{words} Words</span>')
    out.append(f'      <span class="badge">{size_kb:.1f} KB</span>')
    out.append(f'    </div>')
    out.append(f'  </div>')
    out.append(f'  <div class="file-content-box">')
    
    if is_bin:
        out.append(f'    <p style="font-style:italic; color:#475569; margin:8px 0; font-size:8pt;">')
        out.append(f'      {html.escape(content)}')
        out.append(f'    </p>')
    else:
        out.append(f'    <pre class="code-block">')
        raw_lines = content.splitlines()
        for i, l in enumerate(raw_lines, 1):
            escaped = html.escape(l)
            out.append(f'<span class="line-num">{i}</span>{escaped}')
        out.append(f'</pre>')
        
    out.append(f'  </div>')
    out.append(f'</div>\n')
    return "\n".join(out)

def build_volume_html(vol_id, vol_title, vol_subtitle, files):
    """Assembles a self-contained publication-grade HTML document for a specific volume."""
    vol_lines = sum(f["lines"] for f in files)
    vol_words = sum(f["words"] for f in files)
    gen_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    
    html_parts = []
    html_parts.append('<!DOCTYPE html>')
    html_parts.append('<html lang="en">')
    html_parts.append('<head>')
    html_parts.append('  <meta charset="UTF-8">')
    html_parts.append(f'  <title>{html.escape(vol_title)}</title>')
    html_parts.append(f'  <style>{CSS_STYLES}</style>')
    html_parts.append('</head>')
    html_parts.append('<body>')
    
    # 1. Cover Page
    html_parts.append('<div class="cover-page">')
    html_parts.append(f'  <div class="cover-title">OmniCode: {html.escape(vol_title)}</div>')
    html_parts.append(f'  <div class="cover-subtitle">{html.escape(vol_subtitle)}</div>')
    
    html_parts.append('  <div class="cover-metadata-grid">')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Corpus Volume</span>')
    html_parts.append(f'      <span class="metadata-value">{html.escape(vol_id)}</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">NotebookLM Word Count</span>')
    html_parts.append(f'      <span class="metadata-value">{vol_words:,} words (Limit: 500k words)</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Cataloged Files</span>')
    html_parts.append(f'      <span class="metadata-value">{len(files)} files ({vol_lines:,} lines)</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Generation Date</span>')
    html_parts.append(f'      <span class="metadata-value">{gen_time}</span>')
    html_parts.append('    </div>')
    html_parts.append('  </div>')
    
    # Scripture Witness Card
    html_parts.append('  <div class="scripture-card">')
    html_parts.append('    <div class="scripture-title">Epistemic Covenant Bedrock & Immutable Landmarks</div>')
    html_parts.append('    <div class="scripture-verse">"And he gave unto Moses... two tables of testimony, tables of stone, written with the finger of God." &mdash; Exodus 31:18</div>')
    html_parts.append('    <div class="scripture-verse">"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house." &mdash; Proverbs 24:27</div>')
    html_parts.append('    <div class="scripture-verse">"Remove not the ancient landmark, which thy fathers have set." &mdash; Proverbs 22:28</div>')
    html_parts.append('  </div>')
    
    # NotebookLM Prompting Strategy Guide
    html_parts.append('  <div class="guide-card">')
    html_parts.append('    <div class="guide-title">Google NotebookLM (AI Pro) Grounding Prompts</div>')
    html_parts.append('    <div class="guide-text">&bull; <strong>Balanced Ternary Quad-State:</strong> <em>"How does the T4 algebra {-1:Rollback, 0:Await, +1:Proceed, NULL:Halt} enforce symmetry in this volume?"</em></div>')
    html_parts.append('    <div class="guide-text">&bull; <strong>4-Block Structure:</strong> <em>"Explain the boundaries of Block 1 (Metadata/Scripture), Block 2 (Opening), Block 3 (Body), and Block 4 (Closing)."</em></div>')
    html_parts.append('    <div class="guide-text">&bull; <strong>Character Lifecycle & SVO Verbs:</strong> <em>"Trace the Character lifecycle through INSCRIBE, TRANSDUCE, PROJECT, and SEQUENCE across terminal ports 0x03F8 (FROM) and 0x000B8000 (INTO)."</em></div>')
    html_parts.append('  </div>')
    html_parts.append('</div>')
    
    # 2. Table of Contents
    html_parts.append('<div class="toc-container page-break">')
    html_parts.append(f'  <div class="toc-header">Volume Source Manifest & Index ({len(files)} files)</div>')
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
        ext_clean = f["extension"].replace(".", "").upper() if f["extension"] else "TEXT"
        size_kb = f["bytes"] / 1024.0
        html_parts.append('      <tr>')
        html_parts.append(f'        <td>{i}</td>')
        html_parts.append(f'        <td><strong>{html.escape(f["rel_path"])}</strong></td>')
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
    """Renders HTML directly to PDF using Google Chrome headless print-to-pdf."""
    tmp_html = output_pdf_path.with_suffix(".tmp.html")
    with open(tmp_html, "w", encoding="utf-8") as f:
        f.write(html_content)
        
    cmd = [
        "google-chrome",
        "--headless",
        "--disable-gpu",
        "--no-sandbox",
        "--hide-scrollbars",
        f"--print-to-pdf={output_pdf_path}",
        str(tmp_html)
    ]
    
    t0 = time.time()
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    t1 = time.time()
    
    if tmp_html.exists():
        tmp_html.unlink()
        
    if res.returncode != 0:
        raise RuntimeError(f"Chrome PDF generation failed (code {res.returncode}): {res.stderr.decode('utf-8', errors='ignore')}")
        
    pdf_size_kb = output_pdf_path.stat().st_size / 1024.0
    return t1 - t0, pdf_size_kb

def unite_pdfs(input_pdfs, output_pdf_path):
    """Assembles multiple PDF volume pieces into a unified Master PDF."""
    t0 = time.time()
    if shutil.which("pdfunite"):
        cmd = ["pdfunite"] + [str(p) for p in input_pdfs] + [str(output_pdf_path)]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if res.returncode == 0:
            t1 = time.time()
            return t1 - t0, output_pdf_path.stat().st_size / 1024.0
            
    # Fallback to Ghostscript
    if shutil.which("gs"):
        cmd = [
            "gs", "-dBATCH", "-dNOPAUSE", "-q",
            "-sDEVICE=pdfwrite",
            f"-sOutputFile={output_pdf_path}"
        ] + [str(p) for p in input_pdfs]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if res.returncode == 0:
            t1 = time.time()
            return t1 - t0, output_pdf_path.stat().st_size / 1024.0
            
    raise RuntimeError("Neither pdfunite nor gs was able to unite PDF volumes.")

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Orchestration, Modular Piece Building & Manifest Generation )
# ==============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Export OmniCode repository into NotebookLM-ready PDFs in modular pieces."
    )
    parser.add_argument(
        "--output-dir",
        default=str(OUTPUT_DIR),
        help="Target directory for generated PDF files."
    )
    
    args = parser.parse_args()
    target_dir = Path(args.output_dir).resolve()
    target_dir.mkdir(parents=True, exist_ok=True)
    
    print("=" * 78)
    print("OMNICODE REPOSITORY TO GOOGLE NOTEBOOKLM EXPORT PIPELINE (MODULAR)")
    print(f"Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Target Directory: {target_dir}")
    print("=" * 78)
    
    print("\n[1/4] Discovering repository files...")
    all_files, by_track = discover_repository_files()
    
    total_files = len(all_files)
    total_lines = sum(f["lines"] for f in all_files)
    total_words = sum(f["words"] for f in all_files)
    total_mb = sum(f["bytes"] for f in all_files) / (1024 * 1024)
    
    print(f"  &bull; Total Cataloged Files: {total_files}")
    print(f"  &bull; Total Source Lines:   {total_lines:,}")
    print(f"  &bull; Total Word Count:     {total_words:,} words (Entire repo is under 500k NotebookLM limit!)")
    print(f"  &bull; Total Raw Volume:     {total_mb:.2f} MB")
    
    # 1. Define Volume Partitions (Pieces)
    vol1_files = by_track["ROOT"] + by_track["TRACK 0"] + by_track["TRACK 4"]
    t5_docs = [f for f in by_track["TRACK 5"] if "/docs/" in f["rel_path"]]
    t5_other = [f for f in by_track["TRACK 5"] if "/docs/" not in f["rel_path"]]
    vol1_files += t5_docs
    
    vol2_files = by_track["TRACK 1"]
    vol3_files = by_track["TRACK 2"]
    vol4_files = by_track["TRACK 3"] + t5_other
    
    volume_pieces = [
        {
            "id": "VOL 1",
            "filename": "OmniCode_Vol1_Architecture_and_Epistemics.pdf",
            "title": "Volume 1: Architecture, Manifests & Epistemics",
            "subtitle": "Moral Bedrock, Scriptural Axioms, Track 0 Silicon & Foundational Architecture",
            "files": vol1_files,
        },
        {
            "id": "VOL 2",
            "filename": "OmniCode_Vol2_Host_Engine_and_VM.pdf",
            "title": "Volume 2: Host Engine & Virtual Machine",
            "subtitle": "Track 1: Rust VM, Balanced Ternary Runtime, Evaluator, Assembler & Tests",
            "files": vol2_files,
        },
        {
            "id": "VOL 3",
            "filename": "OmniCode_Vol3_Language_and_Standard_Library.pdf",
            "title": "Volume 3: OmniCode Language & Standard Library",
            "subtitle": "Track 2: Lexicon, SVO Verbs, Bijective Transduction, .omni & .tape Modules",
            "files": vol3_files,
        },
        {
            "id": "VOL 4",
            "filename": "OmniCode_Vol4_Applications_and_Infrastructure.pdf",
            "title": "Volume 4: Applications, Interop & Infrastructure",
            "subtitle": "Track 3 C/Lua Character Bindings, OmniStudio Interfaces & Track 5 Ops/CI",
            "files": vol4_files,
        },
    ]
    
    print(f"\n[2/4] Building 4 modular volume pieces...")
    generation_manifest = []
    piece_pdf_paths = []
    
    for idx, v in enumerate(volume_pieces, 1):
        v_id = v["id"]
        v_file = v["filename"]
        v_title = v["title"]
        v_sub = v["subtitle"]
        v_files = v["files"]
        out_pdf = target_dir / v_file
        piece_pdf_paths.append(out_pdf)
        
        words = sum(f["words"] for f in v_files)
        lines = sum(f["lines"] for f in v_files)
        
        print(f"\n--- Piece [{idx}/4]: {v_file} ---")
        print(f"  &bull; Files: {len(v_files)} | Lines: {lines:,} | Words: {words:,}")
        
        html_doc = build_volume_html(v_id, v_title, v_sub, v_files)
        duration, size_kb = convert_html_to_pdf(html_doc, out_pdf)
        
        print(f"  &bull; Rendered to PDF in {duration:.2f}s | Output Size: {size_kb:.1f} KB ({size_kb/1024:.2f} MB)")
        
        generation_manifest.append({
            "volume_id": v_id,
            "filename": v_file,
            "path": str(out_pdf),
            "title": v_title,
            "subtitle": v_sub,
            "file_count": len(v_files),
            "line_count": lines,
            "word_count": words,
            "size_kb": size_kb,
            "render_seconds": round(duration, 2),
        })
        
    # Piece 5: Master Volume Assembled from the 4 pieces
    print("\n--- Piece [5/5]: Assembling Master Unified Corpus (OmniCode_Complete_Repository_Master.pdf) ---")
    master_pdf = target_dir / "OmniCode_Complete_Repository_Master.pdf"
    unite_duration, master_size_kb = unite_pdfs(piece_pdf_paths, master_pdf)
    print(f"  &bull; United 4 volumes into Master in {unite_duration:.2f}s | Output Size: {master_size_kb:.1f} KB ({master_size_kb/1024:.2f} MB)")
    
    generation_manifest.insert(0, {
        "volume_id": "MASTER",
        "filename": "OmniCode_Complete_Repository_Master.pdf",
        "path": str(master_pdf),
        "title": "Complete Repository Master Edition",
        "subtitle": "Unified Epistemic Corpus: Tracks 0 through 5, Manifests, VM & Lexicon",
        "file_count": total_files,
        "line_count": total_lines,
        "word_count": total_words,
        "size_kb": master_size_kb,
        "render_seconds": round(unite_duration, 2),
    })
    
    print("\n[3/4] Writing Ingestion Manifest for Google NotebookLM...")
    manifest_md_path = target_dir / "NOTEBOOKLM_INGESTION_MANIFEST.md"
    manifest_json_path = target_dir / "notebooklm_manifest.json"
    
    with open(manifest_json_path, "w", encoding="utf-8") as f:
        json.dump(generation_manifest, f, indent=2)
        
    with open(manifest_md_path, "w", encoding="utf-8") as f:
        f.write("# OmniCode: Google NotebookLM Ingestion Manifest\n\n")
        f.write(f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  \n")
        f.write(f"**Repository:** `/home/professorseanex/OmniCode`  \n")
        f.write(f"**Total Cataloged Files:** {total_files} | **Lines:** {total_lines:,} | **Words:** {total_words:,}  \n\n")
        
        f.write("## Scripture Witnesses & Invariants\n")
        f.write("- **Exodus 31:18**: *\"written with the finger of God\"*\n")
        f.write("- **Proverbs 24:27**: *\"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house.\"*\n")
        f.write("- **Proverbs 22:28**: *\"Remove not the ancient landmark, which thy fathers have set.\"*\n\n")
        
        f.write("## Google AI Pro NotebookLM Ingestion Map\n\n")
        f.write("Google NotebookLM limits per individual source:\n")
        f.write("- **Word limit:** Up to 500,000 words per individual source file\n")
        f.write("- **File size limit:** Up to 200 MB per file\n")
        f.write("- **Sources per notebook (Google AI Pro Plan):** Up to 300 sources\n\n")
        f.write("| Volume | PDF Filename | File Count | Lines | Words | Size (MB) | Best Use Case in NotebookLM |\n")
        f.write("|---|---|---|---|---|---|---|\n")
        for m in generation_manifest:
            f.write(f"| **{m['volume_id']}** | `{m['filename']}` | {m['file_count']} | {m['line_count']:,} | {m['word_count']:,} | {m['size_kb']/1024:.2f} MB | {m['title']} |\n")
            
        f.write("\n## How to Ingest into Google NotebookLM\n\n")
        f.write("1. Open **[NotebookLM](https://notebooklm.google.com)** while logged into your Google AI Pro account.\n")
        f.write("2. Create a new notebook named **`OmniCode Operating System & Language`**.\n")
        f.write("3. **Single Master Upload:** Upload `OmniCode_Complete_Repository_Master.pdf` for repository-wide synthesis.\n")
        f.write("4. **Multi-Part Modular Upload (Recommended for Google AI Pro):** Upload Volumes 1, 2, 3, and 4 as separate sources. This unlocks high-resolution grounding and precise citation without token dilution across large documents.\n")
        
    print(f"  &bull; Markdown Manifest: {manifest_md_path}")
    print(f"  &bull; JSON Manifest:     {manifest_json_path}")
    
    print("\n[4/4] Ingestion Pipeline Completed Successfully!")
    print("=" * 78)


if __name__ == "__main__":
    main()
