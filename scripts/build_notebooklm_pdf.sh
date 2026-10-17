#!/usr/bin/env bash
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Google NotebookLM Chapter-Based PDF Exporter Runner )
# ==============================================================================
# ARCHITECTURAL TOPOLOGY:
#   Orchestrates chapter-partitioned documentation export for Google NotebookLM
#   and AI model training. Ensures files are split into manageable chapter
#   volumes (10-35 pages) to prevent token dilution and needle-in-haystack loss.
#
# SCRIPTURAL WITNESSES:
#   - Exodus 31:18: "written with the finger of God"
#   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself"
#   - Proverbs 22:28: "Remove not the ancient landmark"
# ==============================================================================

# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Environment Validation, Toolchain Verification & Path Setup )
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUTPUT_DIR="${REPO_ROOT}/docs/notebooklm"

echo "========================================================================"
echo "==> OMNICODE NOTEBOOKLM CHAPTER EXPORTER (4-BLOCK RUNNER)"
echo "========================================================================"

# Validate Python 3
if ! command -v python3 >/dev/null 2>&1; then
    echo "[-] Error: python3 is required but not installed." >&2
    exit 1
fi

# Validate Headless Browser
if ! command -v google-chrome >/dev/null 2>&1 && \
   ! command -v chromium >/dev/null 2>&1; then
    echo "    HTML artifacts created, but PDF conversion may fail." >&2
fi

# Ensure output directory exists
mkdir -p "${OUTPUT_DIR}"

# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Pipeline Execution & Chapter Generation )
# ==============================================================================
echo "==> Invoking Chapter Exporter Pipeline..."
python3 "${REPO_ROOT}/scripts/export_notebooklm_pdf.py" "$@"

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Artifact Verification, Summary & Affirmative Sealing )
# ==============================================================================
echo ""
echo "========================================================================"
echo "==> CHAPTER ARTIFACTS VERIFICATION"
echo "========================================================================"

PDF_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "*.pdf" | wc -l)
MD_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "*.md" | wc -l)

echo "[+] Total Chapter PDFs Generated: ${PDF_COUNT}"
echo "[+] Total Ingestion Manifests:    ${MD_COUNT}"
echo "[+] Manifest Location: ${OUTPUT_DIR}/NOTEBOOKLM_INGESTION_MANIFEST.md"

if [ "${PDF_COUNT}" -gt 0 ]; then
    echo "[√] Status: PROCEED (+1) - Chapter documentation ready for ingestion."
    exit 0
else
    echo "[-] Status: ROLLBACK (-1) - No PDF artifacts generated." >&2
    exit 1
fi
