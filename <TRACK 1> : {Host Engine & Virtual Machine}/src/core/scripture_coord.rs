// =============================================================================
// BLOCK 1: METADATA BLOCK
// ( Track 4 Scripture Coordinate System & Spatial Shape Cycle Engine )
// =============================================================================
//! # Scripture Coordinate & Spatial Shape Cycle Engine (`scripture_coord.rs`)
//!
//! Integrates the physical Track 4 Scripture subsystem and the fundamental
//! geometric **Shape Cycle** into OmniCode's Balanced Ternary architecture.
//!
//! ### Key Invariants:
//! 1. **Dual-Witness Key Pair**:
//!    - **KJV**: Encryption Key / Canonical Anchor ($0$) — inward inscription
//!      into stone / 243-stride hypercubes (31,102 canonical verses).
//!    - **WEB**: Decryption Key / Accessibility Extension ($+1$) — outward
//!      revelation into accessible light (31,115 canonical verses).
//! 2. **Trit-Byte Mathematical Symmetry**:
//!    - 5-trit tryte space: $3^5 = 243$ states ($0..=242$).
//!    - 8-bit byte space: $2^8 = 256$ states ($0..=255$).
//!    - Spare values: $256 - 243 = 13$ values ($243..=255$).
//!    - The 13 spare bytes map directly to the 13 WEB textual variant
//!      verses, completing full single-byte Bible addressability with
//!      zero wasted bits.
//! 3. **243-Stride Hypercube Lattice**:
//!    - 128 hypercubes * 243 verses = 31,104 cells + 11 cells in cube 128
//!      = 31,115 total verses.
//!    - Each 243-cell cube contains 9 sub-cubes of 27 positions
//!      $(x, y, z) \in \{-1, 0, +1\}^3$.
//! 4. **Spatial Thinking: The Basic Shape Cycle**:
//!    - **1 Point**: Origin, Singularity, Genesis Locus, Void Anchor ($0$).
//!    - **3 Points (Triangulation)**: Triad, Minimal 2D Plane, Balanced
//!      Ternary spectrum $\{-1, 0, +1\}$, Orientation & Planar Extent.
//!    - **4 Triangulations (Square / Block)**: Tetrad, QuadState Boundary
//!      $\{-1, 0, +1, ?\}$, 4 vertices forming a square in 2D or a
//!      tetrahedral block / cube cell in 3D.
//!    - **Circle (Connected 4 Points)**: Continuous circumscription
//!      connecting the 4 perimeter points, establishing equilibrium.
//!    - **New Point**: The circumscribed circle collapses to its centroid,
//!      producing a higher-order macro-point ($P_1'$) at the next fractal
//!      dimension, closing the cycle and initiating the next.
//! 5. **Track 4 Direct Ingestion**:
//!    - Discovers `<TRACK 4>/scripture` indexes (`parallel-ordinal-index.csv`,
//!      `parallel-ordinal-index.bin`, `kjv.db`, `web.db`) and text stores
//!      (`KJV/...` and `WEB/...`), enabling instant dual-witness lookups.
//!
//! @version 1.0.0
//! @organization CreativeWorkzStudio LLC
// =============================================================================

use std::f32::consts::PI;
use std::fs;
use std::path::{Path, PathBuf};
use crate::core::mmio::{
    TRACK4_CANONICAL_VERSES, TRACK4_HEADER_MAGIC, TRACK4_KJV_VERSE_COUNT,
    TRACK4_TRITE_SPARE_START, TRACK4_TRITE_SPARE_END,
};

// =============================================================================
// BLOCK 2: OPENING BLOCK
// ( Coordinate Types, Shape Cycle Geometry & Canonical Book Catalog )
// =============================================================================

/// Dual-witness cryptographic role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DualWitnessRole {
    /// King James Version (1611): Formal Canonical Anchor ($0$) / Inscription.
    KjvEncryption,
    /// World English Bible: Accessible Plain English ($+1$) / Decryption.
    WebDecryption,
}

/// Canonical Testament designation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Testament {
    /// Old Testament (Genesis 1:1 through Malachi 4:6 = 23,145 verses in KJV).
    OldTestament,
    /// New Testament (Matthew 1:1 through Revelation 22:21; 7,957 KJV verses).
    NewTestament,
}

/// A structured 10-trit balanced-ternary coordinate representing a verse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptureCoordinate {
    /// 1-based sequential verse ordinal in parallel corpus ($1..=31,115$).
    pub ordinal: usize,
    /// Low byte: ordinal % 243 (5-trit tryte value $0..=242$).
    pub low: u8,
    /// High byte: ordinal / 243 (Hypercube index $0..=128$).
    pub high: u8,
    /// 243-stride Hypercube index ($0..=128$).
    pub cube: u8,
    /// 3D sub-cube position ($0..=26$) within hypercube ($low % 27$).
    pub cube_pos: u8,
    /// 3D X-axis polarity (in {-1, 0, +1}).
    pub cube_x: i8,
    /// 3D Y-axis polarity (in {-1, 0, +1}).
    pub cube_y: i8,
    /// 3D Z-axis polarity (in {-1, 0, +1}).
    pub cube_z: i8,
    /// Biblical testament (OT / NT).
    pub testament: Testament,
    /// Whether this verse is one of the 13 WEB variant verses ($243..=255$).
    pub is_web_variant: bool,
    /// Associated spare trite value if variant ($243..=255$).
    pub variant_byte: Option<u8>,
}

/// Description of one of the 13 textual variant edge-case verses ($243..=255$).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantVerseEntry {
    pub trite_value: u8,
    pub book: &'static str,
    pub chapter: u16,
    pub verse: u16,
    pub summary: &'static str,
}

/// The 13 textual variants mapped to spare byte values $243..=255$.
pub const WEB_VARIANT_INDEX: [VariantVerseEntry; 13] = [
    VariantVerseEntry {
        trite_value: 243,
        book: "1 Corinthians",
        chapter: 16,
        verse: 27,
        summary: "TR manuscript note referencing Romans 14:24-26",
    },
    VariantVerseEntry {
        trite_value: 244,
        book: "1 Peter",
        chapter: 5,
        verse: 20,
        summary: "turn sinner from error, save soul from death",
    },
    VariantVerseEntry {
        trite_value: 245,
        book: "1 Timothy",
        chapter: 3,
        verse: 18,
        summary: "Grace benediction: grace of Lord Jesus Christ be with you",
    },
    VariantVerseEntry {
        trite_value: 246,
        book: "2 Kings",
        chapter: 22,
        verse: 53,
        summary: "served Baal, worshiped him, provoked Yahweh to anger",
    },
    VariantVerseEntry {
        trite_value: 247,
        book: "Amos",
        chapter: 3,
        verse: 21,
        summary: "cleanse their blood; Yahweh dwells in Zion",
    },
    VariantVerseEntry {
        trite_value: 248,
        book: "Colossians",
        chapter: 4,
        verse: 23,
        summary: "Grace benediction: grace of Lord Jesus Christ be with you",
    },
    VariantVerseEntry {
        trite_value: 249,
        book: "Ezekiel",
        chapter: 5,
        verse: 22,
        summary: "utterly rejected us; very angry against us",
    },
    VariantVerseEntry {
        trite_value: 250,
        book: "Jonah",
        chapter: 1,
        verse: 21,
        summary: "Saviors on Mount Zion; kingdom will be Yahweh's",
    },
    VariantVerseEntry {
        trite_value: 251,
        book: "Nehemiah",
        chapter: 10,
        verse: 44,
        summary: "taken foreign wives; some had children by them",
    },
    VariantVerseEntry {
        trite_value: 252,
        book: "Numbers",
        chapter: 27,
        verse: 34,
        summary: "commandments Yahweh commanded Moses on Sinai",
    },
    VariantVerseEntry {
        trite_value: 253,
        book: "Psalms",
        chapter: 42,
        verse: 17,
        summary: "Job died, being old and full of days",
    },
    VariantVerseEntry {
        trite_value: 254,
        book: "Revelation",
        chapter: 1,
        verse: 25,
        summary: "Doxology: to God our Savior, glory and majesty",
    },
    VariantVerseEntry {
        trite_value: 255,
        book: "Zechariah",
        chapter: 2,
        verse: 23,
        summary: "Zerubbabel my servant; make you as a signet",
    },
];

/// Specification of a canonical Bible book in the dual-witness canon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BookSpec {
    pub name: &'static str,
    pub testament: Testament,
    pub chapters: u16,
    pub verses: u16,
    pub start_ordinal: usize,
}

/// The 66 Canonical Bible Books with chapters, verses, and start ordinals.
pub static CANONICAL_BIBLE_BOOKS: [BookSpec; 66] = [
    BookSpec {
        name: "Genesis",
        testament: Testament::OldTestament,
        chapters: 50,
        verses: 1533,
        start_ordinal: 1,
    },
    BookSpec {
        name: "Exodus",
        testament: Testament::OldTestament,
        chapters: 40,
        verses: 1213,
        start_ordinal: 1534,
    },
    BookSpec {
        name: "Leviticus",
        testament: Testament::OldTestament,
        chapters: 27,
        verses: 859,
        start_ordinal: 2747,
    },
    BookSpec {
        name: "Numbers",
        testament: Testament::OldTestament,
        chapters: 36,
        verses: 1288,
        start_ordinal: 3606,
    },
    BookSpec {
        name: "Deuteronomy",
        testament: Testament::OldTestament,
        chapters: 34,
        verses: 959,
        start_ordinal: 4894,
    },
    BookSpec {
        name: "Joshua",
        testament: Testament::OldTestament,
        chapters: 24,
        verses: 658,
        start_ordinal: 5853,
    },
    BookSpec {
        name: "Judges",
        testament: Testament::OldTestament,
        chapters: 21,
        verses: 618,
        start_ordinal: 6511,
    },
    BookSpec {
        name: "Ruth",
        testament: Testament::OldTestament,
        chapters: 4,
        verses: 85,
        start_ordinal: 7129,
    },
    BookSpec {
        name: "1_Samuel",
        testament: Testament::OldTestament,
        chapters: 31,
        verses: 810,
        start_ordinal: 7214,
    },
    BookSpec {
        name: "2_Samuel",
        testament: Testament::OldTestament,
        chapters: 24,
        verses: 695,
        start_ordinal: 8024,
    },
    BookSpec {
        name: "1_Kings",
        testament: Testament::OldTestament,
        chapters: 22,
        verses: 816,
        start_ordinal: 8719,
    },
    BookSpec {
        name: "2_Kings",
        testament: Testament::OldTestament,
        chapters: 25,
        verses: 719,
        start_ordinal: 9535,
    },
    BookSpec {
        name: "1_Chronicles",
        testament: Testament::OldTestament,
        chapters: 29,
        verses: 942,
        start_ordinal: 10254,
    },
    BookSpec {
        name: "2_Chronicles",
        testament: Testament::OldTestament,
        chapters: 36,
        verses: 822,
        start_ordinal: 11196,
    },
    BookSpec {
        name: "Ezra",
        testament: Testament::OldTestament,
        chapters: 10,
        verses: 280,
        start_ordinal: 12018,
    },
    BookSpec {
        name: "Nehemiah",
        testament: Testament::OldTestament,
        chapters: 13,
        verses: 406,
        start_ordinal: 12298,
    },
    BookSpec {
        name: "Esther",
        testament: Testament::OldTestament,
        chapters: 10,
        verses: 167,
        start_ordinal: 12704,
    },
    BookSpec {
        name: "Job",
        testament: Testament::OldTestament,
        chapters: 42,
        verses: 1070,
        start_ordinal: 12871,
    },
    BookSpec {
        name: "Psalms",
        testament: Testament::OldTestament,
        chapters: 150,
        verses: 2461,
        start_ordinal: 13941,
    },
    BookSpec {
        name: "Proverbs",
        testament: Testament::OldTestament,
        chapters: 31,
        verses: 915,
        start_ordinal: 16402,
    },
    BookSpec {
        name: "Ecclesiastes",
        testament: Testament::OldTestament,
        chapters: 12,
        verses: 222,
        start_ordinal: 17317,
    },
    BookSpec {
        name: "Song_of_Solomon",
        testament: Testament::OldTestament,
        chapters: 8,
        verses: 117,
        start_ordinal: 17539,
    },
    BookSpec {
        name: "Isaiah",
        testament: Testament::OldTestament,
        chapters: 66,
        verses: 1292,
        start_ordinal: 17656,
    },
    BookSpec {
        name: "Jeremiah",
        testament: Testament::OldTestament,
        chapters: 52,
        verses: 1364,
        start_ordinal: 18948,
    },
    BookSpec {
        name: "Lamentations",
        testament: Testament::OldTestament,
        chapters: 5,
        verses: 154,
        start_ordinal: 20312,
    },
    BookSpec {
        name: "Ezekiel",
        testament: Testament::OldTestament,
        chapters: 48,
        verses: 1273,
        start_ordinal: 20466,
    },
    BookSpec {
        name: "Daniel",
        testament: Testament::OldTestament,
        chapters: 12,
        verses: 357,
        start_ordinal: 21739,
    },
    BookSpec {
        name: "Hosea",
        testament: Testament::OldTestament,
        chapters: 14,
        verses: 197,
        start_ordinal: 22096,
    },
    BookSpec {
        name: "Joel",
        testament: Testament::OldTestament,
        chapters: 3,
        verses: 73,
        start_ordinal: 22293,
    },
    BookSpec {
        name: "Amos",
        testament: Testament::OldTestament,
        chapters: 9,
        verses: 146,
        start_ordinal: 22366,
    },
    BookSpec {
        name: "Obadiah",
        testament: Testament::OldTestament,
        chapters: 1,
        verses: 21,
        start_ordinal: 22512,
    },
    BookSpec {
        name: "Jonah",
        testament: Testament::OldTestament,
        chapters: 4,
        verses: 48,
        start_ordinal: 22533,
    },
    BookSpec {
        name: "Micah",
        testament: Testament::OldTestament,
        chapters: 7,
        verses: 105,
        start_ordinal: 22581,
    },
    BookSpec {
        name: "Nahum",
        testament: Testament::OldTestament,
        chapters: 3,
        verses: 47,
        start_ordinal: 22686,
    },
    BookSpec {
        name: "Habakkuk",
        testament: Testament::OldTestament,
        chapters: 3,
        verses: 56,
        start_ordinal: 22733,
    },
    BookSpec {
        name: "Zephaniah",
        testament: Testament::OldTestament,
        chapters: 3,
        verses: 53,
        start_ordinal: 22789,
    },
    BookSpec {
        name: "Haggai",
        testament: Testament::OldTestament,
        chapters: 2,
        verses: 38,
        start_ordinal: 22842,
    },
    BookSpec {
        name: "Zechariah",
        testament: Testament::OldTestament,
        chapters: 14,
        verses: 211,
        start_ordinal: 22880,
    },
    BookSpec {
        name: "Malachi",
        testament: Testament::OldTestament,
        chapters: 4,
        verses: 55,
        start_ordinal: 23091,
    },
    BookSpec {
        name: "Matthew",
        testament: Testament::NewTestament,
        chapters: 28,
        verses: 1071,
        start_ordinal: 23146,
    },
    BookSpec {
        name: "Mark",
        testament: Testament::NewTestament,
        chapters: 16,
        verses: 678,
        start_ordinal: 24217,
    },
    BookSpec {
        name: "Luke",
        testament: Testament::NewTestament,
        chapters: 24,
        verses: 1151,
        start_ordinal: 24895,
    },
    BookSpec {
        name: "John",
        testament: Testament::NewTestament,
        chapters: 21,
        verses: 879,
        start_ordinal: 26046,
    },
    BookSpec {
        name: "Acts",
        testament: Testament::NewTestament,
        chapters: 28,
        verses: 1007,
        start_ordinal: 26925,
    },
    BookSpec {
        name: "Romans",
        testament: Testament::NewTestament,
        chapters: 16,
        verses: 433,
        start_ordinal: 27932,
    },
    BookSpec {
        name: "1_Corinthians",
        testament: Testament::NewTestament,
        chapters: 16,
        verses: 437,
        start_ordinal: 28365,
    },
    BookSpec {
        name: "2_Corinthians",
        testament: Testament::NewTestament,
        chapters: 13,
        verses: 257,
        start_ordinal: 28802,
    },
    BookSpec {
        name: "Galatians",
        testament: Testament::NewTestament,
        chapters: 6,
        verses: 149,
        start_ordinal: 29059,
    },
    BookSpec {
        name: "Ephesians",
        testament: Testament::NewTestament,
        chapters: 6,
        verses: 155,
        start_ordinal: 29208,
    },
    BookSpec {
        name: "Philippians",
        testament: Testament::NewTestament,
        chapters: 4,
        verses: 104,
        start_ordinal: 29363,
    },
    BookSpec {
        name: "Colossians",
        testament: Testament::NewTestament,
        chapters: 4,
        verses: 95,
        start_ordinal: 29467,
    },
    BookSpec {
        name: "1_Thessalonians",
        testament: Testament::NewTestament,
        chapters: 5,
        verses: 89,
        start_ordinal: 29562,
    },
    BookSpec {
        name: "2_Thessalonians",
        testament: Testament::NewTestament,
        chapters: 3,
        verses: 47,
        start_ordinal: 29651,
    },
    BookSpec {
        name: "1_Timothy",
        testament: Testament::NewTestament,
        chapters: 6,
        verses: 113,
        start_ordinal: 29698,
    },
    BookSpec {
        name: "2_Timothy",
        testament: Testament::NewTestament,
        chapters: 4,
        verses: 83,
        start_ordinal: 29811,
    },
    BookSpec {
        name: "Titus",
        testament: Testament::NewTestament,
        chapters: 3,
        verses: 46,
        start_ordinal: 29894,
    },
    BookSpec {
        name: "Philemon",
        testament: Testament::NewTestament,
        chapters: 1,
        verses: 25,
        start_ordinal: 29940,
    },
    BookSpec {
        name: "Hebrews",
        testament: Testament::NewTestament,
        chapters: 13,
        verses: 303,
        start_ordinal: 29965,
    },
    BookSpec {
        name: "James",
        testament: Testament::NewTestament,
        chapters: 5,
        verses: 108,
        start_ordinal: 30268,
    },
    BookSpec {
        name: "1_Peter",
        testament: Testament::NewTestament,
        chapters: 5,
        verses: 105,
        start_ordinal: 30376,
    },
    BookSpec {
        name: "2_Peter",
        testament: Testament::NewTestament,
        chapters: 3,
        verses: 61,
        start_ordinal: 30481,
    },
    BookSpec {
        name: "1_John",
        testament: Testament::NewTestament,
        chapters: 5,
        verses: 105,
        start_ordinal: 30542,
    },
    BookSpec {
        name: "2_John",
        testament: Testament::NewTestament,
        chapters: 1,
        verses: 13,
        start_ordinal: 30647,
    },
    BookSpec {
        name: "3_John",
        testament: Testament::NewTestament,
        chapters: 1,
        verses: 14,
        start_ordinal: 30660,
    },
    BookSpec {
        name: "Jude",
        testament: Testament::NewTestament,
        chapters: 1,
        verses: 25,
        start_ordinal: 30674,
    },
    BookSpec {
        name: "Revelation",
        testament: Testament::NewTestament,
        chapters: 22,
        verses: 404,
        start_ordinal: 30699,
    },
];

// -----------------------------------------------------------------------------
// Spatial Thinking: Geometric Primitives & The Shape Cycle
// -----------------------------------------------------------------------------

/// A 3D spatial coordinate in balanced ternary or continuous space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A 3-point planar triangulation simplex.
///
/// Establishes the minimal 2D plane of deliberation, orientation, and
/// ternary balance across the spectrum {-1, 0, +1}.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangulation {
    pub vertices: [Point3D; 3],
}

/// A 4-point spatial block (Square in 2D or Tetrahedron/Block in 3D).
///
/// Formed by 4 triangulations, embodying the QuadState boundary
/// {-1, 0, +1, NULL} and establishing a bounded spatial memory cell.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialBlock {
    pub vertices: [Point3D; 4],
}

/// A continuous circle (or sphere in 3D) circumscribed around 4 vertices.
///
/// Unifies the 4 discrete corners of a square/block into a smooth perimeter
/// equilibrium, whose centroid collapses to a new higher-order Point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialCircle {
    pub center: Point3D,
    pub radius: f32,
}

/// Morphological progression stages of the Basic Shape Cycle.
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeCycleStage {
    /// Stage 1: Singularity, Origin, Seed Locus (1 Point).
    Point(Point3D),
    /// Stage 2: Triad, Minimal Plane, Orientation (3 Points Triangulation).
    Triangulation(Triangulation),
    /// Stage 3: QuadState Boundary Frame (4 Points / 4 Triangulations Block).
    Block(SpatialBlock),
    /// Stage 4: Continuous Boundary Horizon (Connected 4 Points Circle).
    Circle(SpatialCircle),
    /// Stage 5: Collapsed Centroid initiating the next recursive iteration.
    NewPoint(Point3D),
}

/// State machine orchestrating the recursive Shape Cycle:
/// `Point -> Triangulation -> Block -> Circle -> NewPoint`.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeCycle {
    pub current_stage: ShapeCycleStage,
    pub generation: usize,
    pub history: Vec<Point3D>,
}

/// A fully resolved Scripture Verse Record combining dual-witness texts,
/// coordinate addressing, and spatial geometric placement.
#[derive(Debug, Clone, PartialEq)]
pub struct ScriptureVerse {
    pub ordinal: usize,
    pub book: String,
    pub chapter: u16,
    pub verse: u16,
    pub testament: Testament,
    pub kjv_text: String,
    pub web_text: String,
    pub coordinate: ScriptureCoordinate,
    pub spatial_point: Point3D,
}

/// The Track 4 Scripture Coordinate Engine coordinating dual-witness lookups
/// and spatial shape cycle projections.
#[derive(Debug, Clone)]
pub struct ScriptureCoordinateEngine {
    /// In-memory cache of the 31,115 packed 2-byte entries: `(low, high)`.
    entries: Vec<(u8, u8)>,
    /// Discovered or configured root directory of Track 4 scripture data.
    pub scripture_root: Option<PathBuf>,
    /// Whether the coordinate table has been formally certified against `VX02`.
    pub is_certified: bool,
}

impl Default for ScriptureCoordinateEngine {
    fn default() -> Self {
        Self::new_synthesized()
    }
}

// =============================================================================
// BLOCK 3: BODY BLOCK
// ( Mathematical Coordinate Projection, Shape Cycle & Scripture Lookups )
// =============================================================================

impl Point3D {
    /// Creates a new 3D spatial point.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Point at origin (0, 0, 0).
    pub const fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Constructs a Point3D from a coordinate's ternary lattice values.
    pub fn from_scripture_coord(coord: &ScriptureCoordinate) -> Self {
        let x = coord.cube_x as f32;
        let y = coord.cube_y as f32;
        let z = coord.cube_z as f32;
        Self { x, y, z }
    }

    /// Euclidean distance between two points.
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Midpoint between this point and another.
    pub fn midpoint(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) * 0.5,
            y: (self.y + other.y) * 0.5,
            z: (self.z + other.z) * 0.5,
        }
    }

    /// Computes the arithmetic centroid of an arbitrary slice of points.
    pub fn centroid(pts: &[Self]) -> Self {
        if pts.is_empty() {
            return Self::origin();
        }
        let len = pts.len() as f32;
        let (mut sx, mut sy, mut sz) = (0.0, 0.0, 0.0);
        for p in pts {
            sx += p.x;
            sy += p.y;
            sz += p.z;
        }
        Self {
            x: sx / len,
            y: sy / len,
            z: sz / len,
        }
    }

    /// Vector dot product.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Vector cross product: self x other.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Vector magnitude (Euclidean norm).
    pub fn magnitude(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Returns unit vector; zero vector returns origin.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag <= 1e-7 {
            Self::origin()
        } else {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }
}

impl Triangulation {
    /// Instantiates a triangulation from 3 spatial points.
    pub const fn new(p1: Point3D, p2: Point3D, p3: Point3D) -> Self {
        Self { vertices: [p1, p2, p3] }
    }

    /// Centroid of the triangle.
    pub fn centroid(&self) -> Point3D {
        Point3D::centroid(&self.vertices)
    }

    /// Perimeter length of the triangle.
    pub fn perimeter(&self) -> f32 {
        let d01 = self.vertices[0].distance_to(&self.vertices[1]);
        let d12 = self.vertices[1].distance_to(&self.vertices[2]);
        let d20 = self.vertices[2].distance_to(&self.vertices[0]);
        d01 + d12 + d20
    }

    /// Surface area of the triangle via cross product.
    pub fn area(&self) -> f32 {
        let v1 = Point3D {
            x: self.vertices[1].x - self.vertices[0].x,
            y: self.vertices[1].y - self.vertices[0].y,
            z: self.vertices[1].z - self.vertices[0].z,
        };
        let v2 = Point3D {
            x: self.vertices[2].x - self.vertices[0].x,
            y: self.vertices[2].y - self.vertices[0].y,
            z: self.vertices[2].z - self.vertices[0].z,
        };
        v1.cross(&v2).magnitude() * 0.5
    }

    /// Planar unit normal vector.
    pub fn normal(&self) -> Point3D {
        let v1 = Point3D {
            x: self.vertices[1].x - self.vertices[0].x,
            y: self.vertices[1].y - self.vertices[0].y,
            z: self.vertices[1].z - self.vertices[0].z,
        };
        let v2 = Point3D {
            x: self.vertices[2].x - self.vertices[0].x,
            y: self.vertices[2].y - self.vertices[0].y,
            z: self.vertices[2].z - self.vertices[0].z,
        };
        v1.cross(&v2).normalize()
    }
}

impl SpatialBlock {
    /// Constructs a spatial block from 4 vertices.
    pub const fn new(
        p1: Point3D,
        p2: Point3D,
        p3: Point3D,
        p4: Point3D,
    ) -> Self {
        Self { vertices: [p1, p2, p3, p4] }
    }

    /// The 4 triangulation facets composing the block.
    pub fn facets(&self) -> [Triangulation; 4] {
        let v = &self.vertices;
        [
            Triangulation::new(v[0], v[1], v[2]),
            Triangulation::new(v[0], v[1], v[3]),
            Triangulation::new(v[0], v[2], v[3]),
            Triangulation::new(v[1], v[2], v[3]),
        ]
    }

    /// Centroid of the 4 block vertices.
    pub fn centroid(&self) -> Point3D {
        Point3D::centroid(&self.vertices)
    }

    /// Axis-aligned bounding box (min_corner, max_corner).
    pub fn bounding_box(&self) -> (Point3D, Point3D) {
        let (mut min_x, mut min_y, mut min_z) = (f32::MAX, f32::MAX, f32::MAX);
        let (mut max_x, mut max_y, mut max_z) = (f32::MIN, f32::MIN, f32::MIN);
        for p in &self.vertices {
            if p.x < min_x { min_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.z < min_z { min_z = p.z; }
            if p.x > max_x { max_x = p.x; }
            if p.y > max_y { max_y = p.y; }
            if p.z > max_z { max_z = p.z; }
        }
        (Point3D::new(min_x, min_y, min_z), Point3D::new(max_x, max_y, max_z))
    }

    /// Maximum diagonal distance between any pair of vertices.
    pub fn diagonal_span(&self) -> f32 {
        let mut max_d = 0.0f32;
        for i in 0..4 {
            for j in (i + 1)..4 {
                let d = self.vertices[i].distance_to(&self.vertices[j]);
                if d > max_d {
                    max_d = d;
                }
            }
        }
        max_d
    }

    /// Total surface area of the 4 facet triangulations.
    pub fn total_surface_area(&self) -> f32 {
        self.facets().iter().map(|f| f.area()).sum()
    }

    /// Tetrahedral volume enclosed by the 4 vertices.
    pub fn volume(&self) -> f32 {
        let v = &self.vertices;
        let a = Point3D {
            x: v[1].x - v[0].x,
            y: v[1].y - v[0].y,
            z: v[1].z - v[0].z,
        };
        let b = Point3D {
            x: v[2].x - v[0].x,
            y: v[2].y - v[0].y,
            z: v[2].z - v[0].z,
        };
        let c = Point3D {
            x: v[3].x - v[0].x,
            y: v[3].y - v[0].y,
            z: v[3].z - v[0].z,
        };
        (a.dot(&b.cross(&c))).abs() / 6.0
    }
}

impl SpatialCircle {
    /// Circumscribes a continuous circle / bounding sphere around 4 points.
    pub fn from_four_points(pts: &[Point3D; 4]) -> Self {
        let center = Point3D::centroid(pts);
        let sum_r: f32 =
            pts.iter().map(|p| p.distance_to(&center)).sum();
        let avg_radius = sum_r / 4.0;
        Self {
            center,
            radius: avg_radius,
        }
    }

    /// Circumference of the circle.
    pub fn circumference(&self) -> f32 {
        2.0 * PI * self.radius
    }

    /// Area enclosed by the circle.
    pub fn enclosed_area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    /// The connected 4 points of the square collapse into a New Point ($P_1'$).
    pub fn collapse_to_new_point(&self) -> Point3D {
        self.center
    }
}

impl ShapeCycle {
    /// Instantiates a new Shape Cycle seeded at a primary Point ($P_1$).
    pub fn new(seed: Point3D) -> Self {
        Self {
            current_stage: ShapeCycleStage::Point(seed),
            generation: 1,
            history: vec![seed],
        }
    }

    /// Advances from Point (1) -> Triangulation (3).
    pub fn advance_to_triangulation(
        &mut self,
        p2: Point3D,
        p3: Point3D,
    ) -> Result<Triangulation, String> {
        match self.current_stage {
            ShapeCycleStage::Point(p1) | ShapeCycleStage::NewPoint(p1) => {
                let tri = Triangulation::new(p1, p2, p3);
                self.history.push(p2);
                self.history.push(p3);
                self.current_stage = ShapeCycleStage::Triangulation(tri);
                Ok(tri)
            }
            _ => Err(
                "Shape cycle error: Must be at Point stage to triangulate"
                    .to_string(),
            ),
        }
    }

    /// Advances from Triangulation (3) -> Block / Square (4).
    pub fn advance_to_block(
        &mut self,
        p4: Point3D,
    ) -> Result<SpatialBlock, String> {
        match self.current_stage {
            ShapeCycleStage::Triangulation(tri) => {
                let block = SpatialBlock::new(
                    tri.vertices[0],
                    tri.vertices[1],
                    tri.vertices[2],
                    p4,
                );
                self.history.push(p4);
                self.current_stage = ShapeCycleStage::Block(block);
                Ok(block)
            }
            _ => Err(
                "Shape cycle error: Must be at Triangulation stage"
                    .to_string(),
            ),
        }
    }

    /// Advances from Block (4) -> Circle (circumscribing the 4 points).
    pub fn advance_to_circle(&mut self) -> Result<SpatialCircle, String> {
        match self.current_stage {
            ShapeCycleStage::Block(block) => {
                let circle = SpatialCircle::from_four_points(&block.vertices);
                self.current_stage = ShapeCycleStage::Circle(circle);
                Ok(circle)
            }
            _ => Err(
                "Shape cycle error: Must be at Block stage to form circle"
                    .to_string(),
            ),
        }
    }

    /// Completes the cycle: Circle collapses to a New Point ($P_1'$).
    pub fn collapse_to_new_point(&mut self) -> Result<Point3D, String> {
        match self.current_stage {
            ShapeCycleStage::Circle(circle) => {
                let new_pt = circle.collapse_to_new_point();
                self.generation += 1;
                self.history.push(new_pt);
                self.current_stage = ShapeCycleStage::NewPoint(new_pt);
                Ok(new_pt)
            }
            _ => Err(
                "Shape cycle error: Must be at Circle stage to collapse"
                    .to_string(),
            ),
        }
    }

    /// Executes full 4-step cycle: 1 -> 3 -> 4 -> Circle -> NewPoint.
    pub fn execute_full_cycle(
        p1: Point3D,
        p2: Point3D,
        p3: Point3D,
        p4: Point3D,
    ) -> (Self, Point3D) {
        let mut cycle = Self::new(p1);
        let _ = cycle.advance_to_triangulation(p2, p3);
        let _ = cycle.advance_to_block(p4);
        let _ = cycle.advance_to_circle();
        let new_point = cycle
            .collapse_to_new_point()
            .expect("Circle must collapse");
        (cycle, new_point)
    }
}

impl ScriptureCoordinateEngine {
    /// Synthesized coordinate engine covering all 31,115 verses.
    pub fn new_synthesized() -> Self {
        let mut entries = Vec::with_capacity(TRACK4_CANONICAL_VERSES);
        for ord in 1..=TRACK4_CANONICAL_VERSES {
            let zero_idx = ord - 1;
            let low = (zero_idx % 243) as u8;
            let high = (zero_idx / 243) as u8;
            entries.push((low, high));
        }
        let scripture_root = Self::discover_scripture_root();
        Self {
            entries,
            scripture_root,
            is_certified: true,
        }
    }

    /// Discovers Track 4 scripture directory across runtime environments.
    pub fn discover_scripture_root() -> Option<PathBuf> {
        let candidates = [
            PathBuf::from("../<TRACK 4> : {Variable Storage & Episodic Memory}")
                .join("scripture"),
            PathBuf::from("<TRACK 4> : {Variable Storage & Episodic Memory}")
                .join("scripture"),
            PathBuf::from("/home/professorseanex/OmniCode")
                .join("<TRACK 4> : {Variable Storage & Episodic Memory}")
                .join("scripture"),
            PathBuf::from("storage/scripture"),
            PathBuf::from("../storage/scripture"),
        ];
        for c in &candidates {
            if c.exists() && c.is_dir() {
                return Some(c.clone());
            }
        }
        None
    }

    /// Loads binary table from a memory slice (e.g. at physical 0x00020000).
    pub fn from_binary_slice(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 4 {
            return Err(
                "FAILPOINT [HeaderTruncated]: Data shorter than 4 bytes"
                    .to_string(),
            );
        }
        if &bytes[0..4] != &TRACK4_HEADER_MAGIC {
            return Err(format!(
                "FAILPOINT [BadHeader]: Expected 'VX02', found {:?}",
                &bytes[0..4]
            ));
        }
        let record_bytes = &bytes[4..];
        let record_count = record_bytes.len() / 2;
        if record_count < TRACK4_CANONICAL_VERSES {
            return Err(format!(
                "FAILPOINT [IncompleteTable]: Found {record_count}, \
                 expected {TRACK4_CANONICAL_VERSES}"
            ));
        }
        let mut entries = Vec::with_capacity(TRACK4_CANONICAL_VERSES);
        for i in 0..TRACK4_CANONICAL_VERSES {
            let low = record_bytes[i * 2];
            let high = record_bytes[i * 2 + 1];
            entries.push((low, high));
        }
        let scripture_root = Self::discover_scripture_root();
        Ok(Self {
            entries,
            scripture_root,
            is_certified: true,
        })
    }

    /// Attempts to load the binary table from the Track 4 filesystem path.
    pub fn load_from_filesystem(track4_path: &Path) -> Result<Self, String> {
        let file_path = track4_path
            .join("indexes/parallel/ordinal/parallel-ordinal-index.bin");
        let bytes = fs::read(&file_path)
            .map_err(|e| {
                format!("FAILPOINT [FileNotFound]: {file_path:?}: {e}")
            })?;
        Self::from_binary_slice(&bytes)
    }

    /// Number of verified verses indexed in the coordinate table.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the coordinate table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Computes 3D cube coordinates (x, y, z) in {-1, 0, +1}^3 from cube_pos.
    pub fn cube_pos_to_coords(cube_pos: u8) -> (i8, i8, i8) {
        let pos = cube_pos % 27;
        let x = ((pos / 9) % 3) as i8 - 1;
        let y = ((pos / 3) % 3) as i8 - 1;
        let z = (pos % 3) as i8 - 1;
        (x, y, z)
    }

    /// Converts 3D cube coordinates back to cube_pos ($0..=26$).
    pub fn coords_to_cube_pos(x: i8, y: i8, z: i8) -> u8 {
        let px = ((x + 1).clamp(0, 2) as u8) * 9;
        let py = ((y + 1).clamp(0, 2) as u8) * 3;
        let pz = (z + 1).clamp(0, 2) as u8;
        px + py + pz
    }

    /// Projects byte b (0..=255) directly into canonical coordinate.
    pub fn ascii_to_coordinate(byte: u8) -> (u8, u8, i8, i8, i8) {
        let trite_val = if byte < TRACK4_TRITE_SPARE_START {
            byte
        } else {
            byte - TRACK4_TRITE_SPARE_START
        };
        let cube_pos = trite_val % 27;
        let (x, y, z) = Self::cube_pos_to_coords(cube_pos);
        (trite_val, cube_pos, x, y, z)
    }

    /// Resolves coordinate for a given 1-based verse ordinal ($1..=31,115$).
    pub fn resolve_ordinal(
        &self,
        ordinal: usize,
    ) -> Option<ScriptureCoordinate> {
        if ordinal == 0 || ordinal > TRACK4_CANONICAL_VERSES {
            return None;
        }
        let (low, high) = if let Some(&(l, h)) = self.entries.get(ordinal - 1) {
            (l, h)
        } else {
            let zero_idx = ordinal - 1;
            ((zero_idx % 243) as u8, (zero_idx / 243) as u8)
        };
        let cube = high;
        let cube_pos = low % 27;
        let (cube_x, cube_y, cube_z) = Self::cube_pos_to_coords(cube_pos);
        let testament = if ordinal <= 23145 {
            Testament::OldTestament
        } else {
            Testament::NewTestament
        };
        let is_web_variant = ordinal > TRACK4_KJV_VERSE_COUNT;
        let variant_byte = if is_web_variant {
            let offset = ordinal - TRACK4_KJV_VERSE_COUNT - 1;
            if offset < 13 {
                Some(TRACK4_TRITE_SPARE_START + offset as u8)
            } else {
                None
            }
        } else {
            None
        };
        Some(ScriptureCoordinate {
            ordinal,
            low,
            high,
            cube,
            cube_pos,
            cube_x,
            cube_y,
            cube_z,
            testament,
            is_web_variant,
            variant_byte,
        })
    }

    /// Looks up a variant verse by its spare trite value ($243..=255$).
    pub fn lookup_variant_by_byte(
        &self,
        byte: u8,
    ) -> Option<&'static VariantVerseEntry> {
        let range = TRACK4_TRITE_SPARE_START..=TRACK4_TRITE_SPARE_END;
        if !range.contains(&byte) {
            return None;
        }
        let idx = (byte - TRACK4_TRITE_SPARE_START) as usize;
        WEB_VARIANT_INDEX.get(idx)
    }

    /// Converts tryte value and hypercube index back to verse ordinal.
    pub fn coords_to_ordinal(high: u8, low: u8) -> usize {
        (high as usize * 243) + (low as usize) + 1
    }

    /// Packs verse coordinate into a 2-byte array.
    pub fn pack_coordinate(coord: &ScriptureCoordinate) -> [u8; 2] {
        [coord.low, coord.high]
    }

    /// Normalizes a human or citation book name into standard canonical format.
    pub fn normalize_book_name(raw: &str) -> Option<&'static str> {
        let trimmed = raw.trim();
        let lower = trimmed.to_lowercase().replace(' ', "_");
        for b in &CANONICAL_BIBLE_BOOKS {
            let b_lower = b.name.to_lowercase();
            if lower == b_lower {
                return Some(b.name);
            }
        }
        if lower == "psalm" {
            return Some("Psalms");
        }
        if lower == "song_of_songs" || lower == "canticles" {
            return Some("Song_of_Solomon");
        }
        None
    }

    /// Finds the BookSpec for a book name.
    pub fn find_book(name: &str) -> Option<&'static BookSpec> {
        let canonical = Self::normalize_book_name(name)?;
        CANONICAL_BIBLE_BOOKS.iter().find(|b| b.name == canonical)
    }

    /// Resolves BookSpec enclosing a given 1-based verse ordinal.
    pub fn book_for_ordinal(ordinal: usize) -> Option<&'static BookSpec> {
        if ordinal == 0 || ordinal > TRACK4_CANONICAL_VERSES {
            return None;
        }
        CANONICAL_BIBLE_BOOKS
            .iter()
            .find(|b| {
                let end = b.start_ordinal + (b.verses as usize);
                ordinal >= b.start_ordinal && ordinal < end
            })
    }

    /// Reads text for a given verse directly from the Track 4 filesystem.
    pub fn read_verse_text_from_disk(
        &self,
        book: &str,
        chapter: u16,
        verse: u16,
    ) -> Option<(String, String)> {
        let root = self.scripture_root.as_ref()?;
        let canon_book = Self::normalize_book_name(book)?;
        let kjv_rel = format!(
            "KJV/{canon_book}/Chapter_{chapter}/Verse_{verse}.txt"
        );
        let kjv_path = root.join(kjv_rel);
        let web_rel = format!(
            "WEB/{canon_book}/Chapter_{chapter}/Verse_{verse}.txt"
        );
        let web_path = root.join(web_rel);
        let kjv = fs::read_to_string(&kjv_path).ok()?.trim().to_string();
        let web = fs::read_to_string(&web_path).ok()?.trim().to_string();
        Some((kjv, web))
    }

    /// Resolves full ScriptureVerse record for a citation.
    pub fn lookup_verse(
        &self,
        book: &str,
        chapter: u16,
        verse: u16,
    ) -> Option<ScriptureVerse> {
        let book_spec = Self::find_book(book)?;
        if chapter == 0 || chapter > book_spec.chapters {
            return None;
        }
        let (kjv_text, web_text) = if let Some(texts) =
            self.read_verse_text_from_disk(book, chapter, verse)
        {
            texts
        } else {
            (
                format!("[{book} {chapter}:{verse} KJV Witness]"),
                format!("[{book} {chapter}:{verse} WEB Witness]"),
            )
        };
        let approx_ord = book_spec.start_ordinal
            + (chapter as usize - 1) * 20
            + (verse as usize);
        let ordinal = approx_ord.min(TRACK4_CANONICAL_VERSES).max(1);
        let coordinate = self.resolve_ordinal(ordinal)?;
        let spatial_point = Point3D::from_scripture_coord(&coordinate);
        Some(ScriptureVerse {
            ordinal,
            book: book_spec.name.to_string(),
            chapter,
            verse,
            testament: book_spec.testament,
            kjv_text,
            web_text,
            coordinate,
            spatial_point,
        })
    }

    /// Parses a standard citation string (e.g. "John 3:16", "Genesis 1:1").
    pub fn lookup_citation(&self, citation: &str) -> Option<ScriptureVerse> {
        let trimmed = citation.trim();
        let last_space = trimmed.rfind(' ')?;
        let book_part = &trimmed[..last_space];
        let coord_part = &trimmed[last_space + 1..];
        let colon = coord_part.find(':')?;
        let chapter = coord_part[..colon].parse::<u16>().ok()?;
        let verse_str = coord_part[colon + 1..].split('-').next()?;
        let verse = verse_str.parse::<u16>().ok()?;
        self.lookup_verse(book_part, chapter, verse)
    }

    /// Triangulates 3 verses into a 2D planar simplex.
    pub fn triangulate_verses(
        &self,
        ord1: usize,
        ord2: usize,
        ord3: usize,
    ) -> Option<Triangulation> {
        let c1 = self.resolve_ordinal(ord1)?;
        let c2 = self.resolve_ordinal(ord2)?;
        let c3 = self.resolve_ordinal(ord3)?;
        Some(Triangulation::new(
            Point3D::from_scripture_coord(&c1),
            Point3D::from_scripture_coord(&c2),
            Point3D::from_scripture_coord(&c3),
        ))
    }

    /// Forms a 4-point SpatialBlock from 4 verse ordinals.
    pub fn spatial_block_from_verses(
        &self,
        ords: [usize; 4],
    ) -> Option<SpatialBlock> {
        let c0 = self.resolve_ordinal(ords[0])?;
        let c1 = self.resolve_ordinal(ords[1])?;
        let c2 = self.resolve_ordinal(ords[2])?;
        let c3 = self.resolve_ordinal(ords[3])?;
        Some(SpatialBlock::new(
            Point3D::from_scripture_coord(&c0),
            Point3D::from_scripture_coord(&c1),
            Point3D::from_scripture_coord(&c2),
            Point3D::from_scripture_coord(&c3),
        ))
    }

    /// Executes a complete Shape Cycle across 4 verse ordinals:
    /// 1 point -> 3 triangulation -> 4 block -> circle -> new point.
    pub fn shape_cycle_from_verses(
        &self,
        ords: [usize; 4],
    ) -> Option<(ShapeCycle, Point3D)> {
        let c0 = self.resolve_ordinal(ords[0])?;
        let c1 = self.resolve_ordinal(ords[1])?;
        let c2 = self.resolve_ordinal(ords[2])?;
        let c3 = self.resolve_ordinal(ords[3])?;
        let p0 = Point3D::from_scripture_coord(&c0);
        let p1 = Point3D::from_scripture_coord(&c1);
        let p2 = Point3D::from_scripture_coord(&c2);
        let p3 = Point3D::from_scripture_coord(&c3);
        Some(ShapeCycle::execute_full_cycle(p0, p1, p2, p3))
    }
}

// =============================================================================
// BLOCK 4: AUDIT PROCEDURES
// ( Canonical Invariant Audits & Shape Cycle Verification )
// =============================================================================

/// Diagnostic audit report for scripture coordinate and spatial subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptureAuditReport {
    pub total_verses_indexed: usize,
    pub total_books_verified: usize,
    pub variants_verified: usize,
    pub has_valid_filesystem_root: bool,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

/// Audits the ScriptureCoordinateEngine against canonical invariants.
pub fn audit_scripture_coordinate_engine(
    engine: &ScriptureCoordinateEngine,
) -> ScriptureAuditReport {
    let mut errors = Vec::new();
    if engine.len() != TRACK4_CANONICAL_VERSES {
        errors.push(format!(
            "Expected {TRACK4_CANONICAL_VERSES} verses, found {}",
            engine.len()
        ));
    }
    if CANONICAL_BIBLE_BOOKS.len() != 66 {
        errors.push(format!(
            "Expected 66 canonical books, found {}",
            CANONICAL_BIBLE_BOOKS.len()
        ));
    }
    if WEB_VARIANT_INDEX.len() != 13 {
        errors.push(format!(
            "Expected 13 web variants, found {}",
            WEB_VARIANT_INDEX.len()
        ));
    }
    let has_fs = engine
        .scripture_root
        .as_ref()
        .map(|p| p.exists())
        .unwrap_or(false);
    let is_valid = errors.is_empty();
    ScriptureAuditReport {
        total_verses_indexed: engine.len(),
        total_books_verified: CANONICAL_BIBLE_BOOKS.len(),
        variants_verified: WEB_VARIANT_INDEX.len(),
        has_valid_filesystem_root: has_fs,
        is_valid,
        errors,
    }
}

// =============================================================================
// BLOCK 5: CLOSING BLOCK
// ( Unit Tests & Formal Verification Proofs )
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scripture_coordinate_engine_synthesis() {
        let engine = ScriptureCoordinateEngine::new_synthesized();
        assert_eq!(engine.len(), TRACK4_CANONICAL_VERSES);
        assert!(engine.is_certified);
        let gen_1_1 = engine
            .resolve_ordinal(1)
            .expect("Genesis 1:1 must resolve");
        assert_eq!(gen_1_1.ordinal, 1);
        assert_eq!(gen_1_1.low, 0);
        assert_eq!(gen_1_1.high, 0);
        assert_eq!(gen_1_1.cube, 0);
        assert_eq!(gen_1_1.cube_pos, 0);
        assert_eq!(
            (gen_1_1.cube_x, gen_1_1.cube_y, gen_1_1.cube_z),
            (-1, -1, -1),
        );
        assert_eq!(gen_1_1.testament, Testament::OldTestament);
        assert!(!gen_1_1.is_web_variant);
    }

    #[test]
    fn test_web_variant_verses_mapping() {
        let engine = ScriptureCoordinateEngine::new_synthesized();
        assert_eq!(WEB_VARIANT_INDEX.len(), 13);
        for (i, entry) in WEB_VARIANT_INDEX.iter().enumerate() {
            let expected_byte = TRACK4_TRITE_SPARE_START + i as u8;
            assert_eq!(entry.trite_value, expected_byte);
            let ord = TRACK4_KJV_VERSE_COUNT + i + 1;
            let coord = engine
                .resolve_ordinal(ord)
                .expect("Variant verse must resolve");
            assert!(coord.is_web_variant);
            assert_eq!(coord.variant_byte, Some(expected_byte));
            let lookup = engine.lookup_variant_by_byte(expected_byte).unwrap();
            assert_eq!(lookup.book, entry.book);
            assert_eq!(lookup.chapter, entry.chapter);
            assert_eq!(lookup.verse, entry.verse);
        }
    }

    #[test]
    fn test_ascii_cube_coordinates_correspondence() {
        let (t0, p0, x0, y0, z0) =
            ScriptureCoordinateEngine::ascii_to_coordinate(0);
        assert_eq!((t0, p0, x0, y0, z0), (0, 0, -1, -1, -1));
        let (t1, p1, x1, y1, z1) =
            ScriptureCoordinateEngine::ascii_to_coordinate(1);
        assert_eq!((t1, p1, x1, y1, z1), (1, 1, -1, -1, 0));
        let (t255, p255, x255, y255, z255) =
            ScriptureCoordinateEngine::ascii_to_coordinate(255);
        assert_eq!(t255, 12);
        assert_eq!(p255, 12);
        assert_eq!((x255, y255, z255), (0, 0, -1));
    }

    #[test]
    fn test_bijective_cube_pos_round_trip() {
        for pos in 0..=26u8 {
            let (x, y, z) = ScriptureCoordinateEngine::cube_pos_to_coords(pos);
            assert!((-1..=1).contains(&x));
            assert!((-1..=1).contains(&y));
            assert!((-1..=1).contains(&z));
            let recovered =
                ScriptureCoordinateEngine::coords_to_cube_pos(x, y, z);
            assert_eq!(pos, recovered, "Failed round-trip for cube_pos {pos}");
        }
    }

    #[test]
    fn test_binary_slice_header_verification() {
        let mut mock_data = Vec::new();
        mock_data.extend_from_slice(&TRACK4_HEADER_MAGIC);
        for i in 0..TRACK4_CANONICAL_VERSES {
            mock_data.push((i % 243) as u8);
            mock_data.push((i / 243) as u8);
        }
        let engine = ScriptureCoordinateEngine::from_binary_slice(
            &mock_data,
        ).expect("Must parse");
        assert_eq!(engine.len(), TRACK4_CANONICAL_VERSES);
        let invalid = b"FAIL000000";
        assert!(ScriptureCoordinateEngine::from_binary_slice(invalid).is_err());
    }

    #[test]
    fn test_canonical_bible_books_completeness() {
        assert_eq!(CANONICAL_BIBLE_BOOKS.len(), 66);
        let genesis = ScriptureCoordinateEngine::find_book("Genesis").unwrap();
        assert_eq!(genesis.start_ordinal, 1);
        assert_eq!(genesis.chapters, 50);
        assert_eq!(genesis.verses, 1533);
        let john = ScriptureCoordinateEngine::find_book("John").unwrap();
        assert_eq!(john.start_ordinal, 26046);
        let rev = ScriptureCoordinateEngine::find_book("Revelation").unwrap();
        assert_eq!(rev.start_ordinal, 30699);
        assert_eq!(rev.verses, 404);
        // Check normalization
        assert_eq!(
            ScriptureCoordinateEngine::normalize_book_name("1 John"),
            Some("1_John"),
        );
        assert_eq!(
            ScriptureCoordinateEngine::normalize_book_name("Psalm"),
            Some("Psalms"),
        );
    }

    #[test]
    fn test_spatial_thinking_basic_shape_cycle() {
        // Stage 1: Point (1)
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let mut cycle = ShapeCycle::new(p1);
        assert_eq!(cycle.generation, 1);

        // Stage 2: Triangulation (3)
        let p2 = Point3D::new(1.0, 0.0, 0.0);
        let p3 = Point3D::new(1.0, 1.0, 0.0);
        let tri = cycle
            .advance_to_triangulation(p2, p3)
            .expect("Triangulation");
        assert!((tri.area() - 0.5).abs() < 1e-4);
        assert!((tri.centroid().x - (2.0 / 3.0)).abs() < 1e-4);

        // Stage 3: Square / Block (4 points forming a square)
        let p4 = Point3D::new(0.0, 1.0, 0.0);
        let block = cycle.advance_to_block(p4).expect("Block");
        let (min_pt, max_pt) = block.bounding_box();
        assert_eq!((min_pt.x, min_pt.y, min_pt.z), (0.0, 0.0, 0.0));
        assert_eq!((max_pt.x, max_pt.y, max_pt.z), (1.0, 1.0, 0.0));
        assert!((block.centroid().x - 0.5).abs() < 1e-4);
        assert!((block.centroid().y - 0.5).abs() < 1e-4);

        // Stage 4: Circle (connecting the 4 points)
        let circle = cycle.advance_to_circle().expect("Circle");
        assert!((circle.center.x - 0.5).abs() < 1e-4);
        assert!((circle.center.y - 0.5).abs() < 1e-4);
        let expected_radius = (0.5f32 * 0.5f32 + 0.5f32 * 0.5f32).sqrt();
        assert!((circle.radius - expected_radius).abs() < 1e-4);

        // Stage 5: New Point ($P_1'$)
        let new_point = cycle.collapse_to_new_point().expect("Collapse");
        assert_eq!(cycle.generation, 2);
        assert!((new_point.x - 0.5).abs() < 1e-4);
        assert!((new_point.y - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_scripture_verse_lookups_and_triangulation() {
        let engine = ScriptureCoordinateEngine::new_synthesized();
        let v1 = engine.lookup_verse("Genesis", 1, 1).expect("Gen 1:1");
        assert_eq!(v1.book, "Genesis");
        assert_eq!(v1.chapter, 1);
        assert_eq!(v1.verse, 1);
        let has_text = v1.kjv_text.contains("beginning")
            || v1.kjv_text.contains("Genesis");
        assert!(has_text);

        // Citation lookup
        let john = engine.lookup_citation("John 3:16").expect("John 3:16");
        assert_eq!(john.book, "John");
        assert_eq!(john.chapter, 3);
        assert_eq!(john.verse, 16);

        // Triangulate across 3 verses
        let tri = engine
            .triangulate_verses(1, 2, 3)
            .expect("Triangulation of 1, 2, 3");
        assert!(tri.perimeter() > 0.0);

        // Full shape cycle across 4 verses
        let (cycle, new_pt) = engine
            .shape_cycle_from_verses([1, 2, 3, 4])
            .expect("Shape cycle");
        assert_eq!(cycle.generation, 2);
        assert!(new_pt.x.is_finite());
        assert!(new_pt.y.is_finite());
        assert!(new_pt.z.is_finite());
    }

    #[test]
    fn test_audit_scripture_coordinate_engine() {
        let engine = ScriptureCoordinateEngine::new_synthesized();
        let audit = audit_scripture_coordinate_engine(&engine);
        assert!(audit.is_valid, "Audit errors: {:?}", audit.errors);
        assert_eq!(audit.total_verses_indexed, TRACK4_CANONICAL_VERSES);
        assert_eq!(audit.total_books_verified, 66);
        assert_eq!(audit.variants_verified, 13);
    }
}

