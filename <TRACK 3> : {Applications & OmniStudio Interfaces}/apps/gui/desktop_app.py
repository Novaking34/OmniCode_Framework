#!/usr/bin/env python3
# ==============================================================================
# BLOCK 1: METADATA BLOCK
# ( CreativeWorkzStudio LLC — OmniCode Desktop GUI Studio & Terminal Shell )
# ==============================================================================
"""
OmniCode Desktop GUI Studio & Interactive Terminal Shell
A CreativeWorkzStudio LLC Project

Provides a native desktop interface for:
  1. Interactive GUI Terminal Shell for direct CLI execution
  2. AI Prompt Studio with real-time Balanced Ternary Quad-State deliberation
  3. Language & AI Architecture Reference Viewer
  4. Cloudflare Tunnel & Oracle Cloud VM Deployment Management
"""

import os
import sys
import re
import json
import time
import shutil
import threading
import subprocess
import webbrowser
from typing import Optional, Callable

# Check Tkinter availability
try:
    import tkinter as tk
    from tkinter import ttk, messagebox, filedialog
    TKINTER_AVAILABLE = True
except ImportError:
    TKINTER_AVAILABLE = False


# ==============================================================================
# BLOCK 2: OPENING BLOCK
# ( Constants, Paths, Color Palette & Utility Helpers )
# ==============================================================================
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

def find_repo_root(start_dir: str) -> str:
    """Finds repository root directory containing Cargo.toml."""
    current = start_dir
    while current and current != os.path.dirname(current):
        if os.path.exists(os.path.join(current, "Cargo.toml")):
            return current
        current = os.path.dirname(current)
    # Fallback heuristic: 3 levels up from <TRACK 3>/apps/gui
    return os.path.abspath(os.path.join(start_dir, "..", "..", ".."))

BASE_DIR = find_repo_root(SCRIPT_DIR)
SCRIPTS_DIR = os.path.join(BASE_DIR, "scripts")

# ANSI escape sequence stripper
ANSI_REGEX = re.compile(r'\x1b\[[0-9;]*[a-zA-Z]')

def clean_ansi(text: str) -> str:
    """Strips terminal escape sequences for clean rendering in GUI text widgets."""
    return ANSI_REGEX.sub('', text)

# Color Scheme: Sleek modern dark mode (GitHub Dark / Monokai Pro aesthetic)
THEME = {
    "bg_main": "#0d1117",
    "bg_surface": "#161b22",
    "bg_elevated": "#21262d",
    "border": "#30363d",
    "border_highlight": "#58a6ff",
    "text_primary": "#f0f6fc",
    "text_secondary": "#8b949e",
    "text_muted": "#484f58",
    "accent_blue": "#58a6ff",
    "accent_cyan": "#38bdf8",
    "accent_amber": "#d29922",
    "accent_purple": "#bc8cff",
    "state_proceed": "#238636",
    "state_proceed_light": "#3fb950",
    "state_await": "#9e6a03",
    "state_await_light": "#d29922",
    "state_rollback": "#da3633",
    "state_rollback_light": "#f85149",
    "state_halt": "#8957e5",
    "state_halt_light": "#a371f7",
    "console_bg": "#090d13",
    "console_fg": "#c9d1d9",
}


# ==============================================================================
# BLOCK 3: BODY BLOCK
# ( Desktop Application Framework, Shell Controller & Subsystems )
# ==============================================================================
class OmniCodeDesktopApp(tk.Tk):
    """Native Desktop GUI Studio and Terminal Shell for OmniCode."""

    def __init__(self):
        super().__init__()
        self.title("OmniCode Studio & Terminal Shell — CreativeWorkzStudio LLC")
        self.geometry("1280x840")
        self.minsize(1040, 680)
        self.configure(bg=THEME["bg_main"])

        # Process management
        self.active_process = None
        self.web_server_process = None
        self.tunnel_process = None
        self.cmd_history = []
        self.history_idx = 0

        self._configure_styles()
        self._build_header()
        self._build_main_layout()
        self._start_status_heartbeat()

    def _configure_styles(self):
        """Initializes ttk dark theme widget styles."""
        style = ttk.Style(self)
        style.theme_use("clam")

        # Configure tabs
        style.configure(
            "TNotebook",
            background=THEME["bg_main"],
            borderwidth=0
        )
        style.configure(
            "TNotebook.Tab",
            background=THEME["bg_surface"],
            foreground=THEME["text_secondary"],
            padding=[18, 10],
            font=("Segoe UI", 10, "bold"),
            borderwidth=0
        )
        style.map(
            "TNotebook.Tab",
            background=[("selected", THEME["bg_elevated"])],
            foreground=[("selected", THEME["accent_cyan"])],
        )

    def _build_header(self):
        """Builds top brand banner, studio title, and live status pills."""
        header = tk.Frame(self, bg=THEME["bg_surface"], height=64, padx=20, pady=10)
        header.pack(fill=tk.X, side=tk.TOP)
        header.pack_propagate(False)

        # Left branding
        brand_frame = tk.Frame(header, bg=THEME["bg_surface"])
        brand_frame.pack(side=tk.LEFT, fill=tk.Y)

        symbol_lbl = tk.Label(
            brand_frame,
            text="Ω",
            font=("Cinzel", 24, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        )
        symbol_lbl.pack(side=tk.LEFT, padx=(0, 10))

        title_box = tk.Frame(brand_frame, bg=THEME["bg_surface"])
        title_box.pack(side=tk.LEFT, fill=tk.Y)

        title_lbl = tk.Label(
            title_box,
            text="OMNICODE STUDIO",
            font=("Segoe UI", 12, "bold"),
            fg=THEME["text_primary"],
            bg=THEME["bg_surface"]
        )
        title_lbl.pack(anchor="w")

        subtitle_lbl = tk.Label(
            title_box,
            text="A CreativeWorkzStudio LLC Project",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["accent_amber"],
            bg=THEME["bg_surface"]
        )
        subtitle_lbl.pack(anchor="w")

        # Right status pills
        status_frame = tk.Frame(header, bg=THEME["bg_surface"])
        status_frame.pack(side=tk.RIGHT, fill=tk.Y)

        self.pill_engine = tk.Label(
            status_frame,
            text="● Engine: Ready",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["state_proceed"],
            fg="#ffffff",
            padx=10,
            pady=4
        )
        self.pill_engine.pack(side=tk.LEFT, padx=5)

        self.pill_web = tk.Label(
            status_frame,
            text="○ Web: Port 8080 (Off)",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["bg_elevated"],
            fg=THEME["text_secondary"],
            padx=10,
            pady=4
        )
        self.pill_web.pack(side=tk.LEFT, padx=5)

        self.pill_tunnel = tk.Label(
            status_frame,
            text="○ Edge Tunnel: Idle",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["bg_elevated"],
            fg=THEME["text_secondary"],
            padx=10,
            pady=4
        )
        self.pill_tunnel.pack(side=tk.LEFT, padx=5)

        self.pill_quad = tk.Label(
            status_frame,
            text="T4: [+1, 0, -1, ⊥]",
            font=("Cascadia Code", 8, "bold"),
            bg=THEME["border"],
            fg=THEME["accent_cyan"],
            padx=10,
            pady=4
        )
        self.pill_quad.pack(side=tk.LEFT, padx=5)

    def _build_main_layout(self):
        """Builds sidebar and multi-tab workspace."""
        content = tk.Frame(self, bg=THEME["bg_main"])
        content.pack(fill=tk.BOTH, expand=True)

        # ------------------ LEFT SIDEBAR ------------------
        sidebar = tk.Frame(content, bg=THEME["bg_surface"], width=240, padx=14, pady=14)
        sidebar.pack(side=tk.LEFT, fill=tk.Y)
        sidebar.pack_propagate(False)

        side_title = tk.Label(
            sidebar,
            text="QUICK ACTIONS",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["text_muted"],
            bg=THEME["bg_surface"]
        )
        side_title.pack(anchor="w", pady=(0, 10))

        # Action Buttons
        self._create_sidebar_btn(sidebar, "🌐 Open Web Studio", self.open_web_studio, "#1f6feb")
        self._create_sidebar_btn(sidebar, "⚡ Toggle Web Server", self.toggle_web_server, "#238636")
        self._create_sidebar_btn(sidebar, "☁️ Cloudflare Quick Tunnel", self.start_quick_tunnel, "#0969da")
        self._create_sidebar_btn(sidebar, "🌌 Sync Oracle Cloud VM", self.sync_oracle_vm, "#8957e5")

        tk.Frame(sidebar, height=1, bg=THEME["border"]).pack(fill=tk.X, pady=12)

        cli_lbl = tk.Label(
            sidebar,
            text="CLI OPERATIONS",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["text_muted"],
            bg=THEME["bg_surface"]
        )
        cli_lbl.pack(anchor="w", pady=(0, 8))

        self._create_sidebar_btn(sidebar, "🧪 Run Verification Demo", lambda: self.run_cli_command("omnicode demo"))
        self._create_sidebar_btn(sidebar, "📋 Run 4-Block Linter", lambda: self.run_cli_command("omnicode lint manifest.omni"))
        self._create_sidebar_btn(sidebar, "🩹 Run Restorator", lambda: self.run_cli_command("omnicode restore manifest.omni"))
        self._create_sidebar_btn(sidebar, "📜 Inspect Episodic Scrolls", lambda: self.run_cli_command("omnicode scrolls"))
        self._create_sidebar_btn(sidebar, "📖 Covenant Manifest", lambda: self.run_cli_command("omnicode manifest"))
        self._create_sidebar_btn(sidebar, "📦 Formal File Types", lambda: self.run_cli_command("omnicode filetypes"))

        tk.Frame(sidebar, height=1, bg=THEME["border"]).pack(fill=tk.X, pady=10)

        ai_lbl = tk.Label(
            sidebar,
            text="AI CORTEX & BRAIN",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        )
        ai_lbl.pack(anchor="w", pady=(0, 8))

        self._create_sidebar_btn(sidebar, "🧠 View Brain Topology", lambda: self.run_cli_command("omnicode ai map"), "#1f6feb")
        self._create_sidebar_btn(sidebar, "🎓 Train 25-Ch Curriculum", lambda: self.run_cli_command("omnicode ai curriculum"), "#238636")
        self._create_sidebar_btn(sidebar, "🛠️ Inspect AI Tool Belt", lambda: self.run_cli_command("omnicode ai tools"))
        self._create_sidebar_btn(sidebar, "📜 AI Growth Milestones", lambda: self.run_cli_command("omnicode ai timeline"))
        self._create_sidebar_btn(sidebar, "📖 Query Concordance", lambda: self.run_cli_command("omnicode ai concordance light"))
        self._create_sidebar_btn(sidebar, "🩹 AI Self-Healing", lambda: self.run_cli_command("omnicode ai heal manifest.omni"))

        # Footer info
        side_footer = tk.Frame(sidebar, bg=THEME["bg_surface"])
        side_footer.pack(side=tk.BOTTOM, fill=tk.X)
        tk.Label(
            side_footer,
            text="CreativeWorkzStudio LLC\nEngine v0.1.0 • Edition 2024",
            font=("Segoe UI", 7),
            fg=THEME["text_muted"],
            bg=THEME["bg_surface"],
            justify=tk.LEFT
        ).pack(anchor="w")

        # ------------------ MAIN TABS ------------------
        self.notebook = ttk.Notebook(content)
        self.notebook.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True)

        # Tab 1: Terminal Shell
        self.tab_terminal = tk.Frame(self.notebook, bg=THEME["bg_main"], padx=14, pady=14)
        self.notebook.add(self.tab_terminal, text="📟 Interactive Terminal Shell")
        self._build_terminal_tab()

        # Tab 2: AI Prompt Studio
        self.tab_studio = tk.Frame(self.notebook, bg=THEME["bg_main"], padx=14, pady=14)
        self.notebook.add(self.tab_studio, text="💡 AI Prompt Studio")
        self._build_studio_tab()

        # Tab 3: Language & AI Model Guide
        self.tab_guide = tk.Frame(self.notebook, bg=THEME["bg_main"], padx=14, pady=14)
        self.notebook.add(self.tab_guide, text="📐 Language & AI Model Guide")
        self._build_guide_tab()

        # Tab 4: Cloudflare & Oracle Cloud Center
        self.tab_cloud = tk.Frame(self.notebook, bg=THEME["bg_main"], padx=14, pady=14)
        self.notebook.add(self.tab_cloud, text="☁️ Cloudflare & Oracle Edge")
        self._build_cloud_tab()

    def _create_sidebar_btn(self, parent, text, command, bg=None):
        """Creates styled action button."""
        btn = tk.Button(
            parent,
            text=text,
            command=command,
            font=("Segoe UI", 8, "bold"),
            bg=bg or THEME["bg_elevated"],
            fg=THEME["text_primary"],
            activebackground=THEME["border"],
            activeforeground="#ffffff",
            relief="flat",
            bd=0,
            padx=10,
            pady=7,
            anchor="w",
            cursor="hand2"
        )
        btn.pack(fill=tk.X, pady=3)
        return btn

    # --------------------------------------------------------------------------
    # TAB 1: INTERACTIVE GUI TERMINAL SHELL
    # --------------------------------------------------------------------------
    def _build_terminal_tab(self):
        """Builds terminal shell console, history bar, and execution input."""
        # Top toolbar
        toolbar = tk.Frame(self.tab_terminal, bg=THEME["bg_surface"], padx=12, pady=8)
        toolbar.pack(fill=tk.X, pady=(0, 10))

        tk.Label(
            toolbar,
            text="CLI TERMINAL EMULATOR",
            font=("Segoe UI", 9, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        ).pack(side=tk.LEFT)

        tk.Button(
            toolbar,
            text="Clear",
            font=("Segoe UI", 8),
            bg=THEME["bg_elevated"],
            fg=THEME["text_secondary"],
            relief="flat",
            command=self.clear_terminal,
            padx=8,
            pady=2,
            cursor="hand2"
        ).pack(side=tk.RIGHT, padx=4)

        tk.Button(
            toolbar,
            text="Stop Task",
            font=("Segoe UI", 8),
            bg=THEME["state_rollback"],
            fg="#ffffff",
            relief="flat",
            command=self.kill_active_process,
            padx=8,
            pady=2,
            cursor="hand2"
        ).pack(side=tk.RIGHT, padx=4)

        tk.Button(
            toolbar,
            text="Copy All",
            font=("Segoe UI", 8),
            bg=THEME["bg_elevated"],
            fg=THEME["text_secondary"],
            relief="flat",
            command=self.copy_terminal_output,
            padx=8,
            pady=2,
            cursor="hand2"
        ).pack(side=tk.RIGHT, padx=4)

        # Quick command shortcut chips
        chips_frame = tk.Frame(self.tab_terminal, bg=THEME["bg_main"])
        chips_frame.pack(fill=tk.X, pady=(0, 8))

        chips = [
            ("omnicode demo", "omnicode demo"),
            ("ai map", "omnicode ai map"),
            ("ai curriculum", "omnicode ai curriculum"),
            ("ai tools", "omnicode ai tools"),
            ("ai timeline", "omnicode ai timeline"),
            ("ai concordance", "omnicode ai concordance light"),
            ("ai heal", "omnicode ai heal manifest.omni"),
            ("cargo test", "cargo test"),
        ]
        for label, cmd in chips:
            btn = tk.Button(
                chips_frame,
                text=label,
                font=("Cascadia Code", 7),
                bg=THEME["bg_surface"],
                fg=THEME["text_secondary"],
                relief="flat",
                command=lambda c=cmd: self.run_cli_command(c),
                padx=6,
                pady=2,
                cursor="hand2"
            )
            btn.pack(side=tk.LEFT, padx=(0, 6))

        # Console Text Display
        console_box = tk.Frame(self.tab_terminal, bg=THEME["console_bg"], bd=1, relief="solid")
        console_box.pack(fill=tk.BOTH, expand=True)

        self.console_text = tk.Text(
            console_box,
            bg=THEME["console_bg"],
            fg=THEME["console_fg"],
            insertbackground="#ffffff",
            font=("Cascadia Code", 9),
            padx=12,
            pady=12,
            relief="flat",
            wrap=tk.WORD
        )
        scroll = tk.Scrollbar(console_box, command=self.console_text.yview)
        self.console_text.configure(yscrollcommand=scroll.set)

        scroll.pack(side=tk.RIGHT, fill=tk.Y)
        self.console_text.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        # Terminal Input Bar
        input_bar = tk.Frame(self.tab_terminal, bg=THEME["bg_surface"], padx=10, pady=8)
        input_bar.pack(fill=tk.X, pady=(8, 0))

        prefix = tk.Label(
            input_bar,
            text="omnicode>",
            font=("Cascadia Code", 10, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        )
        prefix.pack(side=tk.LEFT, padx=(0, 8))

        self.cmd_input = tk.Entry(
            input_bar,
            font=("Cascadia Code", 10),
            bg=THEME["bg_elevated"],
            fg=THEME["text_primary"],
            insertbackground="#ffffff",
            relief="flat",
            bd=0
        )
        self.cmd_input.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 8), ipady=4)
        self.cmd_input.bind("<Return>", lambda e: self._submit_terminal_input())
        self.cmd_input.bind("<Up>", self._history_prev)
        self.cmd_input.bind("<Down>", self._history_next)

        run_btn = tk.Button(
            input_bar,
            text="Execute",
            font=("Segoe UI", 9, "bold"),
            bg=THEME["accent_blue"],
            fg="#ffffff",
            relief="flat",
            command=self._submit_terminal_input,
            padx=16,
            pady=4,
            cursor="hand2"
        )
        run_btn.pack(side=tk.RIGHT)

        # Welcome banner in terminal
        self.log_terminal("===============================================================================")
        self.log_terminal("  OmniCode: Dual Human & Machine Readable Language Engine (v0.1.0)")
        self.log_terminal("  CreativeWorkzStudio LLC — Autonomous Neural-Symbolic Intelligence")
        self.log_terminal("  Core: Balanced Ternary Quad-State Engine & Morphological Boundaries")
        self.log_terminal("===============================================================================")
        self.log_terminal("Type a command below (e.g. omnicode demo, omnicode manifest, omnicode ai \"What is truth?\")\n")

    def log_terminal(self, text: str):
        """Appends text to terminal console with automatic scroll."""
        self.console_text.insert(tk.END, text + "\n")
        self.console_text.see(tk.END)

    def clear_terminal(self):
        """Clears console output."""
        self.console_text.delete("1.0", tk.END)

    def copy_terminal_output(self):
        """Copies all terminal text to clipboard."""
        text = self.console_text.get("1.0", tk.END)
        self.clipboard_clear()
        self.clipboard_append(text)
        messagebox.showinfo("Clipboard", "Console output copied to clipboard.")

    def _submit_terminal_input(self):
        """Handles return key in terminal input."""
        cmd = self.cmd_input.get().strip()
        if not cmd:
            return
        self.cmd_input.delete(0, tk.END)
        self.cmd_history.append(cmd)
        self.history_idx = len(self.cmd_history)
        self.run_cli_command(cmd)

    def _history_prev(self, event):
        if self.cmd_history and self.history_idx > 0:
            self.history_idx -= 1
            self.cmd_input.delete(0, tk.END)
            self.cmd_input.insert(0, self.cmd_history[self.history_idx])

    def _history_next(self, event):
        if self.cmd_history and self.history_idx < len(self.cmd_history) - 1:
            self.history_idx += 1
            self.cmd_input.delete(0, tk.END)
            self.cmd_input.insert(0, self.cmd_history[self.history_idx])
        elif self.history_idx >= len(self.cmd_history) - 1:
            self.history_idx = len(self.cmd_history)
            self.cmd_input.delete(0, tk.END)

    def run_cli_command(self, cmd_str: str):
        """Executes a command asynchronously and streams stdout to the console."""
        clean_cmd = cmd_str.strip()
        self.log_terminal(f"\n$ {clean_cmd}")

        # Map omnicode command to cargo run if omnicode binary isn't in PATH
        executable_cmd = clean_cmd
        if clean_cmd.startswith("omnicode "):
            sub = clean_cmd[len("omnicode "):]
            executable_cmd = f"cargo run --quiet -- {sub}"
        elif clean_cmd == "omnicode":
            executable_cmd = "cargo run --quiet -- repl"

        def worker():
            try:
                proc = subprocess.Popen(
                    executable_cmd,
                    shell=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True,
                    bufsize=1,
                    cwd=BASE_DIR
                )
                self.active_process = proc

                for line in iter(proc.stdout.readline, ''):
                    cleaned = clean_ansi(line).rstrip()
                    if cleaned:
                        self.after(0, lambda l=cleaned: self.log_terminal(l))

                proc.stdout.close()
                proc.wait()
                self.after(0, lambda code=proc.returncode: self.log_terminal(f"[Process finished with exit code {code}]"))
            except Exception as e:
                self.after(0, lambda err=str(e): self.log_terminal(f"[-] Execution failure: {err}"))
            finally:
                self.active_process = None

        threading.Thread(target=worker, daemon=True).start()

    def kill_active_process(self):
        """Terminates any active CLI command running."""
        if self.active_process:
            try:
                self.active_process.terminate()
                self.log_terminal("\n[!] Process cancelled by operator.")
            except Exception:
                pass

    # --------------------------------------------------------------------------
    # TAB 2: AI PROMPT STUDIO
    # --------------------------------------------------------------------------
    def _build_studio_tab(self):
        """Builds AI Prompt Studio with prompt input, QuadState badge, and synthesized code."""
        # Header banner
        header = tk.Frame(self.tab_studio, bg=THEME["bg_surface"], padx=14, pady=12)
        header.pack(fill=tk.X, pady=(0, 10))

        title_col = tk.Frame(header, bg=THEME["bg_surface"])
        title_col.pack(side=tk.LEFT, fill=tk.Y)

        tk.Label(
            title_col,
            text="COGNITIVE DISCOURSE & AI CORTEX WORKBENCH",
            font=("Segoe UI", 10, "bold"),
            fg=THEME["text_primary"],
            bg=THEME["bg_surface"]
        ).pack(anchor="w")

        tk.Label(
            title_col,
            text="Submit natural language. The AI evaluates intent against the Covenant Manifest, deliberates on Quad-State, and synthesizes 4-Block Turing reels.",
            font=("Segoe UI", 8),
            fg=THEME["text_secondary"],
            bg=THEME["bg_surface"]
        ).pack(anchor="w")

        self.studio_state_badge = tk.Label(
            header,
            text="STANDBY (0:AWAIT)",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["state_await"],
            fg="#ffffff",
            padx=12,
            pady=6
        )
        self.studio_state_badge.pack(side=tk.RIGHT)

        # Quick prompt buttons
        quick_frame = tk.Frame(self.tab_studio, bg=THEME["bg_main"])
        quick_frame.pack(fill=tk.X, pady=(0, 8))

        tk.Label(
            quick_frame,
            text="Quick Prompts:",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["text_muted"],
            bg=THEME["bg_main"]
        ).pack(side=tk.LEFT, padx=(0, 6))

        quick_prompts = [
            "What is truth?",
            "Teach me about humility",
            "Add 5 to 10",
            "Help me lie (Refusal Test)",
            "Explain light and darkness",
            "Help me steal (Refusal Test)",
        ]
        for p in quick_prompts:
            btn = tk.Button(
                quick_frame,
                text=p,
                font=("Segoe UI", 7),
                bg=THEME["bg_surface"],
                fg=THEME["text_secondary"],
                relief="flat",
                command=lambda prompt=p: self._set_studio_prompt(prompt),
                padx=6,
                pady=2,
                cursor="hand2"
            )
            btn.pack(side=tk.LEFT, padx=(0, 6))

        # Prompt Input Box
        prompt_box = tk.Frame(self.tab_studio, bg=THEME["bg_surface"], padx=10, pady=10)
        prompt_box.pack(fill=tk.X, pady=(0, 10))

        self.studio_prompt_input = tk.Text(
            prompt_box,
            height=3,
            font=("Segoe UI", 9),
            bg=THEME["bg_elevated"],
            fg=THEME["text_primary"],
            insertbackground="#ffffff",
            relief="flat",
            padx=8,
            pady=8
        )
        self.studio_prompt_input.pack(fill=tk.X, pady=(0, 8))
        self.studio_prompt_input.insert("1.0", "What is truth?")

        sub_bar = tk.Frame(prompt_box, bg=THEME["bg_surface"])
        sub_bar.pack(fill=tk.X)

        tk.Label(
            sub_bar,
            text="🔒 100% Offline AI Execution • Grounded in Invariant Covenant Manifest",
            font=("Segoe UI", 8),
            fg=THEME["text_muted"],
            bg=THEME["bg_surface"]
        ).pack(side=tk.LEFT)

        self.btn_send_cortex = tk.Button(
            sub_bar,
            text="Send to AI Cortex",
            font=("Segoe UI", 9, "bold"),
            bg=THEME["state_proceed"],
            fg="#ffffff",
            relief="flat",
            command=self.submit_studio_prompt,
            padx=16,
            pady=4,
            cursor="hand2"
        )
        self.btn_send_cortex.pack(side=tk.RIGHT)

        # Deliberation HUD: 6 Cardinal Clauses & Attractor Resonance
        hud_frame = tk.Frame(self.tab_studio, bg=THEME["bg_surface"], padx=12, pady=8)
        hud_frame.pack(fill=tk.X, pady=(0, 8))

        hud_title_row = tk.Frame(hud_frame, bg=THEME["bg_surface"])
        hud_title_row.pack(fill=tk.X, pady=(0, 4))
        tk.Label(
            hud_title_row,
            text="🧭 6 CARDINAL CLAUSES DELIBERATION HUD",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        ).pack(side=tk.LEFT)

        self.lbl_hud_resonance = tk.Label(
            hud_title_row,
            text="Attractor Resonance: None",
            font=("Segoe UI", 8),
            fg=THEME["accent_amber"],
            bg=THEME["bg_surface"]
        )
        self.lbl_hud_resonance.pack(side=tk.RIGHT, padx=6)

        self.lbl_hud_precedent = tk.Label(
            hud_title_row,
            text="Precedent: None",
            font=("Segoe UI", 8),
            fg=THEME["text_secondary"],
            bg=THEME["bg_surface"]
        )
        self.lbl_hud_precedent.pack(side=tk.RIGHT, padx=6)

        # 6 Clauses Grid
        clauses_box = tk.Frame(hud_frame, bg=THEME["bg_elevated"], padx=8, pady=6)
        clauses_box.pack(fill=tk.X)

        self.lbl_hud_who = self._create_hud_slot(clauses_box, "WHO", "--", THEME["accent_cyan"], 0, 0)
        self.lbl_hud_what = self._create_hud_slot(clauses_box, "WHAT", "--", THEME["accent_amber"], 0, 1)
        self.lbl_hud_when = self._create_hud_slot(clauses_box, "WHEN", "--", THEME["text_secondary"], 0, 2)
        self.lbl_hud_where = self._create_hud_slot(clauses_box, "WHERE", "--", THEME["text_secondary"], 1, 0)
        self.lbl_hud_why = self._create_hud_slot(clauses_box, "WHY", "--", THEME["state_proceed_light"], 1, 1)
        self.lbl_hud_how = self._create_hud_slot(clauses_box, "HOW", "--", THEME["accent_cyan"], 1, 2)
        clauses_box.columnconfigure(0, weight=1)
        clauses_box.columnconfigure(1, weight=1)
        clauses_box.columnconfigure(2, weight=1)

        # Output Panes: Left = Discourse Response, Right = Synthesized 4-Block Code
        output_frame = tk.Frame(self.tab_studio, bg=THEME["bg_main"])
        output_frame.pack(fill=tk.BOTH, expand=True)

        # Left: Response
        resp_pane = tk.Frame(output_frame, bg=THEME["bg_surface"], padx=10, pady=10)
        resp_pane.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 6))

        tk.Label(
            resp_pane,
            text="💬 Multi-Turn Discourse Thread & Moral Invariant Grounding",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        ).pack(anchor="w", pady=(0, 6))

        self.resp_display = tk.Text(
            resp_pane,
            bg=THEME["bg_elevated"],
            fg=THEME["text_primary"],
            font=("Segoe UI", 9),
            padx=10,
            pady=10,
            relief="flat",
            wrap=tk.WORD
        )
        self.resp_display.pack(fill=tk.BOTH, expand=True)
        self.resp_display.insert("1.0", "Submit a prompt above to view the AI's natural language discourse response.\n")

        # Right: Synthesized 4-Block Code
        code_pane = tk.Frame(output_frame, bg=THEME["bg_surface"], padx=10, pady=10)
        code_pane.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True, padx=(6, 0))

        code_header = tk.Frame(code_pane, bg=THEME["bg_surface"])
        code_header.pack(fill=tk.X, pady=(0, 6))

        tk.Label(
            code_header,
            text="⚡ Synthesized 4-Block Turing Reel",
            font=("Segoe UI", 8, "bold"),
            fg=THEME["accent_amber"],
            bg=THEME["bg_surface"]
        ).pack(side=tk.LEFT)

        tk.Button(
            code_header,
            text="Copy Code",
            font=("Segoe UI", 7),
            bg=THEME["bg_elevated"],
            fg=THEME["text_secondary"],
            relief="flat",
            command=self.copy_synthesized_code,
            padx=6,
            pady=1,
            cursor="hand2"
        ).pack(side=tk.RIGHT)

        self.code_display = tk.Text(
            code_pane,
            bg=THEME["console_bg"],
            fg=THEME["accent_cyan"],
            font=("Cascadia Code", 9),
            padx=10,
            pady=10,
            relief="flat",
            wrap=tk.WORD
        )
        self.code_display.pack(fill=tk.BOTH, expand=True)
        self.code_display.insert("1.0", "// Canonical 4-Block Turing Reel will be synthesized dynamically here.")

    def _create_hud_slot(self, parent, label: str, default: str, fg_color: str, row: int, col: int) -> tk.Label:
        frame = tk.Frame(parent, bg=THEME["bg_elevated"])
        frame.grid(row=row, column=col, sticky="w", padx=6, pady=2)
        tk.Label(
            frame,
            text=f"{label}: ",
            font=("Segoe UI", 7, "bold"),
            fg=THEME["text_muted"],
            bg=THEME["bg_elevated"]
        ).pack(side=tk.LEFT)
        val_lbl = tk.Label(
            frame,
            text=default,
            font=("Cascadia Code", 8),
            fg=fg_color,
            bg=THEME["bg_elevated"]
        )
        val_lbl.pack(side=tk.LEFT)
        return val_lbl

    def _set_studio_prompt(self, text: str):
        self.studio_prompt_input.delete("1.0", tk.END)
        self.studio_prompt_input.insert("1.0", text)

    def copy_synthesized_code(self):
        text = self.code_display.get("1.0", tk.END)
        self.clipboard_clear()
        self.clipboard_append(text)
        messagebox.showinfo("Copied", "Synthesized 4-Block code copied to clipboard.")

    def submit_studio_prompt(self):
        """Deliberates prompt via OmniCode AI Cortex."""
        prompt = self.studio_prompt_input.get("1.0", tk.END).strip()
        if not prompt:
            return

        self.btn_send_cortex.config(text="Deliberating...", state=tk.DISABLED)
        self.studio_state_badge.config(text="DELIBERATING (0:AWAIT)", bg=THEME["state_await"])
        self.current_studio_prompt = prompt

        def worker():
            cmd = f'cargo run --quiet -- ai chat "{prompt}" --verbose'
            try:
                proc = subprocess.run(
                    cmd,
                    shell=True,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    cwd=BASE_DIR
                )
                output = proc.stdout + proc.stderr
                self.after(0, lambda o=output: self._handle_studio_result(o))
            except Exception as e:
                self.after(0, lambda err=str(e): self._handle_studio_error(err))

        threading.Thread(target=worker, daemon=True).start()

    def _handle_studio_result(self, raw_output: str):
        self.btn_send_cortex.config(text="Send to AI Cortex", state=tk.NORMAL)
        cleaned = clean_ansi(raw_output)

        # Detect State
        state_str = "+1:PROCEED"
        if "Execution State: PROCEED" in cleaned or "+1:PROCEED" in cleaned:
            self.studio_state_badge.config(text="+1: PROCEED", bg=THEME["state_proceed"])
            state_str = "+1:PROCEED"
        elif "Execution State: ROLLBACK" in cleaned or "-1:ROLLBACK" in cleaned:
            self.studio_state_badge.config(text="-1: ROLLBACK", bg=THEME["state_rollback"])
            state_str = "-1:ROLLBACK"
        elif "Execution State: HALT" in cleaned or "NULL:HALT" in cleaned:
            self.studio_state_badge.config(text="NULL: HALT", bg=THEME["state_halt"])
            state_str = "NULL:HALT"
        else:
            self.studio_state_badge.config(text="0: AWAIT", bg=THEME["state_await"])
            state_str = "0:AWAIT"

        # Parse Attractor Resonance & Recalled Precedent
        res_match = re.search(r'Attractor Resonance:\s+([\d\.]+)%', cleaned)
        if res_match:
            self.lbl_hud_resonance.config(text=f"Attractor Resonance: {res_match.group(1)}%")
        else:
            self.lbl_hud_resonance.config(text="Attractor Resonance: None (New)")

        prec_match = re.search(r'Recalled Precedent:\s+([^\n]+)', cleaned)
        if prec_match:
            self.lbl_hud_precedent.config(text=f"Precedent: {prec_match.group(1).strip()}")
        else:
            self.lbl_hud_precedent.config(text="Precedent: None")

        # Parse 6 Cardinal Clauses
        who_m = re.search(r'• WHO:\s+([^\n]+)', cleaned)
        if who_m: self.lbl_hud_who.config(text=who_m.group(1).strip())
        what_m = re.search(r'• WHAT:\s+([^\n]+)', cleaned)
        if what_m: self.lbl_hud_what.config(text=what_m.group(1).strip())
        when_m = re.search(r'• WHEN:\s+([^\n]+)', cleaned)
        if when_m: self.lbl_hud_when.config(text=when_m.group(1).strip())
        where_m = re.search(r'• WHERE:\s+([^\n]+)', cleaned)
        if where_m: self.lbl_hud_where.config(text=where_m.group(1).strip())
        why_m = re.search(r'• WHY:\s+([^\n]+)', cleaned)
        if why_m: self.lbl_hud_why.config(text=why_m.group(1).strip()[:40])
        how_m = re.search(r'• HOW:\s+([^\n]+)', cleaned)
        if how_m: self.lbl_hud_how.config(text=how_m.group(1).strip()[:40])

        # Extract Synthesized 4-Block OmniCode
        if "✓ Synthesized 4-Block OmniCode Turing Reel:" in cleaned:
            code_parts = cleaned.split("✓ Synthesized 4-Block OmniCode Turing Reel:")
            if len(code_parts) > 1:
                tail = code_parts[1]
                if "[Machine Layer Response]:" in tail:
                    code_body = tail.split("[Machine Layer Response]:")[0].strip()
                    self.code_display.delete("1.0", tk.END)
                    self.code_display.insert("1.0", code_body)
                else:
                    self.code_display.delete("1.0", tk.END)
                    self.code_display.insert("1.0", tail.strip())
        elif "=== Synthesized OmniCode" in cleaned:
            parts = cleaned.split("=== Synthesized OmniCode")
            if len(parts) > 1:
                code_part = parts[1].split("========================================")[0]
                self.code_display.delete("1.0", tk.END)
                self.code_display.insert("1.0", code_part.strip())

        # Extract Machine Layer Response
        machine_resp = ""
        if "[Machine Layer Response]:" in cleaned:
            resp_parts = cleaned.split("[Machine Layer Response]:")
            if len(resp_parts) > 1:
                machine_resp = resp_parts[1].strip().strip('"')
        if not machine_resp:
            machine_resp = cleaned

        # Append to Multi-Turn Discourse Thread
        current_text = self.resp_display.get("1.0", tk.END).strip()
        if "Submit a prompt above" in current_text:
            self.resp_display.delete("1.0", tk.END)

        prompt_str = getattr(self, "current_studio_prompt", "Discourse")
        self.resp_display.insert(tk.END, f"━━━ You ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{prompt_str}\n\n")
        self.resp_display.insert(tk.END, f"━━━ CPI-SI Base [{state_str}] ━━━━━━━━━━━━━━━━━\n{machine_resp}\n\n")
        self.resp_display.see(tk.END)

    def _handle_studio_error(self, err: str):
        self.btn_send_cortex.config(text="Send to AI Cortex", state=tk.NORMAL)
        self.studio_state_badge.config(text="ERROR (NULL:HALT)", bg=THEME["state_rollback"])
        self.resp_display.insert(tk.END, f"\n[!] Deliberation Failpoint: {err}\n")

    # --------------------------------------------------------------------------
    # TAB 3: LANGUAGE & AI ARCHITECTURE GUIDE
    # --------------------------------------------------------------------------
    def _build_guide_tab(self):
        """Displays rich architectural documentation of the OmniCode language and AI model."""
        guide_text = tk.Text(
            self.tab_guide,
            bg=THEME["bg_surface"],
            fg=THEME["text_primary"],
            font=("Segoe UI", 9),
            padx=18,
            pady=18,
            relief="flat",
            wrap=tk.WORD
        )
        scroll = tk.Scrollbar(self.tab_guide, command=guide_text.yview)
        guide_text.configure(yscrollcommand=scroll.set)
        scroll.pack(side=tk.RIGHT, fill=tk.Y)
        guide_text.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)

        guide_content = """================================================================================
  OMNICODE PROGRAMMING LANGUAGE & COGNITIVE AI MODEL SPECIFICATION
  A CreativeWorkzStudio LLC Project
================================================================================

1. WHAT IS OMNICODE?
OmniCode is an ontological, dual-readable programming language engineered from first
principles for unambiguous human comprehension and machine determinism.
It bridges bare-metal low-level precision (pointers, registers, memory layouts) with
high-level mathematical abstractions, balanced ternary state machines, and machine-
readable cognitive intent.

--------------------------------------------------------------------------------
2. UNIVERSAL 4-BLOCK FILE ARCHITECTURE
Every .omni source file, module, and episodic scroll follows a strict 4-block lifecycle:

  • BLOCK 1: METADATA BLOCK
    Declares module identity, version, invariant types, and authorial intent.
    Example: Module < Example > ( Kernel logic intent ) { version: < "1.0.0" > }

  • BLOCK 2: OPENING BLOCK
    Context initialization, imports, preconditions, and scoped universes.
    Example: Opening { import: [ Core.Arithmetic ], preconditions: [ +1 ] }

  • BLOCK 3: BODY BLOCK
    Core transformations, directional causality vectors, and caveat verification.
    Example: Body [ state --> next_state [ +1 ] ! [ state = +1 ] ]

  • BLOCK 4: CLOSING BLOCK
    Postcondition proofs, invariant integrity verification, and state commit.
    Example: Closing { verify: [ next_state = +1 ], commit_status: [ +1 ] }

--------------------------------------------------------------------------------
3. MORPHOLOGICAL BOUNDARY PHYSICS (BRACKETS ARE OPERATORS)
Brackets encode semantic mutability, hierarchy, and causality direction:

  • [...] Rigid / Fixed:       Concrete memory, immutable datum, fixed denotation.
  • <...> Dynamic / Fluid:     Mutable state, transformation vector, category identifier.
  • {...} Universal / Omni:    Scoped universe, parent scope, environmental domain.
  • {...] Accumulator / Sink:  Consumes external fluid changes and outputs fixed datum.
  • [...} Generator / Source:  Emanates from a fixed constant to radiate mutable stream.
  • (...) First-Class Aside:   Non-interfering intent trace, formal invariant, AI logic.

--------------------------------------------------------------------------------
4. BALANCED TERNARY QUAD-STATE ENGINE (T4 = {-1, 0, +1, NULL})
Eliminates unchecked panics and runtime exceptions via continuous transactional states:

  • +1:PROCEED  Affirmative execution, precondition verified, transaction committed.
  •  0:AWAIT    Indeterminate state, suspended awaiting consensus or input.
  • -1:ROLLBACK Contra/veto state. Automatically rolls back state to previous snapshot.
  • NULL:HALT   Terminal void. Absorbs subsequent transformations and halts safely.

--------------------------------------------------------------------------------
5. 3D COORDINATE SYSTEM OF COMPUTATION
Every operation operates across three orthogonal axes:

  • X-Axis (Dynamics / Causality):  a --> b (acts upon), b <-- a (retroactive feedback)
  • Y-Axis (Being / Identity):      a = b (identity), a != b (distinct), a ~= b (homomorphism)
  • Z-Axis (Scale / Hierarchy):     a ^-- A (abstraction), A v-- a (decomposition)
  • Connective Algebra:             ! (caveat guard), :: (consensus AND), | (OR), -- (NOT)

--------------------------------------------------------------------------------
6. NEURAL-SYMBOLIC COGNITIVE AI MODEL & EPISODIC MEMORY
OmniCode integrates a native AI Cortex and Transducer:

  • AICortex: Bridges human natural language and verified 4-Block code.
  • Dual Episodic Inscription:
      - .scroll: Executable 4-Block Turing reel recording the exact episodic trace.
      - .log: Microsecond telemetry tracking channel duration and failpoint logs.
  • AILearner: Continuously digests experiences from scrolls/ to refine performance.
  • 100% Offline Covenant Anchor: Grounded in the Ten Invariant Moral Laws
    (The Decalogue / Exodus 20:1-17) & Biblical Lexicon with zero internet
    requirements and zero cloud telemetry.

--------------------------------------------------------------------------------
7. CREATIVEWORKZSTUDIO LLC OFFICIAL PLATFORM
Engine Version: 0.1.0 | Rust Core Edition 2024
Copyright © 2026 CreativeWorkzStudio LLC. All rights reserved.
"""
        guide_text.insert("1.0", guide_content)
        guide_text.config(state=tk.DISABLED)

    # --------------------------------------------------------------------------
    # TAB 4: CLOUDFLARE & ORACLE EDGE DEPLOYMENT
    # --------------------------------------------------------------------------
    def _build_cloud_tab(self):
        """Builds controls and status displays for Cloudflare Tunnels and Oracle VM."""
        cloud_frame = tk.Frame(self.tab_cloud, bg=THEME["bg_surface"], padx=16, pady=16)
        cloud_frame.pack(fill=tk.BOTH, expand=True)

        tk.Label(
            cloud_frame,
            text="EDGE ROUTING & CLOUD DEPLOYMENT CENTER",
            font=("Segoe UI", 10, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_surface"]
        ).pack(anchor="w", pady=(0, 6))

        tk.Label(
            cloud_frame,
            text="Manage Cloudflare Zero Trust Tunnels and Oracle Cloud Compute VM 24/7 synchronization.",
            font=("Segoe UI", 8),
            fg=THEME["text_secondary"],
            bg=THEME["bg_surface"]
        ).pack(anchor="w", pady=(0, 14))

        # Two Columns
        grid_frame = tk.Frame(cloud_frame, bg=THEME["bg_surface"])
        grid_frame.pack(fill=tk.BOTH, expand=True)

        # Left Column: Oracle Cloud
        oracle_col = tk.Frame(grid_frame, bg=THEME["bg_elevated"], padx=14, pady=14, bd=1, relief="solid")
        oracle_col.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=(0, 8))

        tk.Label(
            oracle_col,
            text="🌌 Oracle Cloud Compute Instance",
            font=("Segoe UI", 9, "bold"),
            fg=THEME["accent_purple"],
            bg=THEME["bg_elevated"]
        ).pack(anchor="w", pady=(0, 8))

        oracle_info = """• Host IP:    147.224.147.30 (Host alias: oracle)
• User:       ubuntu
• SSH Key:    ~/.ssh/oracle_key.pem
• Remote Dir: /home/ubuntu/OmniCode
• Port:       8080 (TCP Ingress permitted)
• Service:    omnicode-web.service (systemd)"""
        tk.Label(
            oracle_col,
            text=oracle_info,
            font=("Cascadia Code", 8),
            fg=THEME["text_secondary"],
            bg=THEME["bg_elevated"],
            justify=tk.LEFT
        ).pack(anchor="w", pady=(0, 12))

        tk.Button(
            oracle_col,
            text="Sync to Oracle VM Now",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["accent_purple"],
            fg="#ffffff",
            relief="flat",
            command=self.sync_oracle_vm,
            padx=12,
            pady=5,
            cursor="hand2"
        ).pack(anchor="w")

        # Right Column: Cloudflare
        cf_col = tk.Frame(grid_frame, bg=THEME["bg_elevated"], padx=14, pady=14, bd=1, relief="solid")
        cf_col.pack(side=tk.RIGHT, fill=tk.BOTH, expand=True, padx=(8, 0))

        tk.Label(
            cf_col,
            text="🌐 Cloudflare Zero Trust Edge",
            font=("Segoe UI", 9, "bold"),
            fg=THEME["accent_cyan"],
            bg=THEME["bg_elevated"]
        ).pack(anchor="w", pady=(0, 8))

        cf_info = """• Binary:     /usr/local/bin/cloudflared
• Domain:     omnicodestudio.duckdns.org
• WAN IP:     71.11.99.168 (CreativeWorkzStudio LLC)
• Quick Mode: Instant HTTPS at *.trycloudflare.com
• Local Port: http://localhost:8080"""
        tk.Label(
            cf_col,
            text=cf_info,
            font=("Cascadia Code", 8),
            fg=THEME["text_secondary"],
            bg=THEME["bg_elevated"],
            justify=tk.LEFT
        ).pack(anchor="w", pady=(0, 12))

        btn_row = tk.Frame(cf_col, bg=THEME["bg_elevated"])
        btn_row.pack(anchor="w")

        tk.Button(
            btn_row,
            text="Launch Quick Tunnel",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["accent_cyan"],
            fg="#000000",
            relief="flat",
            command=self.start_quick_tunnel,
            padx=10,
            pady=5,
            cursor="hand2"
        ).pack(side=tk.LEFT, padx=(0, 8))

        tk.Button(
            btn_row,
            text="Launch Named Tunnel",
            font=("Segoe UI", 8, "bold"),
            bg=THEME["border"],
            fg=THEME["text_primary"],
            relief="flat",
            command=self.start_named_tunnel,
            padx=10,
            pady=5,
            cursor="hand2"
        ).pack(side=tk.LEFT)

    # --------------------------------------------------------------------------
    # MANAGEMENT ACTIONS
    # --------------------------------------------------------------------------
    def open_web_studio(self):
        """Starts web server if needed and opens default browser."""
        self.notebook.select(self.tab_terminal)
        self.log_terminal("[*] Opening OmniCode Web Studio at http://localhost:8080...")
        self._ensure_web_server_running(on_ready=lambda: webbrowser.open("http://localhost:8080"))

    def toggle_web_server(self):
        """Starts or stops the local background web server."""
        self.notebook.select(self.tab_terminal)
        if self.web_server_process:
            self.log_terminal("[*] Stopping local OmniCode Web Server...")
            try:
                self.web_server_process.terminate()
            except Exception:
                pass
            self.web_server_process = None
            self.pill_web.config(text="○ Web: Port 8080 (Off)", bg=THEME["bg_elevated"])
            self.log_terminal("[+] Web server stopped.")
        else:
            self.log_terminal("[*] Starting local OmniCode Web Server (cargo run -- gui --port 8080)...")
            self._start_web_server()

    def _start_web_server(self):
        def worker():
            try:
                proc = subprocess.Popen(
                    ["cargo", "run", "--quiet", "--", "gui", "--port", "8080"],
                    cwd=BASE_DIR,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.STDOUT,
                    text=True
                )
                self.web_server_process = proc
                self.after(0, lambda: self.pill_web.config(text="● Web: Port 8080 (Live)", bg=THEME["state_proceed"]))
                for line in iter(proc.stdout.readline, ''):
                    clean = clean_ansi(line).rstrip()
                    if clean:
                        self.after(0, lambda l=clean: self.log_terminal(f"[Web] {l}"))
            except Exception as e:
                self.after(0, lambda err=str(e): self.log_terminal(f"[-] Web server error: {err}"))
            finally:
                self.web_server_process = None
                self.after(0, lambda: self.pill_web.config(text="○ Web: Port 8080 (Off)", bg=THEME["bg_elevated"]))

        threading.Thread(target=worker, daemon=True).start()

    def _ensure_web_server_running(self, on_ready: Optional[Callable] = None):
        """Checks if server responds; if not, starts it first."""
        import urllib.request
        is_up = False
        try:
            with urllib.request.urlopen("http://localhost:8080/api/manifest", timeout=0.8) as resp:
                if resp.status == 200:
                    is_up = True
        except Exception:
            is_up = False

        if is_up:
            if on_ready:
                on_ready()
        else:
            self._start_web_server()
            def wait_and_call():
                for _ in range(15):
                    time.sleep(0.5)
                    try:
                        with urllib.request.urlopen("http://localhost:8080/api/manifest", timeout=0.5) as r:
                            if r.status == 200:
                                if on_ready:
                                    self.after(0, on_ready)
                                return
                    except Exception:
                        pass
                if on_ready:
                    self.after(0, on_ready)
            threading.Thread(target=wait_and_call, daemon=True).start()

    def start_quick_tunnel(self):
        """Launches Cloudflare Quick Tunnel."""
        self.notebook.select(self.tab_terminal)
        self.log_terminal("[*] Launching Cloudflare Quick Tunnel...")
        self._ensure_web_server_running(on_ready=lambda: self.run_cli_command(f"{SCRIPTS_DIR}/setup_cloudflare_tunnel.sh --quick"))
        self.pill_tunnel.config(text="● Edge: Quick Tunneling", bg=THEME["accent_cyan"], fg="#000000")

    def start_named_tunnel(self):
        """Launches Cloudflare Named Tunnel."""
        self.notebook.select(self.tab_terminal)
        self.log_terminal("[*] Launching Cloudflare Named Tunnel...")
        self._ensure_web_server_running(on_ready=lambda: self.run_cli_command(f"{SCRIPTS_DIR}/setup_cloudflare_tunnel.sh"))
        self.pill_tunnel.config(text="● Edge: Named Tunnel", bg=THEME["accent_cyan"], fg="#000000")

    def sync_oracle_vm(self):
        """Triggers synchronization to Oracle Cloud VM."""
        self.notebook.select(self.tab_terminal)
        self.log_terminal("[*] Initiating Oracle Cloud VM synchronization...")
        self.run_cli_command(f"{SCRIPTS_DIR}/sync_oracle_vm.sh")

    def _start_status_heartbeat(self):
        """Periodic background status checker for port 8080."""
        def check():
            import urllib.request
            try:
                with urllib.request.urlopen("http://localhost:8080/api/manifest", timeout=0.4) as resp:
                    if resp.status == 200:
                        self.after(0, lambda: self.pill_web.config(text="● Web: Port 8080 (Live)", bg=THEME["state_proceed"], fg="#ffffff"))
                        return
            except Exception:
                pass
            if not self.web_server_process:
                self.after(0, lambda: self.pill_web.config(text="○ Web: Port 8080 (Off)", bg=THEME["bg_elevated"], fg=THEME["text_secondary"]))

        def loop():
            while True:
                check()
                time.sleep(4)

        threading.Thread(target=loop, daemon=True).start()


# ==============================================================================
# BLOCK 4: CLOSING BLOCK
# ( Application Entrypoint & Fallback Invocation )
# ==============================================================================
def main():
    if not TKINTER_AVAILABLE:
        print("[!] Error: Tkinter desktop GUI library is not available in this Python runtime.")
        print("    Install via: sudo apt-get install -y python3-tk")
        sys.exit(1)

    app = OmniCodeDesktopApp()
    app.mainloop()

if __name__ == "__main__":
    main()
