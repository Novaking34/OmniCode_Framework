// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( OmniCode Native Interactive Terminal & Task Scheduler JavaScript Engine )
// ==============================================================================
/**
 * OmniCode Terminal & Scheduler Frontend Engine
 * CreativeWorkzStudio LLC
 * Zero external dependencies. Connects via standard HTTP JSON endpoints:
 *   - POST /api/terminal          -> Dispatches command to session & scheduler
 *   - GET  /api/scheduler         -> Retrieves live process ledger
 *   - POST /api/scheduler/spawn   -> Spawns new task
 *   - POST /api/scheduler/action  -> Mutates task state (pause, resume, kill)
 */

document.addEventListener("DOMContentLoaded", () => {
    const consoleScreen = document.getElementById("console-screen");
    const historyLog = document.getElementById("term-history-log");
    const termInput = document.getElementById("term-input");
    const btnSend = document.getElementById("btn-term-send");
    const btnClear = document.getElementById("btn-clear-term");
    const taskTbody = document.getElementById("task-tbody");
    const schedPillText = document.getElementById("sched-pill-text");
    const varsContent = document.getElementById("session-vars-content");

    // Command History & Autocomplete State
    const history = JSON.parse(localStorage.getItem("omnicode_terminal_history") || "[]");
    let historyIndex = history.length;

    const AUTOCOMPLETE_LIST = [
        "ps", "tasks", "spawn", "pause", "resume", "kill", "halt", "tick", "vars", "env",
        "let", "help", "clear",
        "omnicode demo",
        "omnicode lint manifest.omni",
        "omnicode restore manifest.omni",
        "omnicode manifest",
        "omnicode scrolls",
        "omnicode lexicon truth",
        "omnicode ai \"What is the covenant?\""
    ];

    // Focus input on console click
    consoleScreen.addEventListener("click", () => {
        termInput.focus();
    });

    // --------------------------------------------------------------------------
    // EXECUTION DISPATCHER
    // --------------------------------------------------------------------------
    async function executeCommand(cmd) {
        const clean = cmd.trim();
        if (!clean) return;

        // Save to history
        if (history.length === 0 || history[history.length - 1] !== clean) {
            history.push(clean);
            if (history.length > 100) history.shift();
            localStorage.setItem("omnicode_terminal_history", JSON.stringify(history));
        }
        historyIndex = history.length;

        // Handle local 'clear'
        if (clean.toLowerCase() === "clear") {
            historyLog.innerHTML = "";
            termInput.value = "";
            return;
        }

        // Render command prompt entry
        const entryEl = document.createElement("div");
        entryEl.className = "term-entry";
        entryEl.innerHTML = `
            <div class="term-command-line">
                <span class="term-prompt">omnicode&gt;</span>
                <span class="term-cmd-text">${escapeHtml(clean)}</span>
            </div>
            <div class="term-loading-spinner dim-text">Deliberating &amp; evaluating...</div>
        `;
        historyLog.appendChild(entryEl);
        consoleScreen.scrollTop = consoleScreen.scrollHeight;
        termInput.value = "";

        const spinner = entryEl.querySelector(".term-loading-spinner");

        try {
            const resp = await fetch("/api/terminal", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ command: clean })
            });

            const data = await resp.json();
            spinner.remove();

            const resultEl = document.createElement("div");
            resultEl.className = "term-result-box";

            const stateClass = getStateClass(data.state || "+1:PROCEED");
            const durationMs = (data.duration_us / 1000).toFixed(2);

            resultEl.innerHTML = `
                <div class="term-meta-bar">
                    <span class="badge ${stateClass}">${escapeHtml(data.state || "+1:PROCEED")}</span>
                    <span class="dim-text">${durationMs} ms</span>
                </div>
                <pre class="term-output-text">${formatTerminalOutput(data.stdout || "(Completed with zero output)")}</pre>
            `;
            entryEl.appendChild(resultEl);
        } catch (err) {
            spinner.remove();
            const errEl = document.createElement("div");
            errEl.className = "term-error-box";
            errEl.innerHTML = `<pre class="text-red">✗ Error dispatching command: ${escapeHtml(err.message)}</pre>`;
            entryEl.appendChild(errEl);
        }

        consoleScreen.scrollTop = consoleScreen.scrollHeight;
        refreshSchedulerTasks();
        refreshVariables();
    }

    // --------------------------------------------------------------------------
    // SCHEDULER TASK POLLING & ACTIONS
    // --------------------------------------------------------------------------
    async function refreshSchedulerTasks() {
        try {
            const resp = await fetch("/api/scheduler");
            if (!resp.ok) return;
            const tasks = await resp.json();

            taskTbody.innerHTML = "";
            let activeCount = 0;

            tasks.forEach(t => {
                if (t.state === "+1:PROCEED") activeCount++;
                const tr = document.createElement("tr");
                const stateClass = getStateClass(t.state);

                tr.innerHTML = `
                    <td class="mono font-bold">${t.id}</td>
                    <td title="${escapeHtml(t.command)}"><strong>${escapeHtml(t.name)}</strong></td>
                    <td><span class="badge badge-subtle">${escapeHtml(t.priority)}</span></td>
                    <td><span class="badge ${stateClass}">${escapeHtml(t.state)}</span></td>
                    <td class="mono">${t.ticks}</td>
                    <td class="action-cell">
                        ${t.state === "+1:PROCEED" 
                            ? `<button class="btn btn-xs btn-ghost btn-pause" data-id="${t.id}" title="Pause (0:AWAIT)">⏸</button>`
                            : `<button class="btn btn-xs btn-ghost btn-resume" data-id="${t.id}" title="Resume (+1:PROCEED)">▶</button>`}
                        ${t.state !== "NULL:HALT" 
                            ? `<button class="btn btn-xs btn-danger-ghost btn-halt" data-id="${t.id}" title="Terminate (NULL:HALT)">⏹</button>`
                            : `<span class="dim-text">Ended</span>`}
                    </td>
                `;
                taskTbody.appendChild(tr);
            });

            schedPillText.textContent = `Scheduler: Online (${tasks.length} tasks, ${activeCount} active)`;

            // Attach action listeners
            document.querySelectorAll(".btn-pause").forEach(b => {
                b.onclick = () => mutateTask(b.dataset.id, "pause");
            });
            document.querySelectorAll(".btn-resume").forEach(b => {
                b.onclick = () => mutateTask(b.dataset.id, "resume");
            });
            document.querySelectorAll(".btn-halt").forEach(b => {
                b.onclick = () => mutateTask(b.dataset.id, "halt");
            });
        } catch (e) {
            console.error("Scheduler fetch error:", e);
        }
    }

    async function mutateTask(id, action) {
        await executeCommand(`${action} ${id}`);
    }

    async function refreshVariables() {
        try {
            const resp = await fetch("/api/terminal", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ command: "vars" })
            });
            const data = await resp.json();
            varsContent.innerHTML = `<pre class="mono-code">${escapeHtml(data.stdout || "No session variables")}</pre>`;
        } catch (e) {
            console.error(e);
        }
    }

    // --------------------------------------------------------------------------
    // KEYBOARD NAVIGATION & SHORTCUTS
    // --------------------------------------------------------------------------
    termInput.addEventListener("keydown", (e) => {
        // Enter to execute
        if (e.key === "Enter") {
            e.preventDefault();
            executeCommand(termInput.value);
            return;
        }

        // Up arrow (history backward)
        if (e.key === "ArrowUp") {
            e.preventDefault();
            if (historyIndex > 0) {
                historyIndex--;
                termInput.value = history[historyIndex] || "";
            }
            return;
        }

        // Down arrow (history forward)
        if (e.key === "ArrowDown") {
            e.preventDefault();
            if (historyIndex < history.length - 1) {
                historyIndex++;
                termInput.value = history[historyIndex] || "";
            } else {
                historyIndex = history.length;
                termInput.value = "";
            }
            return;
        }

        // Tab autocomplete
        if (e.key === "Tab") {
            e.preventDefault();
            const val = termInput.value.trim().toLowerCase();
            if (!val) return;
            const match = AUTOCOMPLETE_LIST.find(item => item.toLowerCase().startsWith(val));
            if (match) {
                termInput.value = match;
            }
            return;
        }

        // Ctrl + L (Clear)
        if (e.ctrlKey && e.key.toLowerCase() === "l") {
            e.preventDefault();
            historyLog.innerHTML = "";
            return;
        }
    });

    btnSend.addEventListener("click", () => {
        executeCommand(termInput.value);
    });

    btnClear.addEventListener("click", () => {
        historyLog.innerHTML = "";
        termInput.focus();
    });

    // Toolbar Quick Action Buttons
    document.getElementById("btn-quick-demo")?.addEventListener("click", () => executeCommand("omnicode demo"));
    document.getElementById("btn-quick-lint")?.addEventListener("click", () => executeCommand("omnicode lint manifest.omni"));
    document.getElementById("btn-quick-restore")?.addEventListener("click", () => executeCommand("omnicode restore manifest.omni"));
    document.getElementById("btn-quick-tasks")?.addEventListener("click", () => executeCommand("ps"));
    document.getElementById("btn-quick-tick")?.addEventListener("click", () => executeCommand("tick"));
    document.getElementById("btn-refresh-tasks")?.addEventListener("click", refreshSchedulerTasks);
    document.getElementById("btn-refresh-vars")?.addEventListener("click", refreshVariables);

    // Spawn Task Button
    document.getElementById("btn-spawn-task")?.addEventListener("click", () => {
        const nameInput = document.getElementById("spawn-name-input");
        const cmdInput = document.getElementById("spawn-cmd-input");
        const prioSelect = document.getElementById("spawn-priority-select");

        const cmd = cmdInput.value.trim();
        if (!cmd) return;

        executeCommand(`spawn ${cmd}`);
        cmdInput.value = "";
        nameInput.value = "";
    });

    // Helper functions
    function getStateClass(state) {
        if (!state) return "badge-proceed";
        if (state.includes("PROCEED")) return "badge-proceed";
        if (state.includes("AWAIT")) return "badge-await";
        if (state.includes("ROLLBACK")) return "badge-rollback";
        if (state.includes("HALT")) return "badge-halt";
        return "badge-proceed";
    }

    function escapeHtml(str) {
        return (str || "")
            .replace(/&/g, "&amp;")
            .replace(/</g, "&lt;")
            .replace(/>/g, "&gt;")
            .replace(/"/g, "&quot;")
            .replace(/'/g, "&#039;");
    }

    function formatTerminalOutput(text) {
        return escapeHtml(text);
    }

    // Initial scheduler poll & timer
    refreshSchedulerTasks();
    refreshVariables();
    setInterval(refreshSchedulerTasks, 4000);
});
