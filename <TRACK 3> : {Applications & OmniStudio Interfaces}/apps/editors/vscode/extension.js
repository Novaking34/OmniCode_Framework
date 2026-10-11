// ==============================================================================
// OmniCode VS Code Extension: Hover & Docstring Subsystem
// ==============================================================================
// Connects the editor to the native OmniCode language engine (CLI & HTTP).
// Delivers 3-Tier composite hovercards bridging high-level .tape process ribbons
// down to low-level .omni invariant contracts and physical silicon execution states.
// ==============================================================================

const vscode = require('vscode');
const { execFile } = require('child_process');
const http = require('http');

/**
 * Activates the OmniCode VS Code Extension.
 * Registers the HoverProvider for .omni, .scroll, and .tape files.
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    const hoverProvider = vscode.languages.registerHoverProvider('omnicode', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            if (!range) {
                return null;
            }

            const symbol = document.getText(range);
            const line = position.line + 1;
            const col = position.character + 1;
            const filePath = document.fileName;

            return new Promise((resolve) => {
                // Method 1: Try local omnicode CLI binary directly
                execFile(
                    'omnicode',
                    ['hover', filePath, String(line), String(col), '--json'],
                    { timeout: 3000 },
                    (err, stdout) => {
                        if (!err && stdout) {
                            try {
                                const data = JSON.parse(stdout.trim());
                                if (data && data.markdown) {
                                    const md = new vscode.MarkdownString(data.markdown);
                                    md.isTrusted = true;
                                    md.supportHtml = true;
                                    return resolve(new vscode.Hover(md, range));
                                }
                            } catch (_e) {
                                // Fall through to HTTP server fallback
                            }
                        }

                        // Method 2: Fallback to running OmniCode Studio / GUI server (default port 8080)
                        const url = `http://127.0.0.1:8080/api/hover?file=${encodeURIComponent(filePath)}&line=${line}&col=${col}`;
                        const req = http.get(url, (res) => {
                            let body = '';
                            res.on('data', (chunk) => (body += chunk));
                            res.on('end', () => {
                                try {
                                    const data = JSON.parse(body);
                                    if (data && data.markdown) {
                                        const md = new vscode.MarkdownString(data.markdown);
                                        md.isTrusted = true;
                                        md.supportHtml = true;
                                        return resolve(new vscode.Hover(md, range));
                                    }
                                } catch (_err) {
                                    // Ignored
                                }
                                resolve(null);
                            });
                        });

                        req.on('error', () => resolve(null));
                        req.setTimeout(1000, () => {
                            req.destroy();
                            resolve(null);
                        });
                    }
                );
            });
        }
    });

    context.subscriptions.push(hoverProvider);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate,
};
