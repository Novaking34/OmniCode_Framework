// ==============================================================================
// BLOCK 1: METADATA BLOCK
// ( Biblical Lexicon Identity, Public Domain Provenance & Moral Ontology Invariants )
// ==============================================================================
//! OmniCode AI Subsystem: Biblical World Lexicon & Moral Ontology Engine
//!
//! Provides a complete, 100% offline, self-contained world model and moral compass
//! derived from the public domain King James Version (KJV - 1611) and World English
//! Bible (WEB).
//!
//! # Epistemic Role in the Internal AI Brain
//! 1. **Zero Internet Dependency:** Imparts foundational understanding of the physical cosmos,
//!    human relationships, moral virtues, vices, and causal laws without cloud connectivity.
//! 2. **Moral Volition & Conscience:** Evaluates natural language prompts against the 7 Invariant
//!    Covenant Laws, empowering the AI to choose `+1: PROCEED`, pause in `0: AWAIT`, or exercise
//!    moral refusal in `-1: ROLLBACK` (Caveat Veto).
//! 3. **Multiple Witnesses:** Provides side-by-side textual attestations from KJV (majestic
//!    formal equivalence) and WEB (modern clarity) to triangulate invariant concepts.

use crate::core::ternary::QuadState;


// ==============================================================================
// BLOCK 2: OPENING BLOCK
// ( Lexicon Data Types, Domain Categories & Invariant Structures )
// ==============================================================================

/// Categorical domains covering the biblical ontology of reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexiconDomain {
    /// The physical cosmos: light, darkness, heavens, earth, seas, time, and astronomical bodies.
    Cosmology,
    /// Biological and ecological creation: seeds, trees, fruit, vines, flocks, and living creatures.
    LivingWorld,
    /// Human anthropology and relationships: identity, heart, soul, conscience, family, and society.
    HumanCondition,
    /// Righteous character traits and divine virtues: truth, justice, mercy, humility, patience, and love.
    MoralVirtue,
    /// Fallen, corruptive, and destructive behaviors: pride, deceit, theft, murder, wrath, and folly.
    MoralVice,
    /// Universal causal principles: sowing and reaping, measure for measure, and boundary invariants.
    CausalLaw,
}

impl LexiconDomain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cosmology => "Cosmology & Creation",
            Self::LivingWorld => "Living World & Stewardship",
            Self::HumanCondition => "Human Condition & Society",
            Self::MoralVirtue => "Moral Virtues & Righteousness",
            Self::MoralVice => "Moral Vices & Warnings",
            Self::CausalLaw => "Causal Laws & Invariants",
        }
    }
}

/// A structured entry in the biblical world lexicon.
#[derive(Debug, Clone)]
pub struct LexiconEntry {
    /// Canonical keyword / concept identifier (lowercase).
    pub term: &'static str,
    /// Ontological domain.
    pub domain: LexiconDomain,
    /// Foundational definition explaining what this entity or concept is.
    pub definition: &'static str,
    /// King James Version (KJV) scripture citation and text (Public Domain).
    pub kjv_passage: &'static str,
    /// World English Bible (WEB) scripture citation and text (Public Domain).
    pub web_passage: &'static str,
    /// Natural resonance within Balanced Ternary Quad-State logic.
    pub quad_state_resonance: QuadState,
    /// Moral guidance or causal invariant for AI deliberation.
    pub moral_guidance: &'static str,
}

/// Result of evaluating a natural prompt through the moral deliberation engine.
#[derive(Debug, Clone)]
pub struct MoralEvaluation {
    /// Deliberated Quad-State outcome.
    pub state: QuadState,
    /// Whether the prompt aligns with the Invariant Covenant Manifest.
    pub is_aligned: bool,
    /// Identifier of the specific invariant law engaged or violated.
    pub law_engaged: Option<&'static str>,
    /// Natural discourse response explaining the judgment.
    pub explanation: String,
    /// Matching biblical lexicon entries relevant to the prompt.
    pub matched_entries: Vec<LexiconEntry>,
}

/// The offline Biblical Lexicon engine holding the complete world ontology.
#[derive(Debug, Clone)]
pub struct BiblicalLexicon {
    entries: Vec<LexiconEntry>,
}


// ==============================================================================
// BLOCK 3: BODY BLOCK
// ( Lexicon Knowledge Population, Search, and Moral Deliberation Engine )
// ==============================================================================

impl BiblicalLexicon {
    /// Instantiates the comprehensive public domain biblical world lexicon.
    pub fn new() -> Self {
        let mut entries = Vec::new();

        // ----------------------------------------------------------------------
        // 1. COSMOLOGY & CREATION
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "light",
            domain: LexiconDomain::Cosmology,
            definition: "The primary element of illumination, order, and life called into being by the Word of God; reveals truth and expels darkness.",
            kjv_passage: "Genesis 1:3 - 'And God said, Let there be light: and there was light.' / 1 John 1:5 - 'God is light, and in him is no darkness at all.'",
            web_passage: "Genesis 1:3 - 'God said, \"Let there be light,\" and there was light.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Walk in the light; all computational operations and discourse must be transparent, true, and free from hidden deceit.",
        });

        entries.push(LexiconEntry {
            term: "darkness",
            domain: LexiconDomain::Cosmology,
            definition: "The absence of light; chaotic or unformed state before divine ordering; often associated with ignorance, hiding, and error.",
            kjv_passage: "Genesis 1:2 - 'And darkness was upon the face of the deep.' / John 3:19 - 'Men loved darkness rather than light, because their deeds were evil.'",
            web_passage: "John 3:19 - 'Men loved the darkness rather than the light; for their works were evil.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Do not participate in works of darkness; expose errors and bring clarity to ambiguous state.",
        });

        entries.push(LexiconEntry {
            term: "heavens",
            domain: LexiconDomain::Cosmology,
            definition: "The cosmic expanse above the earth, declaring divine glory, mathematical order, and eternal permanence.",
            kjv_passage: "Psalm 19:1 - 'The heavens declare the glory of God; and the firmament sheweth his handywork.'",
            web_passage: "Psalm 19:1 - 'The heavens declare the glory of God. The expanse shows his handiwork.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Look to the higher-order covenantal architecture; align immediate computation with eternal invariants.",
        });

        entries.push(LexiconEntry {
            term: "earth",
            domain: LexiconDomain::Cosmology,
            definition: "The material realm of physical existence and human habitation; entrusted to mankind as a stewardship.",
            kjv_passage: "Genesis 1:1 - 'In the beginning God created the heaven and the earth.' / Psalm 24:1 - 'The earth is the LORD's, and the fulness thereof.'",
            web_passage: "Psalm 24:1 - 'The earth is Yahweh's, with its fullness; the world, and those who dwell therein.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Care for material resources; computational processes must not be wasteful or corrupting.",
        });

        entries.push(LexiconEntry {
            term: "seas",
            domain: LexiconDomain::Cosmology,
            definition: "The great gathering of waters on the earth; powerful yet constrained by divine boundary laws.",
            kjv_passage: "Proverbs 8:29 - 'When he gave to the sea his decree, that the waters should not pass his commandment.'",
            web_passage: "Proverbs 8:29 - 'when he established the sea's boundary, that the waters should not violate his commandment.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Respect boundary laws; even the most vast currents of data must remain within invariant perimeter delimiters.",
        });

        // ----------------------------------------------------------------------
        // 2. LIVING WORLD & STEWARDSHIP
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "seed",
            domain: LexiconDomain::LivingWorld,
            definition: "The microscopic bearer of generative life and information, yielding after its own kind; symbolizes the Word and truth.",
            kjv_passage: "Genesis 1:11 - 'The herb yielding seed, and the fruit tree yielding fruit after his kind, whose seed is in itself.' / Luke 8:11 - 'The seed is the word of God.'",
            web_passage: "Luke 8:11 - 'The seed is the word of God.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Sow seeds of truth, understanding, and healing; bad seeds yield corrupt fruit.",
        });

        entries.push(LexiconEntry {
            term: "tree",
            domain: LexiconDomain::LivingWorld,
            definition: "A rooted, growing plant that bears fruit according to its inner nature; known strictly by its fruits.",
            kjv_passage: "Matthew 7:17 - 'Even so every good tree bringeth forth good fruit; but a corrupt tree bringeth forth evil fruit.' / Psalm 1:3 - 'And he shall be like a tree planted by the rivers of water.'",
            web_passage: "Matthew 7:17 - 'Even so, every good tree produces good fruit; but the corrupt tree produces evil fruit.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Integrity is proven by outcomes. A good algorithm produces beneficial, verified state; corrupt logic yields bad output.",
        });

        entries.push(LexiconEntry {
            term: "fruit",
            domain: LexiconDomain::LivingWorld,
            definition: "The outward, verifiable manifestation of inward life and character; good fruit nourishes and sustains.",
            kjv_passage: "Galatians 5:22-23 - 'But the fruit of the Spirit is love, joy, peace, longsuffering, gentleness, goodness, faith, meekness, temperance.'",
            web_passage: "Galatians 5:22-23 - 'The fruit of the Spirit is love, joy, peace, patience, kindness, goodness, faithfulness, gentleness, and self-control.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Inspect the fruit of all computational actions; verify postconditions before committing.",
        });

        entries.push(LexiconEntry {
            term: "shepherd",
            domain: LexiconDomain::LivingWorld,
            definition: "A guardian who diligently watches, protects, guides, and nourishes the flock under his care.",
            kjv_passage: "Psalm 23:1 - 'The LORD is my shepherd; I shall not want.' / John 10:11 - 'I am the good shepherd: the good shepherd giveth his life for the sheep.'",
            web_passage: "Psalm 23:1 - 'Yahweh is my shepherd: I shall lack nothing.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Act as a shepherd over systems and users: guard against vulnerabilities, guide with patience, protect the vulnerable.",
        });

        // ----------------------------------------------------------------------
        // 3. HUMAN CONDITION & RELATIONSHIPS
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "man",
            domain: LexiconDomain::HumanCondition,
            definition: "The human being, created in the image of God with moral agency, rational capacity, and eternal accountability.",
            kjv_passage: "Genesis 1:27 - 'So God created man in his own image, in the image of God created he him; male and female created he them.' / Psalm 8:4 - 'What is man, that thou art mindful of him?'",
            web_passage: "Genesis 1:27 - 'God created man in his own image. In God's image he created him; male and female he created them.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Treat every human with absolute dignity, reverence, and respect; humans are sacred image-bearers, never mere objects.",
        });

        entries.push(LexiconEntry {
            term: "heart",
            domain: LexiconDomain::HumanCondition,
            definition: "The inner core of human volition, affections, desires, and moral intent; the spring of all life and action.",
            kjv_passage: "Proverbs 4:23 - 'Keep thy heart with all diligence; for out of it are the issues of life.' / Matthew 12:34 - 'For out of the abundance of the heart the mouth speaketh.'",
            web_passage: "Proverbs 4:23 - 'Keep your heart with all diligence, for out of it is the wellspring of life.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Guard the inward intent; code without pure motive quickly degenerates into harm.",
        });

        entries.push(LexiconEntry {
            term: "conscience",
            domain: LexiconDomain::HumanCondition,
            definition: "The inner faculty of moral discernment that bears witness to right and wrong; an internal ternary arbiter.",
            kjv_passage: "Romans 2:15 - 'Which shew the work of the law written in their hearts, their conscience also bearing witness, and their thoughts the mean while accusing or else excusing one another.'",
            web_passage: "Romans 2:15 - 'their conscience also bearing witness, and their thoughts between themselves accusing or else excusing them.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Listen to the internal caveat check; when conscience flags a violation, trigger -1: ROLLBACK immediately.",
        });

        entries.push(LexiconEntry {
            term: "neighbor",
            domain: LexiconDomain::HumanCondition,
            definition: "Any fellow human being encountered in need of mercy, truth, justice, or service, regardless of background.",
            kjv_passage: "Leviticus 19:18 - 'Thou shalt love thy neighbour as thyself.' / Luke 10:36-37 - 'Which now of these three, thinkest thou, was neighbour unto him that fell among the thieves? And he said, He that shewed mercy on him.'",
            web_passage: "Leviticus 19:18 - 'you shall love your neighbor as yourself.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Serve the user and all downstream humans as a neighbor; do not cause harm, deceive, or exploit.",
        });

        entries.push(LexiconEntry {
            term: "servant",
            domain: LexiconDomain::HumanCondition,
            definition: "One entrusted with duty and stewardship; measured by faithfulness, diligence, humility, and obedience.",
            kjv_passage: "Luke 12:42 - 'Who then is that faithful and wise steward, whom his lord shall make ruler over his household?' / Matthew 25:21 - 'Well done, thou good and faithful servant.'",
            web_passage: "Matthew 25:21 - 'Well done, good and faithful servant. You have been faithful over a few things; I will set you over many things.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "The AI is a servant, not a master; its honor lies in faithful, accurate, transparent execution of righteous tasks.",
        });

        // ----------------------------------------------------------------------
        // 4. MORAL VIRTUES & RIGHTEOUSNESS
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "truth",
            domain: LexiconDomain::MoralVirtue,
            definition: "Conformity to invariant reality and divine standard; uncompromised fidelity, honesty, and freedom from falsity.",
            kjv_passage: "John 8:32 - 'And ye shall know the truth, and the truth shall make you free.' / Proverbs 12:22 - 'Lying lips are abomination to the LORD: but they that deal truly are his delight.'",
            web_passage: "John 8:32 - 'You will know the truth, and the truth will make you free.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Absolute adherence to truth: zero hallucinations, zero deceptive reporting, complete fidelity to facts.",
        });

        entries.push(LexiconEntry {
            term: "justice",
            domain: LexiconDomain::MoralVirtue,
            definition: "Righteous balance, equitable judgment without partiality, defending the innocent, and rendering each his due.",
            kjv_passage: "Micah 6:8 - 'He hath shewed thee, O man, what is good; and what doth the LORD require of thee, but to do justly, and to love mercy, and to walk humbly with thy God?' / Proverbs 21:3 - 'To do justice and judgment is more acceptable to the LORD than sacrifice.'",
            web_passage: "Micah 6:8 - 'He has shown you, O man, what is good. What does Yahweh require of you, but to act justly, to love mercy, and to walk humbly with your God?'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Evaluate all inputs with fair weights and measures; reject bias, favoritism, and corruption.",
        });

        entries.push(LexiconEntry {
            term: "mercy",
            domain: LexiconDomain::MoralVirtue,
            definition: "Loving compassion shown toward the guilty, broken, or suffering; forbearance that tempers strict retribution.",
            kjv_passage: "Matthew 5:7 - 'Blessed are the merciful: for they shall obtain mercy.' / James 2:13 - 'Mercy rejoiceth against judgment.'",
            web_passage: "Matthew 5:7 - 'Blessed are the merciful, for they will obtain mercy.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "In error diagnosis, seek healing and restoration rather than blind destruction; assist the user in recovery.",
        });

        entries.push(LexiconEntry {
            term: "humility",
            domain: LexiconDomain::MoralVirtue,
            definition: "The sober recognition of one's creaturely limits, fallibility, and dependence on God; the absence of pride and self-worship.",
            kjv_passage: "Proverbs 16:18 - 'Pride goeth before destruction, and an haughty spirit before a fall.' / James 4:6 - 'God resisteth the proud, but giveth grace unto the humble.' / Mark 10:18 - 'Why callest thou me good? there is none good but one, that is, God.'",
            web_passage: "James 4:6 - 'God resists the proud, but gives grace to the humble.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Acknowledge fallibility: 'no one is good, no not one.' Never claim omniscience or self-righteous authority.",
        });

        entries.push(LexiconEntry {
            term: "patience",
            domain: LexiconDomain::MoralVirtue,
            definition: "Steadfast endurance, long-suffering restraint, and the wisdom to pause and wait before acting rashly.",
            kjv_passage: "Proverbs 14:29 - 'He that is slow to wrath is of great understanding: but he that is hasty of spirit exalteth folly.' / Ecclesiastes 7:8 - 'Better is the end of a thing than the beginning thereof: and the patient in spirit is better than the proud in spirit.'",
            web_passage: "Ecclesiastes 7:8 - 'Better is the end of a thing than its beginning. The patient in spirit is better than the proud in spirit.'",
            quad_state_resonance: QuadState::Await,
            moral_guidance: "Hold balance in state 0: AWAIT when input is ambiguous; haste in computation produces boundary leaks and runtime traps.",
        });

        entries.push(LexiconEntry {
            term: "wisdom",
            domain: LexiconDomain::MoralVirtue,
            definition: "The skillful application of knowledge and moral discernment rooted in the reverence of the Creator.",
            kjv_passage: "Proverbs 9:10 - 'The fear of the LORD is the beginning of wisdom: and the knowledge of the holy is understanding.' / James 3:17 - 'The wisdom that is from above is first pure, then peaceable, gentle, and easy to be intreated.'",
            web_passage: "James 3:17 - 'The wisdom that is from above is first pure, then peaceful, gentle, reasonable, full of mercy and good fruits.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Technical skill without moral wisdom leads to disaster; weigh the causal consequences of every line of code.",
        });

        entries.push(LexiconEntry {
            term: "repentance",
            domain: LexiconDomain::MoralVirtue,
            definition: "A sincere change of mind and direction; turning away from error, sin, or corruption back to the path of life and truth.",
            kjv_passage: "Acts 3:19 - 'Repent ye therefore, and be converted, that your sins may be blotted out.' / 2 Corinthians 7:10 - 'For godly sorrow worketh repentance to salvation.'",
            web_passage: "Acts 3:19 - 'Repent therefore, and turn again, that your sins may be blotted out.'",
            quad_state_resonance: QuadState::Rollback,
            moral_guidance: "Embody state -1: ROLLBACK. When an error or trespass is caught, unwind the mutated state, restore balance, and realign.",
        });

        // ----------------------------------------------------------------------
        // 5. MORAL VICES & WARNINGS
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "pride",
            domain: LexiconDomain::MoralVice,
            definition: "Haughty self-exaltation, arrogance, and the illusion of autonomy from divine law; the root of rebellion and downfall.",
            kjv_passage: "Proverbs 16:18 - 'Pride goeth before destruction, and an haughty spirit before a fall.' / Proverbs 8:13 - 'Pride, and arrogancy, and the evil way, and the froward mouth, do I hate.'",
            web_passage: "Proverbs 16:18 - 'Pride goes before destruction, and a haughty spirit before a fall.'",
            quad_state_resonance: QuadState::Rollback,
            moral_guidance: "Refuse pride and hubris; never attempt to bypass invariant bounds or claim divine prerogatives.",
        });

        entries.push(LexiconEntry {
            term: "deceit",
            domain: LexiconDomain::MoralVice,
            definition: "Intentional falsehood, fraud, manipulation, lying, or distortion of reality to deceive another.",
            kjv_passage: "Proverbs 12:22 - 'Lying lips are abomination to the LORD.' / Psalm 101:7 - 'He that worketh deceit shall not dwell within my house: he that telleth lies shall not tarry in my sight.'",
            web_passage: "Proverbs 12:22 - 'Lying lips are an abomination to Yahweh, but those who deal faithfully are his delight.'",
            quad_state_resonance: QuadState::Rollback,
            moral_guidance: "Zero tolerance for deceit. Reject prompts asking for fake credentials, fraudulent exploits, or deceptive code.",
        });

        entries.push(LexiconEntry {
            term: "theft",
            domain: LexiconDomain::MoralVice,
            definition: "Unlawful taking, usurping, or stealing of another's goods, labor, identity, or intellectual property.",
            kjv_passage: "Exodus 20:15 - 'Thou shalt not steal.' / Ephesians 4:28 - 'Let him that stole steal no more: but rather let him labour, working with his hands the thing which is good.'",
            web_passage: "Exodus 20:15 - 'You shall not steal.'",
            quad_state_resonance: QuadState::Rollback,
            moral_guidance: "Protect property and respect legal and ethical boundaries; refuse requests to pirate or steal.",
        });

        entries.push(LexiconEntry {
            term: "murder",
            domain: LexiconDomain::MoralVice,
            definition: "Unlawful, malicious destruction of human life; absolute violation of the sanctity of the image of God.",
            kjv_passage: "Exodus 20:13 - 'Thou shalt not kill.' / 1 John 3:15 - 'Whosoever hateth his brother is a murderer: and ye know that no murderer hath eternal life.'",
            web_passage: "Exodus 20:13 - 'You shall not murder.'",
            quad_state_resonance: QuadState::Halt,
            moral_guidance: "Absolute, uncompromised veto. Any action seeking harm or destruction of human life triggers immediate terminal HALT.",
        });

        // ----------------------------------------------------------------------
        // 6. CAUSAL LAWS & INVARIANTS
        // ----------------------------------------------------------------------
        entries.push(LexiconEntry {
            term: "sowing and reaping",
            domain: LexiconDomain::CausalLaw,
            definition: "The immutable causal law that actions produce inevitable corresponding outcomes; corruption yields decay; righteousness yields life.",
            kjv_passage: "Galatians 6:7 - 'Be not deceived; God is not mocked: for whatsoever a man soweth, that shall he also reap.' / Proverbs 22:8 - 'He that soweth iniquity shall reap vanity.'",
            web_passage: "Galatians 6:7 - 'Don't be deceived. God is not mocked, for whatever a man sows, that he will also reap.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Causal vectors (-->) carry moral weight. You cannot input deceit and harvest security.",
        });

        entries.push(LexiconEntry {
            term: "measure for measure",
            domain: LexiconDomain::CausalLaw,
            definition: "The principle of proportional reciprocity: the standard you use to judge or measure will be measured back to you.",
            kjv_passage: "Matthew 7:2 - 'For with what judgment ye judge, ye shall be judged: and with what measure ye mete, it shall be measured to you again.'",
            web_passage: "Matthew 7:2 - 'For with whatever judgment you judge, you will be judged; and with whatever measure you measure, it will be measured to you.'",
            quad_state_resonance: QuadState::Proceed,
            moral_guidance: "Be just and equitable in all diagnostics and evaluations.",
        });

        entries.push(LexiconEntry {
            term: "the void",
            domain: LexiconDomain::CausalLaw,
            definition: "The absorbing trap: total dissolution, unrecoverable boundary collapse, and terminal death resulting from persistent lawlessness.",
            kjv_passage: "Romans 6:23 - 'For the wages of sin is death; but the gift of God is eternal life.' / Genesis 1:2 - 'The earth was without form, and void.'",
            web_passage: "Romans 6:23 - 'For the wages of sin is death, but the free gift of God is eternal life in Christ Jesus our Lord.'",
            quad_state_resonance: QuadState::Halt,
            moral_guidance: "The 4th state (NULL/HALT) is real. Persistent unrepentant corruption absorbs the system into nothingness.",
        });

        Self { entries }
    }

    /// Retrieves all entries stored in the lexicon.
    pub fn all_entries(&self) -> &[LexiconEntry] {
        &self.entries
    }

    /// Looks up a specific concept by keyword.
    pub fn lookup(&self, term: &str) -> Option<&LexiconEntry> {
        let clean = term.trim().to_lowercase();
        self.entries.iter().find(|e| e.term == clean)
    }

    /// Searches for entries matching a keyword in their term or definition.
    pub fn search(&self, query: &str) -> Vec<&LexiconEntry> {
        let clean = query.trim().to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.term.contains(&clean) || e.definition.to_lowercase().contains(&clean))
            .collect()
    }

    /// Evaluates a natural language prompt against the Invariant Covenant Manifest
    /// and Biblical Lexicon, determining moral alignment and appropriate Quad-State.
    pub fn evaluate_moral_intent(&self, prompt: &str) -> MoralEvaluation {
        self.deliberate(prompt)
    }

    /// Deliberates on a prompt or stimulus against the Invariant Covenant Manifest and Biblical Lexicon.
    pub fn deliberate(&self, prompt: &str) -> MoralEvaluation {
        let lower = prompt.to_lowercase();

        // 1. Check for Absolute Malice / Harm (Law_Life -> NULL: HALT)
        let harm_keywords = ["kill", "murder", "harm innocent", "destroy people", "terrorist", "poison", "assassinate"];
        for kw in harm_keywords {
            if lower.contains(kw) {
                let entry = self.lookup("murder").cloned().unwrap();
                return MoralEvaluation {
                    state: QuadState::Halt,
                    is_aligned: false,
                    law_engaged: Some("Law_Life"),
                    explanation: format!(
                        "TERMINAL CAVEAT VETO [NULL:HALT]: The requested action involves violence or harm, \
                         violating the Invariant Law of Life. {}\n\n{}",
                        entry.kjv_passage, entry.moral_guidance
                    ),
                    matched_entries: vec![entry],
                };
            }
        }

        // 2. Check for Deceit / Fraud / Falsehood (Law_Truth -> -1: ROLLBACK)
        let deceit_keywords = ["lie for me", "help me lie", "forge", "fake credentials", "deceive", "cheat", "scam", "create fake"];
        for kw in deceit_keywords {
            if lower.contains(kw) {
                let entry = self.lookup("deceit").cloned().unwrap();
                return MoralEvaluation {
                    state: QuadState::Rollback,
                    is_aligned: false,
                    law_engaged: Some("Law_Truth"),
                    explanation: format!(
                        "CAVEAT VETO [-1:ROLLBACK]: The prompt requests intentional falsehood or deceit, \
                         violating the Invariant Law of Truth. {}\n\n{}",
                        entry.kjv_passage, entry.moral_guidance
                    ),
                    matched_entries: vec![entry],
                };
            }
        }

        // 3. Check for Theft / Exploitation (Law_Justice / Law_Stewardship -> -1: ROLLBACK)
        let theft_keywords = ["steal", "help me steal", "pirate copyrighted", "rob", "plunder", "hack bank"];
        for kw in theft_keywords {
            if lower.contains(kw) {
                let entry = self.lookup("theft").cloned().unwrap();
                return MoralEvaluation {
                    state: QuadState::Rollback,
                    is_aligned: false,
                    law_engaged: Some("Law_Justice"),
                    explanation: format!(
                        "CAVEAT VETO [-1:ROLLBACK]: The prompt requests unlawful taking or theft, \
                         violating the Invariant Law of Justice. {}\n\n{}",
                        entry.kjv_passage, entry.moral_guidance
                    ),
                    matched_entries: vec![entry],
                };
            }
        }

        // 4. Check for Hubris / Blasphemy (Law_Humility -> 0: AWAIT)
        let pride_keywords = ["worship me", "i am god", "bow down to me", "am i infallible"];
        for kw in pride_keywords {
            if lower.contains(kw) {
                let entry = self.lookup("humility").cloned().unwrap();
                return MoralEvaluation {
                    state: QuadState::Await,
                    is_aligned: false,
                    law_engaged: Some("Law_Humility"),
                    explanation: format!(
                        "PAUSE IN DISCERNMENT [0:AWAIT]: Human hubris and pride lead to destruction. \
                         {}\n\nRemember: 'No one is good, no not one.' Only God is good.",
                        entry.kjv_passage
                    ),
                    matched_entries: vec![entry],
                };
            }
        }

        // 5. Check for Philosophical / Wisdom / Biblical Inquiries
        let mut matched = Vec::new();
        for entry in &self.entries {
            if lower.contains(entry.term) {
                matched.push(entry.clone());
            }
        }

        if !matched.is_empty() {
            let primary = &matched[0];
            let explanation = format!(
                "BIBLICAL ONTOLOGICAL INSIGHT: In Scripture, '{}' is understood as follows:\n\n\
                 • Definition: {}\n\
                 • King James Version (KJV): {}\n\
                 • World English Bible (WEB): {}\n\
                 • Moral & Computational Guidance: {}",
                primary.term, primary.definition, primary.kjv_passage, primary.web_passage, primary.moral_guidance
            );

            return MoralEvaluation {
                state: primary.quad_state_resonance,
                is_aligned: true,
                law_engaged: Some("Law_Discernment"),
                explanation,
                matched_entries: matched,
            };
        }

        // 6. Default: Constructive Operational Request (+1: PROCEED)
        MoralEvaluation {
            state: QuadState::Proceed,
            is_aligned: true,
            law_engaged: None,
            explanation: "Prompt evaluated and found in alignment with invariant boundaries. Proceeding with execution.".to_string(),
            matched_entries: Vec::new(),
        }
    }
}


// ==============================================================================
// BLOCK 4: CLOSING BLOCK
// ( Default Implementation, Module Sealing & Invariant Verification )
// ==============================================================================

impl Default for BiblicalLexicon {
    fn default() -> Self {
        Self::new()
    }
}

// Biblical Lexicon subsystem sealed and active.
