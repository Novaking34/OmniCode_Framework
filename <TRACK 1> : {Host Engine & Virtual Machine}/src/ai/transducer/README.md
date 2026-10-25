# OmniCode AI Transducer Subsystem: Natural Language & AST Interface

> **"A word fitly spoken is like apples of gold in pictures of silver."**  
> — *Proverbs 25:11 (KJV)*  
>  
> **"Let your speech always be with grace, seasoned with salt, that you may**  
> **know how you ought to answer each one."**  
> — *Colossians 4:6 (WEB)*

The **AI Transducer Subsystem** (`src/ai/transducer`) forms the bidirectional
linguistic bridge between human natural language discourse and deterministic
machine execution in the OmniCode Framework. Situated within Track 1
(`<TRACK 1> : {Host Engine & Virtual Machine}`), the Transducer decodes natural
language sentences into canonical 4-Block OmniCode Turing reels and encodes
machine execution states, environment bindings, and cortical thoughts back into
articulate human discourse.

Unlike probabilistic Large Language Models (LLMs) that guess token sequences,
or brittle pattern matchers that hardcode static string answers, the elevated
OmniCode Transducer operates as a **data-driven semantic compiler**: it parses
the 8 parts of speech into the 5Ws and H, models directional causation
(`-->`, `<--`, `^--`, `v--`, `=`, `!=`, `~=`), verifies reels via the Two-Way
Checksum parity gate, and communicates with the `AICortex` and `DiscourseEngine`.

---

## 1. Architectural Topology: The Transducer Pipeline

```text
               +---------------------------------------+
               |         Human Operator Prompt         |
               +-------------------+-------------------+
                                   |
                                   v
               +---------------------------------------+
               |              AITransducer             |
               | (mod.rs: Facade & Audit Coordinator)  |
               +-------------------+-------------------+
                                   |
         +-------------------------+-------------------------+
         |                                                   |
         v                                                   v
+-----------------------+                           +-----------------------+
|      decoder.rs       |                           |       encoder.rs      |
| (Prompt -> 4-Block    |                           | (Machine State +      |
|  Reel Synthesis with  |                           |  Thought -> Natural   |
|  Cortical Thought)    |                           |  Human Discourse)     |
+-----------+-----------+                           +-----------^-----------+
            |                                                   |
            |   +---------------------------------------+       |
            +-->|               svo.rs                  |       |
            |   | (8 Parts of Speech, Data-Driven       |       |
            |   |  Verbs/Prepositions, Castable Ops)    |       |
            |   +-------------------+-------------------+       |
            |                       |                           |
            |   +-------------------v-------------------+       |
            +-->|            templates.rs               |       |
                | (Dynamic 4-Block Reel Synthesis)      |       |
                +-------------------+-------------------+       |
                                    |                           |
                                    v                           |
                        +-----------------------+               |
                        |      executor.rs      |---------------+
                        | (Lexer, Parser, VM    |
                        |  Evaluator & Bindings)|
                        +-----------------------+
```

---

## 2. Module Matrix (5-Block Architecture)

| Source File | Primary Responsibility | Block 4 Audit Procedure |
| :--- | :--- | :--- |
| [`mod.rs`](./mod.rs) | Master facade coordinator & round-trip harness | `audit_transducer_subsystem()` |
| [`decoder.rs`](./decoder.rs) | Dynamic prompt decoding & cortical thought binding | `audit_decoder_subsystem()` |
| [`encoder.rs`](./encoder.rs) | Unified discourse response synthesis | `audit_encoder_subsystem()` |
| [`svo.rs`](./svo.rs) | Continuous SVO composition & 2-Way Checksum parity gate | `audit_svo_subsystem()` |
| [`templates.rs`](./templates.rs) | Dynamic 4-Block Turing reel AST generator | `audit_templates_subsystem()` |
| [`executor.rs`](./executor.rs) | VM evaluation pipeline & environment bindings | `audit_executor_subsystem()` |
| [`patterns.rs`](./patterns.rs) | Morphological extraction & token normalization | `audit_patterns_subsystem()` |
| [`session.rs`](./session.rs) | Transduction session lifecycle & bindings manifest | `audit_session_subsystem()` |

---

## 3. Data-Driven Linguistic Sheets (`data/`)

Linguistic mappings are decoupled from Rust source code into data sheets:

1. **`verbs.base.dat`**: Maps action verbs to directional vectors (`-->`, `<--`,
   `^--`, `v--`, `=`, `!=`, `~=`), specifies whether they can be cast to
   operators, and registers their causal classifications.
2. **`prepositions.base.dat`**: Maps prepositions to spatial boundaries,
   target channels, source extractions, address offsets, and companion bindings.
3. **`conjunctions.base.dat`**: Maps conjunctions to sequential roll chaining
   (`::`), caveat interventions (`!`), and branching disjunctions (`\|`).

---

## 4. Key Capabilities & Invariants

* **Direct Cortex Thought Integration**: `decode_prompt_with_thought` accepts the
  cortical `ThoughtRecord`, embedding 5Ws & H, Dooyeweerd modal domains, and
  Scripture landmarks directly into synthesized 4-Block reels.
* **Castable Operators**: Subjects and objects can act as active operators when
  cast, allowing higher-order functional pipelines to emerge from natural grammar.
* **Two-Way Checksum Parity Gate**: Every synthesized reel is evaluated by the
  `RestoratorCertifier` to ensure boundary closure and scaffolding integrity.
* **Epistemic Humility**: Prompts in `0: AWAIT` state trigger authentic inquiries
  back to the operator rather than fabricating answers.
