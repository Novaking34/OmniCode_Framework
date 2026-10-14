// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Procedural Process Ribbon Lowering Engine & Memory Partition Sizing )
// ==============================================================================
//! OmniCode Runtime: Tape Process Ribbon Lowering
//!
//! # Architectural Scope & Purpose:
//! Lowers an entire temporal procedural process ribbon (`.tape`) into physical
//! silicon machine layout:
//! 1. Ingests the `TapeHead` covenant manifest.
//! 2. Allocates memory partition geometry and symbol tables across all rolls.
//! 3. Sequentially lowers instruction rolls into `Word64` containers.
//! 4. Validates final affirmative commit status (`commit_status`).
//!
//! # Scripture Witness:
//! - **Exodus 31:18 (KJV)**:
//!   "And he gave unto Moses, when he had made an end of communing with him upon mount Sinai,
//!    two tables of testimony, tables of stone, written with the finger of God."
//!   (The immutable inscription of atomic runes onto bounded physical tablets).
//! - **Proverbs 24:27 (KJV)**:
//!   "Prepare thy work without, and make it fit for thyself in the field;
//!    and afterwards build thine house."
//!   (The discipline of proving atomic types and substrates before constructing systems).
//! - **Proverbs 22:28 (KJV)**:
//!   "Remove not the ancient landmark, which thy fathers have set."
//!   (The preservation of historical memory boundaries, silicon addresses, and constants).
// ==============================================================================

use std::collections::HashMap;

use crate::core::tape::Tape;
use crate::core::ternary::QuadState;
use crate::runtime::assembler::core::OmniAssembler;
use crate::runtime::assembler::universal::types::{
    AssembledInstruction,
    AssembledTape,
    PhysicalSymbol,
};

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( assemble_tape Method Implementation )
// ==============================================================================

impl OmniAssembler {
    /// Assembles an entire .tape process ribbon into physical silicon layout.
    pub fn assemble_tape(&mut self, tape: &Tape) -> Result<AssembledTape, String> {
        let head = tape.head.clone();
        let partition_size = head.required_partition_size.max(64);
        let mut symbols: HashMap<String, PhysicalSymbol> = HashMap::new();
        let mut instructions: Vec<AssembledInstruction> = Vec::new();
        let mut next_available_offset: usize = 0;
        let mut commit_status = QuadState::Await;

        // ======================================================================
        // BLOCK 3: BODY BLOCK
        // ( Sequential Roll Iteration & Invariant Aggregation )
        // ======================================================================
        for (idx, roll) in tape.rolls.iter().enumerate() {
            let inst = self.assemble_roll(
                roll,
                idx,
                &head.where_clause,
                &mut symbols,
                &mut next_available_offset,
                &mut commit_status,
            )?;
            instructions.push(inst);
        }

        self.logger.log(
            "ASSEMBLER",
            format!("Tape [{}] assembled: {} instructions, {} symbols", head.name, instructions.len(), symbols.len()),
        );

        Ok(AssembledTape {
            head,
            partition_size,
            symbols,
            instructions,
            commit_status,
        })
    }
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Subsystem Status & Module Closure )
// ==============================================================================
// Tape lowering engine verified and closed.
