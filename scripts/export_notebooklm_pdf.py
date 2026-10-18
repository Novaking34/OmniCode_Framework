#!/usr/bin/env python3
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Multi-Volume Repository & Chapter PDF Manual Generator )
# ==============================================================================
# SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
#   - Exodus 31:18: "written with the finger of God"
#     Inscription of eternal moral truth upon stone.
#   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself"
#     Systematic modularization, track-by-track volumes, and chapter-based order.
#   - Proverbs 22:28: "Remove not the ancient landmark"
#     Preservation of invariant boundaries across all tracks and manual volumes.
#   - Colossians 2:19: "From whom the whole body fitly joined together"
#     Harmonious integration of systems, subsystems, and documentation manuals.
#
# ARCHITECTURAL STRATEGY:
#   Organizes the entire OmniCode repository into a two-tier publication hierarchy:
#     1. VOLUMES: Track-by-track standalone operational manuals (Vol 00 to Vol 06).
#     2. CHAPTERS: Thematic, bounded subject modules (Ch 00 to Ch 24) optimized
#        for Google NotebookLM (10-35 pages) to prevent token dilution.
#     3. MASTER OMNIBUS: Unified master reference uniting all volumes.
#
# VOLUME MAPPING:
#   - Volume 00: Foundations, Manifest & Ontological Bedrock (Root & Ontological)
#   - Volume 01: Track 0 - Silicon & Bare-Metal Hardware Kernel Manual
#   - Volume 02: Track 1 - Host Engine & Virtual Machine Manual
#   - Volume 03: Track 2 - OmniCode Language & Standard Library Manual
#   - Volume 04: Track 3 - Applications & OmniStudio Interfaces Manual
#   - Volume 05: Track 4 - Variable Storage & Episodic Memory Manual
#   - Volume 06: Track 5 - Infrastructure, Operations & Archives Manual
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

# Optional markdown library for rich rendering of READMEs and design documents
try:
    import markdown
    HAS_MARKDOWN = True
except ImportError:
    HAS_MARKDOWN = False

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Configuration Constants, Volume & Chapter Definitions & CSS Styles )
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

VOLUME_DEFINITIONS = [
    {
        "id": "VOL 00",
        "slug": "Vol00_Foundations_and_Manifest",
        "filename": "OmniCode_Vol00_Foundations_and_Manifest.pdf",
        "title": "Volume 00: Foundations, Manifest & Ontological Bedrock",
        "track": "Root & Ontological Foundations",
        "subtitle": "Moral Bedrock, Exodus 20 Decalogue, Manifest & Ontological Principles",
        "witness": "Exodus 31:18 (KJV) - Written with the finger of God",
        "description": "Foundational bedrock documents, root system manifests, packaging configurations, and ontological boundaries of computation.",
        "chapters": ["CH 00", "CH 01"],
    },
    {
        "id": "VOL 01",
        "slug": "Vol01_Track0_Silicon_Kernel_Manual",
        "filename": "OmniCode_Vol01_Track0_Silicon_Kernel_Manual.pdf",
        "title": "Volume 01: Track 0 - Silicon & Bare-Metal Hardware Kernel Manual",
        "track": "<TRACK 0> : {Silicon & Bare-Metal Kernel}",
        "subtitle": "Bare-Metal Boot, BIOS MBR, UEFI Long Mode, Multiboot & Alternate States",
        "witness": "Job 38:5 (KJV) - Who hath laid the measures thereof",
        "description": "Hardware inception ladder, 16-bit real mode BIOS, 32-bit protected mode CPI-SI substrate, 64-bit native UEFI PE32+, multiboot hypervisor, and safe mode cascades.",
        "chapters": ["CH 02", "CH 03"],
    },
    {
        "id": "VOL 02",
        "slug": "Vol02_Track1_Host_Engine_Manual",
        "filename": "OmniCode_Vol02_Track1_Host_Engine_Manual.pdf",
        "title": "Volume 02: Track 1 - Host Engine & Virtual Machine Manual",
        "track": "<TRACK 1> : {Host Engine & Virtual Machine}",
        "subtitle": "Host Runtime, AST, AI Cortex, Causal Clock, Bytecode Lowering & Diagnostics",
        "witness": "Colossians 2:19 (KJV) - The whole body fitly joined together",
        "description": "Complete implementation of Track 1: Abstract Syntax Tree, process tape ribbon, neural-symbolic AI cortex, runtime evaluator, bytecode assembler, linter, logger, restorator, terminal, and verification suites.",
        "chapters": ["CH 04", "CH 05", "CH 06", "CH 07", "CH 08", "CH 09", "CH 10", "CH 11", "CH 12", "CH 13"],
    },
    {
        "id": "VOL 03",
        "slug": "Vol03_Track2_Language_and_StdLib_Manual",
        "filename": "OmniCode_Vol03_Track2_Language_and_StdLib_Manual.pdf",
        "title": "Volume 03: Track 2 - OmniCode Language & Standard Library Manual",
        "track": "<TRACK 2> : {OmniCode Language & Standard Library}",
        "subtitle": "Grammar, Parts of Speech, Primitive Lexicons & Core Standard Libraries",
        "witness": "1 Corinthians 14:40 (KJV) - Let all things be done decently and in order",
        "description": "Formal language specification: Character, Number, State, Containers, Systems, Verbs, Modifiers, Connectives, and Standard Library modules.",
        "chapters": ["CH 14", "CH 15", "CH 16", "CH 17", "CH 18", "CH 19", "CH 20"],
    },
    {
        "id": "VOL 04",
        "slug": "Vol04_Track3_Applications_Manual",
        "filename": "OmniCode_Vol04_Track3_Applications_Manual.pdf",
        "title": "Volume 04: Track 3 - Applications & OmniStudio Interfaces Manual",
        "track": "<TRACK 3> : {Applications & OmniStudio Interfaces}",
        "subtitle": "Language Bridges, C99/Lua Environments, IDE Extensions & Studio HUD",
        "witness": "Proverbs 24:27 (KJV) - Prepare thy work without, and make it fit for thyself",
        "description": "User-facing applications, language bridges to C and Lua, VSCode editor integrations, and the OmniStudio bare-metal graphical projection HUD.",
        "chapters": ["CH 21"],
    },
    {
        "id": "VOL 05",
        "slug": "Vol05_Track4_Variable_Storage_Manual",
        "filename": "OmniCode_Vol05_Track4_Variable_Storage_Manual.pdf",
        "title": "Volume 05: Track 4 - Variable Storage & Episodic Memory Manual",
        "track": "<TRACK 4> : {Variable Storage & Episodic Memory}",
        "subtitle": "Immutable Bibles, Episodic Scrolls, Neural Weights & Persistent Storage",
        "witness": "Psalm 119:89 (KJV) - For ever, O LORD, thy word is settled in heaven",
        "description": "Data storage layer: immutable scriptural canons, episodic event scrolls, long-term neural memory weights, and variable state persistence.",
        "chapters": ["CH 22"],
    },
    {
        "id": "VOL 06",
        "slug": "Vol06_Track5_Infrastructure_Manual",
        "filename": "OmniCode_Vol06_Track5_Infrastructure_Manual.pdf",
        "title": "Volume 06: Track 5 - Infrastructure, Operations & Archives Manual",
        "track": "<TRACK 5> : {Infrastructure, Operations & Archives}",
        "subtitle": "Systems Architecture, Logic & Algebra, CI/CD, Containerization & Operations",
        "witness": "Proverbs 3:19 (KJV) - The LORD by wisdom hath founded the earth",
        "description": "Infrastructure engineering: logic and algebraic specifications, four-block architectures, autonomous restorative agents, build toolchains, and deployment archives.",
        "chapters": ["CH 23", "CH 24"],
    },
]

CHAPTER_DEFINITIONS = [
    {
        "id": "CH 00",
        "volume_id": "VOL 00",
        "slug": "Ch00_Decalogue_and_Root_Manifest",
        "filename": "OmniCode_Ch00_Decalogue_and_Root_Manifest.pdf",
        "title": "Chapter 0: Foundations, Root Manifest & Decalogue Bedrock",
        "subtitle": "Moral Bedrock, Exodus 20 Decalogue, Root Manifest & System Configuration",
        "prompt": "How does the Exodus 20 Decalogue govern the AI cortex and immutable constraints of OmniCode?",
    },
    {
        "id": "CH 01",
        "volume_id": "VOL 00",
        "slug": "Ch01_Ontological_Principles_and_Boundaries",
        "filename": "OmniCode_Ch01_Ontological_Principles_and_Boundaries.pdf",
        "title": "Chapter 1: Ontological Principles, Boundaries & Parts of Speech",
        "subtitle": "Ontological Foundations, Asymmetric Boundary Algebra & Parts of Speech Matrix",
        "prompt": "Explain the philosophical and mathematical difference between BottomUp emergence [ } and TopDown imposition { ].",
    },
    {
        "id": "CH 02",
        "volume_id": "VOL 01",
        "slug": "Ch02_Track0_Bare_Metal_Substrate_and_BIOS",
        "filename": "OmniCode_Ch02_Track0_Bare_Metal_Substrate_and_BIOS.pdf",
        "title": "Chapter 2: Track 0 - Bare-Metal Substrate & BIOS Inception",
        "subtitle": "16-Bit Real Mode MBR, Stage 2 Protected Mode, CPI-SI Substrate & Alternate States",
        "prompt": "Trace how the Genesis Boot sequence establishes protected mode and mounts the CPI-SI substrate.",
    },
    {
        "id": "CH 03",
        "volume_id": "VOL 01",
        "slug": "Ch03_Track0_UEFI_and_Multiboot_Hypervisor",
        "filename": "OmniCode_Ch03_Track0_UEFI_and_Multiboot_Hypervisor.pdf",
        "title": "Chapter 3: Track 0 - UEFI Long Mode & Multiboot Hypervisor",
        "subtitle": "64-Bit Native UEFI PE32+, OVMF Firmware, Direct Hypervisor Handshake & Telemetry",
        "prompt": "How does the native 64-bit UEFI application verify the balanced (+1)+(-1)=0 equilibrium in bare-metal RAM?",
    },
    {
        "id": "CH 04",
        "volume_id": "VOL 02",
        "slug": "Ch04_Track1_Core_and_Process_Tape_Ribbon",
        "filename": "OmniCode_Ch04_Track1_Core_and_Process_Tape_Ribbon.pdf",
        "title": "Chapter 4: Track 1 - Core Types & Process Tape Ribbon",
        "subtitle": "TapeHead 5-Ws Ribbon, Cell, Word, Reels, Registers & System Entry Points",
        "prompt": "Explain how the TapeHead coordinates Who, What, Where, When, Why, and How across causal time.",
    },
    {
        "id": "CH 05",
        "volume_id": "VOL 02",
        "slug": "Ch05_Track1_Parser_and_AST",
        "filename": "OmniCode_Ch05_Track1_Parser_and_AST.pdf",
        "title": "Chapter 5: Track 1 - Abstract Syntax Tree & Parser",
        "subtitle": "Lexical Tokens, Grammar Rules, AST Entities, Invariant Asides & Span Tracking",
        "prompt": "How does the OmniCode parser preserve first-class intent asides without polluting execution semantics?",
    },
    {
        "id": "CH 06",
        "volume_id": "VOL 02",
        "slug": "Ch06_Track1_AI_Cortex_and_Neural_Engine",
        "filename": "OmniCode_Ch06_Track1_AI_Cortex_and_Neural_Engine.pdf",
        "title": "Chapter 6: Track 1 - AI Cortex & Neural Learning Engine",
        "subtitle": "Cognitive Loop, Synapses, Episodic Learner, Transducer & Non-Hallucinatory Reflection",
        "prompt": "How does the AI Cortex reconcile probabilistic deliberations with immutable biblical truth?",
    },
    {
        "id": "CH 07",
        "volume_id": "VOL 02",
        "slug": "Ch07_Track1_Runtime_and_Evaluator",
        "filename": "OmniCode_Ch07_Track1_Runtime_and_Evaluator.pdf",
        "title": "Chapter 7: Track 1 - Runtime Environment & Evaluator",
        "subtitle": "SVO Verbs, Causal Clock, Monotonic Time, Transactions & Rollback Semantics",
        "prompt": "How does the runtime environment guarantee that causal clock ticks move strictly forward during rollback?",
    },
    {
        "id": "CH 08",
        "volume_id": "VOL 02",
        "slug": "Ch08_Track1_Bytecode_Assembler",
        "filename": "OmniCode_Ch08_Track1_Bytecode_Assembler.pdf",
        "title": "Chapter 8: Track 1 - Bytecode Assembler & Lowering",
        "subtitle": "Instruction Encodings, Lowering Lexicon Tapes to Machine Words, Baton Passes",
        "prompt": "Trace the lowering pipeline from high-level process tape rolls to 64-bit machine instruction words.",
    },
    {
        "id": "CH 09",
        "volume_id": "VOL 02",
        "slug": "Ch09_Track1_Diagnostics_Linter",
        "filename": "OmniCode_Ch09_Track1_Diagnostics_Linter.pdf",
        "title": "Chapter 9: Track 1 - Diagnostics & 4-Block Linter Subsystem",
        "subtitle": "Preflight Boundary Audits, 4-Block Slicing, SVO Syntax, Scaffolding & QuadState Gates",
        "prompt": "Why is sensory telemetry logging inside the linter essential for the autonomic DAR self-healing loop?",
    },
    {
        "id": "CH 10",
        "volume_id": "VOL 02",
        "slug": "Ch10_Track1_Diagnostics_Logger_and_Query",
        "filename": "OmniCode_Ch10_Track1_Diagnostics_Logger_and_Query.pdf",
        "title": "Chapter 10: Track 1 - Diagnostics Logger & Query Engine",
        "subtitle": "Universal Bus, Dual-Rail Failpoints, Inscriber Scrolls, Flight Recorder & Query Engine",
        "prompt": "How does the query engine index episodic memory notes to guide automated developer diagnostics?",
    },
    {
        "id": "CH 11",
        "volume_id": "VOL 02",
        "slug": "Ch11_Track1_Restorator_and_Debugger",
        "filename": "OmniCode_Ch11_Track1_Restorator_and_Debugger.pdf",
        "title": "Chapter 11: Track 1 - Restorator & Debugger Autonomic Self-Healing",
        "subtitle": "DAR (Detect, Assess, Restore) Closed Loop, Test Oracles, Patch Synthesis & Auto-Commit",
        "prompt": "Explain how the Restorator consumes failpoint remediations to heal broken source delimiters autonomously.",
    },
    {
        "id": "CH 12",
        "volume_id": "VOL 02",
        "slug": "Ch12_Track1_Terminal_Studio_and_Hover",
        "filename": "OmniCode_Ch12_Track1_Terminal_Studio_and_Hover.pdf",
        "title": "Chapter 12: Track 1 - Terminal, REPL & Hover Inspection",
        "subtitle": "Interactive REPL, Task Scheduler, OmniStudio HUD & 3-Tier Hover Cards",
        "prompt": "How does the hover engine bridge tape execution rolls back to original .omni source specifications?",
    },
    {
        "id": "CH 13",
        "volume_id": "VOL 02",
        "slug": "Ch13_Track1_Verification_and_Tripwires",
        "filename": "OmniCode_Ch13_Track1_Verification_and_Tripwires.pdf",
        "title": "Chapter 13: Track 1 - Verification & Functional Test Suites",
        "subtitle": "Tripwires, Boot Verification, Assemble Invariants, Functional Tests & Self-Audit",
        "prompt": "How do assemble tripwire tests mathematically prove zero-disk-mutation on dry run invocations?",
    },
    {
        "id": "CH 14",
        "volume_id": "VOL 03",
        "slug": "Ch14_Track2_Lexicon_Character_Primitive",
        "filename": "OmniCode_Ch14_Track2_Lexicon_Character_Primitive.pdf",
        "title": "Chapter 14: Track 2 - Lexicon: Character Primitive Type",
        "subtitle": "Single-Cell Enclosure, Bit Rails (Bit_0..Bit_7), 5-Trit / 8-Bit ASCII Bijective Invariance",
        "prompt": "Explain the bijective mapping of character 'A' (ASCII 65) to balanced ternary trits and bit rails.",
    },
    {
        "id": "CH 15",
        "volume_id": "VOL 03",
        "slug": "Ch15_Track2_Lexicon_Number_Primitive",
        "filename": "OmniCode_Ch15_Track2_Lexicon_Number_Primitive.pdf",
        "title": "Chapter 15: Track 2 - Lexicon: Number Primitive Type",
        "subtitle": "Dual-Rail Polar Planes, 32-Trit Word, 0b Binary Literals & Zero-Sum Balanced ALU",
        "prompt": "How does the dual-rail balanced ternary number representation enable O(1) rail-swap negation?",
    },
    {
        "id": "CH 16",
        "volume_id": "VOL 03",
        "slug": "Ch16_Track2_Lexicon_State_and_Containers",
        "filename": "OmniCode_Ch16_Track2_Lexicon_State_and_Containers.pdf",
        "title": "Chapter 16: Track 2 - Lexicon: State, Containers & Primitive Types",
        "subtitle": "Quad-State T4 Clock, Memory Envelopes, Bounded Sites, Text Records & Types",
        "prompt": "How do IT, BOUND, and TURN govern canonical loop cadence within bounded containers?",
    },
    {
        "id": "CH 17",
        "volume_id": "VOL 03",
        "slug": "Ch17_Track2_Lexicon_Nouns_and_Vessels",
        "filename": "OmniCode_Ch17_Track2_Lexicon_Nouns_and_Vessels.pdf",
        "title": "Chapter 17: Track 2 - Lexicon: Systems, Places & Covenant Vessels",
        "subtitle": "Operators, Supervisors, Storehouses, Partitions, Physical Places & Stewards",
        "prompt": "Trace how the Covenant Storehouse partitions memory vaults via the v-- operator.",
    },
    {
        "id": "CH 18",
        "volume_id": "VOL 03",
        "slug": "Ch18_Track2_Lexicon_Verbs_and_Operators",
        "filename": "OmniCode_Ch18_Track2_Lexicon_Verbs_and_Operators.pdf",
        "title": "Chapter 18: Track 2 - Lexicon: Verbs, Pronouns & Directional Operators",
        "subtitle": "Directional SVO Verbs, Scale Vectors, Polarity Operators, Ascension & Descension",
        "prompt": "Explain the functional difference between ascension (/-^) and descension (\\-v) across ontological scales.",
    },
    {
        "id": "CH 19",
        "volume_id": "VOL 03",
        "slug": "Ch19_Track2_Lexicon_Modifiers_and_Connectives",
        "filename": "OmniCode_Ch19_Track2_Lexicon_Modifiers_and_Connectives.pdf",
        "title": "Chapter 19: Track 2 - Modifiers, Conjunctions & Guard Traps",
        "subtitle": "Invariants, Quad-States, Cadence, Pipelines, Conjunctions & Adversative Caveats (!)",
        "prompt": "How does the adversative caveat operator (!) guard ontological truth against semantic drift?",
    },
    {
        "id": "CH 20",
        "volume_id": "VOL 03",
        "slug": "Ch20_Track2_Standard_Library_Modules",
        "filename": "OmniCode_Ch20_Track2_Standard_Library_Modules.pdf",
        "title": "Chapter 20: Track 2 - Standard Library Modules",
        "subtitle": "Core Logic, Vector Mathematics, High-Precision Arithmetic & Memory Management",
        "prompt": "How do the standard library modules export pure algebraic invariants without side effects?",
    },
    {
        "id": "CH 21",
        "volume_id": "VOL 04",
        "slug": "Ch21_Track3_Applications_and_Studio",
        "filename": "OmniCode_Ch21_Track3_Applications_and_Studio.pdf",
        "title": "Chapter 21: Track 3 - Applications, Bridges & OmniStudio",
        "subtitle": "C99 Bare-Metal Bridge, Lua Coroutine Fibers, VSCode Extensions & Studio HUD",
        "prompt": "How does the Lua coroutine bridge execute process tape rolls as non-blocking fiber threads?",
    },
    {
        "id": "CH 22",
        "volume_id": "VOL 05",
        "slug": "Ch22_Track4_Storage_and_Episodic_Scrolls",
        "filename": "OmniCode_Ch22_Track4_Storage_and_Episodic_Scrolls.pdf",
        "title": "Chapter 22: Track 4 - Variable Storage & Episodic Scrolls",
        "subtitle": "Canonical Bibles, Invariant Truth Tables, Episodic Memory Logs & Variable States",
        "prompt": "How does Track 4 preserve immutable scriptural anchors across distributed variable updates?",
    },
    {
        "id": "CH 23",
        "volume_id": "VOL 06",
        "slug": "Ch23_Track5_Architecture_and_Logic",
        "filename": "OmniCode_Ch23_Track5_Architecture_and_Logic.pdf",
        "title": "Chapter 23: Track 5 - Systems Architecture & Balanced Logic",
        "subtitle": "Coordinate Computation, Execution Pipelines, 4-Block Architecture & Quad-State Loops",
        "prompt": "Explain the architectural synergy between the 4-block layout and balanced ternary arithmetic.",
    },
    {
        "id": "CH 24",
        "volume_id": "VOL 06",
        "slug": "Ch24_Track5_Cognitive_Systems_and_Infra",
        "filename": "OmniCode_Ch24_Track5_Cognitive_Systems_and_Infra.pdf",
        "title": "Chapter 24: Track 5 - Cognitive Systems, Infrastructure & CI/CD",
        "subtitle": "Neural-Symbolic Feedback, Autonomous Restorator, Build Scripts & DNS Infrastructure",
        "prompt": "How does the autonomous restorator interface with cross-platform toolchains to certify zero-defect builds?",
    },
]

CSS_STYLES = """
@page {
    size: letter portrait;
    margin: 1.5cm 1.2cm 1.5cm 1.2cm;
    @top-center {
        content: "OmniCode Framework & AI OS | Official Reference Manual";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #64748b;
    }
    @bottom-left {
        content: "Witnesses: Exodus 31:18, Proverbs 24:27, Proverbs 22:28, Colossians 2:19";
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #64748b;
    }
    @bottom-right {
        content: "Page " counter(page);
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        font-size: 7pt;
        color: #475569;
    }
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    color: #0f172a;
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

/* Cover Page */
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

.cover-volume-badge {
    display: inline-block;
    background: #1e3a8a;
    color: #ffffff;
    font-size: 8pt;
    font-weight: 700;
    letter-spacing: 1px;
    text-transform: uppercase;
    padding: 3px 10px;
    border-radius: 4px;
    margin-bottom: 12px;
}

.cover-title {
    font-size: 20pt;
    font-weight: 800;
    color: #0f172a;
    margin-bottom: 6px;
    line-height: 1.15;
}

.cover-subtitle {
    font-size: 11pt;
    font-weight: 500;
    color: #2563eb;
    margin-bottom: 24px;
    line-height: 1.35;
}

.cover-scripture {
    border-left: 3px solid #f59e0b;
    background: #fffbeb;
    padding: 10px 14px;
    font-style: italic;
    color: #78350f;
    font-size: 8pt;
    margin-bottom: 24px;
    border-radius: 0 4px 4px 0;
}

.cover-metadata-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    border-top: 1px solid #cbd5e1;
    padding-top: 16px;
    margin-bottom: 20px;
}

.metadata-item {
    display: flex;
    flex-direction: column;
}

.metadata-label {
    font-size: 6.5pt;
    text-transform: uppercase;
    color: #64748b;
    font-weight: 700;
    letter-spacing: 0.5px;
}

.metadata-value {
    font-size: 8.5pt;
    color: #1e293b;
    font-weight: 600;
}

.cover-prompt-box {
    background: #f1f5f9;
    border: 1px solid #94a3b8;
    border-radius: 4px;
    padding: 10px 12px;
    margin-top: 10px;
}

.cover-prompt-label {
    font-size: 7pt;
    font-weight: 700;
    text-transform: uppercase;
    color: #334155;
    margin-bottom: 3px;
}

.cover-prompt-text {
    font-size: 8pt;
    color: #0f172a;
    font-style: italic;
}

/* TOC */
.toc-container {
    margin-top: 20px;
    margin-bottom: 25px;
}

.toc-header {
    font-size: 13pt;
    font-weight: 700;
    color: #1e3a8a;
    border-bottom: 2px solid #1e3a8a;
    padding-bottom: 5px;
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
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

/* File Cards */
.file-card {
    margin-top: 14px;
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

.badge-doc {
    background: #065f46;
    border-color: #047857;
    color: #a7f3d0;
}

.file-content-box {
    background: #f8fafc;
    border: 1px solid #cbd5e1;
    border-top: none;
    padding: 8px 10px;
    border-radius: 0 0 4px 4px;
    overflow-x: auto;
}

/* Code Display */
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

/* Rendered Markdown Typography */
.markdown-doc-container {
    padding: 4px 8px;
    color: #1e293b;
    font-size: 8.2pt;
    line-height: 1.45;
}

.markdown-doc-container h1 {
    font-size: 13pt;
    font-weight: 800;
    color: #0f172a;
    border-bottom: 1.5px solid #cbd5e1;
    padding-bottom: 4px;
    margin-top: 8px;
    margin-bottom: 8px;
}

.markdown-doc-container h2 {
    font-size: 11pt;
    font-weight: 700;
    color: #1e3a8a;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 3px;
    margin-top: 10px;
    margin-bottom: 6px;
}

.markdown-doc-container h3 {
    font-size: 9.5pt;
    font-weight: 700;
    color: #0f172a;
    margin-top: 8px;
    margin-bottom: 4px;
}

.markdown-doc-container h4 {
    font-size: 8.5pt;
    font-weight: 600;
    color: #334155;
    margin-top: 6px;
    margin-bottom: 3px;
}

.markdown-doc-container p {
    margin: 5px 0;
}

.markdown-doc-container blockquote {
    border-left: 3.5px solid #2563eb;
    background: #f0f7ff;
    padding: 6px 12px;
    margin: 6px 0;
    font-style: italic;
    color: #1e3a8a;
    border-radius: 0 3px 3px 0;
}

.markdown-doc-container table {
    width: 100%;
    border-collapse: collapse;
    margin: 8px 0;
    font-size: 7.2pt;
}

.markdown-doc-container th {
    background: #f1f5f9;
    color: #0f172a;
    font-weight: 700;
    padding: 4px 7px;
    border: 1px solid #cbd5e1;
    text-align: left;
}

.markdown-doc-container td {
    padding: 3px 7px;
    border: 1px solid #e2e8f0;
}

.markdown-doc-container tr:nth-child(even) {
    background: #f8fafc;
}

.markdown-doc-container code {
    background: #f1f5f9;
    color: #0f172a;
    padding: 1px 4px;
    border-radius: 3px;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", monospace;
    font-size: 7.5pt;
}

.markdown-doc-container pre {
    background: #0f172a;
    color: #f8fafc;
    padding: 8px 10px;
    border-radius: 4px;
    overflow-x: auto;
    margin: 6px 0;
}

.markdown-doc-container pre code {
    background: transparent;
    color: #f8fafc;
    padding: 0;
    font-size: 6.8pt;
    line-height: 1.35;
}

.markdown-doc-container ul, .markdown-doc-container ol {
    margin: 4px 0;
    padding-left: 20px;
}

.markdown-doc-container hr {
    border: none;
    border-top: 1px solid #cbd5e1;
    margin: 10px 0;
}
"""

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

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Routing, Discovery, Publication Cards, Volumes & Chapter Lowering )
# ==============================================================================

def categorize_file(rel_path: str) -> str:
    """Categorizes any file in the repository into its canonical Chapter ID."""
    p = rel_path.lower()
    
    # CH 00: Foundations, Root Manifests, Cargo, Decalogue & Environment
    if '/' not in rel_path or any(
        rel_path.startswith(x) for x in ['README', 'manifest', 'Cargo', '.env', '.markdownlint']
    ):
        return "CH 00"
        
    # CH 01: Ontological Principles and Boundaries (Track 5 / docs / foundations)
    if 'track 5' in p and '01_foundations' in p:
        return "CH 01"
        
    # CH 02 & CH 03: Track 0 Silicon & Bare-Metal Kernel
    if 'track 0' in p:
        if any(x in p for x in ['uefi', 'multiboot', 'bootx64']):
            return "CH 03"
        return "CH 02"
        
    # Track 1: Host Engine & Virtual Machine (CH 04 through CH 13)
    if 'track 1' in p:
        if '/tests/' in p:
            return "CH 13"
        if '/src/parser/' in p or '/src/ast/' in p:
            return "CH 05"
        if '/src/ai/' in p:
            return "CH 06"
        if '/src/runtime/environment/' in p or '/src/runtime/evaluator/' in p:
            return "CH 07"
        if '/src/runtime/assembler/' in p:
            return "CH 08"
        if '/src/diagnostics/linter/' in p:
            return "CH 09"
        if '/src/diagnostics/logger/' in p:
            return "CH 10"
        if '/src/diagnostics/debugger/' in p or '/src/diagnostics/restorator/' in p or 'dar_detect' in p:
            return "CH 11"
        if '/src/terminal/' in p or '/src/cli/' in p or 'hover' in p:
            return "CH 12"
        return "CH 04"  # Default Track 1: Core, Cogs, Ribbon, Lib, Main
        
    # Track 2: OmniCode Language & Standard Library (CH 14 through CH 20)
    if 'track 2' in p:
        if '01_nouns/ideas/types/character' in p:
            return "CH 14"
        if '01_nouns/ideas/types/number' in p:
            return "CH 15"
        if '01_nouns/ideas/types' in p:
            return "CH 16"
        if '01_nouns' in p:
            return "CH 17"
        if '02_pronouns' in p or '03_verbs' in p or '/operators/' in p:
            return "CH 18"
        if any(x in p for x in [
            '04_adjectives', '05_adverbs', '06_prepositions',
            '07_conjunctions', '08_interjections'
        ]):
            return "CH 19"
        if '/lib/' in p:
            return "CH 20"
        return "CH 16"
        
    # Track 3: Applications & Studio (CH 21)
    if 'track 3' in p:
        return "CH 21"
        
    # Track 4: Variable Storage & Bibles (CH 22)
    if 'track 4' in p:
        return "CH 22"
        
    # Track 5: Infrastructure, Operations & Archives (CH 23 & CH 24)
    if 'track 5' in p or 'scripts' in p:
        if '02_logic_and_algebra' in p or '03_architecture' in p:
            return "CH 23"
        return "CH 24"
        
    return "CH 00"

def discover_repository_files():
    """Scans and catalogs text, code, documentation, and manifest files."""
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
                    with open(full_path, "r", encoding="utf-8", errors="replace") as f:
                        content = f.read()
                        lines_count = content.count("\n") + (1 if content else 0)
                        words_count = len(content.split())
                except Exception as e:
                    content = f"[Error reading file: {e}]"
            else:
                if full_path.suffix.lower() == ".pdf":
                    size_mb = file_bytes / (1024 * 1024)
                    content = f"[Immutable Scripture Reference: {file} ({size_mb:.2f} MB)]"
                else:
                    size_kb = file_bytes / 1024.0
                    content = f"[Binary Artifact: {file} ({size_kb:.1f} KB)]"
                lines_count = 1
                words_count = len(content.split())
                
            ch_id = categorize_file(rel_str)
            
            entry = {
                "rel_path": rel_str,
                "full_path": str(full_path),
                "filename": file,
                "extension": full_path.suffix.lower(),
                "top_dir": top_component,
                "chapter_id": ch_id,
                "is_binary": is_bin,
                "lines": lines_count,
                "words": words_count,
                "bytes": file_bytes,
                "content": content,
            }
            all_files.append(entry)
            
    return all_files

def render_file_card_html(entry):
    """Renders a single file into a publication card (rich Markdown or code block)."""
    rel_path = entry["rel_path"]
    ext = entry["extension"]
    is_bin = entry["is_binary"]
    lines = entry["lines"]
    words = entry["words"]
    size_kb = entry["bytes"] / 1024.0
    content = entry["content"]
    
    is_markdown = ext in [".md", ".markdown"]
    
    badge_type = ext.replace(".", "").upper() if ext else "TEXT"
    if is_bin:
        badge_type = "DOC/BINARY"
    elif is_markdown:
        badge_type = "MARKDOWN / DOC"
        
    badge_class = "badge badge-doc" if is_markdown else "badge"
    
    out = []
    out.append('<div class="file-card page-break">')
    out.append('  <div class="file-header-box">')
    out.append(f'    <div class="file-path">{html.escape(rel_path)}</div>')
    out.append('    <div class="file-meta-badges">')
    out.append(f'      <span class="{badge_class}">{badge_type}</span>')
    out.append(f'      <span class="badge">{lines} Lines</span>')
    out.append(f'      <span class="badge">{words} Words</span>')
    out.append(f'      <span class="badge">{size_kb:.1f} KB</span>')
    out.append('    </div>')
    out.append('  </div>')
    out.append('  <div class="file-content-box">')
    
    if is_bin:
        out.append('    <p style="font-style:italic; color:#475569; margin:8px 0; font-size:8pt;">')
        out.append(f'      {html.escape(content)}')
        out.append('    </p>')
    elif is_markdown and HAS_MARKDOWN:
        # Beautifully rendered Markdown typography for manuals and specifications
        rendered_md = markdown.markdown(
            content,
            extensions=['tables', 'fenced_code', 'toc']
        )
        out.append('    <div class="markdown-doc-container">')
        out.append(rendered_md)
        out.append('    </div>')
    else:
        # Monospace code block with line numbers for source code
        out.append('    <pre class="code-block">')
        raw_lines = content.splitlines()
        for i, l in enumerate(raw_lines, 1):
            escaped = html.escape(l)
            out.append(f'<span class="line-num">{i}</span>{escaped}')
        out.append('</pre>')
        
    out.append('  </div>')
    out.append('</div>\n')
    return "\n".join(out)

def build_chapter_html(chapter_def, volume_def, files):
    """Assembles a publication-grade HTML document for a single chapter."""
    ch_id = chapter_def["id"]
    ch_title = chapter_def["title"]
    ch_sub = chapter_def["subtitle"]
    ch_prompt = chapter_def["prompt"]
    vol_title = volume_def["title"]
    vol_track = volume_def["track"]
    vol_witness = volume_def["witness"]
    
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
    
    # 1. Chapter Cover Page
    html_parts.append('<div class="cover-page">')
    html_parts.append(f'  <div class="cover-volume-badge">{html.escape(vol_title)}</div>')
    html_parts.append(f'  <div class="cover-title">{html.escape(ch_title)}</div>')
    html_parts.append(f'  <div class="cover-subtitle">{html.escape(ch_sub)}</div>')
    
    html_parts.append('  <div class="cover-scripture">')
    html_parts.append(f'    <strong>Witness:</strong> {html.escape(vol_witness)}')
    html_parts.append('  </div>')
    
    html_parts.append('  <div class="cover-metadata-grid">')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">System Track</span>')
    html_parts.append(f'      <span class="metadata-value">{html.escape(vol_track)}</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Chapter Identifier</span>')
    html_parts.append(f'      <span class="metadata-value">{html.escape(ch_id)}</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">NotebookLM Word Count</span>')
    html_parts.append(f'      <span class="metadata-value">{ch_words:,} words</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Cataloged Chapter Files</span>')
    html_parts.append(f'      <span class="metadata-value">{len(files)} files ({ch_lines:,} lines)</span>')
    html_parts.append('    </div>')
    html_parts.append('  </div>')
    
    html_parts.append('  <div class="cover-prompt-box">')
    html_parts.append('    <div class="cover-prompt-label">Recommended NotebookLM Cognitive Query</div>')
    html_parts.append(f'    <div class="cover-prompt-text">"{html.escape(ch_prompt)}"</div>')
    html_parts.append('  </div>')
    html_parts.append('</div>')
    
    # 2. Table of Contents
    html_parts.append('<div class="toc-container page-break">')
    html_parts.append(f'  <div class="toc-header">Chapter Manifest & Index ({len(files)} files)</div>')
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
    
    # 3. Publication File Cards
    for f in files:
        html_parts.append(render_file_card_html(f))
        
    html_parts.append('</body>')
    html_parts.append('</html>')
    
    return "\n".join(html_parts)

def build_volume_cover_html(volume_def, chapters, total_files, total_lines, total_words):
    """Assembles a formal volume cover page for uniting multiple chapter PDFs into a Volume Manual."""
    vol_title = volume_def["title"]
    vol_track = volume_def["track"]
    vol_sub = volume_def["subtitle"]
    vol_witness = volume_def["witness"]
    vol_desc = volume_def["description"]
    
    html_parts = []
    html_parts.append('<!DOCTYPE html>')
    html_parts.append('<html lang="en">')
    html_parts.append('<head>')
    html_parts.append('  <meta charset="UTF-8">')
    html_parts.append(f'  <title>{html.escape(vol_title)}</title>')
    html_parts.append(f'  <style>{CSS_STYLES}</style>')
    html_parts.append('</head>')
    html_parts.append('<body>')
    
    html_parts.append('<div class="cover-page">')
    html_parts.append(f'  <div class="cover-volume-badge">Official OmniCode System Manual</div>')
    html_parts.append(f'  <div class="cover-title">{html.escape(vol_title)}</div>')
    html_parts.append(f'  <div class="cover-subtitle">{html.escape(vol_sub)}</div>')
    
    html_parts.append('  <div class="cover-scripture">')
    html_parts.append(f'    <strong>Foundational Scripture Anchor:</strong> {html.escape(vol_witness)}')
    html_parts.append('  </div>')
    
    html_parts.append('  <p style="font-size:9pt; color:#334155; line-height:1.5; margin-bottom:20px;">')
    html_parts.append(f'    {html.escape(vol_desc)}')
    html_parts.append('  </p>')
    
    html_parts.append('  <div class="cover-metadata-grid">')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">System Track</span>')
    html_parts.append(f'      <span class="metadata-value">{html.escape(vol_track)}</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Total Volume Chapters</span>')
    html_parts.append(f'      <span class="metadata-value">{len(chapters)} Chapters</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Cataloged Source Files</span>')
    html_parts.append(f'      <span class="metadata-value">{total_files} files ({total_lines:,} lines)</span>')
    html_parts.append('    </div>')
    html_parts.append('    <div class="metadata-item">')
    html_parts.append('      <span class="metadata-label">Total Word Count</span>')
    html_parts.append(f'      <span class="metadata-value">{total_words:,} words</span>')
    html_parts.append('    </div>')
    html_parts.append('  </div>')
    
    html_parts.append('  <div class="toc-container">')
    html_parts.append('    <div class="toc-header">Volume Chapters Catalog</div>')
    html_parts.append('    <table class="toc-table">')
    html_parts.append('      <thead>')
    html_parts.append('        <tr>')
    html_parts.append('          <th>Chapter</th>')
    html_parts.append('          <th>Title</th>')
    html_parts.append('          <th>Subsystem Coverage</th>')
    html_parts.append('        </tr>')
    html_parts.append('      </thead>')
    html_parts.append('      <tbody>')
    for ch in chapters:
        html_parts.append('        <tr>')
        html_parts.append(f'          <td><strong>{html.escape(ch["id"])}</strong></td>')
        html_parts.append(f'          <td>{html.escape(ch["title"])}</td>')
        html_parts.append(f'          <td>{html.escape(ch["subtitle"])}</td>')
        html_parts.append('        </tr>')
    html_parts.append('      </tbody>')
    html_parts.append('    </table>')
    html_parts.append('  </div>')
    
    html_parts.append('</div>')
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
        f"file://{tmp_html}"
    ]
    
    t0 = time.time()
    res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    t1 = time.time()
    
    if tmp_html.exists():
        tmp_html.unlink()
        
    if res.returncode != 0:
        err = res.stderr.decode('utf-8', errors='ignore')
        raise RuntimeError(f"Chrome PDF generation failed (code {res.returncode}): {err}")
        
    pdf_size_kb = output_pdf_path.stat().st_size / 1024.0
    return t1 - t0, pdf_size_kb

def unite_pdfs(input_pdfs, output_pdf_path):
    """Assembles multiple PDF pieces into a unified Master or Volume PDF."""
    t0 = time.time()
    if shutil.which("pdfunite"):
        cmd = ["pdfunite"] + [str(p) for p in input_pdfs] + [str(output_pdf_path)]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if res.returncode == 0:
            t1 = time.time()
            return t1 - t0, output_pdf_path.stat().st_size / 1024.0
            
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
# ( Orchestration, Volume Assembly, Master Omnibus & Ingestion Manifest )
# ==============================================================================

def main():
    parser = argparse.ArgumentParser(
        description="Export OmniCode repository into Track Volumes and Chapter PDFs."
    )
    parser.add_argument(
        "--output-dir",
        default=str(OUTPUT_DIR),
        help="Target directory for generated PDF files."
    )
    parser.add_argument(
        "--volume",
        default=None,
        help="Optional single volume ID or slug to generate (e.g. 'VOL 02' or 'Track1')."
    )
    parser.add_argument(
        "--chapter",
        default=None,
        help="Optional single chapter ID or slug to generate (e.g. 'CH 09' or 'Linter')."
    )
    parser.add_argument(
        "--no-volumes",
        action="store_true",
        help="Skip assembling track-by-track Volume PDFs."
    )
    parser.add_argument(
        "--no-master",
        action="store_true",
        help="Skip assembling the complete unified Master Omnibus PDF."
    )
    
    args = parser.parse_args()
    target_dir = Path(args.output_dir).resolve()
    target_dir.mkdir(parents=True, exist_ok=True)
    
    print("=" * 80)
    print("OMNICODE NOTEBOOKLM MULTI-VOLUME & CHAPTER MANUAL GENERATOR")
    print(f"Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Target Directory: {target_dir}")
    print("=" * 80)
    
    print("\n[1/5] Discovering repository files across all tracks...")
    all_files = discover_repository_files()
    
    total_files = len(all_files)
    total_lines = sum(f["lines"] for f in all_files)
    total_words = sum(f["words"] for f in all_files)
    total_mb = sum(f["bytes"] for f in all_files) / (1024 * 1024)
    
    print(f"  • Total Cataloged Files: {total_files}")
    print(f"  • Total Source Lines:   {total_lines:,}")
    print(f"  • Total Word Count:     {total_words:,} words")
    print(f"  • Total Raw Volume:     {total_mb:.2f} MB")
    
    # Map files by chapter ID
    files_by_ch = {}
    for f in all_files:
        files_by_ch.setdefault(f["chapter_id"], []).append(f)
        
    # Map chapters and volumes
    vol_by_id = {v["id"]: v for v in VOLUME_DEFINITIONS}
    ch_by_id = {c["id"]: c for c in CHAPTER_DEFINITIONS}
    
    # Filter chapters if requested
    active_chapters = CHAPTER_DEFINITIONS
    if args.chapter:
        query = args.chapter.lower()
        active_chapters = [
            c for c in CHAPTER_DEFINITIONS
            if query in c["slug"].lower() or query in c["id"].lower() or query in c["title"].lower()
        ]
        if not active_chapters:
            print(f"[-] Error: No chapter matched query '{args.chapter}'.")
            sys.exit(1)
            
    if args.volume:
        v_query = args.volume.lower()
        matched_vols = [
            v for v in VOLUME_DEFINITIONS
            if v_query in v["id"].lower() or v_query in v["slug"].lower() or v_query in v["track"].lower()
        ]
        if not matched_vols:
            print(f"[-] Error: No volume matched query '{args.volume}'.")
            sys.exit(1)
        allowed_ch_ids = set()
        for v in matched_vols:
            allowed_ch_ids.update(v["chapters"])
        active_chapters = [c for c in active_chapters if c["id"] in allowed_ch_ids]
        
    print(f"\n[2/5] Rendering {len(active_chapters)} bounded Chapter PDFs...")
    chapter_manifest = []
    generated_chapter_pdfs = {}
    
    for idx, c in enumerate(active_chapters, 1):
        c_id = c["id"]
        c_file = c["filename"]
        v_id = c["volume_id"]
        v_def = vol_by_id.get(v_id, VOLUME_DEFINITIONS[0])
        
        c_files = files_by_ch.get(c_id, [])
        if not c_files:
            continue
            
        words = sum(f["words"] for f in c_files)
        lines = sum(f["lines"] for f in c_files)
        out_pdf = target_dir / c_file
        
        print(f"  [{idx:02d}/{len(active_chapters):02d}] {c_file} ({v_id})")
        print(f"       Files: {len(c_files)} | Lines: {lines:,} | Words: {words:,}")
        
        html_doc = build_chapter_html(c, v_def, c_files)
        duration, size_kb = convert_html_to_pdf(html_doc, out_pdf)
        generated_chapter_pdfs[c_id] = out_pdf
        
        print(f"       Rendered in {duration:.2f}s | Size: {size_kb:.1f} KB")
        
        chapter_manifest.append({
            "chapter_id": c_id,
            "volume_id": v_id,
            "volume_title": v_def["title"],
            "slug": c["slug"],
            "filename": c_file,
            "path": str(out_pdf),
            "title": c["title"],
            "subtitle": c["subtitle"],
            "prompt": c["prompt"],
            "file_count": len(c_files),
            "line_count": lines,
            "word_count": words,
            "size_kb": size_kb,
            "render_seconds": round(duration, 2),
        })
        
    # Assemble Track-by-Track Volume PDFs
    volume_manifest = []
    if not args.no_volumes and not args.chapter:
        print("\n[3/5] Assembling Track-by-Track Volume Manuals...")
        for v in VOLUME_DEFINITIONS:
            v_id = v["id"]
            v_filename = v["filename"]
            v_title = v["title"]
            out_vol_pdf = target_dir / v_filename
            
            # Find generated chapter PDFs in this volume
            ch_pdfs_in_vol = []
            vol_chapters_meta = []
            v_files_count = 0
            v_lines_count = 0
            v_words_count = 0
            
            for ch_id in v["chapters"]:
                if ch_id in generated_chapter_pdfs:
                    ch_pdfs_in_vol.append(generated_chapter_pdfs[ch_id])
                    ch_meta = ch_by_id[ch_id]
                    vol_chapters_meta.append(ch_meta)
                    ch_files = files_by_ch.get(ch_id, [])
                    v_files_count += len(ch_files)
                    v_lines_count += sum(f["lines"] for f in ch_files)
                    v_words_count += sum(f["words"] for f in ch_files)
                    
            if not ch_pdfs_in_vol:
                continue
                
            print(f"  • Assembling {v_filename} ({len(ch_pdfs_in_vol)} chapters)...")
            
            # Build and render volume cover PDF
            vol_cover_html = build_volume_cover_html(
                v, vol_chapters_meta, v_files_count, v_lines_count, v_words_count
            )
            vol_cover_pdf = target_dir / f"{v['slug']}_Cover.tmp.pdf"
            convert_html_to_pdf(vol_cover_html, vol_cover_pdf)
            
            # Unite cover + chapter PDFs
            all_vol_pieces = [vol_cover_pdf] + ch_pdfs_in_vol
            u_sec, u_size_kb = unite_pdfs(all_vol_pieces, out_vol_pdf)
            
            if vol_cover_pdf.exists():
                vol_cover_pdf.unlink()
                
            print(f"       United in {u_sec:.2f}s | Size: {u_size_kb/1024.0:.2f} MB")
            
            volume_manifest.append({
                "volume_id": v_id,
                "title": v_title,
                "track": v["track"],
                "filename": v_filename,
                "path": str(out_vol_pdf),
                "chapter_count": len(ch_pdfs_in_vol),
                "file_count": v_files_count,
                "line_count": v_lines_count,
                "word_count": v_words_count,
                "size_mb": round(u_size_kb / 1024.0, 2),
            })
    else:
        print("\n[3/5] Skipping Volume PDF assembly (--no-volumes or single chapter requested).")
        
    # Assemble Unified Master Omnibus Manual
    master_manifest_entry = None
    if not args.no_master and not args.chapter and not args.volume:
        print("\n[4/5] Assembling Master Omnibus Manual (All Volumes & Tracks Combined)...")
        master_pdf = target_dir / "OmniCode_Master_Omnibus_Manual.pdf"
        try:
            # Order all chapter PDFs sequentially CH 00 through CH 24
            ordered_chapter_pdfs = [
                generated_chapter_pdfs[c["id"]]
                for c in CHAPTER_DEFINITIONS
                if c["id"] in generated_chapter_pdfs
            ]
            u_sec, u_size_kb = unite_pdfs(ordered_chapter_pdfs, master_pdf)
            print(f"  • Complete Master Omnibus united in {u_sec:.2f}s | Size: {u_size_kb/1024.0:.2f} MB")
            master_manifest_entry = {
                "title": "OmniCode Master Omnibus Manual",
                "filename": "OmniCode_Master_Omnibus_Manual.pdf",
                "path": str(master_pdf),
                "chapter_count": len(ordered_chapter_pdfs),
                "file_count": total_files,
                "line_count": total_lines,
                "word_count": total_words,
                "size_mb": round(u_size_kb / 1024.0, 2),
            }
        except Exception as e:
            print(f"  [!] Master assembly warning: {e}")
    else:
        print("\n[4/5] Skipping Master Omnibus assembly.")
        
    # Write Ingestion Manifests
    print("\n[5/5] Writing Ingestion Manifests for Google NotebookLM...")
    manifest_md = target_dir / "NOTEBOOKLM_INGESTION_MANIFEST.md"
    manifest_json = target_dir / "notebooklm_manifest.json"
    
    full_manifest = {
        "generated_at": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "repository": str(REPO_ROOT),
        "total_files": total_files,
        "total_lines": total_lines,
        "total_words": total_words,
        "master_omnibus": master_manifest_entry,
        "volumes": volume_manifest,
        "chapters": chapter_manifest,
    }
    
    with open(manifest_json, "w", encoding="utf-8") as f:
        json.dump(full_manifest, f, indent=2)
        
    with open(manifest_md, "w", encoding="utf-8") as f:
        f.write("# OmniCode: Google NotebookLM Multi-Volume Manual Ingestion Manifest\n\n")
        f.write(f"**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}  \n")
        f.write(f"**Repository:** `{REPO_ROOT}`  \n")
        f.write(f"**Total Cataloged Files:** {total_files} | **Lines:** {total_lines:,} | **Words:** {total_words:,}  \n\n")
        
        f.write("## Scripture Witnesses & Invariants\n")
        f.write("- **Exodus 31:18**: *\"written with the finger of God\"*\n")
        f.write("- **Proverbs 24:27**: *\"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house.\"*\n")
        f.write("- **Proverbs 22:28**: *\"Remove not the ancient landmark, which thy fathers have set.\"*\n")
        f.write("- **Colossians 2:19**: *\"From whom the whole body fitly joined together and compacted by that which every joint supplieth...\"*\n\n")
        
        f.write("## Volume Manuals (Track-by-Track Architecture)\n\n")
        f.write("| Volume | Manual Title | System Track | Chapters | Files | Lines | Words | Size |\n")
        f.write("|---|---|---|---|---|---|---|---|\n")
        for v in volume_manifest:
            f.write(f"| **{v['volume_id']}** | `{v['filename']}` | {v['track']} | {v['chapter_count']} | {v['file_count']} | {v['line_count']:,} | {v['word_count']:,} | {v['size_mb']} MB |\n")
            
        f.write("\n## Chapter Curriculum Map for AI Model Training\n\n")
        f.write("| Ch | Volume | PDF Filename | Files | Lines | Words | Size | Recommended Cognitive Prompt |\n")
        f.write("|---|---|---|---|---|---|---|---|\n")
        for c in chapter_manifest:
            size_str = f"{c['size_kb']/1024.0:.2f} MB"
            f.write(f"| **{c['chapter_id']}** | {c['volume_id']} | `{c['filename']}` | {c['file_count']} | {c['line_count']:,} | {c['word_count']:,} | {size_str} | {c['prompt']} |\n")
            
        f.write("\n## How to Ingest into Google NotebookLM\n\n")
        f.write("1. Open **[NotebookLM](https://notebooklm.google.com)** with your Google AI account.\n")
        f.write("2. Create a new notebook named **`OmniCode Operating System & Language`**.\n")
        f.write("3. **Recommended Strategy - Modular Chapter Ingestion:** Upload the individual Chapter PDFs (`OmniCode_Ch00_*.pdf` through `OmniCode_Ch24_*.pdf`). Each chapter is bounded (10-35 pages), enabling high-resolution grounding and citation without token dilution.\n")
        f.write("4. **Track Manual Strategy:** Alternatively, upload the Volume Manuals (`OmniCode_Vol00_*.pdf` through `OmniCode_Vol06_*.pdf`) to focus on a specific track.\n")
        f.write("5. **Omnibus Strategy:** Upload `OmniCode_Master_Omnibus_Manual.pdf` for repository-wide synthesis.\n")
        
    print(f"  • Markdown Ingestion Manifest: {manifest_md}")
    print(f"  • Structured JSON Manifest:     {manifest_json}")
    
    print("\n" + "=" * 80)
    print("✓ MULTI-VOLUME & CHAPTER MANUAL PIPELINE COMPLETED SUCCESSFULLY")
    print("=" * 80)

if __name__ == "__main__":
    main()
