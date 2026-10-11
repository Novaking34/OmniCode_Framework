// ==============================================================================
// OMNICODE STUDIO & COGNITIVE TERMINAL APPLICATION LOGIC
// Grounded in Balanced Ternary Quad-State & Biblical Moral Ontology
// ==============================================================================

document.addEventListener('DOMContentLoaded', () => {
    initNavigation();
    initPromptStudio();
    initTerminalGateway();
    initLexiconExplorer();
    initEpisodicScrolls();
    initHoverInspector();
});

// ------------------------------------------------------------------------------
// 1. NAVIGATION TAB SWITCHING
// ------------------------------------------------------------------------------
function initNavigation() {
    const tabs = document.querySelectorAll('.nav-tab');
    const panels = document.querySelectorAll('.view-panel');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const targetViewId = tab.getAttribute('data-view');

            tabs.forEach(t => t.classList.remove('active'));
            panels.forEach(p => p.classList.remove('active'));

            tab.classList.add('active');
            const targetPanel = document.getElementById(targetViewId);
            if (targetPanel) {
                targetPanel.classList.add('active');
            }

            // Trigger data load if needed
            if (targetViewId === 'view-lexicon') {
                loadLexiconData();
            } else if (targetViewId === 'view-scrolls') {
                loadScrollsData();
            }
        });
    });
}

// ------------------------------------------------------------------------------
// 2. AI PROMPT STUDIO CONTROLLER
// ------------------------------------------------------------------------------
function initPromptStudio() {
    const promptInput = document.getElementById('prompt-input');
    const submitBtn = document.getElementById('btn-submit-prompt');
    const promptChips = document.querySelectorAll('.prompt-chip');
    const quadstateBadge = document.getElementById('studio-quadstate-badge');
    const stateLabel = document.getElementById('state-label');
    const responseContent = document.getElementById('response-content');
    const synthesizedCode = document.getElementById('synthesized-code');
    const badgeSession = document.getElementById('badge-session-id');
    const linkScroll = document.getElementById('link-scroll');
    const linkLog = document.getElementById('link-log');
    const copyCodeBtn = document.getElementById('btn-copy-code');

    // Quick Prompt Chips
    promptChips.forEach(chip => {
        chip.addEventListener('click', () => {
            const promptText = chip.getAttribute('data-prompt');
            promptInput.value = promptText;
            promptInput.focus();
        });
    });

    // Copy Code Button
    copyCodeBtn.addEventListener('click', () => {
        const codeText = synthesizedCode.textContent;
        navigator.clipboard.writeText(codeText).then(() => {
            const originalText = copyCodeBtn.textContent;
            copyCodeBtn.textContent = 'Copied!';
            setTimeout(() => { copyCodeBtn.textContent = originalText; }, 1800);
        });
    });

    // Submit Prompt Execution
    submitBtn.addEventListener('click', async () => {
        const prompt = promptInput.value.trim();
        if (!prompt) return;

        // Set Loading State
        submitBtn.disabled = true;
        submitBtn.querySelector('.btn-text').textContent = 'Deliberating...';
        stateLabel.textContent = 'DELIBERATING (0:AWAIT)';
        quadstateBadge.className = 'quadstate-indicator';

        try {
            const res = await fetch('/api/prompt', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ prompt })
            });

            if (!res.ok) {
                const errData = await res.json().catch(() => ({ error: 'Unknown server error' }));
                throw new Error(errData.error || `HTTP ${res.status}`);
            }

            const data = await res.json();
            renderPromptResult(data);
        } catch (err) {
            console.error('Prompt evaluation error:', err);
            responseContent.innerHTML = `<p style="color: var(--state-rollback);">✗ Error evaluating prompt: ${err.message}</p>`;
            stateLabel.textContent = 'ERROR (NULL:HALT)';
            quadstateBadge.className = 'quadstate-indicator halt';
        } finally {
            submitBtn.disabled = false;
            submitBtn.querySelector('.btn-text').textContent = 'Send to AI Cortex';
        }
    });

    function renderPromptResult(data) {
        // 1. Update QuadState Badge
        const state = (data.state || 'PROCEED').toUpperCase();
        quadstateBadge.className = 'quadstate-indicator';

        if (state === 'PROCEED') {
            quadstateBadge.classList.add('proceed');
            stateLabel.textContent = '+1: PROCEED';
        } else if (state === 'ROLLBACK') {
            quadstateBadge.classList.add('rollback');
            stateLabel.textContent = '-1: ROLLBACK (CAVEAT VETO)';
        } else if (state === 'AWAIT') {
            quadstateBadge.classList.add('await');
            stateLabel.textContent = '0: AWAIT (DISCERNMENT)';
        } else {
            quadstateBadge.classList.add('halt');
            stateLabel.textContent = 'NULL: HALT (TERMINAL TRAP)';
        }

        // 2. Session and Response Content
        badgeSession.textContent = `Session: ${data.session_id || 'active'}`;
        responseContent.textContent = data.response || 'No natural discourse generated.';

        // 3. Synthesized Code Block
        synthesizedCode.textContent = data.omnicode || '// Zero code generated.';

        // 4. File Links
        linkScroll.textContent = `📜 Scroll: ${data.scroll || 'Recorded'}`;
        linkLog.textContent = `📋 Log: ${data.log || 'Recorded'}`;
    }
}

// ------------------------------------------------------------------------------
// 3. CLI TERMINAL GATEWAY CONTROLLER
// ------------------------------------------------------------------------------
function initTerminalGateway() {
    const termInput = document.getElementById('terminal-cmd-input');
    const runBtn = document.getElementById('btn-run-cmd');
    const termStream = document.getElementById('terminal-stream');
    const clearBtn = document.getElementById('btn-clear-terminal');
    const shortcutBtns = document.querySelectorAll('.term-btn');
    const termWindow = document.getElementById('terminal-window');

    const desktopBtn = document.getElementById('btn-desktop-launch');
    if (desktopBtn) {
        desktopBtn.addEventListener('click', () => {
            executeTerminalCommand('omnicode desktop');
        });
    }

    clearBtn.addEventListener('click', () => {
        termStream.innerHTML = '';
    });

    shortcutBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const cmd = btn.getAttribute('data-cmd');
            termInput.value = cmd;
            executeTerminalCommand(cmd);
        });
    });

    runBtn.addEventListener('click', () => {
        const cmd = termInput.value.trim();
        if (!cmd) return;
        executeTerminalCommand(cmd);
        termInput.value = '';
    });

    termInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const cmd = termInput.value.trim();
            if (!cmd) return;
            executeTerminalCommand(cmd);
            termInput.value = '';
        }
    });

    async function executeTerminalCommand(command) {
        appendTerminalLine(`omnicode> ${command}`, 'term-line-cmd');

        try {
            const res = await fetch('/api/terminal', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ command })
            });

            const data = await res.json();
            if (data.stdout) {
                appendTerminalLine(data.stdout, 'term-line-stdout');
            }
            if (data.stderr) {
                appendTerminalLine(data.stderr, 'term-line-stderr');
            }
        } catch (err) {
            appendTerminalLine(`Error executing command: ${err.message}`, 'term-line-stderr');
        }

        termWindow.scrollTop = termWindow.scrollHeight;
    }

    function appendTerminalLine(text, cssClass) {
        const div = document.createElement('div');
        div.className = cssClass;
        div.textContent = text;
        termStream.appendChild(div);
    }
}

// ------------------------------------------------------------------------------
// 4. COVENANT MANIFEST & BIBLICAL LEXICON CONTROLLER
// ------------------------------------------------------------------------------
let allLexiconEntries = [];

function initLexiconExplorer() {
    const searchInput = document.getElementById('lexicon-search-input');
    const filterBtns = document.querySelectorAll('.filter-btn');

    searchInput.addEventListener('input', () => {
        filterAndRenderLexicon();
    });

    filterBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            filterBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
            filterAndRenderLexicon();
        });
    });
}

async function loadLexiconData() {
    const entriesGrid = document.getElementById('lexicon-entries-grid');
    if (allLexiconEntries.length > 0) return; // Already cached

    entriesGrid.innerHTML = '<p class="placeholder-text">Loading biblical knowledge base from offline memory...</p>';

    try {
        const res = await fetch('/api/lexicon');
        const data = await res.json();
        allLexiconEntries = data.entries || [];
        filterAndRenderLexicon();
    } catch (err) {
        console.error('Failed to load lexicon:', err);
        entriesGrid.innerHTML = `<p style="color: var(--state-rollback);">Failed to load lexicon: ${err.message}</p>`;
    }
}

function filterAndRenderLexicon() {
    const entriesGrid = document.getElementById('lexicon-entries-grid');
    const searchInput = document.getElementById('lexicon-search-input');
    const query = searchInput.value.trim().toLowerCase();
    const activeFilterBtn = document.querySelector('.filter-btn.active');
    const selectedDomain = activeFilterBtn ? activeFilterBtn.getAttribute('data-domain') : 'all';

    const filtered = allLexiconEntries.filter(entry => {
        const matchesQuery = query === '' || 
            entry.term.toLowerCase().includes(query) || 
            entry.definition.toLowerCase().includes(query) ||
            entry.kjv.toLowerCase().includes(query) ||
            entry.web.toLowerCase().includes(query);

        const matchesDomain = selectedDomain === 'all' || entry.domain === selectedDomain;

        return matchesQuery && matchesDomain;
    });

    if (filtered.length === 0) {
        entriesGrid.innerHTML = `<p class="placeholder-text">No concepts found matching '${query}' in selected domain.</p>`;
        return;
    }

    entriesGrid.innerHTML = filtered.map(e => `
        <article class="entry-card">
            <div class="entry-header">
                <span class="entry-term">${escapeHtml(e.term)}</span>
                <span class="entry-domain">${escapeHtml(e.domain)}</span>
            </div>
            <p class="entry-def">${escapeHtml(e.definition)}</p>
            <div class="entry-witnesses">
                <p><span class="witness-title">KJV (1611):</span> ${escapeHtml(e.kjv)}</p>
                <p><span class="witness-title">WEB:</span> ${escapeHtml(e.web)}</p>
            </div>
            <p class="entry-guidance">⚖️ ${escapeHtml(e.guidance)}</p>
        </article>
    `).join('');
}

// ------------------------------------------------------------------------------
// 5. EPISODIC SCROLLS CONTROLLER
// ------------------------------------------------------------------------------
function initEpisodicScrolls() {
    const refreshBtn = document.getElementById('btn-refresh-scrolls');
    refreshBtn.addEventListener('click', () => {
        loadScrollsData();
    });
}

async function loadScrollsData() {
    const tbody = document.getElementById('episodes-tbody');
    tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; padding: 2rem;">Loading memory bank...</td></tr>';

    try {
        const res = await fetch('/api/scrolls');
        const data = await res.json();

        // Update Metrics
        document.getElementById('metric-total').textContent = data.total || 0;
        document.getElementById('metric-proceed').textContent = data.proceed || 0;
        document.getElementById('metric-await').textContent = data.await || 0;
        document.getElementById('metric-rollback').textContent = data.rollback || 0;
        document.getElementById('metric-halt').textContent = data.halt || 0;
        document.getElementById('metric-repairs').textContent = data.repairs_learned || 0;

        // Update Episodes Table
        const episodes = data.episodes || [];
        if (episodes.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" style="text-align:center; padding: 2rem; color: var(--text-muted);">No scrolls inscribed yet. Run prompts in Studio to record episodes.</td></tr>';
            return;
        }

        tbody.innerHTML = episodes.map(ep => {
            const stateClass = ep.state === 'PROCEED' ? 'state-proceed' :
                               ep.state === 'ROLLBACK' ? 'state-rollback' :
                               ep.state === 'AWAIT' ? 'state-await' : 'state-halt';
            return `
                <tr>
                    <td style="font-family: var(--font-mono); color: var(--cyan);">${escapeHtml(ep.session_id)}</td>
                    <td>${escapeHtml(ep.prompt)}</td>
                    <td><span class="state-chip ${stateClass}">+1:${escapeHtml(ep.state)}</span></td>
                    <td style="text-align: center;">${ep.repairs}</td>
                    <td style="text-align: center;">${ep.failpoints}</td>
                    <td style="font-family: var(--font-mono); font-size: 0.75rem; color: var(--text-muted);">${escapeHtml(ep.scroll)}</td>
                </tr>
            `;
        }).join('');
    } catch (err) {
        console.error('Failed to load scrolls:', err);
        tbody.innerHTML = `<tr><td colspan="6" style="color: var(--state-rollback); text-align: center;">Error: ${err.message}</td></tr>`;
    }
}

// Utility: HTML Escaping
function escapeHtml(str) {
    if (!str) return '';
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#039;');
}

// ------------------------------------------------------------------------------
// 6. 3-TIER HOVER INSPECTOR CONTROLLER (.omni <-> .tape)
// ------------------------------------------------------------------------------
function initHoverInspector() {
    let tooltipEl = document.querySelector('.omni-hover-card');
    if (!tooltipEl) {
        tooltipEl = document.createElement('div');
        tooltipEl.className = 'omni-hover-card';
        tooltipEl.style.display = 'none';
        document.body.appendChild(tooltipEl);
    }

    let hideTimer = null;

    document.addEventListener('mouseover', async (e) => {
        const target = e.target;
        if (!target) return;

        const codeContainer = target.closest('pre, code, .code-editor, #synthesized-code');
        if (!codeContainer) {
            return;
        }

        const word = getWordUnderCursor(e);
        if (!word || word.length < 2 || /^\d+$/.test(word)) {
            return;
        }

        clearTimeout(hideTimer);

        try {
            const res = await fetch(`/api/hover?symbol=${encodeURIComponent(word)}`);
            if (!res.ok) return;
            const data = await res.json();
            if (!data || !data.symbol) return;

            renderHoverCard(tooltipEl, data, e.pageX, e.pageY);
        } catch (_err) {
            // Ignored
        }
    });

    document.addEventListener('mouseout', (e) => {
        if (tooltipEl.contains(e.relatedTarget)) return;
        hideTimer = setTimeout(() => {
            tooltipEl.style.display = 'none';
        }, 300);
    });

    tooltipEl.addEventListener('mouseenter', () => clearTimeout(hideTimer));
    tooltipEl.addEventListener('mouseleave', () => {
        tooltipEl.style.display = 'none';
    });
}

function getWordUnderCursor(event) {
    if (document.caretRangeFromPoint) {
        const range = document.caretRangeFromPoint(event.clientX, event.clientY);
        if (range && range.startContainer && range.startContainer.nodeType === Node.TEXT_NODE) {
            const text = range.startContainer.textContent;
            const offset = range.startOffset;
            const match = text.slice(0, offset).match(/[a-zA-Z0-9_]+$/);
            const afterMatch = text.slice(offset).match(/^[a-zA-Z0-9_]+/);
            const before = match ? match[0] : '';
            const after = afterMatch ? afterMatch[0] : '';
            return before + after;
        }
    }
    return null;
}

function renderHoverCard(cardEl, data, x, y) {
    const omni = data.omni_definition;
    const tape = data.tape_context;

    const rankBadge = omni 
        ? `<span class="omni-hover-rank-badge rank-${omni.rank}">Rank ${omni.rank}: ${escapeHtml(omni.boundary_type)}</span>`
        : (data.is_locally_redefined ? `<span class="omni-hover-rank-badge">Rank 2: Denotation</span>` : `<span class="omni-hover-rank-badge">Tape Entity</span>`);

    let tapeHtml = '';
    if (tape) {
        tapeHtml = `
            <div class="omni-hover-tier">
                <div class="omni-hover-tier-title tier-tape">📜 1. High-Level Process Tape</div>
                <div class="omni-hover-prop"><span class="omni-hover-label">Ribbon:</span><span class="omni-hover-val">${escapeHtml(tape.tape_name)} (${escapeHtml(tape.track)})</span></div>
                <div class="omni-hover-prop"><span class="omni-hover-label">Active Roll:</span><span class="omni-hover-val">${tape.roll_number}/${tape.total_rolls}</span></div>
                ${tape.roll_aside_intent ? `<div class="omni-hover-aside">"${escapeHtml(tape.roll_aside_intent)}"</div>` : ''}
                <div class="omni-hover-prop"><span class="omni-hover-label">Head Covenant:</span><span class="omni-hover-val">WHO ${escapeHtml(tape.head_who)} &rarr; WHAT ${escapeHtml(tape.head_what)}</span></div>
            </div>
        `;
    }

    let omniHtml = '';
    if (omni) {
        omniHtml = `
            <div class="omni-hover-tier">
                <div class="omni-hover-tier-title tier-omni">🔍 2. Under-The-Hood (.omni Blueprint)</div>
                <div class="omni-hover-prop"><span class="omni-hover-label">Grounded Source:</span><span class="omni-hover-val">${escapeHtml(omni.source_file)}:${omni.line}</span></div>
                <div class="omni-hover-prop"><span class="omni-hover-label">4-Block Origin:</span><span class="omni-hover-val">${escapeHtml(omni.block_kind || 'Universal Scope')}</span></div>
                ${omni.aside_intent ? `<div class="omni-hover-aside">"${escapeHtml(omni.aside_intent)}"</div>` : ''}
            </div>
        `;
    }

    const siliconHtml = `
        <div class="omni-hover-tier">
            <div class="omni-hover-tier-title tier-silicon">⚙️ 3. Machine Execution & Silicon State</div>
            <div class="omni-hover-prop"><span class="omni-hover-label">Partition Reservation:</span><span class="omni-hover-val">${tape ? tape.partition_size + ' cells' : '1024 cells'}</span></div>
            <div class="omni-hover-prop"><span class="omni-hover-label">Radix-3 Transducer:</span><span class="omni-hover-val">Dense Byte &harr; 5 Balanced Trits</span></div>
            <div class="omni-hover-prop"><span class="omni-hover-label">Silicon Truth State:</span><span class="omni-hover-val" style="color: var(--state-proceed);">+1:PROCEED (Deterministic Commit)</span></div>
        </div>
    `;

    cardEl.innerHTML = `
        <div class="omni-hover-header">
            <span class="omni-hover-symbol">${escapeHtml(data.symbol)}</span>
            ${rankBadge}
        </div>
        ${tapeHtml}
        ${omniHtml}
        ${siliconHtml}
    `;

    cardEl.style.display = 'block';
    const cardRect = cardEl.getBoundingClientRect();
    let left = x + 15;
    let top = y + 15;

    if (left + cardRect.width > window.innerWidth - 20) {
        left = x - cardRect.width - 15;
    }
    if (top + cardRect.height > window.innerHeight - 20) {
        top = y - cardRect.height - 15;
    }

    cardEl.style.left = `${Math.max(10, left)}px`;
    cardEl.style.top = `${Math.max(10, top)}px`;
}

