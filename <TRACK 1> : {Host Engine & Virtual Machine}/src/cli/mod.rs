// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CLI Subsystem Identity, Module Architecture & Core Re-exports )
// ==============================================================================
//! OmniCode Command Line Subsystem
//!
//! Exposes interactive terminal REPL, script runners, structural 4-block linters,
//! and template scaffolding generators.

pub mod repl;
pub mod server;
pub mod commands;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Re-export Declarations & Public Interface )
// ==============================================================================
pub use repl::run_repl;
pub use server::run_gui_server;
pub use commands::{
    BootArch, BootConfig, BootContext, run_boot, run_boot_with_config,
    execute_kernel_pipeline, synthesize_hypervisor_command, execute_boot_pipeline, execute_boot_machine,
    DEFAULT_BOOT_ARCH, DEFAULT_GRAPHICAL_MODE, REAL_MODE_BOOT_VECTOR, VGA_TEXT_FRAMEBUFFER_BASE, COM1_SERIAL_IO_BASE,
    AssembleFormat, AssembleConfig, AssembleContext, run_assemble, run_assemble_with_config,
    execute_assemble_pipeline, serialize_assembled_output,
    RunFormat, RunConfig, RunContext, RunResult, run_with_config, execute_run_pipeline,
    run_on_virtual_terminal, run_omni_source, run_tape_ribbon, run_c_transpile, run_lua_transpile,
    AiConfig, AiSubcommand, AiResult, run_ai_with_config, execute_ai_pipeline,
    run_curriculum_training, run_ai_heal_file, render_ai_tools_overview, render_concordance_overview,
    print_verbose_deliberation,
};


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( CLI Orchestration Framework )
// ==============================================================================
// Command line entrypoints route to interactive REPL or single-file runners.


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing )
// ==============================================================================
// CLI subsystem verified and sealed.
