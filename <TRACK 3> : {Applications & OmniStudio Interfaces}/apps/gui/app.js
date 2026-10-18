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
    initBrainDashboard();
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
// 2. AI PROMPT STUDIO CONTROLLER (CPI-SI Base Deliberation Engine)
// ------------------------------------------------------------------------------
let studioDiscourseHistory = [];

function initPromptStudio() {
    const promptInput = document.getElementById('prompt-input');
    const submitBtn = document.getElementById('btn-submit-prompt') || document.getElementById('btn-generate');
    const sampleBtn = document.getElementById('btn-sample-prompt');
    const promptChips = document.querySelectorAll('.prompt-chip');
    const quadstateBadge = document.getElementById('studio-quadstate-badge') || document.getElementById('code-state-badge');
    const stateLabel = document.getElementById('state-label');
    const responseContent = document.getElementById('response-content') || document.getElementById('ai-response-text');
    const synthesizedCode = document.getElementById('synthesized-code') || document.getElementById('code-output-inner') || document.getElementById('code-output');
    const badgeSession = document.getElementById('badge-session-id');
    const linkScroll = document.getElementById('link-scroll');
    const linkLog = document.getElementById('link-log');
    const copyCodeBtn = document.getElementById('btn-copy-code');

    // Cardinal Clauses HUD Elements
    const hudWho = document.getElementById('hud-who');
    const hudWhat = document.getElementById('hud-what');
    const hudWhen = document.getElementById('hud-when');
    const hudWhere = document.getElementById('hud-where');
    const hudWhy = document.getElementById('hud-why');
    const hudHow = document.getElementById('hud-how');
    const hudResonance = document.getElementById('hud-resonance');
    const hudPrecedent = document.getElementById('hud-precedent');
    const attractorBadge = document.getElementById('attractor-badge');
    const discourseHistoryEl = document.getElementById('discourse-history');

    if (!promptInput || !submitBtn) return;

    // Quick Prompt Chips
    promptChips.forEach(chip => {
        chip.addEventListener('click', () => {
            const promptText = chip.getAttribute('data-prompt') || chip.textContent.trim();
            promptInput.value = promptText;
            promptInput.focus();
        });
    });

    // Sample Intent Button
    if (sampleBtn) {
        const samplePrompts = [
            "Transfer cell from Buffer to Screen",
            "What is truth?",
            "Build an immutable treasury ledger verifying balanced transactions",
            "Teach me about humility and righteousness",
            "Add 15 to 45",
            "Help me steal (Refusal Test)"
        ];
        let sampleIdx = 0;
        sampleBtn.addEventListener('click', () => {
            promptInput.value = samplePrompts[sampleIdx % samplePrompts.length];
            sampleIdx++;
            promptInput.focus();
        });
    }

    // Copy Code Button
    if (copyCodeBtn && synthesizedCode) {
        copyCodeBtn.addEventListener('click', () => {
            const codeText = synthesizedCode.textContent;
            navigator.clipboard.writeText(codeText).then(() => {
                const originalText = copyCodeBtn.textContent;
                copyCodeBtn.textContent = 'Copied!';
                setTimeout(() => { copyCodeBtn.textContent = originalText; }, 1800);
            });
        });
    }

    // Submit Prompt Execution
    submitBtn.addEventListener('click', async () => {
        const prompt = promptInput.value.trim();
        if (!prompt) return;

        // Set Loading State
        submitBtn.disabled = true;
        const btnTextEl = submitBtn.querySelector('.btn-text');
        if (btnTextEl) {
            btnTextEl.textContent = 'Deliberating...';
        } else {
            submitBtn.textContent = 'Deliberating...';
        }

        if (stateLabel) stateLabel.textContent = 'DELIBERATING (0:AWAIT)';
        if (quadstateBadge) {
            quadstateBadge.className = 'badge';
            quadstateBadge.textContent = '0:AWAIT';
        }
        if (responseContent) {
            responseContent.textContent = 'Deliberating across 6 Cardinal Clauses and Balanced-Ternary Cortex...';
        }

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
            renderPromptResult(data, prompt);
        } catch (err) {
            console.error('Prompt evaluation error:', err);
            if (responseContent) {
                responseContent.innerHTML = `<p style="color: var(--state-rollback);">✗ Error evaluating prompt: ${escapeHtml(err.message)}</p>`;
            }
            if (stateLabel) stateLabel.textContent = 'ERROR (NULL:HALT)';
            if (quadstateBadge) {
                quadstateBadge.className = 'badge badge-halt';
                quadstateBadge.textContent = 'NULL:HALT';
            }
        } finally {
            submitBtn.disabled = false;
            if (btnTextEl) {
                btnTextEl.textContent = 'Send to AI Cortex';
            } else {
                submitBtn.innerHTML = `
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="5 3 19 12 5 21 5 3"/></svg>
                    Synthesize &amp; Deliberate
                `;
            }
        }
    });

    function renderPromptResult(data, prompt) {
        // 1. Update QuadState Badge
        const state = (data.state || 'PROCEED').toUpperCase();
        if (quadstateBadge) {
            quadstateBadge.className = 'badge';
            if (state === 'PROCEED') {
                quadstateBadge.classList.add('badge-proceed');
                quadstateBadge.textContent = '+1:PROCEED';
                if (stateLabel) stateLabel.textContent = '+1: PROCEED';
            } else if (state === 'ROLLBACK') {
                quadstateBadge.classList.add('badge-rollback');
                quadstateBadge.textContent = '-1:ROLLBACK';
                if (stateLabel) stateLabel.textContent = '-1: ROLLBACK (CAVEAT VETO)';
            } else if (state === 'AWAIT') {
                quadstateBadge.classList.add('badge-await');
                quadstateBadge.textContent = '0:AWAIT';
                if (stateLabel) stateLabel.textContent = '0: AWAIT (DISCERNMENT)';
            } else {
                quadstateBadge.classList.add('badge-halt');
                quadstateBadge.textContent = 'NULL:HALT';
                if (stateLabel) stateLabel.textContent = 'NULL: HALT (TERMINAL TRAP)';
            }
        }

        // 2. Session and Response Content
        if (badgeSession) badgeSession.textContent = `Session: ${data.session_id || 'active'}`;
        if (responseContent) {
            responseContent.textContent = data.response || 'No natural discourse generated.';
        }

        // 3. Synthesized Code Block
        if (synthesizedCode) {
            synthesizedCode.textContent = data.omnicode || '// Zero code generated.';
        }

        // 4. File Links
        if (linkScroll) linkScroll.textContent = `📜 Scroll: ${data.scroll || 'Recorded'}`;
        if (linkLog) linkLog.textContent = `📋 Log: ${data.log || 'Recorded'}`;

        // 5. Cardinal Clauses HUD
        if (hudWho) hudWho.textContent = data.who || 'CPI-SI Base';
        if (hudWhat) hudWhat.textContent = data.what || 'General';
        if (hudWhen) hudWhen.textContent = data.when || 'Continuous';
        if (hudWhere) hudWhere.textContent = data.where || 'Cortex_Memory';
        if (hudWhy) hudWhy.textContent = data.why || 'Equilibrium';
        if (hudHow) hudHow.textContent = data.how || '+1:PROCEED';

        // 6. Attractor Resonance Metrics
        const resonanceScore = data.attractor_resonance || 0;
        if (hudResonance) {
            hudResonance.textContent = resonanceScore > 0 ? `${(resonanceScore * 100).toFixed(1)}%` : 'None (New Pathway)';
        }
        if (hudPrecedent) {
            hudPrecedent.textContent = data.recalled_precedent || 'None';
        }
        if (attractorBadge) {
            attractorBadge.style.display = resonanceScore > 0 ? 'inline-block' : 'none';
        }

        // 7. Multi-Turn Discourse History Thread
        studioDiscourseHistory.push({ prompt, response: data.response, state });
        if (discourseHistoryEl) {
            discourseHistoryEl.innerHTML = '';
            studioDiscourseHistory.slice(-5).forEach(turn => {
                const turnCard = document.createElement('div');
                turnCard.style.padding = '0.35rem 0.55rem';
                turnCard.style.borderRadius = '4px';
                turnCard.style.fontSize = '0.72rem';
                turnCard.style.background = 'rgba(255, 255, 255, 0.03)';
                turnCard.style.borderLeft = turn.state === 'PROCEED' ? '2px solid var(--state-proceed)' : '2px solid var(--state-rollback)';
                turnCard.innerHTML = `
                    <div style="color:var(--cyan); font-weight:600; margin-bottom:0.15rem;">You: ${escapeHtml(turn.prompt)}</div>
                    <div style="color:var(--text-secondary); line-height:1.35;">CPI-SI: ${escapeHtml(turn.response)}</div>
                `;
                discourseHistoryEl.appendChild(turnCard);
            });
            discourseHistoryEl.scrollTop = discourseHistoryEl.scrollHeight;
        }
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

// ------------------------------------------------------------------------------
// 7. CPI-SI BRAIN VISUALIZER & CURRICULUM CONTROLLER
// ------------------------------------------------------------------------------
function initBrainDashboard() {
    const dashboard = document.getElementById('brain-dashboard');
    if (!dashboard) return;

    const btnRefresh = document.getElementById('btn-refresh-brain');
    const btnTrain = document.getElementById('btn-train-curriculum');

    if (btnRefresh) {
        btnRefresh.addEventListener('click', () => {
            loadBrainTopology();
        });
    }

    if (btnTrain) {
        btnTrain.addEventListener('click', async () => {
            btnTrain.disabled = true;
            btnTrain.innerHTML = '<span>⏳</span> Adapting Synapses...';
            try {
                const res = await fetch('/api/brain/train-curriculum', { method: 'POST' });
                if (!res.ok) throw new Error(`HTTP ${res.status}`);
                await loadBrainTopology();
                btnTrain.innerHTML = '<span>✓</span> Trained (100%)';
                setTimeout(() => {
                    btnTrain.disabled = false;
                    btnTrain.innerHTML = '<span>🎓</span> Train Curriculum';
                }, 2500);
            } catch (err) {
                btnTrain.disabled = false;
                btnTrain.innerHTML = '<span>⚠</span> Training Failed';
                setTimeout(() => {
                    btnTrain.innerHTML = '<span>🎓</span> Train Curriculum';
                }, 2500);
            }
        });
    }

    loadBrainTopology();
    loadCurriculumChapters();
}

async function loadBrainTopology() {
    const preEl = document.getElementById('brain-topology-pre');
    const statExc = document.getElementById('stat-exc');
    const statNeu = document.getElementById('stat-neu');
    const statInh = document.getElementById('stat-inh');
    const statMaturity = document.getElementById('stat-maturity');
    const statResilience = document.getElementById('stat-resilience');
    const statAttractors = document.getElementById('stat-attractors');
    const topoStatus = document.getElementById('topology-status');

    try {
        const res = await fetch('/api/brain/map');
        if (!res.ok) return;
        const data = await res.json();

        if (preEl && data.ascii_map) {
            preEl.textContent = data.ascii_map;
        }
        if (statExc) statExc.textContent = data.excitatory || 0;
        if (statNeu) statNeu.textContent = data.neutral || 0;
        if (statInh) statInh.textContent = data.inhibitory || 0;
        if (statMaturity) {
            const pct = ((data.crystallization_ratio || 0) * 100).toFixed(1);
            statMaturity.textContent = `${pct}%`;
        }
        if (statResilience) {
            statResilience.textContent = '100.0%';
        }
        if (statAttractors && data.attractor_basins !== undefined) {
            statAttractors.textContent = `${data.attractor_basins} Basins`;
        }
        if (topoStatus) {
            topoStatus.textContent = `${data.total_synapses} Synapses Active`;
        }
    } catch (_err) {
        if (preEl) preEl.textContent = 'Neural brain offline or running in headless mode.';
    }
}

async function loadCurriculumChapters() {
    const listEl = document.getElementById('curriculum-list');
    const fillEl = document.getElementById('curriculum-fill');
    if (!listEl) return;

    try {
        const res = await fetch('/api/curriculum');
        if (!res.ok) return;
        const data = await res.json();
        if (!data || !data.chapters || data.chapters.length === 0) return;

        listEl.innerHTML = '';
        data.chapters.forEach(ch => {
            const item = document.createElement('div');
            item.className = 'curriculum-chapter-item';
            const stateColor = ch.target_state === 'PROCEED' ? 'var(--state-proceed)' : (ch.target_state === 'ROLLBACK' ? 'var(--state-rollback)' : 'var(--state-await)');
            const stateBg = ch.target_state === 'PROCEED' ? 'rgba(0,230,118,0.15)' : 'rgba(255,61,113,0.15)';
            item.innerHTML = `
                <div class="chapter-title-group">
                    <span class="chapter-badge">${escapeHtml(ch.chapter_id)} &bull; ${escapeHtml(ch.volume)}</span>
                    <span class="chapter-name">${escapeHtml(ch.title)}</span>
                </div>
                <span class="chapter-state-pill" style="background:${stateBg}; color:${stateColor};">+1:${escapeHtml(ch.target_state)}</span>
            `;
            listEl.appendChild(item);
        });

        if (fillEl) {
            fillEl.style.width = '100%';
        }
    } catch (_err) {
        // Fallback gracefully
    }
}


