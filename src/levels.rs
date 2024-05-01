use crate::src::enum_map::EnumKey;
use std::ops::Neg;
use strum::EnumCount;
use strum::FromRepr;
use zerocopy::FromZeroes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum ObuMetaType {
    HdrCll = 1,
    HdrMdcv = 2,
    Scalability = 3,
    ItutT32 = 4,
    Timecode = 5,
}

pub type TxfmSize = u8;
pub const N_TX_SIZES: usize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
pub enum BlockLevel {
    #[default]
    Bl128x128 = 0,
    Bl64x64 = 1,
    Bl32x32 = 2,
    Bl16x16 = 3,
    Bl8x8 = 4,
}

impl BlockLevel {
    pub const fn decrease(self) -> Option<Self> {
        match self {
            BlockLevel::Bl8x8 => None,
            BlockLevel::Bl16x16 => Some(BlockLevel::Bl8x8),
            BlockLevel::Bl32x32 => Some(BlockLevel::Bl16x16),
            BlockLevel::Bl64x64 => Some(BlockLevel::Bl32x32),
            BlockLevel::Bl128x128 => Some(BlockLevel::Bl64x64),
        }
    }
}

pub type RectTxfmSize = u8;
pub const N_RECT_TX_SIZES: usize = 19; // TODO(kkysen) symbolicate in Dav1dFrameContext::qm once deduplicated
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;

pub type TxfmType = u8;
pub const N_TX_TYPES_PLUS_LL: usize = 17;
pub const WHT_WHT: TxfmType = 16;
pub const _N_TX_TYPES: usize = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TxClass {
    TwoD,
    H,
    V,
}

pub type IntraPredMode = u8;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: usize = 14; // TODO(kkysen) symbolicate in struct Rav1dIntraPredDSPContext::intra_pred once deduplicated
pub const N_UV_INTRA_PRED_MODES: usize = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: usize = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, Default)]
pub enum InterIntraPredMode {
    #[default]
    Dc = 0,
    Vert = 1,
    Hor = 2,
    Smooth = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, FromRepr, EnumCount)]
pub enum BlockPartition {
    #[default]
    None = 0,
    H = 1,
    V = 2,
    Split = 3,
    TopSplit = 4,
    BottomSplit = 5,
    LeftSplit = 6,
    RightSplit = 7,
    H4 = 8,
    V4 = 9,
}

impl BlockPartition {
    pub const N_SUB8X8_PARTITIONS: usize = 4;
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, FromRepr, EnumCount, FromZeroes)]
pub enum BlockSize {
    Bs128x128 = 0,
    Bs128x64 = 1,
    Bs64x128 = 2,
    Bs64x64 = 3,
    Bs64x32 = 4,
    Bs64x16 = 5,
    Bs32x64 = 6,
    Bs32x32 = 7,
    Bs32x16 = 8,
    Bs32x8 = 9,
    Bs16x64 = 10,
    Bs16x32 = 11,
    Bs16x16 = 12,
    Bs16x8 = 13,
    Bs16x4 = 14,
    Bs8x32 = 15,
    Bs8x16 = 16,
    Bs8x8 = 17,
    Bs8x4 = 18,
    Bs4x16 = 19,
    Bs4x8 = 20,
    Bs4x4 = 21,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumCount, Default)]
pub enum Filter2d {
    #[default] // TODO(kkysen) Maybe temporary.
    Regular8Tap = 0,
    RegularSmooth8Tap = 1,
    RegularSharp8Tap = 2,
    SharpRegular8Tap = 3,
    SharpSmooth8Tap = 4,
    Sharp8Tap = 5,
    SmoothRegular8Tap = 6,
    Smooth8Tap = 7,
    SmoothSharp8Tap = 8,
    Bilinear = 9,
}

impl EnumKey<{ Self::COUNT }> for Filter2d {
    const VALUES: [Self; Self::COUNT] = [
        Self::Regular8Tap,
        Self::RegularSmooth8Tap,
        Self::RegularSharp8Tap,
        Self::SharpRegular8Tap,
        Self::SharpSmooth8Tap,
        Self::Sharp8Tap,
        Self::SmoothRegular8Tap,
        Self::Smooth8Tap,
        Self::SmoothSharp8Tap,
        Self::Bilinear,
    ];

    fn as_usize(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromRepr, EnumCount)]
pub enum MVJoint {
    Zero = 0,
    H = 1,
    V = 2,
    HV = 3,
}

pub type InterPredMode = u8;
pub const _N_INTER_PRED_MODES: usize = 4;
pub const NEWMV: InterPredMode = 3;
pub const GLOBALMV: InterPredMode = 2;
pub const NEARMV: InterPredMode = 1;
pub const NEARESTMV: InterPredMode = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DrlProximity {
    #[default]
    Nearest,
    Nearer,
    Near,
    Nearish,
}

pub type CompInterPredMode = u8;
pub const N_COMP_INTER_PRED_MODES: usize = 8;
pub const NEWMV_NEWMV: CompInterPredMode = 7;
pub const GLOBALMV_GLOBALMV: CompInterPredMode = 6;
pub const NEWMV_NEARMV: CompInterPredMode = 5;
pub const NEARMV_NEWMV: CompInterPredMode = 4;
pub const NEWMV_NEARESTMV: CompInterPredMode = 3;
pub const NEARESTMV_NEWMV: CompInterPredMode = 2;
pub const NEARMV_NEARMV: CompInterPredMode = 1;
pub const NEARESTMV_NEARESTMV: CompInterPredMode = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompInterType {
    WeightedAvg = 1,
    Avg = 2,
    Seg = 3,
    Wedge = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterIntraType {
    Blend,
    Wedge,
}

/// Note that this is legitimately [`Copy`]
/// (unlike other transpiled types that are [`Copy`] due to being from C).
/// This is needed because [`mv`] is used within packed structs like [`refmvs_block`],
/// meaning a reference to [`mv`] cannot always be take,
/// which includes `&self` methods, including autogenerated ones like [`PartialEq::eq`].
///
/// [`refmvs_block`]: crate::src::refmvs::refmvs_block
#[derive(Clone, Copy, PartialEq, Eq, Default, FromZeroes)]
#[repr(C)]
pub struct mv {
    pub y: i16,
    pub x: i16,
}

impl mv {
    pub const ZERO: Self = Self { y: 0, x: 0 };

    pub const INVALID: Self = Self {
        y: i16::MIN,
        x: i16::MIN,
    };

    pub fn is_invalid(self) -> bool {
        self == Self::INVALID
    }

    #[allow(dead_code)]
    pub fn is_valid(self) -> bool {
        !self.is_invalid()
    }
}

impl Neg for mv {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            y: -self.y,
            x: -self.x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, Default)]
pub enum MotionMode {
    #[default]
    Translation = 0,
    Obmc = 1,
    Warp = 2,
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Av1BlockIntra {
    pub y_mode: u8,
    pub uv_mode: u8,
    pub tx: u8,
    pub pal_sz: [u8; 2],
    pub y_angle: i8,
    pub uv_angle: i8,
    pub cfl_alpha: [i8; 2],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Av1BlockInter1d {
    pub mv: [mv; 2],
    pub wedge_idx: u8,
    pub mask_sign: u8,
    pub interintra_mode: InterIntraPredMode,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Av1BlockInter2d {
    pub mv2d: mv,
    pub matrix: [i16; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union Av1BlockInterNd {
    pub one_d: Av1BlockInter1d,
    pub two_d: Av1BlockInter2d,
}

#[derive(Clone)]
#[repr(C)]
pub struct Av1BlockInter {
    pub nd: Av1BlockInterNd,
    pub comp_type: Option<CompInterType>,
    pub inter_mode: u8,
    pub motion_mode: MotionMode,
    pub drl_idx: DrlProximity,
    pub r#ref: [i8; 2],
    pub max_ytx: RectTxfmSize,
    pub filter2d: Filter2d,
    pub interintra_type: Option<InterIntraType>,
    pub tx_split0: u8,
    pub tx_split1: u16,
}

#[repr(C)]
pub enum Av1BlockIntraInter {
    Intra(Av1BlockIntra),
    Inter(Av1BlockInter),
}

impl Av1BlockIntraInter {
    pub const fn inter(&self) -> &Av1BlockInter {
        match self {
            Self::Inter(inter) => inter,
            _ => panic!(),
        }
    }
}

impl Default for Av1BlockIntraInter {
    fn default() -> Self {
        Self::Intra(Default::default())
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Av1Block {
    pub bl: BlockLevel,
    pub bs: u8,
    pub bp: BlockPartition,
    pub seg_id: u8,
    pub skip_mode: u8,
    pub skip: u8,
    pub uvtx: RectTxfmSize,
    pub ii: Av1BlockIntraInter,
}
