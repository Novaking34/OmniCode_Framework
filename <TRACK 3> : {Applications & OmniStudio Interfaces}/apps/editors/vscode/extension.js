// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode VS Code Extension: Modular Language Engine & Hover Subsystem )
// ==============================================================================
// SCRIPTURAL WITNESS:
//   "Write the vision, and make it plain upon tables, that he may run that readeth it."
//   — Habakkuk 2:2 (KJV)
//
// ARCHITECTURAL TOPOLOGY:
//   Connects VS Code editors to the OmniCode compiler, linter, and runtime engine:
//     - 3-Tier Composite Hover Provider (.omni, .tape, .scroll).
//     - Cross-Scale Definition Provider (F12: .tape roll -> .omni low-level contract).
//     - Document Symbol & Outline Provider (4-Blocks, Modules, Rolls, Asides).
//     - Real-Time Structural Diagnostics Collection (Linter on save).
//     - Multi-Tier Binary Locator (Workspace target/, ~/.cargo/bin, $PATH, HTTP server).
// ==============================================================================

const vscode = require('vscode');
const { execFile } = require('child_process');
const http = require('http');
const path = require('path');
const fs = require('fs');

let outputChannel;
let diagnosticCollection;

/**
 * Resolves the absolute path to the omnicode binary across workspace,
 * cargo bin directory, and system environment.
 */
function resolveOmnicodeBinary() {
    const config = vscode.workspace.getConfiguration('omnicode');
    const customPath = config.get('executablePath');
    if (customPath && fs.existsSync(customPath)) {
        return customPath;
    }

    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
        for (const folder of workspaceFolders) {
            const root = folder.uri.fsPath;
            const debugBin = path.join(root, 'target', 'debug', 'omnicode');
            if (fs.existsSync(debugBin)) {
                return debugBin;
            }
            const releaseBin = path.join(root, 'target', 'release', 'omnicode');
            if (fs.existsSync(releaseBin)) {
                return releaseBin;
            }
            const track1DebugBin = path.join(root, '<TRACK 1> : {Host Engine & Virtual Machine}', 'target', 'debug', 'omnicode');
            if (fs.existsSync(track1DebugBin)) {
                return track1DebugBin;
            }
        }
    }

    const homeDir = process.env.HOME || process.env.USERPROFILE || '';
    if (homeDir) {
        const cargoBin = path.join(homeDir, '.cargo', 'bin', 'omnicode');
        if (fs.existsSync(cargoBin)) {
            return cargoBin;
        }
    }

    return 'omnicode';
}

/**
 * Executes a CLI query against the OmniCode language engine.
 */
function queryOmnicodeCli(args, timeoutMs = 4000) {
    return new Promise((resolve, reject) => {
        const binary = resolveOmnicodeBinary();
        execFile(binary, args, { timeout: timeoutMs }, (err, stdout, stderr) => {
            if (err) {
                return reject(err);
            }
            resolve(stdout ? stdout.trim() : '');
        });
    });
}

/**
 * Fallback query to the running OmniCode Studio HTTP server.
 */
function queryHttpHover(filePath, line, col, serverUrl = 'http://127.0.0.1:8080') {
    return new Promise((resolve) => {
        const url = `${serverUrl}/api/hover?file=${encodeURIComponent(filePath)}&line=${line}&col=${col}`;
        const req = http.get(url, (res) => {
            let body = '';
            res.on('data', (chunk) => (body += chunk));
            res.on('end', () => {
                try {
                    const data = JSON.parse(body);
                    resolve(data);
                } catch (_e) {
                    resolve(null);
                }
            });
        });
        req.on('error', () => resolve(null));
        req.setTimeout(1500, () => {
            req.destroy();
            resolve(null);
        });
    });
}

/**
 * Runs the 4-Block structural linter on the active document and posts diagnostics.
 */
async function runDocumentLinter(document) {
    if (!document || !['omnicode'].includes(document.languageId)) {
        return;
    }

    const config = vscode.workspace.getConfiguration('omnicode');
    if (!config.get('enableLintOnSave', true)) {
        return;
    }

    const filePath = document.fileName;
    try {
        const binary = resolveOmnicodeBinary();
        execFile(binary, ['lint', filePath], { timeout: 5000 }, (err, stdout, stderr) => {
            diagnosticCollection.delete(document.uri);
            const diagnostics = [];
            const output = stdout + '\n' + stderr;

            const lines = output.split('\n');
            for (const line of lines) {
                // Parse: [FAILPOINT] line N: message or Error at line N:
                const lineMatch = line.match(/(?:line\s+|:)(\d+)(?::\s*|\s*-\s*)(.*)/i);
                if (lineMatch) {
                    const lineNum = Math.max(0, parseInt(lineMatch[1], 10) - 1);
                    const msg = lineMatch[2] || line;
                    const range = new vscode.Range(lineNum, 0, lineNum, 100);
                    const severity = line.toLowerCase().includes('failpoint') || line.toLowerCase().includes('error')
                        ? vscode.DiagnosticSeverity.Error
                        : vscode.DiagnosticSeverity.Warning;
                    diagnostics.push(new vscode.Diagnostic(range, msg, severity));
                }
            }
            diagnosticCollection.set(document.uri, diagnostics);
        });
    } catch (e) {
        outputChannel.appendLine(`[Linter Error]: ${e.message}`);
    }
}

/**
 * Activates the OmniCode VS Code Extension.
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    outputChannel = vscode.window.createOutputChannel('OmniCode Language Engine');
    diagnosticCollection = vscode.languages.createDiagnosticCollection('omnicode');
    context.subscriptions.push(outputChannel, diagnosticCollection);

    outputChannel.appendLine('[OmniCode] Activating extension v0.2.0...');
    const resolvedBin = resolveOmnicodeBinary();
    outputChannel.appendLine(`[OmniCode] Engine Binary Resolved: ${resolvedBin}`);

    // --------------------------------------------------------------------------
    // 1. 3-TIER COMPOSITE HOVER PROVIDER
    // --------------------------------------------------------------------------
    const hoverProvider = vscode.languages.registerHoverProvider('omnicode', {
        async provideHover(document, position, token) {
            const config = vscode.workspace.getConfiguration('omnicode');
            if (!config.get('enableHover', true)) {
                return null;
            }

            const range = document.getWordRangeAtPosition(position);
            if (!range) {
                return null;
            }

            const symbol = document.getText(range);
            const line = position.line + 1;
            const col = position.character + 1;
            const filePath = document.fileName;

            // Attempt 1: Direct native CLI query
            try {
                const stdout = await queryOmnicodeCli(['hover', filePath, String(line), String(col), '--json']);
                if (stdout) {
                    const data = JSON.parse(stdout);
                    if (data && data.markdown) {
                        const md = new vscode.MarkdownString(data.markdown);
                        md.isTrusted = true;
                        md.supportHtml = true;
                        return new vscode.Hover(md, range);
                    }
                }
            } catch (err) {
                outputChannel.appendLine(`[Hover CLI Miss]: ${err.message || err}`);
            }

            // Attempt 2: HTTP server query fallback
            const serverUrl = config.get('serverUrl', 'http://127.0.0.1:8080');
            const httpData = await queryHttpHover(filePath, line, col, serverUrl);
            if (httpData && httpData.markdown) {
                const md = new vscode.MarkdownString(httpData.markdown);
                md.isTrusted = true;
                md.supportHtml = true;
                return new vscode.Hover(md, range);
            }

            return null;
        }
    });

    // --------------------------------------------------------------------------
    // 2. DEFINITION PROVIDER (F12: Jump .tape Roll -> Low-Level .omni Blueprint)
    // --------------------------------------------------------------------------
    const definitionProvider = vscode.languages.registerDefinitionProvider('omnicode', {
        async provideDefinition(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            if (!range) {
                return null;
            }

            const line = position.line + 1;
            const col = position.character + 1;
            const filePath = document.fileName;

            try {
                const stdout = await queryOmnicodeCli(['hover', filePath, String(line), String(col), '--json']);
                if (stdout) {
                    const data = JSON.parse(stdout);
                    if (data && data.omni_definition) {
                        const def = data.omni_definition;
                        if (def.source_file && def.line) {
                            let targetPath = def.source_file;
                            if (!path.isAbsolute(targetPath)) {
                                const workspaceFolders = vscode.workspace.workspaceFolders;
                                if (workspaceFolders && workspaceFolders.length > 0) {
                                    targetPath = path.join(workspaceFolders[0].uri.fsPath, targetPath);
                                }
                            }
                            if (fs.existsSync(targetPath)) {
                                const targetUri = vscode.Uri.file(targetPath);
                                const targetPos = new vscode.Position(Math.max(0, def.line - 1), 0);
                                return new vscode.Location(targetUri, targetPos);
                            }
                        }
                    }
                }
            } catch (e) {
                outputChannel.appendLine(`[Definition Resolution Error]: ${e.message}`);
            }

            return null;
        }
    });

    // --------------------------------------------------------------------------
    // 3. DOCUMENT SYMBOL / OUTLINE PROVIDER (4-Blocks, Modules, Rolls)
    // --------------------------------------------------------------------------
    const symbolProvider = vscode.languages.registerDocumentSymbolProvider('omnicode', {
        provideDocumentSymbols(document, token) {
            const symbols = [];
            const text = document.getText();
            const lines = text.split('\n');

            for (let i = 0; i < lines.length; i++) {
                const rawLine = lines[i];
                const trimmed = rawLine.trim();

                // 4-Block Headers
                const blockMatch = trimmed.match(/^#?\s*(BLOCK\s+[1-4]:\s+[A-Z\s]+)/i);
                if (blockMatch) {
                    const blockName = blockMatch[1];
                    const range = new vscode.Range(i, 0, i, rawLine.length);
                    const sym = new vscode.DocumentSymbol(
                        blockName,
                        '4-Block Canonical Architectural Zone',
                        vscode.SymbolKind.Namespace,
                        range,
                        range
                    );
                    symbols.push(sym);
                    continue;
                }

                // Process Tape Rolls
                const rollMatch = trimmed.match(/^\[\s*ROLL\s+(\d+)\s*\](?:\s*:\s*\[\s*([^\]]+)\s*\])?/i);
                if (rollMatch) {
                    const rollNum = rollMatch[1];
                    const rollAction = rollMatch[2] || 'Execution Step';
                    const range = new vscode.Range(i, 0, i, rawLine.length);
                    const sym = new vscode.DocumentSymbol(
                        `Roll ${rollNum}: ${rollAction}`,
                        'Atomic Process Ribbon Step',
                        vscode.SymbolKind.Function,
                        range,
                        range
                    );
                    symbols.push(sym);
                    continue;
                }

                // Invariant Definitions < Name > = ...
                const invMatch = trimmed.match(/^<([a-zA-Z_][a-zA-Z0-9_]*)>\s*=/);
                if (invMatch) {
                    const name = invMatch[1];
                    const range = new vscode.Range(i, 0, i, rawLine.length);
                    const sym = new vscode.DocumentSymbol(
                        `< ${name} >`,
                        'Rank 1 Axiomatic Invariant Type',
                        vscode.SymbolKind.Class,
                        range,
                        range
                    );
                    symbols.push(sym);
                    continue;
                }

                // Module Declarations = [Module]: < ... >
                const modMatch = trimmed.match(/^=\s*\[(Module|Tape|Scroll)\]:\s*<([^>]+)>/i);
                if (modMatch) {
                    const kind = modMatch[1];
                    const target = modMatch[2];
                    const range = new vscode.Range(i, 0, i, rawLine.length);
                    const sym = new vscode.DocumentSymbol(
                        `= [${kind}]: < ${target} >`,
                        'Top-Level Reel Declaration',
                        vscode.SymbolKind.Module,
                        range,
                        range
                    );
                    symbols.push(sym);
                }
            }

            return symbols;
        }
    });

    // --------------------------------------------------------------------------
    // 4. EDITOR COMMANDS & WORKSPACE EVENT LISTENERS
    // --------------------------------------------------------------------------
    context.subscriptions.push(
        hoverProvider,
        definitionProvider,
        symbolProvider,
        vscode.commands.registerCommand('omnicode.hover', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                await vscode.commands.executeCommand('editor.action.showHover');
            }
        }),
        vscode.commands.registerCommand('omnicode.lint', async () => {
            const editor = vscode.window.activeTextEditor;
            if (editor) {
                await runDocumentLinter(editor.document);
                vscode.window.showInformationMessage('OmniCode: 4-Block lint verification complete.');
            }
        }),
        vscode.commands.registerCommand('omnicode.assemble', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            const filePath = editor.document.fileName;
            if (!filePath.endsWith('.tape')) {
                vscode.window.showWarningMessage('OmniCode: Active file is not a process tape (.tape).');
                return;
            }
            try {
                const stdout = await queryOmnicodeCli(['assemble', filePath]);
                outputChannel.appendLine(`[Assemble Success]:\n${stdout}`);
                vscode.window.showInformationMessage('OmniCode: Process tape assembled into memory words.');
            } catch (err) {
                vscode.window.showErrorMessage(`OmniCode Assemble Failed: ${err.message}`);
            }
        }),
        vscode.commands.registerCommand('omnicode.studio', () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) return;
            const filePath = editor.document.fileName;
            const binary = resolveOmnicodeBinary();
            const terminal = vscode.window.createTerminal('OmniStudio Projector');
            terminal.show();
            terminal.sendText(`"${binary}" studio "${filePath}"`);
        }),
        vscode.workspace.onDidSaveTextDocument((doc) => {
            if (doc.languageId === 'omnicode') {
                runDocumentLinter(doc);
            }
        }),
        vscode.workspace.onDidOpenTextDocument((doc) => {
            if (doc.languageId === 'omnicode') {
                runDocumentLinter(doc);
            }
        })
    );

    // Initial lint run if editor is active
    if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document.languageId === 'omnicode') {
        runDocumentLinter(vscode.window.activeTextEditor.document);
    }

    outputChannel.appendLine('[OmniCode] Language Support activated successfully.');
}

function deactivate() {
    if (diagnosticCollection) {
        diagnosticCollection.clear();
    }
}

module.exports = {
    activate,
    deactivate,
};
