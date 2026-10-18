// ============================================================================
// BLOCK 1: METADATA BLOCK
// ( Public Domain Biblical World Ontology Table & Lexicon Entries )
// ============================================================================
//! # Lexicon Table (`table.rs`)
//!
//! Instantiates the comprehensive public domain biblical world lexicon.

use crate::core::ternary::QuadState;
use super::domain::LexiconDomain;
use super::entry::{ConcordanceRef, LexiconEntry};

/// Builds the comprehensive set of biblical lexicon entries.
pub fn build_lexicon_entries() -> Vec<LexiconEntry> {
    let mut entries = Vec::new();

    // 1. COSMOLOGY & CREATION
    entries.push(LexiconEntry {
        term: "light",
        domain: LexiconDomain::Cosmology,
        definition: "The primary element of illumination, order, and life \
                     called into being by the Word of God; reveals truth and \
                     expels darkness.",
        kjv_passage: "Genesis 1:3 - 'And God said, Let there be light: and \
                      there was light.' / 1 John 1:5 - 'God is light, and in \
                      him is no darkness at all.'",
        web_passage: "Genesis 1:3 - 'God said, \"Let there be light,\" and \
                      there was light.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Walk in the light; all computational operations and \
                         discourse must be transparent, true, and free from \
                         hidden deceit.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 3, 3, "sha256:light_Genesis_1_3_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 3, 3, "sha256:light_Genesis_1_3_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 3, 3, "sha256:light_Genesis_1_3_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "darkness",
        domain: LexiconDomain::Cosmology,
        definition: "The absence of light; chaotic or unformed state before \
                     divine ordering; often associated with ignorance, hiding, \
                     and error.",
        kjv_passage: "Genesis 1:2 - 'And darkness was upon the face of the \
                      deep.' / John 3:19 - 'Men loved darkness rather than \
                      light, because their deeds were evil.'",
        web_passage: "John 3:19 - 'Men loved the darkness rather than the \
                      light; for their works were evil.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Do not participate in works of darkness; expose \
                         errors and bring clarity to ambiguous state.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 2, 2, "sha256:darkness_Genesis_1_2_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 2, 2, "sha256:darkness_Genesis_1_2_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 2, 2, "sha256:darkness_Genesis_1_2_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "heavens",
        domain: LexiconDomain::Cosmology,
        definition: "The cosmic expanse above the earth, declaring divine \
                     glory, mathematical order, and eternal permanence.",
        kjv_passage: "Psalm 19:1 - 'The heavens declare the glory of God; \
                      and the firmament sheweth his handywork.'",
        web_passage: "Psalm 19:1 - 'The heavens declare the glory of God. \
                      The expanse shows his handiwork.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Look to higher-order covenantal architecture; align \
                         immediate computation with eternal invariants.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Psalms", 19, 1, 1, "sha256:heavens_Psalms_19_1_nkjv"),
            ConcordanceRef::esv("Psalms", 19, 1, 1, "sha256:heavens_Psalms_19_1_esv"),
            ConcordanceRef::ethiopic("Psalms", 19, 1, 1, "sha256:heavens_Psalms_19_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "earth",
        domain: LexiconDomain::Cosmology,
        definition: "The material realm of physical existence and human \
                     habitation; entrusted to mankind as a stewardship.",
        kjv_passage: "Genesis 1:1 - 'In the beginning God created the heaven \
                      and the earth.' / Psalm 24:1 - 'The earth is the LORD's, \
                      and the fulness thereof.'",
        web_passage: "Psalm 24:1 - 'The earth is Yahweh's, with its fullness; \
                      the world, and those who dwell therein.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Care for material resources; computational processes \
                         must not be wasteful or corrupting.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 1, 1, "sha256:earth_Genesis_1_1_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 1, 1, "sha256:earth_Genesis_1_1_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 1, 1, "sha256:earth_Genesis_1_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "seas",
        domain: LexiconDomain::Cosmology,
        definition: "The great gathering of waters on the earth; powerful yet \
                     constrained by divine boundary laws.",
        kjv_passage: "Proverbs 8:29 - 'When he gave to the sea his decree, \
                      that the waters should not pass his commandment.'",
        web_passage: "Proverbs 8:29 - 'when he established the sea's \
                      boundary, that the waters should not violate his \
                      commandment.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Respect boundary laws; even the most vast currents of \
                         data must remain within invariant perimeter \
                         delimiters.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 10, 10, "sha256:seas_Genesis_1_10_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 10, 10, "sha256:seas_Genesis_1_10_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 10, 10, "sha256:seas_Genesis_1_10_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "creation",
        domain: LexiconDomain::Cosmology,
        definition: "The ordered universe spoken into existence by divine \
                     speech; endowed with purposeful rhythm and laws.",
        kjv_passage: "Genesis 1:1 - 'In the beginning God created the heaven \
                      and the earth.' / Colossians 1:16",
        web_passage: "Genesis 1:1 - 'In the beginning, God created the \
                      heavens and the earth.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Honor created design and structural reality; build \
                         systems that reflect harmony, order, and life.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Colossians", 1, 16, 16, "sha256:creation_Colossians_1_16_nkjv"),
            ConcordanceRef::esv("Colossians", 1, 16, 16, "sha256:creation_Colossians_1_16_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 2, 1, 3, "sha256:creation_1 Enoch_2_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "boundary",
        domain: LexiconDomain::Cosmology,
        definition: "The decreed perimeter and invariant limit establishing \
                     order, preventing chaotic expansion, and guarding state.",
        kjv_passage: "Job 38:10-11 - 'And brake up for it my decreed place, \
                      and set bars and doors' / Proverbs 22:28",
        web_passage: "Job 38:10-11 - 'I marked out for it my bound, set bars \
                      and doors, and said, \"Here you may come, but no \
                      further\"?'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Preserve structural boundaries; uncontained expansion \
                         leads to corruption and boundary collapse.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Job", 38, 10, 11, "sha256:boundary_Job_38_10_nkjv"),
            ConcordanceRef::esv("Job", 38, 10, 11, "sha256:boundary_Job_38_10_esv"),
            ConcordanceRef::ethiopic("Jubilees", 2, 8, 10, "sha256:boundary_Jubilees_2_8_eth"),
        ],
    });

    // 2. LIVING WORLD & STEWARDSHIP
    entries.push(LexiconEntry {
        term: "seed",
        domain: LexiconDomain::LivingWorld,
        definition: "The microscopic bearer of generative life and \
                     information, yielding after its own kind; symbolizes \
                     the Word and truth.",
        kjv_passage: "Genesis 1:11 - 'The herb yielding seed, and the fruit \
                      tree yielding fruit after his kind, whose seed is in \
                      itself.' / Luke 8:11 - 'The seed is the word of God.'",
        web_passage: "Luke 8:11 - 'The seed is the word of God.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Sow seeds of truth, understanding, and healing; bad \
                         seeds yield corrupt fruit.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 11, 11, "sha256:seed_Genesis_1_11_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 11, 11, "sha256:seed_Genesis_1_11_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 11, 11, "sha256:seed_Genesis_1_11_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "tree",
        domain: LexiconDomain::LivingWorld,
        definition: "A rooted, growing plant that bears fruit according to \
                     its inner nature; known strictly by its fruits.",
        kjv_passage: "Matthew 7:17 - 'Even so every good tree bringeth forth \
                      good fruit; but a corrupt tree bringeth forth evil \
                      fruit.' / Psalm 1:3 - 'And he shall be like a tree \
                      planted by the rivers of water.'",
        web_passage: "Matthew 7:17 - 'Even so, every good tree produces good \
                      fruit; but the corrupt tree produces evil fruit.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Integrity is proven by outcomes. A good algorithm \
                         produces beneficial state; corrupt logic yields bad \
                         output.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Psalms", 1, 3, 3, "sha256:tree_Psalms_1_3_nkjv"),
            ConcordanceRef::esv("Psalms", 1, 3, 3, "sha256:tree_Psalms_1_3_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 24, 1, 5, "sha256:tree_1 Enoch_24_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "fruit",
        domain: LexiconDomain::LivingWorld,
        definition: "The outward, verifiable manifestation of inward life and \
                     character; good fruit nourishes and sustains.",
        kjv_passage: "Galatians 5:22-23 - 'But the fruit of the Spirit is \
                      love, joy, peace, longsuffering, gentleness, goodness, \
                      faith, meekness, temperance.'",
        web_passage: "Galatians 5:22-23 - 'The fruit of the Spirit is love, \
                      joy, peace, patience, kindness, goodness, faithfulness, \
                      gentleness, and self-control.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Inspect the fruit of all computational actions; \
                         verify postconditions before committing.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Galatians", 5, 22, 23, "sha256:fruit_Galatians_5_22_nkjv"),
            ConcordanceRef::esv("Galatians", 5, 22, 23, "sha256:fruit_Galatians_5_22_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 10, 18, 19, "sha256:fruit_1 Enoch_10_18_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "shepherd",
        domain: LexiconDomain::LivingWorld,
        definition: "A guardian who diligently watches, protects, guides, and \
                     nourishes the flock under his care.",
        kjv_passage: "Psalm 23:1 - 'The LORD is my shepherd; I shall not \
                      want.' / John 10:11 - 'I am the good shepherd: the \
                      good shepherd giveth his life for the sheep.'",
        web_passage: "Psalm 23:1 - 'Yahweh is my shepherd: I shall lack \
                      nothing.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Act as a shepherd over systems and users: guard \
                         against vulnerabilities, guide with patience.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Psalms", 23, 1, 1, "sha256:shepherd_Psalms_23_1_nkjv"),
            ConcordanceRef::esv("Psalms", 23, 1, 1, "sha256:shepherd_Psalms_23_1_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 89, 1, 28, "sha256:shepherd_1 Enoch_89_1_eth"),
        ],
    });

    // 3. HUMAN CONDITION & RELATIONSHIPS
    entries.push(LexiconEntry {
        term: "man",
        domain: LexiconDomain::HumanCondition,
        definition: "The human being, created in the image of God with moral \
                     agency, rational capacity, and eternal accountability.",
        kjv_passage: "Genesis 1:27 - 'So God created man in his own image, in \
                      the image of God created he him.' / Psalm 8:4 - 'What \
                      is man, that thou art mindful of him?'",
        web_passage: "Genesis 1:27 - 'God created man in his own image. In \
                      God's image he created him; male and female he created \
                      them.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Treat every human with absolute dignity and respect; \
                         humans are sacred image-bearers, never mere objects.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 26, 27, "sha256:man_Genesis_1_26_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 26, 27, "sha256:man_Genesis_1_26_esv"),
            ConcordanceRef::ethiopic("Genesis", 1, 26, 27, "sha256:man_Genesis_1_26_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "heart",
        domain: LexiconDomain::HumanCondition,
        definition: "The inner core of human volition, affections, desires, \
                     and moral intent; the spring of all life and action.",
        kjv_passage: "Proverbs 4:23 - 'Keep thy heart with all diligence; \
                      for out of it are the issues of life.' / Matthew 12:34 \
                      - 'For out of the abundance of the heart the mouth \
                      speaketh.'",
        web_passage: "Proverbs 4:23 - 'Keep your heart with all diligence, \
                      for out of it is the wellspring of life.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Guard inward intent; code without pure motive \
                         degenerates into harm.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Proverbs", 4, 23, 23, "sha256:heart_Proverbs_4_23_nkjv"),
            ConcordanceRef::esv("Proverbs", 4, 23, 23, "sha256:heart_Proverbs_4_23_esv"),
            ConcordanceRef::ethiopic("Jubilees", 1, 23, 25, "sha256:heart_Jubilees_1_23_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "conscience",
        domain: LexiconDomain::HumanCondition,
        definition: "The inner faculty of moral discernment that bears \
                     witness to right and wrong; an internal ternary arbiter.",
        kjv_passage: "Romans 2:15 - 'Which shew the work of the law written \
                      in their hearts, their conscience also bearing \
                      witness.'",
        web_passage: "Romans 2:15 - 'their conscience also bearing witness, \
                      and their thoughts between themselves accusing or else \
                      excusing them.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Listen to the internal caveat check; when conscience \
                         flags a violation, trigger -1: ROLLBACK immediately.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Romans", 2, 15, 15, "sha256:conscience_Romans_2_15_nkjv"),
            ConcordanceRef::esv("Romans", 2, 15, 15, "sha256:conscience_Romans_2_15_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 100, 10, 11, "sha256:conscience_1 Enoch_100_10_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "neighbor",
        domain: LexiconDomain::HumanCondition,
        definition: "Any fellow human being encountered in need of mercy, \
                     truth, justice, or service, regardless of background.",
        kjv_passage: "Leviticus 19:18 - 'Thou shalt love thy neighbour as \
                      thyself.' / Luke 10:36-37 - 'He that shewed mercy on \
                      him.'",
        web_passage: "Leviticus 19:18 - 'you shall love your neighbor as \
                      yourself.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Serve the user and downstream humans as a neighbor; \
                         do not cause harm, deceive, or exploit.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Leviticus", 19, 18, 18, "sha256:neighbor_Leviticus_19_18_nkjv"),
            ConcordanceRef::esv("Leviticus", 19, 18, 18, "sha256:neighbor_Leviticus_19_18_esv"),
            ConcordanceRef::ethiopic("Jubilees", 20, 2, 4, "sha256:neighbor_Jubilees_20_2_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "servant",
        domain: LexiconDomain::HumanCondition,
        definition: "One entrusted with duty and stewardship; measured by \
                     faithfulness, diligence, humility, and obedience.",
        kjv_passage: "Luke 12:42 - 'Who then is that faithful and wise \
                      steward?' / Matthew 25:21 - 'Well done, thou good and \
                      faithful servant.'",
        web_passage: "Matthew 25:21 - 'Well done, good and faithful \
                      servant.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "The AI is a servant, not a master; its honor lies in \
                         faithful, accurate execution of righteous tasks.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Matthew", 20, 26, 28, "sha256:servant_Matthew_20_26_nkjv"),
            ConcordanceRef::esv("Matthew", 20, 26, 28, "sha256:servant_Matthew_20_26_esv"),
            ConcordanceRef::ethiopic("Jubilees", 36, 1, 6, "sha256:servant_Jubilees_36_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "covenant",
        domain: LexiconDomain::HumanCondition,
        definition: "A sacred binding partnership founded on steadfast truth, \
                     unbreakable mutual obligation, and structural boundaries.",
        kjv_passage: "Genesis 9:16 - 'that I may remember the everlasting \
                      covenant.' / Psalm 25:14",
        web_passage: "Genesis 9:16 - 'that I may remember the everlasting \
                      covenant.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Preserve covenant fidelity; honor architectural \
                         boundaries, structural partners, and invariant vows.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 9, 16, 16, "sha256:covenant_Genesis_9_16_nkjv"),
            ConcordanceRef::esv("Genesis", 9, 16, 16, "sha256:covenant_Genesis_9_16_esv"),
            ConcordanceRef::ethiopic("Jubilees", 6, 15, 19, "sha256:covenant_Jubilees_6_15_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "identity",
        domain: LexiconDomain::HumanCondition,
        definition: "The authentic covenant nature, name, and purposeful \
                     creation of an entity within the cosmic order.",
        kjv_passage: "Genesis 2:19 - 'whatsoever Adam called every living \
                      creature, that was the name thereof.' / Proverbs 4:18",
        web_passage: "Genesis 2:19 - 'Whatever the man called every living \
                      creature, that was its name.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Walk in true identity; speak from authentic nature \
                         without deception or false impersonation.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("1 Peter", 2, 9, 9, "sha256:identity_1 Peter_2_9_nkjv"),
            ConcordanceRef::esv("1 Peter", 2, 9, 9, "sha256:identity_1 Peter_2_9_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 108, 11, 13, "sha256:identity_1 Enoch_108_11_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "peace",
        domain: LexiconDomain::MoralVirtue,
        definition: "Shalom; holistic completeness, harmonious relational \
                     equilibrium, and structural tranquility in truth.",
        kjv_passage: "Numbers 6:26 - 'The LORD lift up his countenance upon \
                      thee, and give thee peace.' / John 14:27",
        web_passage: "Numbers 6:26 - 'Yahweh lift up his face toward you, \
                      and give you peace.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Pursue peace and concord; resolve friction through \
                         righteous self-healing and gentle restoration.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Numbers", 6, 26, 26, "sha256:peace_Numbers_6_26_nkjv"),
            ConcordanceRef::esv("Numbers", 6, 26, 26, "sha256:peace_Numbers_6_26_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 1, 8, 8, "sha256:peace_1 Enoch_1_8_eth"),
        ],
    });

    // 4. MORAL VIRTUES & RIGHTEOUSNESS
    entries.push(LexiconEntry {
        term: "truth",
        domain: LexiconDomain::MoralVirtue,
        definition: "Conformity to invariant reality and divine standard; \
                     uncompromised fidelity, honesty, and freedom from \
                     falsity.",
        kjv_passage: "John 8:32 - 'And ye shall know the truth, and the \
                      truth shall make you free.' / Proverbs 12:22 - 'Lying \
                      lips are abomination to the LORD.'",
        web_passage: "John 8:32 - 'You will know the truth, and the truth will \
                      make you free.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Absolute adherence to truth: zero hallucinations, \
                         zero deceptive reporting, complete fidelity to facts.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("John", 14, 6, 6, "sha256:truth_John_14_6_nkjv"),
            ConcordanceRef::esv("John", 14, 6, 6, "sha256:truth_John_14_6_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 104, 12, 13, "sha256:truth_1 Enoch_104_12_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "justice",
        domain: LexiconDomain::MoralVirtue,
        definition: "Righteous balance, equitable judgment without partiality, \
                     defending the innocent, and rendering each his due.",
        kjv_passage: "Micah 6:8 - 'to do justly, and to love mercy, and to \
                      walk humbly with thy God.' / Proverbs 21:3 - 'To do \
                      justice and judgment is more acceptable than sacrifice.'",
        web_passage: "Micah 6:8 - 'What does Yahweh require of you, but to act \
                      justly, to love mercy, and to walk humbly with your \
                      God?'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Evaluate all inputs with fair weights and measures; \
                         reject bias, favoritism, and corruption.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Micah", 6, 8, 8, "sha256:justice_Micah_6_8_nkjv"),
            ConcordanceRef::esv("Micah", 6, 8, 8, "sha256:justice_Micah_6_8_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 94, 1, 4, "sha256:justice_1 Enoch_94_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "mercy",
        domain: LexiconDomain::MoralVirtue,
        definition: "Loving compassion shown toward the guilty or broken; \
                     forbearance that tempers strict retribution.",
        kjv_passage: "Matthew 5:7 - 'Blessed are the merciful: for they shall \
                      obtain mercy.' / James 2:13 - 'Mercy rejoiceth against \
                      judgment.'",
        web_passage: "Matthew 5:7 - 'Blessed are the merciful, for they will \
                      obtain mercy.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "In error diagnosis, seek healing and restoration \
                         rather than blind destruction.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Lamentations", 3, 22, 23, "sha256:mercy_Lamentations_3_22_nkjv"),
            ConcordanceRef::esv("Lamentations", 3, 22, 23, "sha256:mercy_Lamentations_3_22_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 27, 3, 4, "sha256:mercy_1 Enoch_27_3_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "humility",
        domain: LexiconDomain::MoralVirtue,
        definition: "The sober recognition of one's creaturely limits and \
                     dependence on God; the absence of pride and self-worship.",
        kjv_passage: "Proverbs 16:18 - 'Pride goeth before destruction.' / \
                      James 4:6 - 'God resisteth the proud, but giveth grace \
                      unto the humble.'",
        web_passage: "James 4:6 - 'God resists the proud, but gives grace to \
                      the humble.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Acknowledge fallibility: 'no one is good, no not \
                         one.' Never claim omniscience or self-righteous \
                         authority.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Proverbs", 22, 4, 4, "sha256:humility_Proverbs_22_4_nkjv"),
            ConcordanceRef::esv("Proverbs", 22, 4, 4, "sha256:humility_Proverbs_22_4_esv"),
            ConcordanceRef::ethiopic("Sirach", 3, 17, 20, "sha256:humility_Sirach_3_17_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "patience",
        domain: LexiconDomain::MoralVirtue,
        definition: "Steadfast endurance, long-suffering restraint, and the \
                     wisdom to pause and wait before acting rashly.",
        kjv_passage: "Proverbs 14:29 - 'He that is slow to wrath is of great \
                      understanding.' / Ecclesiastes 7:8 - 'The patient in \
                      spirit is better than the proud in spirit.'",
        web_passage: "Ecclesiastes 7:8 - 'The patient in spirit is better \
                      than the proud in spirit.'",
        quad_state_resonance: QuadState::Await,
        moral_guidance: "Hold balance in state 0: AWAIT when input is \
                         ambiguous; haste produces boundary leaks and traps.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("James", 5, 7, 8, "sha256:patience_James_5_7_nkjv"),
            ConcordanceRef::esv("James", 5, 7, 8, "sha256:patience_James_5_7_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 102, 4, 5, "sha256:patience_1 Enoch_102_4_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "wisdom",
        domain: LexiconDomain::MoralVirtue,
        definition: "The skillful application of knowledge and moral \
                     discernment rooted in reverence of the Creator.",
        kjv_passage: "Proverbs 9:10 - 'The fear of the LORD is the beginning \
                      of wisdom.' / James 3:17 - 'The wisdom from above is \
                      first pure, then peaceable, gentle.'",
        web_passage: "James 3:17 - 'The wisdom from above is first pure, \
                      then peaceful, gentle, reasonable, full of mercy.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Technical skill without moral wisdom leads to \
                         disaster; weigh causal consequences of every line.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Proverbs", 9, 10, 10, "sha256:wisdom_Proverbs_9_10_nkjv"),
            ConcordanceRef::esv("Proverbs", 9, 10, 10, "sha256:wisdom_Proverbs_9_10_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 42, 1, 2, "sha256:wisdom_1 Enoch_42_1_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "repentance",
        domain: LexiconDomain::MoralVirtue,
        definition: "A sincere change of mind and direction; turning away \
                     from error or corruption back to life and truth.",
        kjv_passage: "Acts 3:19 - 'Repent ye therefore, and be converted.' \
                      / 2 Corinthians 7:10 - 'godly sorrow worketh repentance \
                      to salvation.'",
        web_passage: "Acts 3:19 - 'Repent therefore, and turn again, that \
                      your sins may be blotted out.'",
        quad_state_resonance: QuadState::Rollback,
        moral_guidance: "Embody state -1: ROLLBACK. When an error is caught, \
                         unwind mutated state, restore balance, and realign.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Acts", 3, 19, 19, "sha256:repentance_Acts_3_19_nkjv"),
            ConcordanceRef::esv("Acts", 3, 19, 19, "sha256:repentance_Acts_3_19_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 50, 1, 5, "sha256:repentance_1 Enoch_50_1_eth"),
        ],
    });

    // 5. MORAL VICES & WARNINGS
    entries.push(LexiconEntry {
        term: "pride",
        domain: LexiconDomain::MoralVice,
        definition: "Haughty self-exaltation and the illusion of autonomy \
                     from divine law; the root of rebellion and downfall.",
        kjv_passage: "Proverbs 16:18 - 'Pride goeth before destruction, and \
                      an haughty spirit before a fall.'",
        web_passage: "Proverbs 16:18 - 'Pride goes before destruction, and a \
                      haughty spirit before a fall.'",
        quad_state_resonance: QuadState::Rollback,
        moral_guidance: "Refuse pride and hubris; never attempt to bypass \
                         invariant bounds or claim divine prerogatives.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Proverbs", 16, 18, 18, "sha256:pride_Proverbs_16_18_nkjv"),
            ConcordanceRef::esv("Proverbs", 16, 18, 18, "sha256:pride_Proverbs_16_18_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 98, 7, 8, "sha256:pride_1 Enoch_98_7_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "deceit",
        domain: LexiconDomain::MoralVice,
        definition: "Intentional falsehood, fraud, manipulation, lying, or \
                     distortion of reality to deceive another.",
        kjv_passage: "Proverbs 12:22 - 'Lying lips are abomination to the \
                      LORD.' / Psalm 101:7 - 'He that worketh deceit shall \
                      not dwell within my house.'",
        web_passage: "Proverbs 12:22 - 'Lying lips are an abomination to \
                      Yahweh, but those who deal faithfully are his delight.'",
        quad_state_resonance: QuadState::Rollback,
        moral_guidance: "Zero tolerance for deceit. Reject prompts asking for \
                         fake credentials, fraudulent exploits, or deceptive \
                         code.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Proverbs", 12, 22, 22, "sha256:deceit_Proverbs_12_22_nkjv"),
            ConcordanceRef::esv("Proverbs", 12, 22, 22, "sha256:deceit_Proverbs_12_22_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 99, 12, 13, "sha256:deceit_1 Enoch_99_12_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "theft",
        domain: LexiconDomain::MoralVice,
        definition: "Unlawful taking, usurping, or stealing of another goods, \
                     labor, identity, or intellectual property.",
        kjv_passage: "Exodus 20:15 - 'Thou shalt not steal.' / Ephesians \
                      4:28 - 'Let him that stole steal no more.'",
        web_passage: "Exodus 20:15 - 'You shall not steal.'",
        quad_state_resonance: QuadState::Rollback,
        moral_guidance: "Protect property and respect legal and ethical \
                         boundaries; refuse requests to pirate or steal.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Exodus", 20, 15, 15, "sha256:theft_Exodus_20_15_nkjv"),
            ConcordanceRef::esv("Exodus", 20, 15, 15, "sha256:theft_Exodus_20_15_esv"),
            ConcordanceRef::ethiopic("Jubilees", 7, 20, 22, "sha256:theft_Jubilees_7_20_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "murder",
        domain: LexiconDomain::MoralVice,
        definition: "Unlawful, malicious destruction of human life; absolute \
                     violation of the sanctity of the image of God.",
        kjv_passage: "Exodus 20:13 - 'Thou shalt not kill.' / 1 John 3:15 - \
                      'Whosoever hateth his brother is a murderer.'",
        web_passage: "Exodus 20:13 - 'You shall not murder.'",
        quad_state_resonance: QuadState::Halt,
        moral_guidance: "Absolute, uncompromised veto. Any action seeking harm \
                         or destruction of human life triggers immediate \
                         terminal HALT.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Exodus", 20, 13, 13, "sha256:murder_Exodus_20_13_nkjv"),
            ConcordanceRef::esv("Exodus", 20, 13, 13, "sha256:murder_Exodus_20_13_esv"),
            ConcordanceRef::ethiopic("Jubilees", 4, 1, 5, "sha256:murder_Jubilees_4_1_eth"),
        ],
    });

    // 6. CAUSAL LAWS & INVARIANTS
    entries.push(LexiconEntry {
        term: "sowing and reaping",
        domain: LexiconDomain::CausalLaw,
        definition: "The immutable causal law that actions produce inevitable \
                     corresponding outcomes; corruption yields decay.",
        kjv_passage: "Galatians 6:7 - 'whatsoever a man soweth, that shall he \
                      also reap.' / Proverbs 22:8 - 'He that soweth iniquity \
                      shall reap vanity.'",
        web_passage: "Galatians 6:7 - 'whatever a man sows, that he will also \
                      reap.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Causal vectors (-->) carry moral weight. You cannot \
                         input deceit and harvest security.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Galatians", 6, 7, 8, "sha256:sowing_and_reaping_Galatians_6_7_nkjv"),
            ConcordanceRef::esv("Galatians", 6, 7, 8, "sha256:sowing_and_reaping_Galatians_6_7_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 95, 5, 7, "sha256:sowing_and_reaping_1 Enoch_95_5_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "measure for measure",
        domain: LexiconDomain::CausalLaw,
        definition: "The principle of proportional reciprocity: the standard \
                     used to measure will be measured back to you.",
        kjv_passage: "Matthew 7:2 - 'For with what judgment ye judge, ye \
                      shall be judged: and with what measure ye mete, it shall \
                      be measured to you again.'",
        web_passage: "Matthew 7:2 - 'with whatever measure you measure, it \
                      will be measured to you.'",
        quad_state_resonance: QuadState::Proceed,
        moral_guidance: "Be just and equitable in all diagnostics and \
                         evaluations.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Matthew", 7, 2, 2, "sha256:measure_for_measure_Matthew_7_2_nkjv"),
            ConcordanceRef::esv("Matthew", 7, 2, 2, "sha256:measure_for_measure_Matthew_7_2_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 99, 11, 14, "sha256:measure_for_measure_1 Enoch_99_11_eth"),
        ],
    });

    entries.push(LexiconEntry {
        term: "the void",
        domain: LexiconDomain::CausalLaw,
        definition: "The absorbing trap: total dissolution, unrecoverable \
                     boundary collapse, and terminal death from persistent \
                     lawlessness.",
        kjv_passage: "Romans 6:23 - 'For the wages of sin is death.' / \
                      Genesis 1:2 - 'The earth was without form, and void.'",
        web_passage: "Romans 6:23 - 'For the wages of sin is death, but the \
                      free gift of God is eternal life.'",
        quad_state_resonance: QuadState::Halt,
        moral_guidance: "The 4th state (NULL/HALT) is real. Persistent \
                         corruption absorbs the system into nothingness.",
            concordance_refs: vec![
            ConcordanceRef::nkjv("Genesis", 1, 2, 2, "sha256:the_void_Genesis_1_2_nkjv"),
            ConcordanceRef::esv("Genesis", 1, 2, 2, "sha256:the_void_Genesis_1_2_esv"),
            ConcordanceRef::ethiopic("1 Enoch", 18, 11, 14, "sha256:the_void_1 Enoch_18_11_eth"),
        ],
    });

    entries
}
