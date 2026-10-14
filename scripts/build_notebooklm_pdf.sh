#!/usr/bin/env bash
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( OmniCode Google NotebookLM PDF Exporter Runner )
# ==============================================================================
# Scripture Witnesses:
#   Exodus 31:18, Proverbs 24:27, Proverbs 22:28
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo "==> Running OmniCode Google NotebookLM PDF Exporter..."
python3 "${REPO_ROOT}/scripts/export_notebooklm_pdf.py" "$@"

echo "==> Artifacts ready in: ${REPO_ROOT}/docs/notebooklm/"
ls -lh "${REPO_ROOT}/docs/notebooklm/"*.pdf "${REPO_ROOT}/docs/notebooklm/"*.md 2>/dev/null || true
