// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CLI Executable Identity, Subsystem Routing & Core Imports )
// ==============================================================================
//! OmniCode CLI Entrypoint
//!
//! Provides the primary terminal interface for the OmniCode language engine:
//!     - Interactive REPL Terminal (default)
//!     - Single-file Script Runner (`run <file.omni>`)
//!     - 4-Block Structural Linter (`lint <file.omni>`)
//!     - Canonical Template Scaffolder (`scaffold <Name>`)
//!     - Ontological Verification Demo (`demo`)

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;

use omnicode::{
    run_repl, run_gui_server, DiagnosticLogger, OmniDebugger, OmniLinter, Restorator,
    AICortex, AILearner, OmniFileType, Lexer, Parser, Evaluator, Environment,
    BiblicalLexicon, HoverEngine, Tape, OmniStudio,
};
use omnicode::cli::commands::{
    BootConfig, run_boot_with_config,
    AssembleConfig, run_assemble_with_config,
    RunConfig, run_with_config,
    AiConfig, run_ai_with_config,
};


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( CLI Help Display, Version Banner, and Command Route Setup )
// ==============================================================================
fn print_cli_help() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: Dual Human & Machine Readable Language Engine (v0.1.0)\x1b[0m");
    println!("\x1b[1;36m  Core: Balanced Ternary & Boundary Physics (Rust Engine)\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
    println!("\x1b[1;33mUSAGE:\x1b[0m");
    println!("  omnicode                    Start the interactive terminal shell (REPL)");
    println!("  omnicode repl               Start the interactive terminal shell (REPL)");
    println!("  omnicode run <file>         Execute an OmniCode source (.omni) or scroll (.scroll)
  omnicode echo <char|msg>    Echo character or message through native balanced ternary I/O transducer");
    println!("  omnicode assemble <file>    Assemble an OmniCode process tape into concrete memory layout (.tape)");
    println!("  omnicode studio <file.tape> Project process tape in the cinematic HUD terminal projector");
    println!("  omnicode project <file.tape> Project process tape in the cinematic HUD terminal projector");
    println!("  omnicode boot [--uefi|--multiboot|--bios] [--gui] Boot bare-metal OS in QEMU");
    println!("  omnicode lint <file>        Validate 4-Block architecture compliance (.omni / .scroll)");
    println!("  omnicode hover <file> <l> <c> Inspect cross-scale 3-tier hovercard for symbol at position");
    println!("  omnicode doc <symbol>       Query architectural docstring, invariants, and machine intent");
    println!("  omnicode restore <file>     Auto-heal syntax, boundary, and block anomalies");
    println!("  omnicode ai \"<prompt>\"      Transduce prompt with AI Cortex, self-heal & inscribe");
    println!("  omnicode ai map             Render real-time ASCII/ANSI synaptic brain topology");
    println!("  omnicode ai timeline        Render historical cognitive growth milestones timeline");
    println!("  omnicode ai curriculum      Train NeuralBrain across 25-chapter NotebookLM curriculum");
    println!("  omnicode ai heal <file>     Self-heal code with AI ToolBelt Two-Way Checksum parity");
    println!("  omnicode ai tools           Inspect 5 registered diagnostic tools in AI ToolBelt");
    println!("  omnicode ai concordance [t] Query multi-translation Scripture concordance pointers");
    println!("  omnicode interact           Launch live CPI-SI cognitive discourse & learning shell");
    println!("  omnicode train              Train balanced-ternary brain on ideal reels and logs");
    println!("  omnicode noise [stress]     Simulate noise degradation breakdown ladder");
    println!("  omnicode query <session>    Query an episodic memory scroll in O(1) time");
    println!("  omnicode diagnose <session> Diagnose failpoints and view immune repairs");
    println!("  omnicode rotate [--cap N]   Enforce storehouse capacity bounds (Occupancy <= Capacity)");
    println!("  omnicode scrolls            Inspect episodic memory bank and learning statistics");
    println!("  omnicode manifest           Display the canonical Covenant Manifest and moral axioms");
    println!("  omnicode lexicon [term]     Inspect the 100% offline Biblical world lexicon");
    println!("  omnicode gui [--port 8080]  Launch the local web GUI Studio & Terminal interface");
    println!("  omnicode desktop            Launch the native desktop GUI Studio & Terminal shell");
    println!("  omnicode scaffold <Name> [--type omni|scroll] Generate canonical 4-Block template");
    println!("  omnicode filetypes          Display all formal OmniCode file types and MIME definitions");
    println!("  omnicode demo               Run the built-in ontological verification suite");
    println!("  omnicode --help, -h         Show this help information");
    println!("  omnicode --version, -v      Display engine version\n");
    println!("\x1b[1;33mREPL FEATURES:\x1b[0m");
    println!("  • Real-time evaluation of balanced ternary expressions (-1, 0, +1, NULL)");
    println!("  • Boundary delimiter matching with multiline input buffering");
    println!("  • Interactive failpoint alerts and transaction snapshot rollback");
    println!("  • Live symbol inspection via `:env` and AST inspection via `:ast <expr>`");
    println!("  • Persistent command line history\n");
}

fn print_version() {
    println!("omnicode 0.1.0 (Rust Core: Balanced Ternary & Boundary Physics)");
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Command Handlers: Script Runner, Linter, Scaffolder, Ontological Demo )
// ==============================================================================
fn run_studio(path: &str) {
    let logger = DiagnosticLogger::new(false);
    let tape = match Tape::from_file(path, logger) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("\x1b[1;31mError loading tape for studio:\x1b[0m {e}");
            process::exit(1);
        }
    };
    let mut studio = OmniStudio::new(tape.head.name.clone(), tape.head.required_partition_size);
    studio.project_tape(&tape);
}

fn run_lint_file(path: &str) {
    let ft = OmniFileType::from_path(path);
    let title = ft.map(|f| f.display_name()).unwrap_or("OmniCode file");
    println!("\x1b[1;33m--- Linting Architecture ({title}): {} ---\x1b[0m", path);
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[1;31m✗ File read error:\x1b[0m Could not open '{}': {}", path, e);
            process::exit(1);
        }
    };

    let logger = DiagnosticLogger::new(false);
    let linter = OmniLinter::new(logger);
    let report = linter.lint_source(path, &content);

    if report.is_valid {
        println!("\x1b[1;32m✓ {title} Validation PASSED\x1b[0m");
        println!("  Blocks verified in sequence:");
        for b in &report.blocks_found {
            println!("    - Line {}: {}", b.line_number, b.kind.name());
        }

        if let Some(audit) = &report.audit_score {
            let is_tape = matches!(ft, Some(OmniFileType::Tape));

            // Block 1: Metadata / Tape Head
            let b1 = &audit.block1_metadata;
            let b1_label = if is_tape { "Tape Head Manifest" } else { "Metadata Block" };
            println!("\n  \x1b[1;34m[ BLOCK 1 : {b1_label} ]\x1b[0m");
            let score_color = if b1.total >= 90 { "\x1b[1;32m" } else if b1.total >= 70 { "\x1b[1;32m" } else if b1.total >= 50 { "\x1b[1;33m" } else { "\x1b[1;31m" };
            println!("  {score_color}★ Block 1 Invariant Score: {}/100 [{}]\x1b[0m", b1.total, b1.rating);
            println!("    Subsystem Breakdown:");
            if is_tape {
                println!("      • Formal Tape Head:        {}/20", b1.identity_score);
                println!("      • Partition & Domain:      {}/25", b1.provenance_score);
                println!("      • Invariant Scripture:     {}/20", b1.witness_score);
                println!("      • Cardinal Coordinates:    {}/20", b1.ontology_score);
                println!("      • Spatial & Naming Hygiene:{}/15", b1.spatial_hygiene_score);
            } else {
                println!("      • Identity & Declaration:  {}/20", b1.identity_score);
                println!("      • Provenance & Context:    {}/25", b1.provenance_score);
                println!("      • Covenant Witness Anchor: {}/20", b1.witness_score);
                println!("      • Ontology & Coordinates:  {}/20", b1.ontology_score);
                println!("      • Spatial & Naming Hygiene:{}/15", b1.spatial_hygiene_score);
            }
            for item in &b1.breakdown {
                println!("      {}", item);
            }

            // Block 2: Opening / Tape Setup
            let b2 = &audit.block2_opening;
            let b2_label = if is_tape { "Roll 1 Initial Setup & Preconditions" } else { "Opening Block & Precondition Equilibrium" };
            println!("\n  \x1b[1;34m[ BLOCK 2 : {b2_label} ]\x1b[0m");
            let score_color = if b2.total >= 90 { "\x1b[1;32m" } else if b2.total >= 70 { "\x1b[1;32m" } else if b2.total >= 50 { "\x1b[1;33m" } else { "\x1b[1;31m" };
            println!("  {score_color}★ Block 2 Invariant Score: {}/100 [{}]\x1b[0m", b2.total, b2.rating);
            println!("    Subsystem Breakdown:");
            if is_tape {
                println!("      • Roll 1 Entity Setup:     {}/25", b2.entity_allocation_score);
                println!("      • Memory Cell Target:      {}/20", b2.cell_container_score);
                println!("      • Inscription Vector:      {}/20", b2.substrate_bridge_score);
                println!("      • Container Parity Guard:  {}/20", b2.preconditions_target_score);
                println!("      • Spatial Ruler & Asides:  {}/15", b2.spatial_asides_score);
            } else {
                println!("      • Typed Entity Allocation: {}/25", b2.entity_allocation_score);
                println!("      • Cell Container Sizing:   {}/20", b2.cell_container_score);
                println!("      • Substrate Bridge/Trits:  {}/20", b2.substrate_bridge_score);
                println!("      • Preconditions & Targets: {}/20", b2.preconditions_target_score);
                println!("      • Spatial Ruler & Asides:  {}/15", b2.spatial_asides_score);
            }
            for item in &b2.breakdown {
                println!("      {}", item);
            }

            // Block 3: Body / Tape Rolls
            let b3 = &audit.block3_body;
            let b3_label = if is_tape { "Operational Tape Rolls Section" } else { "Body Block & Causal Pipelines" };
            println!("\n  \x1b[1;34m[ BLOCK 3 : {b3_label} ]\x1b[0m");
            let score_color = if b3.total >= 90 { "\x1b[1;32m" } else if b3.total >= 70 { "\x1b[1;32m" } else if b3.total >= 50 { "\x1b[1;33m" } else { "\x1b[1;31m" };
            println!("  {score_color}★ Block 3 Invariant Score: {}/100 [{}]\x1b[0m", b3.total, b3.rating);
            println!("    Subsystem Breakdown:");
            println!("      • Operational SVO Verbs:   {}/25", b3.svo_verbs_score);
            println!("      • Directional Vectors:     {}/20", b3.directional_vectors_score);
            println!("      • State-Prime Progression: {}/20", b3.state_prime_score);
            println!("      • Adversative Guard Caveat:{}/20", b3.guard_caveats_score);
            println!("      • Spatial Ruler & Asides:  {}/15", b3.spatial_asides_score);
            for item in &b3.breakdown {
                println!("      {}", item);
            }

            // Block 4: Closing / Commit Status
            let b4 = &audit.block4_closing;
            let b4_label = if is_tape { "Terminal Commit Status Roll" } else { "Closing Block & Sealing Gate" };
            println!("\n  \x1b[1;34m[ BLOCK 4 : {b4_label} ]\x1b[0m");
            let score_color = if b4.total >= 90 { "\x1b[1;32m" } else if b4.total >= 70 { "\x1b[1;32m" } else if b4.total >= 50 { "\x1b[1;33m" } else { "\x1b[1;31m" };
            println!("  {score_color}★ Block 4 Invariant Score: {}/100 [{}]\x1b[0m", b4.total, b4.rating);
            println!("    Subsystem Breakdown:");
            if is_tape {
                println!("      • Commit Roll Identity:    {}/20", b4.verification_proofs_score);
                println!("      • Preceding Roll Condition:{}/20", b4.invariant_validation_score);
                println!("      • Zero-Tolerance Tripwire: {}/20", b4.tripwire_breaker_score);
                println!("      • Fourfold T4 Machine:     {}/25", b4.fourfold_state_machine_score);
                println!("      • Spatial Ruler & Asides:  {}/15", b4.telemetry_asides_score);
            } else {
                println!("      • Operational Proofs:      {}/20", b4.verification_proofs_score);
                println!("      • Invariant Bounds Parity: {}/20", b4.invariant_validation_score);
                println!("      • Zero-Tolerance Tripwire: {}/20", b4.tripwire_breaker_score);
                println!("      • Fourfold T4 Machine:     {}/25", b4.fourfold_state_machine_score);
                println!("      • Telemetry & Spatial:     {}/15", b4.telemetry_asides_score);
            }
            for item in &b4.breakdown {
                println!("      {}", item);
            }

            // Composite 4-Block Summary & QuadState Gate
            println!("\n  \x1b[1;36m==============================================================================\x1b[0m");
            let comp_color = if audit.composite_score >= 70 {
                "\x1b[1;32m"
            } else if audit.composite_score >= 50 {
                "\x1b[1;33m"
            } else {
                "\x1b[1;31m"
            };
            println!("  {comp_color}COMPOSITE 4-BLOCK AUDIT SCORE: {:+}/100\x1b[0m", audit.composite_score);
            println!("  COMPILATION GATE: \x1b[1m{}\x1b[0m", audit.gate.name());
            if audit.eligible_to_compile {
                println!("  \x1b[1;32m✓ ELIGIBLE TO COMPILE: Source unit certified for AST lowering & assembly\x1b[0m");
            } else {
                println!("  \x1b[1;31m✗ COMPILATION DISQUALIFIED: Fails threshold for assembly lowering\x1b[0m");
            }
            println!("  \x1b[1;36m==============================================================================\x1b[0m\n");
        }

        if !report.warnings.is_empty() {
            println!("\x1b[1;33m⚠ Lint Warnings / Recommendations ({}):\x1b[0m", report.warnings.len());
            for warn in &report.warnings {
                println!("    • {}", warn);
            }
        }
    } else {
        println!("\x1b[1;31m✗ {title} Validation FAILED\x1b[0m");
        for err in &report.errors {
            eprintln!("    • {}", err);
        }
        process::exit(1);
    }
}

fn run_scaffold(name: &str, file_type: Option<&str>) {
    let ft = file_type
        .and_then(OmniFileType::from_extension)
        .unwrap_or(OmniFileType::SourceReel);
    let scaffolded = match ft {
        OmniFileType::SourceReel => OmniLinter::auto_scaffold_omni(name, "User-defined module"),
        OmniFileType::EpisodicScroll => OmniLinter::auto_scaffold_scroll(name, "User cognitive discourse intent"),
        OmniFileType::Tape => ft.scaffold_template(name, "User process execution tape"),
    };
    println!("{}", scaffolded);
}

fn run_filetypes() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode File Type System & Taxonomy Manifest                                \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    for (i, ft) in OmniFileType::all().iter().enumerate() {
        println!("  \x1b[1;33m[{}]\x1b[0m \x1b[1;32m.{}\x1b[0m  —  \x1b[1m{}\x1b[0m", i + 1, ft.extension(), ft.display_name());
        println!("      MIME:        \x1b[36m{}\x1b[0m", ft.mime_type());
        println!("      Language ID: \x1b[35m{}\x1b[0m", ft.language_id());
        println!("      Purpose:     {}\n", ft.description());
    }
}

fn run_demo() {
    println!("\x1b[1;36m=====================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode: Ontological Conceptual Testbed           \x1b[0m");
    println!("\x1b[1;36m=====================================================\x1b[0m\n");

    let demo_code = fs::read_to_string("examples/canonical_notation.omni")
        .unwrap_or_else(|_| OmniLinter::auto_scaffold_omni("DemoModule", "Ontological verification module"));


    println!("\x1b[1;33m--- Running 4-Block Structural Linter ---\x1b[0m");
    let logger = DiagnosticLogger::new(true);
    let linter = OmniLinter::new(logger.clone());
    let lint_report = linter.lint_source("demo_code", &demo_code);

    if lint_report.is_valid {
        println!("\x1b[1;32m✓ 4-Block Structural Check Passed:\x1b[0m Found {} blocks in order.", lint_report.blocks_found.len());
    } else {
        println!("\x1b[1;31m✗ Structural Linter Errors:\x1b[0m {:?}", lint_report.errors);
    }

    println!("\n\x1b[1;33m--- Running Code Evaluation Pipeline ---\x1b[0m");
    let mut lexer = Lexer::new(&demo_code, logger.clone());

    match lexer.tokenize() {
        Ok(tokens) => {
            println!("\x1b[1;32m✓ Lexer Tokenization Succeeded:\x1b[0m ({} tokens)", tokens.len());
            let debugger = OmniDebugger::new(logger.clone(), 5);
            let mut parser = Parser::new(tokens, debugger);

            match parser.parse_program() {
                Ok(prog) => {
                    println!("\x1b[1;32m✓ Parser Succeeded:\x1b[0m Generated AST with {} statements", prog.statements.len());
                    let mut evaluator = Evaluator::new(Environment::new(), parser.debugger);
                    match evaluator.eval(&omnicode::ast::nodes::ASTNode::Program(prog)) {
                        Ok(res) => {
                            println!("\n\x1b[1;32m✓ Execution Final Result:\x1b[0m => {}", res.format_colored());
                        }
                        Err(e) => eprintln!("\x1b[1;31m✗ Evaluator Error:\x1b[0m {}", e),
                    }
                }
                Err(e) => eprintln!("\x1b[1;31m✗ Parser Error:\x1b[0m {}", e),
            }
        }
        Err(e) => eprintln!("\x1b[1;31m✗ Lexer Error:\x1b[0m {}", e),
    }

    if logger.has_failpoints() {
        println!("\n\x1b[1;33m--- Diagnostic Failpoints Caught During Run ---\x1b[0m");
        for fp in logger.failpoints() {
            println!("  * [{}] {}", fp.channel, fp.message);
        }
    } else {
        println!("\n\x1b[1;32m✓ No unexpected failpoints encountered during demo run.\x1b[0m");
    }
}

fn run_restore(path: &str, in_place: bool) {
    println!("\x1b[1;36mAutonomous Self-Healing Restoration:\x1b[0m {}", path);
    let logger = DiagnosticLogger::new(false);
    let restorator = Restorator::new(logger);

    match restorator.restore_file(path, in_place) {
        Ok(report) => {
            if !report.was_modified {
                println!("\x1b[1;32m✓ Source is already healthy:\x1b[0m No repairs needed.");
            } else {
                println!("\x1b[1;33m⚠ Repairs Applied ({}):\x1b[0m", report.repairs_applied.len());
                for repair in &report.repairs_applied {
                    println!("    • L{}: [{:?}] {}", repair.line, repair.kind, repair.description);
                }
                if in_place {
                    println!("\x1b[1;32m✓ Healed file written in-place to {}\x1b[0m", path);
                } else {
                    println!("\n\x1b[1;36m--- Restored Content Preview ---\x1b[0m\n{}", report.restored_content);
                }
            }

            if report.verified {
                println!("\x1b[1;32m✓ Post-Restoration Verification PASSED:\x1b[0m Code executes to => +1:PROCEED");
            } else if !report.errors.is_empty() {
                println!("\x1b[1;33m⚠ Verification Diagnostics:\x1b[0m {:?}", report.errors);
            }
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Restoration Failed:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}



fn run_scrolls() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Episodic Memory Bank & Experiential Learning Archive           \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");
    let scroll_dir = omnicode::core::filetype::resolve_scroll_storage_dir();
    let learner = AILearner::new(&scroll_dir);
    match learner.digest_experiences() {
        Ok(summary) => {
            println!("\x1b[1;33mEPISODIC LEARNING SUMMARY:\x1b[0m");
            println!("  • Total Inscribed Episodes:     {}", summary.total_episodes);
            println!("  • State Distribution:           Proceed (+1): {} | Halt (NULL): {} | Rollback (-1): {} | Await (0): {}",
                summary.proceed_count, summary.halt_count, summary.rollback_count, summary.await_count
            );
            println!("  • Restorator Repairs Learned:   {}\n", summary.total_repairs_learned);

            if summary.experiences.is_empty() {
                println!("  (No scrolls recorded yet in `scrolls/`. Run `omnicode ai \"...\"` to generate experiences.)\n");
            } else {
                println!("\x1b[1;33mRECENT EPISODES:\x1b[0m");
                for (i, exp) in summary.experiences.iter().enumerate() {
                    let log_status = if exp.log_path.is_some() { "yes" } else { "no" };
                    println!(
                        "  [{:>2}] Session: {:<12} | State: +1:{:<8} | Repairs: {:>2} | Failpoints: {:>2} | .log: {}",
                        i + 1, exp.session_id, exp.final_state.macro_op(), exp.repair_count, exp.failpoint_count, log_status
                    );
                    println!("       Prompt: \"{}\"", exp.prompt);
                    println!("       Scroll: {:?}\n", exp.scroll_path);
                }
            }
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Failed to read scrolls:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_query(session_id: &str) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: O(1) Episodic Memory & Telemetry Query Engine                  \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let cortex = AICortex::new();
    match cortex.query_episode(session_id) {
        Some(session) => {
            let state_str = match session.final_state.as_int() {
                Some(i) => format!("{i:+}:{}", session.final_state.macro_op()),
                _ => format!("*:{}", session.final_state.macro_op()),
            };
            println!("\x1b[1;32m✓ Session Located in O(1) Time:\x1b[0m");
            println!("  • Session ID:        {}", session.session_id);
            println!("  • Domain Category:   {}", session.category);
            println!("  • Commit Quad-State: {}", state_str);
            println!("  • Inscribed Prompt:  \"{}\"", session.prompt);
            println!("  • Trapped Failpoints: {}", session.failpoint_count);
            println!("  • Restorator Repairs: {}", session.repair_count);
            println!("  • Inscribed Scroll:  {:?}", session.scroll_path);
            if let Some(ref lp) = session.log_path {
                println!("  • Paired Telemetry:  {:?}", lp);
            } else {
                println!("  • Paired Telemetry:  (none)");
            }
            println!();
        }
        None => {
            eprintln!(
                "\x1b[1;31m✗ Episode Not Found:\x1b[0m Session '{}' does not exist in storehouse.",
                session_id
            );
            process::exit(1);
        }
    }
}

fn run_diagnose(session_id: &str) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Failpoint & Anomaly Diagnostic Oracle                          \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let cortex = AICortex::new();
    match cortex.diagnose_failpoint(session_id) {
        Some(report) => {
            println!("{report}");
        }
        None => {
            eprintln!(
                "\x1b[1;31m✗ Episode Not Found:\x1b[0m Session '{}' does not exist in storehouse.",
                session_id
            );
            process::exit(1);
        }
    }
}

fn run_rotate(max_files: usize) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode Storehouse: Vessel Capacity Rotation Engine                        \x1b[0m");
    println!("\x1b[1;36m  Invariant Law: Occupancy <= Capacity (2 Kings 4:4)                          \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let scroll_dir = omnicode::core::filetype::resolve_scroll_storage_dir();
    let policy = omnicode::diagnostics::logger::RotationPolicy::new(
        max_files, true,
    );
    let logger = omnicode::ai::ScrollLogger::with_rotation_policy(
        &scroll_dir, policy,
    );

    match logger.rotate_storehouse() {
        Ok(rotated) => {
            println!("\x1b[1;32m✓ Storehouse Rotation Enforced:\x1b[0m");
            println!("  • Target Partition Capacity:  {} files", max_files);
            println!(
                "  • Excess Files Archived:     {} files moved to 'archive/'\n",
                rotated
            );
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Storehouse Rotation Failed:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_train() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: CPI-SI Base Neural Trainer & Weight Freezing                   \x1b[0m");
    println!("\x1b[1;36m  Ingesting Archetypes & Runtime Logs into Balanced-Ternary Weights           \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let scroll_dir = omnicode::core::filetype::resolve_scroll_storage_dir();
    let learner = omnicode::ai::AILearner::new(&scroll_dir);

    match learner.digest_experiences() {
        Ok(summary) => {
            let logger = DiagnosticLogger::new(false);
            let mut brain = omnicode::ai::NeuralBrain::new(
                &[64, 32, 16, 4], 1, logger,
            );
            let trainer = omnicode::ai::NeuralTrainer::new();

            // 1. Stage 1 Foundational Archetype Training
            let archetypes = [
                "Idea { Type } : Character (Atomic rune, 1 Cell, 5 trits)",
                "Exemplar 'A': 65 <=> [ -1, +1, +1, -1, +1 ]",
                "Idea { Type } : Number (Radix-3 balanced scalar value)",
                "Exemplar '0': 0 <=> [ 0, 0, 0, 0, 0 ]",
                "Idea { Type } : Container (Primordial boundary perimeter)",
                "Boundary Invariant: Strict capacity and leak defense",
                "Covenant Identity: CPI-SI Base model conceived 2 years ago",
                "Neural Plasticity: BCM homeostatic threshold adaptation",
            ];
            let archetypes_adapted =
                trainer.train_archetypes(&mut brain, &archetypes);

            // 2. Episodic Experience Training
            let report = trainer.train(&mut brain, &summary.experiences);

            // 3. Persist Frozen Baseline Weights
            let weights_path =
                PathBuf::from("storage/weights/cpisi_base.weights");
            let save_result = brain.save_weights(&weights_path);

            let (exc, neu, inh) = brain.synaptic_counts();

            println!("\x1b[1;32m✓ CPI-SI Base Training Completed:\x1b[0m");
            println!("  • Archetype Inscriptions:    {}", archetypes_adapted);
            println!("  • Episodes Digested:         {}", report.total_episodes);
            println!("  • Total Synapses Adapted:    {}", report.synapses_adapted);
            println!("  • Clean (+1:Proceed) Paths:  {}", report.clean_proceed_count);
            println!("  • Rollback (-1) Protections: {}", report.degraded_rollback_count);
            println!(
                "  • Synapse Breakdown:         +1: {} | 0: {} | -1: {}",
                exc, neu, inh
            );
            println!(
                "  • Resilience Before:         {:.1}%",
                report.noise_resilience_before * 100.0
            );
            println!(
                "  • Resilience After:          {:.1}%",
                report.noise_resilience_after * 100.0
            );
            match save_result {
                Ok(_) => println!("  • Saved Frozen Weights:      {:?}\n", weights_path),
                Err(e) => println!("  • Weight Save Warning:       {}\n", e),
            }
        }
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Training Failed:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_interact() {
    omnicode::cli::commands::ai::run_ai_interactive().ok();
}

fn run_noise(stress: Option<f64>) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Structured Non-Random Degradation & Noise Engine               \x1b[0m");
    println!("\x1b[1;36m  Simulating Systemic Entropy & Physical Mind Breakdown Dynamics              \x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let engine = omnicode::ai::NoiseEngine::default();
    let ideal_stimulus = &[
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
        omnicode::core::ternary::QuadState::Proceed,
    ];

    let logger = DiagnosticLogger::new(false);
    let brain = omnicode::ai::NeuralBrain::new(&[8, 16, 4], 1, logger);

    if let Some(s) = stress {
        let (perturbed, out) =
            brain.think_with_noise(ideal_stimulus, &engine, s);
        println!("\x1b[1;33mSTRESS LEVEL: {:.2}\x1b[0m", s);
        println!("  • Input Vector:     {:?}", perturbed);
        println!("  • Cortical Output:  {:?}\n", out);
    } else {
        println!("\x1b[1;33mDEGRADATION LADDER (Ideal -> Breakdown):\x1b[0m");
        let ladder = engine.simulate_breakdown_ladder(ideal_stimulus);
        for step in &ladder {
            let out = brain.think(&step.perturbed_vector);
            let mode_str = match step.dominant_mode {
                omnicode::ai::DegradationMode::Ideal => "Pristine (Ideal)",
                omnicode::ai::DegradationMode::TritDecay => {
                    "Trit Attenuation (Decay)"
                }
                omnicode::ai::DegradationMode::BoundaryLeak => {
                    "Boundary Cross-Talk (Leak)"
                }
                omnicode::ai::DegradationMode::ThermalInversion => {
                    "Thermal Perturbation (Inversion)"
                }
                omnicode::ai::DegradationMode::FailpointTrap => {
                    "Circuit Halt (Catastrophic)"
                }
            };
            println!(
                "  [Stress {:>4.0}%] Mode: {:<32} | Fidelity: {:>5.1}%",
                step.stress_level * 100.0,
                mode_str,
                step.signal_fidelity * 100.0
            );
            println!(
                "    Sensory: {:?} -> Motor: {:?}",
                step.perturbed_vector, out
            );
        }
        let resilience =
            brain.evaluate_noise_resilience(ideal_stimulus, &engine);
        println!(
            "\n  • Overall Neural Noise Resilience Score: {:.1}%\n",
            resilience * 100.0
        );
    }
}

fn run_manifest() {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: Invariant Covenant Manifest (manifest.omni)                     \x1b[0m");
    println!("\x1b[1;36m  Anchored in Public Domain Scriptures (King James Version & World English Bible)\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    match fs::read_to_string("manifest.omni") {
        Ok(content) => println!("{}", content),
        Err(e) => {
            eprintln!("\x1b[1;31m✗ Could not open manifest.omni:\x1b[0m {}", e);
            process::exit(1);
        }
    }
}

fn run_lexicon(query: Option<&str>) {
    println!("\x1b[1;36m===============================================================================\x1b[0m");
    println!("\x1b[1;36m  OmniCode AI: 100% Offline Biblical World Lexicon & Moral Ontology           \x1b[0m");
    println!("\x1b[1;36m  Public Domain Primary Witness: King James Version (1611) & World English Bible\x1b[0m");
    println!("\x1b[1;36m===============================================================================\x1b[0m\n");

    let lex = BiblicalLexicon::new();
    if let Some(q) = query {
        let results = lex.search(q);
        if results.is_empty() {
            println!("No concepts found matching '{}'. Total concepts available: {}\n", q, lex.all_entries().len());
        } else {
            println!("\x1b[1;33mSEARCH RESULTS FOR '{}' ({} found):\x1b[0m\n", q, results.len());
            for e in results {
                println!("\x1b[1;32m[{}]\x1b[0m \x1b[1m{}\x1b[0m (State Resonance: +1:{})", e.domain.as_str(), e.term.to_uppercase(), e.quad_state_resonance.macro_op());
                println!("  • Definition: {}", e.definition);
                println!("  • KJV:        {}", e.kjv_passage);
                println!("  • WEB:        {}", e.web_passage);
                println!("  • Guidance:   {}\n", e.moral_guidance);
            }
        }
    } else {
        println!("\x1b[1;33mCANONICAL VOCABULARY OVERVIEW ({} concepts available offline):\x1b[0m\n", lex.all_entries().len());
        for e in lex.all_entries() {
            println!("  • \x1b[1;32m{:<20}\x1b[0m | Domain: {:<30} | Resonance: +1:{}", e.term, e.domain.as_str(), e.quad_state_resonance.macro_op());
        }
        println!("\nTip: Query a specific concept with `omnicode lexicon <term>` (e.g. `omnicode lexicon truth`)\n");
    }
}

fn run_gui(port: u16, auto_open: bool, path: &str) {
    if auto_open {
        let url = format!("http://localhost:{port}{path}");
        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(300));
            omnicode::cogs::platform::HostPlatform::current().open_browser(&url);
        });
    }
    if let Err(e) = run_gui_server(port) {
        eprintln!("\x1b[1;31m✗ GUI Server Fatal Error:\x1b[0m {}", e);
        process::exit(1);
    }
}



fn run_hover(file: &str, line: usize, col: usize, as_json: bool) {
    let logger = DiagnosticLogger::new(false);
    let mut engine = HoverEngine::new(logger);
    engine.index_workspace(".");

    match engine.hover_at(file, line, col) {
        Some(card) => {
            if as_json {
                println!("{}", card.to_json());
            } else {
                println!("{}", card.render_terminal());
            }
        }
        None => {
            if as_json {
                println!(r#"{{"symbol":"","found":false,"error":"No symbol found at coordinate"}}"#);
            } else {
                eprintln!("\x1b[1;33m⚠ No symbol located at {}:{}:{}\x1b[0m", file, line, col);
            }
            process::exit(1);
        }
    }
}

fn run_doc(symbol: &str, file_hint: Option<&str>, as_json: bool) {
    let logger = DiagnosticLogger::new(false);
    let mut engine = HoverEngine::new(logger);
    engine.index_workspace(".");

    match engine.document_symbol(symbol, file_hint) {
        Some(card) => {
            if as_json {
                println!("{}", card.to_json());
            } else {
                println!("{}", card.render_terminal());
            }
        }
        None => {
            if as_json {
                println!(r#"{{"symbol":"{}","found":false,"error":"Symbol definition not found"}}"#, symbol);
            } else {
                eprintln!("\x1b[1;33m⚠ Symbol '{}' not found in workspace or target file\x1b[0m", symbol);
            }
            process::exit(1);
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( CLI Main Entrypoint Dispatcher & Process Termination )
// ==============================================================================
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        // Default behavior: launch interactive terminal environment (REPL)
        if let Err(e) = run_repl() {
            eprintln!("\x1b[1;31mFatal REPL Error:\x1b[0m {}", e);
            process::exit(1);
        }
        return;
    }

    match args[1].as_str() {
        "repl" => {
            if let Err(e) = run_repl() {
                eprintln!("\x1b[1;31mFatal REPL Error:\x1b[0m {}", e);
                process::exit(1);
            }
        }
        "terminal" | "term" | "shell" => {
            let port = if args.len() >= 4 && (args[2] == "--port" || args[2] == "-p") {
                args[3].parse::<u16>().unwrap_or(8080)
            } else if args.len() >= 3 && args[2].starts_with("--port=") {
                args[2]["--port=".len()..].parse::<u16>().unwrap_or(8080)
            } else if args.len() >= 3 && args[2] != "terminal" && args[2] != "term" {
                args[2].parse::<u16>().unwrap_or(8080)
            } else {
                8080
            };
            println!("\x1b[1;36mLaunching OmniCode Native Interactive Terminal (CreativeWorkzStudio LLC)...\x1b[0m");
            run_gui(port, true, "/terminal");
        }
        "project" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode project <file.tape>");
                process::exit(1);
            }
            run_studio(&args[2]);
        }
        "studio" => {
            if args.len() >= 3 && (args[2].ends_with(".tape") || args[2].ends_with(".omni") || !args[2].starts_with("-")) && args[2].parse::<u16>().is_err() {
                run_studio(&args[2]);
            } else {
                let port = if args.len() >= 4 && (args[2] == "--port" || args[2] == "-p") {
                    args[3].parse::<u16>().unwrap_or(8080)
                } else if args.len() >= 3 && args[2].starts_with("--port=") {
                    args[2]["--port=".len()..].parse::<u16>().unwrap_or(8080)
                } else if args.len() >= 3 && args[2] != "studio" {
                    args[2].parse::<u16>().unwrap_or(8080)
                } else {
                    8080
                };
                println!("\x1b[1;36mLaunching OmniCode Studio & Platform (CreativeWorkzStudio LLC)...\x1b[0m");
                run_gui(port, true, "/studio");
            }
        }
        "gui" | "serve" | "desktop" | "app" => {
            let port = if args.len() >= 4 && (args[2] == "--port" || args[2] == "-p") {
                args[3].parse::<u16>().unwrap_or(8080)
            } else if args.len() >= 3 && args[2].starts_with("--port=") {
                args[2]["--port=".len()..].parse::<u16>().unwrap_or(8080)
            } else if args.len() >= 3 && args[2] != "gui" && args[2] != "serve" && args[2] != "desktop" && args[2] != "app" {
                args[2].parse::<u16>().unwrap_or(8080)
            } else {
                8080
            };
            println!("\x1b[1;36mLaunching OmniCode Studio & Platform (CreativeWorkzStudio LLC)...\x1b[0m");
            run_gui(port, true, "/studio");
        }
        "boot" | "qemu" => {
            let config = BootConfig::from_args(&args);
            if let Err(e) = run_boot_with_config(config) {
                eprintln!("\x1b[1;31mMachine Inception Halted:\x1b[0m {e}");
                process::exit(1);
            }
        }
        "assemble" | "asm" => {
            let sub_args: Vec<String> = args.iter().skip(2).cloned().collect();
            let config = match AssembleConfig::from_args(&sub_args) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("\x1b[1;31mError:\x1b[0m {e}");
                    process::exit(1);
                }
            };
            if let Err(e) = run_assemble_with_config(config) {
                eprintln!("\x1b[1;31m✗ Assemble Tripwire Error:\x1b[0m {e}");
                process::exit(1);
            }
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode run <file> [--format omni|tape|c|lua] [--transpile c|lua]");
                process::exit(1);
            }
            let sub_args: Vec<String> = args.iter().skip(2).cloned().collect();
            let config = match RunConfig::from_args(&sub_args) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("\x1b[1;31mError:\x1b[0m {e}");
                    process::exit(1);
                }
            };
            match run_with_config(config) {
                Ok(res) => {
                    println!("{}", res.output);
                }
                Err(e) => {
                    eprintln!("\x1b[1;31m✗ Run Error:\x1b[0m {e}");
                    process::exit(1);
                }
            }
        }
        "echo" => {
            let verbose = args.iter().any(|a| a == "--verbose" || a == "-v" || a == "--log");
            let filtered_args: Vec<String> = args.iter().skip(2).filter(|a| !a.starts_with("-")).cloned().collect();
            let payload = filtered_args.join(" ");
            let unquoted = if (payload.starts_with('\'') && payload.ends_with('\'') && payload.len() >= 2)
                || (payload.starts_with('"') && payload.ends_with('"') && payload.len() >= 2)
            {
                &payload[1..payload.len() - 1]
            } else {
                &payload
            };
            match omnicode::terminal::universal::call_response_engine::EchoEngine::echo_stream(unquoted) {
                Ok(echoed_resp) => {
                    let echoed = &echoed_resp.echoed;
                    let mut log_details = String::new();
                    for (i, c) in unquoted.chars().enumerate() {
                        if let Ok(res) = omnicode::terminal::universal::call_response_engine::EchoEngine::echo_char(c) {
                            log_details.push_str(&format!(
                                "  [{:>2}] Glyph: '{}' | Code: {:>3} (0x{:02X}) | 5-Trits: {:?} | Footprint: {} Cell | State: +1:{}\n",
                                i + 1, res.glyph, res.glyph as u32, res.glyph as u32, res.trits, res.cell_footprint, res.state.macro_op()
                            ));
                        }
                    }

                    // Inscribe persistent dual log (scroll and telemetry log)
                    let session_id = format!("echo_{}", std::process::id());
                    let timestamp_str = omnicode::ai::model::scroll::generate_timestamp_id();
                    let scroll = omnicode::ai::model::scroll::ExecutionScroll {
                        session_id,
                        timestamp_str,
                        prompt: format!("Echo call: \"{unquoted}\""),
                        synthesized_omnicode: format!("# Character Transduction Echo Reel\necho [ \"{unquoted}\" ] => [ \"{echoed}\" ]\n"),
                        execution_state: omnicode::core::ternary::QuadState::Proceed,
                        natural_response: format!("Echoed: \"{echoed}\""),
                        failpoints: Vec::new(),
                        repairs_applied: Vec::new(),
                        remediations_suggested: Vec::new(),
                        elapsed_micros: 100,
                    };
                    let saved_paths = omnicode::ai::model::scroll::ScrollLogger::default().save_dual_logs(&scroll).ok();

                    if verbose {
                        println!("\x1b[1;36m===============================================================================\x1b[0m");
                        println!("\x1b[1;36m  OMNICODE PRIMORDIAL ECHO: CHARACTER TRANSDUCTION TELEMETRY LOG             \x1b[0m");
                        println!("\x1b[1;36m===============================================================================\x1b[0m");
                        println!("  • Call Input (FROM Port 0x03F8):         \"{}\"", unquoted);
                        println!("  • Response Output (INTO Port 0x000B8000): \"{}\"", echoed);
                        println!("  • Stream Length:                         {} glyph(s)", echoed.len());
                        if let Some((ref sp, ref lp)) = saved_paths {
                            println!("  • Inscribed Episodic Scroll:             {:?}", sp);
                            println!("  • Inscribed Telemetry Log:               {:?}", lp);
                        }
                        println!("-------------------------------------------------------------------------------");
                        print!("{log_details}");
                        println!("-------------------------------------------------------------------------------");
                        println!("\x1b[1;32m✓ Parity Verified:\x1b[0m Call-Response dialogue sealed in equilibrium (State: +1:PROCEED)\n");
                    } else {
                        println!("{echoed}");
                    }
                }
                Err(e) => {
                    eprintln!("\x1b[1;31m✗ Echo Anomaly:\x1b[0m {e}");
                    process::exit(1);
                }
            }
        }
        "lint" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode lint <file.omni>");
                process::exit(1);
            }
            run_lint_file(&args[2]);
        }
        "hover" => {
            if args.len() < 5 {
                eprintln!("\x1b[1;31mError:\x1b[0m Usage: omnicode hover <file> <line> <col> [--json]");
                process::exit(1);
            }
            let file = &args[2];
            let line = args[3].parse::<usize>().unwrap_or(1);
            let col = args[4].parse::<usize>().unwrap_or(1);
            let as_json = args.iter().any(|a| a == "--json" || a == "-j");
            run_hover(file, line, col, as_json);
        }
        "doc" | "explain" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Usage: omnicode doc <symbol> [--file <path>] [--json]");
                process::exit(1);
            }
            let symbol = &args[2];
            let as_json = args.iter().any(|a| a == "--json" || a == "-j");
            let file_hint = args.iter().position(|a| a == "--file" || a == "-f").and_then(|idx| args.get(idx + 1).map(|s| s.as_str()));
            run_doc(symbol, file_hint, as_json);
        }
        "restore" => {
            if args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m Missing file path. Usage: omnicode restore <file.omni> [--write]");
                process::exit(1);
            }
            let in_place = args.iter().any(|a| a == "--write" || a == "-w");
            run_restore(&args[2], in_place);
        }
        "query" => {
            if args.len() < 3 {
                eprintln!(
                    "\x1b[1;31mError:\x1b[0m Missing session ID. \
                     Usage: omnicode query <session_id>"
                );
                process::exit(1);
            }
            run_query(&args[2]);
        }
        "diagnose" => {
            if args.len() < 3 {
                eprintln!(
                    "\x1b[1;31mError:\x1b[0m Missing session ID. \
                     Usage: omnicode diagnose <session_id>"
                );
                process::exit(1);
            }
            run_diagnose(&args[2]);
        }
        "rotate" => {
            let cap = if args.len() >= 3 {
                if args[2].starts_with("--cap=") {
                    args[2]["--cap=".len()..]
                        .parse::<usize>()
                        .unwrap_or(1000)
                } else if (args[2] == "--cap" || args[2] == "-c")
                    && args.len() >= 4
                {
                    args[3].parse::<usize>().unwrap_or(1000)
                } else {
                    args[2].parse::<usize>().unwrap_or(1000)
                }
            } else {
                1000
            };
            run_rotate(cap);
        }
        "interact" | "chat" | "cpisi" => {
            run_interact();
        }
        "train" => {
            run_train();
        }
        "noise" => {
            let stress = args.get(2).and_then(|s| s.parse::<f64>().ok());
            run_noise(stress);
        }
        "ai" => {
            let sub_args: Vec<String> = args.iter().skip(2).cloned().collect();
            if sub_args.is_empty() {
                eprintln!(
                    "\x1b[1;31mError:\x1b[0m Missing AI subcommand or prompt.\n\
                     Usage: omnicode ai [chat|map|timeline|curriculum|heal|tools|concordance|query|diagnose|interact] ..."
                );
                process::exit(1);
            }
            let config = match AiConfig::from_args(&sub_args) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("\x1b[1;31mAI Command Error:\x1b[0m {e}");
                    process::exit(1);
                }
            };
            if config.subcommand == omnicode::cli::commands::AiSubcommand::Interact {
                run_interact();
            } else if let Err(e) = run_ai_with_config(config) {
                eprintln!("\x1b[1;31m✗ AI Execution Tripwire:\x1b[0m {e}");
                process::exit(1);
            }
        }
        "scrolls" | "history" | "learn" => {
            run_scrolls();
        }
        "manifest" => {
            run_manifest();
        }
        "lexicon" | "bible" | "vocab" => {
            let query = args.get(2).map(|s| s.as_str());
            run_lexicon(query);
        }
        "filetypes" | "types" => {
            run_filetypes();
        }
        "scaffold" => {
            let (name, kind) = if args.len() >= 4 && (args[2] == "--type" || args[2] == "-t") {
                (args.get(4).map(|s| s.as_str()).unwrap_or("MyModule"), Some(args[3].as_str()))
            } else if args.len() >= 4 && (args[3] == "--type" || args[3] == "-t") {
                (args[2].as_str(), args.get(4).map(|s| s.as_str()))
            } else if args.len() >= 4 && (args[2] == "scroll" || args[2] == "omni") {
                (args[3].as_str(), Some(args[2].as_str()))
            } else if args.len() >= 3 {
                (args[2].as_str(), None)
            } else {
                ("MyModule", None)
            };
            run_scaffold(name, kind);
        }
        "demo" => {
            run_demo();
        }
        "--help" | "-h" | "help" => {
            print_cli_help();
        }
        "--version" | "-v" | "version" => {
            print_version();
        }
        other => {
            eprintln!("\x1b[1;31mUnknown argument:\x1b[0m '{}'", other);
            print_cli_help();
            process::exit(1);
        }
    }
}
