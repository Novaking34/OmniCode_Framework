#!/usr/bin/env bash
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Google NotebookLM Multi-Volume & Chapter Manual Runner )
# ==============================================================================
# ARCHITECTURAL TOPOLOGY:
#   Orchestrates track-by-track Volume Manuals (Vol 00 - Vol 06) and bounded
#   Chapter PDFs (Ch 00 - Ch 24) for Google NotebookLM, human study manuals,
#   and AI model training. Renders markdown documentation with rich typography
#   and source code with line-numbered verification cards.
#
# SCRIPTURAL WITNESSES:
#   - Exodus 31:18: "written with the finger of God"
#   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself"
#   - Proverbs 22:28: "Remove not the ancient landmark"
#   - Colossians 2:19: "From whom the whole body fitly joined together"
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
echo "==> OMNICODE NOTEBOOKLM MULTI-VOLUME MANUAL RUNNER (4-BLOCK RUNNER)"
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
echo "==> Invoking Multi-Volume & Chapter Manual Pipeline..."
python3 "${REPO_ROOT}/scripts/export_notebooklm_pdf.py" "$@"

# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Artifact Verification, Summary & Affirmative Sealing )
# ==============================================================================
echo ""
echo "========================================================================"
echo "==> MANUAL ARTIFACTS VERIFICATION"
echo "========================================================================"

VOL_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "OmniCode_Vol*.pdf" | wc -l)
CH_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "OmniCode_Ch*.pdf" | wc -l)
MASTER_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "OmniCode_Master_Omnibus_Manual.pdf" | wc -l)
MD_COUNT=$(find "${OUTPUT_DIR}" -maxdepth 1 -name "*.md" | wc -l)

echo "[+] Total Volume Manual PDFs Generated:  ${VOL_COUNT}"
echo "[+] Total Chapter PDFs Generated:        ${CH_COUNT}"
echo "[+] Total Master Omnibus Manuals:        ${MASTER_COUNT}"
echo "[+] Ingestion Manifests Generated:       ${MD_COUNT}"
echo "[+] Ingestion Manifest Location: ${OUTPUT_DIR}/NOTEBOOKLM_INGESTION_MANIFEST.md"

if [ "${CH_COUNT}" -gt 0 ]; then
    echo "[√] Status: PROCEED (+1) - Multi-Volume documentation ready for ingestion."
    exit 0
else
    echo "[-] Status: ROLLBACK (-1) - No PDF artifacts generated." >&2
    exit 1
fi
