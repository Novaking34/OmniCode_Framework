// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( AST Unified Node Subsystem: Complete Taxonomic Hierarchy )
// ==============================================================================
// SCRIPTURE ANCHORS & EPISTEMIC WITNESSES:
//   - Exodus 31:18: "written with the finger of God" — Inscription of eternal truth.
//   - Proverbs 24:27: "Prepare thy work without, and make it fit for thyself in the field;
//                     and afterwards build thine house." — Systematic modularization.
//   - Proverbs 22:28: "Remove not the ancient landmark, which thy fathers have set."
//                     — Preservation of invariant boundaries and balanced truth.
//
// SUB-MODULE TAXONOMY:
//   - boundaries: First-class asides, boundary frames, and topological casts.
//   - primitives: Scalar arithmetic, affix polarity modifiers, and unary inversions.
//   - relations:  Ontological being relations, directional causal vectors, connectives.
//   - structural: Program reels, Module headers, Process tapes, 4-Block sections, Rolls, Commit gates.
//   - cardinal:   Six Cardinal Interrogatives (WHO, WHAT, WHEN, WHERE, WHY, HOW).
//   - operational: SVO Action Verbs (INSCRIBE, TRANSDUCE, GUARD, PROJECT, SEQUENCE), MMIO locators.
//   - ontological: Invariant type declarations, entities, analogies, substrates, exemplars.
// ==============================================================================

pub mod boundaries;
pub mod primitives;
pub mod relations;
pub mod structural;
pub mod cardinal;
pub mod operational;
pub mod ontological;

// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Universal Re-Exports Across All Taxonomic Sub-Modules )
// ==============================================================================

pub use boundaries::*;
pub use primitives::*;
pub use relations::*;
pub use structural::*;
pub use cardinal::*;
pub use operational::*;
pub use ontological::*;

use crate::core::ternary::QuadState;

// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Unified ASTNode Enumeration: Complete Multi-Substrate Node Hierarchy )
// ==============================================================================

/// Unified AST Node enumeration representing every syntactic, structural,
/// relational, and operational construct in the OmniCode language.
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    // --------------------------------------------------------------------------
    // 1. First-Class Asides, Literals & Primitives
    // --------------------------------------------------------------------------
    Aside(AsideNode),
    Ternary(QuadState),
    Identifier(String),
    Polarity(PolarityNode),
    BoundaryCast(BoundaryCastNode),
    Number(f64),
    StringLit(String),

    // --------------------------------------------------------------------------
    // 2. Boundary Containers & Concrete Entities
    // --------------------------------------------------------------------------
    Boundary(BoundaryNode),
    Entity(EntityNode),

    // --------------------------------------------------------------------------
    // 3. Relational Algebra, Vectors & Logic
    // --------------------------------------------------------------------------
    Relation(RelationNode),
    Directional(DirectionalNode),
    Connective(ConnectiveNode),
    Arithmetic(ArithmeticNode),
    UnaryNot(UnaryNotNode),

    // --------------------------------------------------------------------------
    // 4. Macrocosmic Program & 4-Block Section Structure
    // --------------------------------------------------------------------------
    Program(ProgramNode),
    Module(ModuleNode),
    Tape(TapeNode),
    Section(SectionNode),
    Roll(RollNode),
    CommitStatus(CommitStatusNode),

    // --------------------------------------------------------------------------
    // 5. Cardinal Interrogatives (Process Tape Clauses & Dialog)
    // --------------------------------------------------------------------------
    Cardinal(CardinalClauseNode),

    // --------------------------------------------------------------------------
    // 6. Operational SVO Verbs, Adversative Guards & Hardware Locators
    // --------------------------------------------------------------------------
    Verb(VerbNode),
    Guard(GuardNode),
    Hardware(HardwareLocatorNode),

    // --------------------------------------------------------------------------
    // 7. Invariant Ontological Typology, Analogies & Exemplars
    // --------------------------------------------------------------------------
    InvariantType(InvariantTypeNode),
    Analogy(AnalogyNode),
    Substrate(SubstrateNode),
    Exemplar(ExemplarNode),
}

// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Trait Implementations, Send/Sync Invariant Verification & Unit Tests )
// ==============================================================================
// All ASTNode variants implement Send, Sync, Clone, PartialEq.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::boundaries::BoundaryType;
    use crate::core::operators::{BeingRelation, Connective, DirectionalVector, Polarity, AffixPosition};

    #[test]
    fn test_boundary_nodes_creation_and_asides() {
        let aside = AsideNode::new("Invariant scripture witness", 1, 1);
        assert_eq!(aside.intent, "Invariant scripture witness");

        let bound = BoundaryNode::new(BoundaryType::Denotation, vec![ASTNode::Identifier("Cell".to_string())], 2, 5)
            .with_aside(aside.clone());
        assert_eq!(bound.boundary_type, BoundaryType::Denotation);
        assert_eq!(bound.elements.len(), 1);
        assert_eq!(bound.aside, Some(aside));
    }

    #[test]
    fn test_structural_nodes_sections_rolls_tapes() {
        let roll = RollNode::new(
            1,
            "Inscribe",
            ASTNode::Identifier("ActionPayload".to_string()),
            10,
            1,
        ).with_aside(AsideNode::new("Roll 1 intent", 10, 20));

        assert_eq!(roll.id, 1);
        assert_eq!(roll.label, "Inscribe");
        assert!(roll.aside.is_some());

        let section = SectionNode::new(
            SectionKind::Body,
            vec![ASTNode::Roll(roll)],
            9,
            1,
        );
        assert_eq!(section.kind, SectionKind::Body);
        assert_eq!(section.elements.len(), 1);

        let tape = TapeNode::new("Character", 1, 1);
        assert_eq!(tape.name, "Character");

        let module = ModuleNode::new("Lexicon", 1, 1);
        assert_eq!(module.name, "Lexicon");
    }

    #[test]
    fn test_cardinal_nodes_interrogatives() {
        let who = CardinalClauseNode::new(
            CardinalKind::Who,
            ASTNode::Identifier("OmniCode Lexicon".to_string()),
            12,
            5,
        );
        assert_eq!(who.kind.keyword(), "WHO");
        assert_eq!(CardinalKind::from_str_keyword("what"), Some(CardinalKind::What));
        assert_eq!(CardinalKind::from_str_keyword("WHEN"), Some(CardinalKind::When));
        assert_eq!(CardinalKind::from_str_keyword("where"), Some(CardinalKind::Where));
        assert_eq!(CardinalKind::from_str_keyword("WHY"), Some(CardinalKind::Why));
        assert_eq!(CardinalKind::from_str_keyword("how"), Some(CardinalKind::How));
    }

    #[test]
    fn test_operational_verbs_and_guards() {
        let verb = VerbNode::new(
            VerbKind::Inscribe,
            Some(ASTNode::Identifier("Glyph".to_string())),
            ASTNode::Identifier("Cell".to_string()),
            Some(ASTNode::Identifier("Character`".to_string())),
            29,
            1,
        );
        assert_eq!(verb.verb.name(), "INSCRIBE");

        let guard = GuardNode::new(
            ASTNode::Ternary(QuadState::Proceed),
            ASTNode::Identifier("ProceedAction".to_string()),
            Some(ASTNode::Identifier("RollbackAction".to_string())),
            45,
            1,
        );
        assert_eq!(*guard.condition, ASTNode::Ternary(QuadState::Proceed));

        let hw = HardwareLocatorNode::new(
            ASTNode::Identifier("Terminal Stdout".to_string()),
            0x01,
            71,
            1,
        );
        assert_eq!(hw.address, 0x01);
    }

    #[test]
    fn test_ontological_invariant_types_and_exemplars() {
        let inv = InvariantTypeNode::new(
            "Character",
            ASTNode::Identifier("Idea { Type }".to_string()),
            25,
            1,
        );
        assert_eq!(inv.name, "Character");

        let exemplar = ExemplarNode::new(
            Some("Exemplar 'A'".to_string()),
            65,
            vec![-1, 1, 1, -1, 1],
            42,
            1,
        );
        assert_eq!(exemplar.scalar, 65);
        assert_eq!(exemplar.trits, vec![-1, 1, 1, -1, 1]);

        let substrate = SubstrateNode::new(8, 5, 33, 1);
        assert_eq!(substrate.physical_bits, 8);
        assert_eq!(substrate.virtual_trits, 5);
    }

    #[test]
    fn test_primitives_and_relations() {
        let arith = ArithmeticNode::new(
            ASTNode::Number(1.0),
            ArithmeticOp::Add,
            ASTNode::Number(2.0),
            1,
            1,
        );
        assert_eq!(arith.op.symbol(), "+");

        let pol = PolarityNode::new("proceed", Polarity::Abstract, AffixPosition::Prefix, 1, 1);
        assert_eq!(pol.formatted(), "+proceed");

        let rel = RelationNode::new(
            ASTNode::Identifier("A".to_string()),
            BeingRelation::Is,
            ASTNode::Identifier("B".to_string()),
            1,
            1,
        );
        assert_eq!(rel.relation, BeingRelation::Is);

        let dir = DirectionalNode::new(
            ASTNode::Identifier("Source".to_string()),
            DirectionalVector::ActUpon,
            ASTNode::Identifier("Target".to_string()),
            1,
            1,
        );
        assert_eq!(dir.vector, DirectionalVector::ActUpon);

        let conn = ConnectiveNode::new(
            ASTNode::Identifier("Left".to_string()),
            Connective::And,
            ASTNode::Identifier("Right".to_string()),
            1,
            1,
        );
        assert_eq!(conn.connective, Connective::And);
    }

    #[test]
    fn test_send_sync_invariants() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ASTNode>();
        assert_send_sync::<AsideNode>();
        assert_send_sync::<BoundaryNode>();
        assert_send_sync::<ProgramNode>();
        assert_send_sync::<ModuleNode>();
        assert_send_sync::<TapeNode>();
        assert_send_sync::<SectionNode>();
        assert_send_sync::<RollNode>();
        assert_send_sync::<CardinalClauseNode>();
        assert_send_sync::<VerbNode>();
        assert_send_sync::<GuardNode>();
        assert_send_sync::<HardwareLocatorNode>();
        assert_send_sync::<InvariantTypeNode>();
        assert_send_sync::<ExemplarNode>();
    }
}
