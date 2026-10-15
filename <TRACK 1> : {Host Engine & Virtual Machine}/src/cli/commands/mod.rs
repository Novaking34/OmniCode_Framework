// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( CLI Commands Subsystem Identity & Command Gear Registry )
// ==============================================================================
//! OmniCode CLI Command Gears
//!
//! Encapsulates discrete CLI command implementations as modular mechanical gears
//! driven by the main orchestrator engine.

pub mod boot;
pub mod assemble;
pub mod run;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Command Re-exports )
// ==============================================================================
pub use boot::{
    BootArch, BootConfig, BootContext, run_boot, run_boot_with_config,
    execute_kernel_pipeline, synthesize_hypervisor_command, execute_boot_pipeline, execute_boot_machine,
    DEFAULT_BOOT_ARCH, DEFAULT_GRAPHICAL_MODE, REAL_MODE_BOOT_VECTOR, VGA_TEXT_FRAMEBUFFER_BASE, COM1_SERIAL_IO_BASE,
};

pub use assemble::{
    AssembleFormat, AssembleConfig, AssembleContext, run_assemble, run_assemble_with_config,
    execute_assemble_pipeline, serialize_assembled_output,
};

pub use run::{
    RunFormat, RunConfig, RunContext, RunResult, run_with_config, execute_run_pipeline,
    run_on_virtual_terminal, run_omni_source, run_tape_ribbon, run_c_transpile, run_lua_transpile,
};

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Command Routing & Common Dispatch Logic )
// ==============================================================================

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Verification and Sealing )
// ==============================================================================
// Command gears verified and sealed.
