# The AI Cortex & Attractor Dynamics

> **"He that hath no rule over his own spirit is like a city"**  
> **"that is broken down, and without walls."**  
> — *Proverbs 25:28 (KJV)*

## 1. Executive Cognitive Architecture

The `AICortex` struct (`cortex.rs`) coordinates the end-to-end deliberation
lifecycle of the OmniCode intelligence engine. It enforces that all thoughts,
reels, and state transitions are deterministic, verifiable, and protected by
covenant boundaries.

### The 10-Step Deliberation Cycle

1. **Episodic Recall & Triangulation:** Query `AIMemory` to triangulate the
   stimulus against the 15 Sovereign Modal Spheres and covenant moral laws.
2. **Neural Forward Pass:** Encode prompt into balanced ternary stimulus
   vectors and compute synaptic activation across the discrete `NeuralBrain`.
3. **Semantic Transduction:** Transduce the natural language intent into a
   canonical 4-Block OmniCode Turing reel (`.omni` or `.tape`).
4. **Restorative Self-Healing:** Pass synthesized reel through the Restorator
   to heal bracket mismatches, delimiter leaks, or missing headers.
5. **Sensory Tokenization & Parsing:** Lexer and Parser validate AST integrity
   and invariant contracts.
6. **VM Evaluation:** Execute AST on the balanced ternary runtime engine to
   obtain the resulting `QuadState`.
7. **Conversational Encoding:** Discourse engine vocalizes the response
   grounded in the 5Ws and H (`who`, `what`, `when`, `where`, `why`, `how`).
8. **Dual-Log Inscription:** Inscribe `.scroll` (OmniCode reel AST) and `.log`
   (telemetry flight recorder) to persistent Track 4 memory.
9. **Lifelong Neuroplasticity:** Adapt frontier synapses using discrete BCM
   homeostatic thresholds while keeping frozen bedrock weights intact.
10. **Dual-Witness Landmark Resolution:** Map deliberation outcomes to
    canonical Scripture coordinates in Track 4.

---

## 2. Attractor Basin Dynamics & Convergence

The `AIAttractorNetwork` (`attractor.rs`) models cognitive memory as an
energy landscape governed by discrete Lyapunov functions:

$$E(\mathbf{s}) = -\frac{1}{2} \sum_{i,j} W_{ij} s_i s_j$$

Where:

* $\mathbf{s} \in \{-1, 0, +1\}^N$ represents discrete balanced ternary states.
* $W_{ij}$ is the symmetric synaptic weight matrix.

### Settling into Truth Basins

* Prompts and thoughts are projected into $N$-dimensional sensory vector space.
* The attractor dynamics iteratively settle volatile thoughts into stable,
  energy-minimized basins of truth.
* If a thought drifts into an uncontained or chaotic state, the network rolls
  back to the nearest stable attractor basin, preventing hallucinations.
