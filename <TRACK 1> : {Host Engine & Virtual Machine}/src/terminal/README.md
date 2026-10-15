# OmniCode Terminal Subsystem

```text
==============================================================================
BLOCK 1: METADATA & ARCHITECTURAL MANIFESTO
==============================================================================
Subsystem: OmniCode Host Engine & Virtual Machine Terminal
Layer:     Interactive Shell, Multi-Track Scheduler & Native Character Echo
Path:      <TRACK 1> : {Host Engine & Virtual Machine}/src/terminal/
==============================================================================
```

> **"And he gave unto Moses, when he had made an end of communing with him upon mount Sinai, two tables of testimony, tables of stone, written with the finger of God."**  
> — *Exodus 31:18 (KJV)*  
> *(The immutable inscription of atomic runes onto bounded physical tablets).*
>
> **"Prepare thy work without, and make it fit for thyself in the field; and afterwards build thine house."**  
> — *Proverbs 24:27 (KJV)*  
> *(The discipline of proving atomic sub-engines and verification invariants before constructing complex systems).*
>
> **"Remove not the ancient landmark, which thy fathers have set."**  
> — *Proverbs 22:28 (KJV)*  
> *(The preservation of historical memory boundaries, silicon addresses, and machine constants).*

---

## 1. Architectural Mission

The **Terminal Subsystem** serves as the sensory-motor cortex and interactive execution environment of OmniCode. It bridges the virtual machine runtime (`src/runtime/`) to external users, host environments, and bare-metal hypervisors.

Rather than relying on external host shell forks or raw byte pass-throughs, the OmniCode Terminal natively executes process ribbons in persistent silicon memory, schedules cooperative tasks across three execution channels, and drives the **Native Character Invariant Echo Protocol**:

1. **Native Character Echo (`universal/`)**: Proves that every keypress or character stream ingests via `FROM ::@ 0x03F8`, inscribes into an atomic `Cell`, transduces into 5 balanced trits, verifies single-cell capacity bounds ($\le 1\text{ Cell}$), and projects outward via `INTO ::@ 0x000B8000` with affirmative `+1:PROCEED`.
2. **Multi-Track Cooperative Scheduler (`scheduler/`)**: Organizes execution across Track 0 (Kernel), Track 1 (Terminal), and Track 2 (App), carving RAM partitions out of the `Root_Storehouse`.
3. **Interactive Session (`session/`)**: Provides direct in-memory roll evaluation and shell command routing.
4. **Cinematic Projector (`studio/`)**: Renders real-time visual HUD frames of 4-directional Memory Reel kinematics.

---

## 2. Topological Layer Structure

```bash
src/terminal/
├── universal/                  <-- Tier 1: Hardware MMIO Ports & Native Character Echo
│   ├── ports.rs                <-- FROM (0x03F8), INTO (0x000B8000), Stream Buffers
│   ├── character_echo.rs       <-- 5-Trit Radix-3 Bijective Character Lifecycle & Echo Engine
│   └── mod.rs                  <-- Re-exports Universal I/O & Echo
│
├── scheduler/                  <-- Tier 2: Multi-Track Cooperative Process Dispatcher
│   ├── tasks.rs                <-- OmniTask, Track, TaskPriority, JSON Models
│   ├── scheduler.rs            <-- OmniScheduler, Partition Carving, Tick Loop
│   └── mod.rs                  <-- Re-exports Scheduler & Models
│
├── session/                    <-- Tier 3: Interactive Shell & Upgraded Character I/O
│   ├── commands.rs             <-- Built-in Shell Command Dispatcher
│   ├── session.rs              <-- TerminalSession Coordinator with Native Character Echo
│   └── mod.rs                  <-- Re-exports Session Engine
│
├── studio/                     <-- Tier 4: Cinematic Projector & Memory HUD
│   ├── hud.rs                  <-- ANSI Box-Drawing Frame Renderer
│   ├── kinematics.rs           <-- Reel Transport & Spindle Depth Controls
│   └── mod.rs                  <-- Re-exports Studio Engine
│
├── mod.rs                      <-- Subsystem Public Facade (100% Backward Compatible)
└── README.md                   <-- Architectural Manifesto & Specification
```

---

## 3. Subsystem Breakdown

### 3.1 Universal (`universal/`)
- **Hardware MMIO Ports (`ports.rs`)**:
  - `UART_COM1_INPUT_PORT = 0x03F8`: Input port for serial keyboard stream.
  - `VGA_TEXT_OUTPUT_PORT = 0x000B8000`: Output port for video console display.
  - `TerminalInputStream` / `TerminalOutputStream`: Bounded FIFO ring buffer queues.
- **Native Character Echo Engine (`character_echo.rs`)**:
  - Bijective Radix-3 transduction: 8-bit ASCII $\iff$ 5 balanced trits $\{-1, 0, +1\}^5$.
  - Inscription into 5-trit `Cell` and single-cell capacity verification under `! GUARD`.
  - Methods: `echo_char(glyph) -> Result<CharacterEchoResult, String>` and `echo_stream(&str) -> Result<String, String>`.

### 3.2 Multi-Track Scheduler (`scheduler/`)
- **`tasks.rs`**: Defines execution channels (`Track::Kernel`, `Track::Terminal`, `Track::App`), priorities (`High`, `Normal`, `Low`, `Background`), and the `OmniTask` descriptor.
- **`scheduler.rs`**: Manages process lifecycles, advances the cooperative `tick` clock, carves partitions from `Root_Storehouse`, and supports manual tape stepping (`step_task`).

### 3.3 Interactive Session (`session/`)
- **`commands.rs`**: Implements built-in shell commands (`ps`, `tracks`, `mem`, `mount`, `run`, `step`, `rolls`, `spawn`, `pause`, `resume`, `kill`, `tick`, `vars`, `help`, `let`).
- **`session.rs`**: Integrates `Environment`, `OmniScheduler`, directional streams, and direct roll evaluation. Exposes `feed_input_char`, `read_output_char`, `process_character_echo`, and `process_character_stream_echo`.

### 3.4 Cinema Studio (`studio/`)
- **`hud.rs`**: ANSI box-drawing projector HUD frame showing active reel, clapper cues, and set props.
- **`kinematics.rs`**: Reel motion evaluator supporting `PLAY`, `PAUSE`, `STOP`, `v--`, `^--`, `<--`, and `-->`.

---

## 4. The Terminal Echo Protocol

```mermaid
graph LR
    A["Host Keypress ('A' = 65)"] --> B["FROM Port (0x03F8 / %from)"]
    B --> C["ROLL 1: INSCRIBE [ Cell ]"]
    C --> D["ROLL 2: TRANSDUCE 5 Trits [-1, 1, 1, -1, 1]"]
    D --> E["ROLL 3: GUARD (<= 1 Cell)"]
    E --> F["ROLL 4: PROJECT [ Target ]"]
    F --> G["INTO Port (0x000B8000 / %into)"]
    G --> H["Host Terminal Screen ('A')"]
```

---

## 5. Architectural Invariants

1. **Bijective Transduction Parity**: Every byte transduced into 5 balanced trits must reconstruct back to the identical byte ($\sum_{i=0}^4 t_i \cdot 3^i$).
2. **Single-Cell Footprint**: Any single character glyph must evaluate to $\le 1\text{ Cell}$.
3. **Channel Isolation**: Kernel tasks on Track 0 cannot be preempted or modified by unprivileged App tasks on Track 2.
4. **Affirmative Seal**: All terminal rolls must commit in `+1:PROCEED`.
