use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::align::*;

use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::src::tables::dav1d_partition_type_count;

use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;

use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::src::r#ref::dav1d_ref_create_using_pool;
use crate::src::r#ref::dav1d_ref_dec;
use crate::src::r#ref::Dav1dRef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext {
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub refp: [Dav1dThreadPicture; 7],
    pub cur: Dav1dPicture,
    pub sr_cur: Dav1dThreadPicture,
    pub mvs_ref: *mut Dav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Dav1dRef; 7],
    pub cur_segmap_ref: *mut Dav1dRef,
    pub prev_segmap_ref: *mut Dav1dRef,
    pub cur_segmap: *mut uint8_t,
    pub prev_segmap: *const uint8_t,
    pub refpoc: [libc::c_uint; 7],
    pub refrefpoc: [[libc::c_uint; 7]; 7],
    pub gmv_warp_allowed: [uint8_t; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [libc::c_int; 2],
    pub resize_start: [libc::c_int; 2],
    pub c: *const Dav1dContext,
    pub ts: *mut Dav1dTileState,
    pub n_ts: libc::c_int,
    pub dsp: *const Dav1dDSPContext,
    pub bd_fn: Dav1dFrameContext_bd_fn,
    pub ipred_edge_sz: libc::c_int,
    pub ipred_edge: [*mut libc::c_void; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: libc::c_int,
    pub h4: libc::c_int,
    pub bw: libc::c_int,
    pub bh: libc::c_int,
    pub sb128w: libc::c_int,
    pub sb128h: libc::c_int,
    pub sbh: libc::c_int,
    pub sb_shift: libc::c_int,
    pub sb_step: libc::c_int,
    pub sr_sb128w: libc::c_int,
    pub dq: [[[uint16_t; 2]; 3]; 8],
    pub qm: [[*const uint8_t; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: libc::c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[uint8_t; 7]; 7],
    pub bitdepth_max: libc::c_int,
    pub frame_thread: Dav1dFrameContext_frame_thread,
    pub lf: Dav1dFrameContext_lf,
    pub task_thread: Dav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;
use crate::src::internal::TaskThreadData;

use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dSequenceHeader;

use crate::src::align::Align16;

use crate::src::internal::Dav1dFrameContext_lf;
use crate::src::lf_mask::Av1Filter;
pub type pixel = ();

use crate::src::internal::Dav1dFrameContext_frame_thread;

pub type coef = ();

use crate::src::levels::Av1Block;
use crate::src::refmvs::refmvs_frame;

use crate::src::env::BlockContext;
use crate::src::refmvs::refmvs_temporal_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option<unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> ()>,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}
pub type read_coef_blocks_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> ()>;
use crate::src::levels::BlockSize;
use crate::src::levels::N_BS_SIZES;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext {
    pub c: *const Dav1dContext,
    pub f: *const Dav1dFrameContext,
    pub ts: *mut Dav1dTileState,
    pub bx: libc::c_int,
    pub by: libc::c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: Dav1dTaskContext_cf,
    pub al_pal: [[[[uint16_t; 8]; 3]; 32]; 2],
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub txtp_map: [uint8_t; 1024],
    pub scratch: Dav1dTaskContext_scratch,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: libc::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Dav1dTaskContext_frame_thread,
    pub task_thread: Dav1dTaskContext_task_thread,
}
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::internal::Dav1dTaskContext_task_thread;
use crate::src::levels::Filter2d;

use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::internal::Dav1dTaskContext_scratch;
use crate::src::refmvs::refmvs_tile;

use crate::src::internal::Dav1dTileState;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfContext {
    pub m: CdfModeContext,
    pub kfym: Align32<[[[uint16_t; 16]; 5]; 5]>,
    pub coef: CdfCoefContext,
    pub mv: CdfMvContext,
    pub dmv: CdfMvContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvContext {
    pub comp: [CdfMvComponent; 2],
    pub joint: Align8<[uint16_t; 4]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfMvComponent {
    pub classes: Align32<[uint16_t; 16]>,
    pub class0_fp: Align8<[[uint16_t; 4]; 2]>,
    pub classN_fp: Align8<[uint16_t; 4]>,
    pub class0_hp: Align4<[uint16_t; 2]>,
    pub classN_hp: Align4<[uint16_t; 2]>,
    pub class0: Align4<[uint16_t; 2]>,
    pub classN: Align4<[[uint16_t; 2]; 10]>,
    pub sign: Align4<[uint16_t; 2]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfCoefContext {
    pub eob_bin_16: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_32: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_64: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_128: Align16<[[[uint16_t; 8]; 2]; 2]>,
    pub eob_bin_256: Align32<[[[uint16_t; 16]; 2]; 2]>,
    pub eob_bin_512: Align32<[[uint16_t; 16]; 2]>,
    pub eob_bin_1024: Align32<[[uint16_t; 16]; 2]>,
    pub eob_base_tok: Align8<[[[[uint16_t; 4]; 4]; 2]; 5]>,
    pub base_tok: Align8<[[[[uint16_t; 4]; 41]; 2]; 5]>,
    pub br_tok: Align8<[[[[uint16_t; 4]; 21]; 2]; 4]>,
    pub eob_hi_bit: Align4<[[[[uint16_t; 2]; 11]; 2]; 5]>,
    pub skip: Align4<[[[uint16_t; 2]; 13]; 5]>,
    pub dc_sign: Align4<[[[uint16_t; 2]; 3]; 2]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfModeContext {
    pub y_mode: Align32<[[uint16_t; 16]; 4]>,
    pub uv_mode: Align32<[[[uint16_t; 16]; 13]; 2]>,
    pub wedge_idx: Align32<[[uint16_t; 16]; 9]>,
    pub partition: Align32<[[[uint16_t; 16]; 4]; 5]>,
    pub cfl_alpha: Align32<[[uint16_t; 16]; 6]>,
    pub txtp_inter1: Align32<[[uint16_t; 16]; 2]>,
    pub txtp_inter2: Align32<[uint16_t; 16]>,
    pub txtp_intra1: Align16<[[[uint16_t; 8]; 13]; 2]>,
    pub txtp_intra2: Align16<[[[uint16_t; 8]; 13]; 3]>,
    pub cfl_sign: Align16<[uint16_t; 8]>,
    pub angle_delta: Align16<[[uint16_t; 8]; 8]>,
    pub filter_intra: Align16<[uint16_t; 8]>,
    pub comp_inter_mode: Align16<[[uint16_t; 8]; 8]>,
    pub seg_id: Align16<[[uint16_t; 8]; 3]>,
    pub pal_sz: Align16<[[[uint16_t; 8]; 7]; 2]>,
    pub color_map: Align16<[[[[uint16_t; 8]; 5]; 7]; 2]>,
    pub filter: Align8<[[[uint16_t; 4]; 8]; 2]>,
    pub txsz: Align8<[[[uint16_t; 4]; 3]; 4]>,
    pub motion_mode: Align8<[[uint16_t; 4]; 22]>,
    pub delta_q: Align8<[uint16_t; 4]>,
    pub delta_lf: Align8<[[uint16_t; 4]; 5]>,
    pub interintra_mode: Align8<[[uint16_t; 4]; 4]>,
    pub restore_switchable: Align8<[uint16_t; 4]>,
    pub restore_wiener: Align4<[uint16_t; 2]>,
    pub restore_sgrproj: Align4<[uint16_t; 2]>,
    pub interintra: Align4<[[uint16_t; 2]; 7]>,
    pub interintra_wedge: Align4<[[uint16_t; 2]; 7]>,
    pub txtp_inter3: Align4<[[uint16_t; 2]; 4]>,
    pub use_filter_intra: Align4<[[uint16_t; 2]; 22]>,
    pub newmv_mode: Align4<[[uint16_t; 2]; 6]>,
    pub globalmv_mode: Align4<[[uint16_t; 2]; 2]>,
    pub refmv_mode: Align4<[[uint16_t; 2]; 6]>,
    pub drl_bit: Align4<[[uint16_t; 2]; 3]>,
    pub intra: Align4<[[uint16_t; 2]; 4]>,
    pub comp: Align4<[[uint16_t; 2]; 5]>,
    pub comp_dir: Align4<[[uint16_t; 2]; 5]>,
    pub jnt_comp: Align4<[[uint16_t; 2]; 6]>,
    pub mask_comp: Align4<[[uint16_t; 2]; 6]>,
    pub wedge_comp: Align4<[[uint16_t; 2]; 9]>,
    pub r#ref: Align4<[[[uint16_t; 2]; 3]; 6]>,
    pub comp_fwd_ref: Align4<[[[uint16_t; 2]; 3]; 3]>,
    pub comp_bwd_ref: Align4<[[[uint16_t; 2]; 3]; 2]>,
    pub comp_uni_ref: Align4<[[[uint16_t; 2]; 3]; 3]>,
    pub txpart: Align4<[[[uint16_t; 2]; 3]; 7]>,
    pub skip: Align4<[[uint16_t; 2]; 3]>,
    pub skip_mode: Align4<[[uint16_t; 2]; 3]>,
    pub seg_pred: Align4<[[uint16_t; 2]; 3]>,
    pub obmc: Align4<[[uint16_t; 2]; 22]>,
    pub pal_y: Align4<[[[uint16_t; 2]; 3]; 7]>,
    pub pal_uv: Align4<[[uint16_t; 2]; 2]>,
    pub intrabc: Align4<[uint16_t; 2]>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext {
    pub fc: *mut Dav1dFrameContext,
    pub n_fc: libc::c_uint,
    pub tc: *mut Dav1dTaskContext,
    pub n_tc: libc::c_uint,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub n_tiles: libc::c_int,
    pub seq_hdr_pool: *mut Dav1dMemPool,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_pool: *mut Dav1dMemPool,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub content_light_ref: *mut Dav1dRef,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display_ref: *mut Dav1dRef,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35_ref: *mut Dav1dRef,
    pub itut_t35: *mut Dav1dITUTT35,
    pub in_0: Dav1dData,
    pub out: Dav1dThreadPicture,
    pub cache: Dav1dThreadPicture,
    pub flush_mem: atomic_int,
    pub flush: *mut atomic_int,
    pub frame_thread: Dav1dContext_frame_thread,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [Dav1dContext_refs; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub intra_edge: Dav1dContext_intra_edge,
    pub allocator: Dav1dPicAllocator,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub operating_point_idc: libc::c_uint,
    pub all_layers: libc::c_int,
    pub max_spatial_id: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub drain: libc::c_int,
    pub frame_flags: PictureFlags,
    pub event_flags: Dav1dEventFlags,
    pub cached_error_props: Dav1dDataProps,
    pub cached_error: libc::c_int,
    pub logger: Dav1dLogger,
    pub picture_pool: *mut Dav1dMemPool,
}
use crate::src::mem::Dav1dMemPool;

use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::src::picture::PictureFlags;

use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;

use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::internal::Dav1dContext_intra_edge;

use crate::src::intra_edge::EdgeFlags;
use crate::src::refmvs::Dav1dRefmvsDSPContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDSPContext {
    pub fg: Dav1dFilmGrainDSPContext,
    pub ipred: Dav1dIntraPredDSPContext,
    pub mc: Dav1dMCDSPContext,
    pub itx: Dav1dInvTxfmDSPContext,
    pub lf: Dav1dLoopFilterDSPContext,
    pub cdef: Dav1dCdefDSPContext,
    pub lr: Dav1dLoopRestorationDSPContext,
}
use crate::src::cdef::Dav1dCdefDSPContext;
use crate::src::loopfilter::Dav1dLoopFilterDSPContext;
use crate::src::looprestoration::Dav1dLoopRestorationDSPContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut libc::c_void, ptrdiff_t, *mut libc::c_void, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_avg_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type entry = int8_t;
pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CdfThreadContext {
    pub r#ref: *mut Dav1dRef,
    pub data: CdfThreadContext_data,
    pub progress: *mut atomic_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union CdfThreadContext_data {
    pub cdf: *mut CdfContext,
    pub qcat: libc::c_uint,
}
use crate::src::internal::Dav1dContext_frame_thread;
use crate::src::internal::Dav1dContext_refs;
use crate::src::internal::Dav1dTileGroup;
use crate::src::picture::Dav1dThreadPicture;
pub type backup_ipred_edge_fn = Option<unsafe extern "C" fn(*mut Dav1dTaskContext) -> ()>;
pub type filter_sbrow_fn = Option<unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> ()>;
pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;
use crate::src::internal::ScalableMotionParams;

use crate::src::levels::N_BL_LEVELS;
use crate::src::levels::N_TX_SIZES;
use crate::src::levels::N_UV_INTRA_PRED_MODES;

use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_MV_JOINTS;

use crate::include::common::intops::imin;
use crate::src::levels::N_COMP_INTER_PRED_MODES;
use crate::src::r#ref::dav1d_ref_inc;
const fn cdf0d<const P: usize, const N: usize>(probs: [u16; P]) -> [u16; N] {
    assert!(P < N);
    let mut cdf0d = [0; N];
    let mut i = 0;
    while i < P {
        cdf0d[i] = (32768 - probs[i]) & !32768;
        i += 1;
    }
    cdf0d
}
const fn cdf1d<const P: usize, const N: usize, const M: usize>(
    probs: [[u16; P]; M],
) -> [[u16; N]; M] {
    let mut cdf1d = [[0; N]; M];
    let mut i = 0;
    while i < M {
        cdf1d[i] = cdf0d(probs[i]);
        i += 1;
    }
    cdf1d
}
const fn cdf2d<const P: usize, const N: usize, const M: usize, const L: usize>(
    probs: [[[u16; P]; M]; L],
) -> [[[u16; N]; M]; L] {
    let mut cdf2d = [[[0; N]; M]; L];
    let mut i = 0;
    while i < L {
        cdf2d[i] = cdf1d(probs[i]);
        i += 1;
    }
    cdf2d
}
const fn cdf3d<const P: usize, const N: usize, const M: usize, const L: usize, const K: usize>(
    probs: [[[[u16; P]; M]; L]; K],
) -> [[[[u16; N]; M]; L]; K] {
    let mut cdf3d = [[[[0; N]; M]; L]; K];
    let mut i = 0;
    while i < K {
        cdf3d[i] = cdf2d(probs[i]);
        i += 1;
    }
    cdf3d
}
static av1_default_cdf: CdfModeContext = CdfModeContext {
    y_mode: Align32(cdf1d([
        [
            22801, 23489, 24293, 24756, 25601, 26123, 26606, 27418, 27945, 29228, 29685, 30349,
        ],
        [
            18673, 19845, 22631, 23318, 23950, 24649, 25527, 27364, 28152, 29701, 29984, 30852,
        ],
        [
            19770, 20979, 23396, 23939, 24241, 24654, 25136, 27073, 27830, 29360, 29730, 30659,
        ],
        [
            20155, 21301, 22838, 23178, 23261, 23533, 23703, 24804, 25352, 26575, 27016, 28049,
        ],
    ])),
    uv_mode: Align32([
        cdf1d([
            [
                22631, 24152, 25378, 25661, 25986, 26520, 27055, 27923, 28244, 30059, 30941, 31961,
            ],
            [
                9513, 26881, 26973, 27046, 27118, 27664, 27739, 27824, 28359, 29505, 29800, 31796,
            ],
            [
                9845, 9915, 28663, 28704, 28757, 28780, 29198, 29822, 29854, 30764, 31777, 32029,
            ],
            [
                13639, 13897, 14171, 25331, 25606, 25727, 25953, 27148, 28577, 30612, 31355, 32493,
            ],
            [
                9764, 9835, 9930, 9954, 25386, 27053, 27958, 28148, 28243, 31101, 31744, 32363,
            ],
            [
                11825, 13589, 13677, 13720, 15048, 29213, 29301, 29458, 29711, 31161, 31441, 32550,
            ],
            [
                14175, 14399, 16608, 16821, 17718, 17775, 28551, 30200, 30245, 31837, 32342, 32667,
            ],
            [
                12885, 13038, 14978, 15590, 15673, 15748, 16176, 29128, 29267, 30643, 31961, 32461,
            ],
            [
                12026, 13661, 13874, 15305, 15490, 15726, 15995, 16273, 28443, 30388, 30767, 32416,
            ],
            [
                19052, 19840, 20579, 20916, 21150, 21467, 21885, 22719, 23174, 28861, 30379, 32175,
            ],
            [
                18627, 19649, 20974, 21219, 21492, 21816, 22199, 23119, 23527, 27053, 31397, 32148,
            ],
            [
                17026, 19004, 19997, 20339, 20586, 21103, 21349, 21907, 22482, 25896, 26541, 31819,
            ],
            [
                12124, 13759, 14959, 14992, 15007, 15051, 15078, 15166, 15255, 15753, 16039, 16606,
            ],
        ]),
        cdf1d([
            [
                10407, 11208, 12900, 13181, 13823, 14175, 14899, 15656, 15986, 20086, 20995, 22455,
                24212,
            ],
            [
                4532, 19780, 20057, 20215, 20428, 21071, 21199, 21451, 22099, 24228, 24693, 27032,
                29472,
            ],
            [
                5273, 5379, 20177, 20270, 20385, 20439, 20949, 21695, 21774, 23138, 24256, 24703,
                26679,
            ],
            [
                6740, 7167, 7662, 14152, 14536, 14785, 15034, 16741, 18371, 21520, 22206, 23389,
                24182,
            ],
            [
                4987, 5368, 5928, 6068, 19114, 20315, 21857, 22253, 22411, 24911, 25380, 26027,
                26376,
            ],
            [
                5370, 6889, 7247, 7393, 9498, 21114, 21402, 21753, 21981, 24780, 25386, 26517,
                27176,
            ],
            [
                4816, 4961, 7204, 7326, 8765, 8930, 20169, 20682, 20803, 23188, 23763, 24455, 24940,
            ],
            [
                6608, 6740, 8529, 9049, 9257, 9356, 9735, 18827, 19059, 22336, 23204, 23964, 24793,
            ],
            [
                5998, 7419, 7781, 8933, 9255, 9549, 9753, 10417, 18898, 22494, 23139, 24764, 25989,
            ],
            [
                10660, 11298, 12550, 12957, 13322, 13624, 14040, 15004, 15534, 20714, 21789, 23443,
                24861,
            ],
            [
                10522, 11530, 12552, 12963, 13378, 13779, 14245, 15235, 15902, 20102, 22696, 23774,
                25838,
            ],
            [
                10099, 10691, 12639, 13049, 13386, 13665, 14125, 15163, 15636, 19676, 20474, 23519,
                25208,
            ],
            [
                3144, 5087, 7382, 7504, 7593, 7690, 7801, 8064, 8232, 9248, 9875, 10521, 29048,
            ],
        ]),
    ]),
    wedge_idx: Align32(cdf1d([
        [
            2438, 4440, 6599, 8663, 11005, 12874, 15751, 18094, 20359, 22362, 24127, 25702, 27752,
            29450, 31171,
        ],
        [
            806, 3266, 6005, 6738, 7218, 7367, 7771, 14588, 16323, 17367, 18452, 19422, 22839,
            26127, 29629,
        ],
        [
            2779, 3738, 4683, 7213, 7775, 8017, 8655, 14357, 17939, 21332, 24520, 27470, 29456,
            30529, 31656,
        ],
        [
            1684, 3625, 5675, 7108, 9302, 11274, 14429, 17144, 19163, 20961, 22884, 24471, 26719,
            28714, 30877,
        ],
        [
            1142, 3491, 6277, 7314, 8089, 8355, 9023, 13624, 15369, 16730, 18114, 19313, 22521,
            26012, 29550,
        ],
        [
            2742, 4195, 5727, 8035, 8980, 9336, 10146, 14124, 17270, 20533, 23434, 25972, 27944,
            29570, 31416,
        ],
        [
            1727, 3948, 6101, 7796, 9841, 12344, 15766, 18944, 20638, 22038, 23963, 25311, 26988,
            28766, 31012,
        ],
        [
            154, 987, 1925, 2051, 2088, 2111, 2151, 23033, 23703, 24284, 24985, 25684, 27259,
            28883, 30911,
        ],
        [
            1135, 1322, 1493, 2635, 2696, 2737, 2770, 21016, 22935, 25057, 27251, 29173, 30089,
            30960, 31933,
        ],
    ])),
    partition: Align32([
        cdf1d([
            [27899, 28219, 28529, 32484, 32539, 32619, 32639],
            [6607, 6990, 8268, 32060, 32219, 32338, 32371],
            [5429, 6676, 7122, 32027, 32227, 32531, 32582],
            [711, 966, 1172, 32448, 32538, 32617, 32664],
        ]),
        cdf1d([
            [
                20137, 21547, 23078, 29566, 29837, 30261, 30524, 30892, 31724,
            ],
            [6732, 7490, 9497, 27944, 28250, 28515, 28969, 29630, 30104],
            [5945, 7663, 8348, 28683, 29117, 29749, 30064, 30298, 32238],
            [870, 1212, 1487, 31198, 31394, 31574, 31743, 31881, 32332],
        ]),
        cdf1d([
            [
                18462, 20920, 23124, 27647, 28227, 29049, 29519, 30178, 31544,
            ],
            [7689, 9060, 12056, 24992, 25660, 26182, 26951, 28041, 29052],
            [6015, 9009, 10062, 24544, 25409, 26545, 27071, 27526, 32047],
            [1394, 2208, 2796, 28614, 29061, 29466, 29840, 30185, 31899],
        ]),
        cdf1d([
            [
                15597, 20929, 24571, 26706, 27664, 28821, 29601, 30571, 31902,
            ],
            [7925, 11043, 16785, 22470, 23971, 25043, 26651, 28701, 29834],
            [5414, 13269, 15111, 20488, 22360, 24500, 25537, 26336, 32117],
            [2662, 6362, 8614, 20860, 23053, 24778, 26436, 27829, 31171],
        ]),
        cdf1d([
            [19132, 25510, 30392],
            [13928, 19855, 28540],
            [12522, 23679, 28629],
            [9896, 18783, 25853],
        ]),
    ]),
    cfl_alpha: Align32(cdf1d([
        [
            7637, 20719, 31401, 32481, 32657, 32688, 32692, 32696, 32700, 32704, 32708, 32712,
            32716, 32720, 32724,
        ],
        [
            14365, 23603, 28135, 31168, 32167, 32395, 32487, 32573, 32620, 32647, 32668, 32672,
            32676, 32680, 32684,
        ],
        [
            11532, 22380, 28445, 31360, 32349, 32523, 32584, 32649, 32673, 32677, 32681, 32685,
            32689, 32693, 32697,
        ],
        [
            26990, 31402, 32282, 32571, 32692, 32696, 32700, 32704, 32708, 32712, 32716, 32720,
            32724, 32728, 32732,
        ],
        [
            17248, 26058, 28904, 30608, 31305, 31877, 32126, 32321, 32394, 32464, 32516, 32560,
            32576, 32593, 32622,
        ],
        [
            14738, 21678, 25779, 27901, 29024, 30302, 30980, 31843, 32144, 32413, 32520, 32594,
            32622, 32656, 32660,
        ],
    ])),
    txtp_inter1: Align32(cdf1d([
        [
            4458, 5560, 7695, 9709, 13330, 14789, 17537, 20266, 21504, 22848, 23934, 25474, 27727,
            28915, 30631,
        ],
        [
            1645, 2573, 4778, 5711, 7807, 8622, 10522, 15357, 17674, 20408, 22517, 25010, 27116,
            28856, 30749,
        ],
    ])),
    txtp_inter2: Align32(cdf0d([
        770, 2421, 5225, 12907, 15819, 18927, 21561, 24089, 26595, 28526, 30529,
    ])),
    txtp_intra1: Align16(cdf2d([
        [
            [1535, 8035, 9461, 12751, 23467, 27825],
            [564, 3335, 9709, 10870, 18143, 28094],
            [672, 3247, 3676, 11982, 19415, 23127],
            [5279, 13885, 15487, 18044, 23527, 30252],
            [4423, 6074, 7985, 10416, 25693, 29298],
            [1486, 4241, 9460, 10662, 16456, 27694],
            [439, 2838, 3522, 6737, 18058, 23754],
            [1190, 4233, 4855, 11670, 20281, 24377],
            [1045, 4312, 8647, 10159, 18644, 29335],
            [202, 3734, 4747, 7298, 17127, 24016],
            [447, 4312, 6819, 8884, 16010, 23858],
            [277, 4369, 5255, 8905, 16465, 22271],
            [3409, 5436, 10599, 15599, 19687, 24040],
        ],
        [
            [1870, 13742, 14530, 16498, 23770, 27698],
            [326, 8796, 14632, 15079, 19272, 27486],
            [484, 7576, 7712, 14443, 19159, 22591],
            [1126, 15340, 15895, 17023, 20896, 30279],
            [655, 4854, 5249, 5913, 22099, 27138],
            [1299, 6458, 8885, 9290, 14851, 25497],
            [311, 5295, 5552, 6885, 16107, 22672],
            [883, 8059, 8270, 11258, 17289, 21549],
            [741, 7580, 9318, 10345, 16688, 29046],
            [110, 7406, 7915, 9195, 16041, 23329],
            [363, 7974, 9357, 10673, 15629, 24474],
            [153, 7647, 8112, 9936, 15307, 19996],
            [3511, 6332, 11165, 15335, 19323, 23594],
        ],
    ])),
    txtp_intra2: Align16(cdf2d([
        [
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
        ],
        [
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
            [6554, 13107, 19661, 26214],
        ],
        [
            [1127, 12814, 22772, 27483],
            [145, 6761, 11980, 26667],
            [362, 5887, 11678, 16725],
            [385, 15213, 18587, 30693],
            [25, 2914, 23134, 27903],
            [60, 4470, 11749, 23991],
            [37, 3332, 14511, 21448],
            [157, 6320, 13036, 17439],
            [119, 6719, 12906, 29396],
            [47, 5537, 12576, 21499],
            [269, 6076, 11258, 23115],
            [83, 5615, 12001, 17228],
            [1968, 5556, 12023, 18547],
        ],
    ])),
    cfl_sign: Align16(cdf0d([1418, 2123, 13340, 18405, 26972, 28343, 32294])),
    angle_delta: Align16(cdf1d([
        [2180, 5032, 7567, 22776, 26989, 30217],
        [2301, 5608, 8801, 23487, 26974, 30330],
        [3780, 11018, 13699, 19354, 23083, 31286],
        [4581, 11226, 15147, 17138, 21834, 28397],
        [1737, 10927, 14509, 19588, 22745, 28823],
        [2664, 10176, 12485, 17650, 21600, 30495],
        [2240, 11096, 15453, 20341, 22561, 28917],
        [3605, 10428, 12459, 17676, 21244, 30655],
    ])),
    filter_intra: Align16(cdf0d([8949, 12776, 17211, 29558])),
    comp_inter_mode: Align16(cdf1d([
        [7760, 13823, 15808, 17641, 19156, 20666, 26891],
        [10730, 19452, 21145, 22749, 24039, 25131, 28724],
        [10664, 20221, 21588, 22906, 24295, 25387, 28436],
        [13298, 16984, 20471, 24182, 25067, 25736, 26422],
        [18904, 23325, 25242, 27432, 27898, 28258, 30758],
        [10725, 17454, 20124, 22820, 24195, 25168, 26046],
        [17125, 24273, 25814, 27492, 28214, 28704, 30592],
        [13046, 23214, 24505, 25942, 27435, 28442, 29330],
    ])),
    seg_id: Align16(cdf1d([
        [5622, 7893, 16093, 18233, 27809, 28373, 32533],
        [14274, 18230, 22557, 24935, 29980, 30851, 32344],
        [27527, 28487, 28723, 28890, 32397, 32647, 32679],
    ])),
    pal_sz: Align16(cdf2d([
        [
            [7952, 13000, 18149, 21478, 25527, 29241],
            [7139, 11421, 16195, 19544, 23666, 28073],
            [7788, 12741, 17325, 20500, 24315, 28530],
            [8271, 14064, 18246, 21564, 25071, 28533],
            [12725, 19180, 21863, 24839, 27535, 30120],
            [9711, 14888, 16923, 21052, 25661, 27875],
            [14940, 20797, 21678, 24186, 27033, 28999],
        ],
        [
            [8713, 19979, 27128, 29609, 31331, 32272],
            [5839, 15573, 23581, 26947, 29848, 31700],
            [4426, 11260, 17999, 21483, 25863, 29430],
            [3228, 9464, 14993, 18089, 22523, 27420],
            [3768, 8886, 13091, 17852, 22495, 27207],
            [2464, 8451, 12861, 21632, 25525, 28555],
            [1269, 5435, 10433, 18963, 21700, 25865],
        ],
    ])),
    color_map: Align16([
        [
            cdf1d([[28710], [16384], [10553], [27036], [31603]]),
            cdf1d([
                [27877, 30490],
                [11532, 25697],
                [6544, 30234],
                [23018, 28072],
                [31915, 32385],
            ]),
            cdf1d([
                [25572, 28046, 30045],
                [9478, 21590, 27256],
                [7248, 26837, 29824],
                [19167, 24486, 28349],
                [31400, 31825, 32250],
            ]),
            cdf1d([
                [24779, 26955, 28576, 30282],
                [8669, 20364, 24073, 28093],
                [4255, 27565, 29377, 31067],
                [19864, 23674, 26716, 29530],
                [31646, 31893, 32147, 32426],
            ]),
            cdf1d([
                [23132, 25407, 26970, 28435, 30073],
                [7443, 17242, 20717, 24762, 27982],
                [6300, 24862, 26944, 28784, 30671],
                [18916, 22895, 25267, 27435, 29652],
                [31270, 31550, 31808, 32059, 32353],
            ]),
            cdf1d([
                [23105, 25199, 26464, 27684, 28931, 30318],
                [6950, 15447, 18952, 22681, 25567, 28563],
                [7560, 23474, 25490, 27203, 28921, 30708],
                [18544, 22373, 24457, 26195, 28119, 30045],
                [31198, 31451, 31670, 31882, 32123, 32391],
            ]),
            cdf1d([
                [21689, 23883, 25163, 26352, 27506, 28827, 30195],
                [6892, 15385, 17840, 21606, 24287, 26753, 29204],
                [5651, 23182, 25042, 26518, 27982, 29392, 30900],
                [19349, 22578, 24418, 25994, 27524, 29031, 30448],
                [31028, 31270, 31504, 31705, 31927, 32153, 32392],
            ]),
        ],
        [
            cdf1d([[29089], [16384], [8713], [29257], [31610]]),
            cdf1d([
                [25257, 29145],
                [12287, 27293],
                [7033, 27960],
                [20145, 25405],
                [30608, 31639],
            ]),
            cdf1d([
                [24210, 27175, 29903],
                [9888, 22386, 27214],
                [5901, 26053, 29293],
                [18318, 22152, 28333],
                [30459, 31136, 31926],
            ]),
            cdf1d([
                [22980, 25479, 27781, 29986],
                [8413, 21408, 24859, 28874],
                [2257, 29449, 30594, 31598],
                [19189, 21202, 25915, 28620],
                [31844, 32044, 32281, 32518],
            ]),
            cdf1d([
                [22217, 24567, 26637, 28683, 30548],
                [7307, 16406, 19636, 24632, 28424],
                [4441, 25064, 26879, 28942, 30919],
                [17210, 20528, 23319, 26750, 29582],
                [30674, 30953, 31396, 31735, 32207],
            ]),
            cdf1d([
                [21239, 23168, 25044, 26962, 28705, 30506],
                [6545, 15012, 18004, 21817, 25503, 28701],
                [3448, 26295, 27437, 28704, 30126, 31442],
                [15889, 18323, 21704, 24698, 26976, 29690],
                [30988, 31204, 31479, 31734, 31983, 32325],
            ]),
            cdf1d([
                [21442, 23288, 24758, 26246, 27649, 28980, 30563],
                [5863, 14933, 17552, 20668, 23683, 26411, 29273],
                [3415, 25810, 26877, 27990, 29223, 30394, 31618],
                [17965, 20084, 22232, 23974, 26274, 28402, 30390],
                [31190, 31329, 31516, 31679, 31825, 32026, 32322],
            ]),
        ],
    ]),
    filter: Align8(cdf2d([
        [
            [31935, 32720],
            [5568, 32719],
            [422, 2938],
            [28244, 32608],
            [31206, 31953],
            [4862, 32121],
            [770, 1152],
            [20889, 25637],
        ],
        [
            [31910, 32724],
            [4120, 32712],
            [305, 2247],
            [27403, 32636],
            [31022, 32009],
            [2963, 32093],
            [601, 943],
            [14969, 21398],
        ],
    ])),
    txsz: Align8(cdf2d([
        [[19968, 0], [19968, 0], [24320, 0]],
        [[12272, 30172], [12272, 30172], [18677, 30848]],
        [[12986, 15180], [12986, 15180], [24302, 25602]],
        [[5782, 11475], [5782, 11475], [16803, 22759]],
    ])),
    motion_mode: Align8(cdf1d([
        [32507, 32558],
        [30878, 31335],
        [28898, 30397],
        [29516, 30701],
        [21679, 26830],
        [29742, 31203],
        [20360, 28062],
        [26260, 29116],
        [11606, 24308],
        [26431, 30774],
        [28973, 31594],
        [5123, 23606],
        [19419, 26810],
        [5391, 25528],
        [0; 2],
        [28799, 31390],
        [4738, 24765],
        [7651, 24760],
        [0; 2],
        [0; 2],
        [0; 2],
        [0; 2],
    ])),
    delta_q: Align8(cdf0d([28160, 32120, 32677])),
    delta_lf: Align8(cdf1d([
        [28160, 32120, 32677],
        [28160, 32120, 32677],
        [28160, 32120, 32677],
        [28160, 32120, 32677],
        [28160, 32120, 32677],
    ])),
    interintra_mode: Align8(cdf1d([
        [8192, 16384, 24576],
        [1875, 11082, 27332],
        [2473, 9996, 26388],
        [4238, 11537, 25926],
    ])),
    restore_switchable: Align8(cdf0d([9413, 22581])),
    restore_wiener: Align4(cdf0d([11570])),
    restore_sgrproj: Align4(cdf0d([16855])),
    interintra: Align4(cdf1d([[16384], [26887], [27597], [30237], [0], [0], [0]])),
    interintra_wedge: Align4(cdf1d([
        [20036],
        [24957],
        [26704],
        [27530],
        [29564],
        [29444],
        [26872],
    ])),
    txtp_inter3: Align4(cdf1d([[16384], [4167], [1998], [748]])),
    use_filter_intra: Align4(cdf1d([
        [16384],
        [16384],
        [16384],
        [16384],
        [16384],
        [16384],
        [16384],
        [22343],
        [12756],
        [18101],
        [16384],
        [14301],
        [12408],
        [9394],
        [10368],
        [20229],
        [12551],
        [7866],
        [5893],
        [12770],
        [6743],
        [4621],
    ])),
    newmv_mode: Align4(cdf1d([[24035], [16630], [15339], [8386], [12222], [4676]])),
    globalmv_mode: Align4(cdf1d([[2175], [1054]])),
    refmv_mode: Align4(cdf1d([
        [23974],
        [24188],
        [17848],
        [28622],
        [24312],
        [19923],
    ])),
    drl_bit: Align4(cdf1d([[13104], [24560], [18945]])),
    intra: Align4(cdf1d([[806], [16662], [20186], [26538]])),
    comp: Align4(cdf1d([[26828], [24035], [12031], [10640], [2901]])),
    comp_dir: Align4(cdf1d([[1198], [2070], [9166], [7499], [22475]])),
    jnt_comp: Align4(cdf1d([[18244], [12865], [7053], [13259], [9334], [4644]])),
    mask_comp: Align4(cdf1d([
        [26607],
        [22891],
        [18840],
        [24594],
        [19934],
        [22674],
    ])),
    wedge_comp: Align4(cdf1d([
        [23431],
        [13171],
        [11470],
        [9770],
        [9100],
        [8233],
        [6172],
        [11820],
        [7701],
    ])),
    r#ref: Align4(cdf2d([
        [[4897], [16973], [29744]],
        [[1555], [16751], [30279]],
        [[4236], [19647], [31194]],
        [[8650], [24773], [31895]],
        [[904], [11014], [26875]],
        [[1444], [15087], [30304]],
    ])),
    comp_fwd_ref: Align4(cdf2d([
        [[4946], [19891], [30731]],
        [[9468], [22441], [31059]],
        [[1503], [15160], [27544]],
    ])),
    comp_bwd_ref: Align4(cdf2d([
        [[2235], [17182], [30606]],
        [[1423], [15175], [30489]],
    ])),
    comp_uni_ref: Align4(cdf2d([
        [[5284], [23152], [31774]],
        [[3865], [14173], [25120]],
        [[3128], [15270], [26710]],
    ])),
    txpart: Align4(cdf2d([
        [[28581], [23846], [20847]],
        [[24315], [18196], [12133]],
        [[18791], [10887], [11005]],
        [[27179], [20004], [11281]],
        [[26549], [19308], [14224]],
        [[28015], [21546], [14400]],
        [[28165], [22401], [16088]],
    ])),
    skip: Align4(cdf1d([[31671], [16515], [4576]])),
    skip_mode: Align4(cdf1d([[32621], [20708], [8127]])),
    seg_pred: Align4(cdf1d([[16384], [16384], [16384]])),
    obmc: Align4(cdf1d([
        [32638],
        [31560],
        [31014],
        [30128],
        [22083],
        [26879],
        [22823],
        [25817],
        [15142],
        [20901],
        [24008],
        [14423],
        [17432],
        [9301],
        [0],
        [23664],
        [9371],
        [10437],
        [0],
        [0],
        [0],
        [0],
    ])),
    pal_y: Align4(cdf2d([
        [[31676], [3419], [1261]],
        [[31912], [2859], [980]],
        [[31823], [3400], [781]],
        [[32030], [3561], [904]],
        [[32309], [7337], [1462]],
        [[32265], [4015], [1521]],
        [[32450], [7946], [129]],
    ])),
    pal_uv: Align4(cdf1d([[32461], [21488]])),
    intrabc: Align4(cdf0d([30531])),
};
static default_mv_component_cdf: CdfMvComponent = CdfMvComponent {
    classes: Align32(cdf0d([
        28672, 30976, 31858, 32320, 32551, 32656, 32740, 32757, 32762, 32767,
    ])),
    class0_fp: Align8(cdf1d([[16384, 24576, 26624], [12288, 21248, 24128]])),
    classN_fp: Align8(cdf0d([8192, 17408, 21248])),
    class0_hp: Align4(cdf0d([20480])),
    classN_hp: Align4(cdf0d([16384])),
    class0: Align4(cdf0d([27648])),
    classN: Align4(cdf1d([
        [17408],
        [17920],
        [18944],
        [20480],
        [22528],
        [24576],
        [28672],
        [29952],
        [29952],
        [30720],
    ])),
    sign: Align4(cdf0d([16384])),
};
static default_mv_joint_cdf: Align8<[uint16_t; 4]> = Align8(cdf0d([4096, 11264, 19328]));
static default_kf_y_mode_cdf: Align32<[[[uint16_t; 16]; 5]; 5]> = Align32(cdf2d([
    [
        [
            15588, 17027, 19338, 20218, 20682, 21110, 21825, 23244, 24189, 28165, 29093, 30466,
        ],
        [
            12016, 18066, 19516, 20303, 20719, 21444, 21888, 23032, 24434, 28658, 30172, 31409,
        ],
        [
            10052, 10771, 22296, 22788, 23055, 23239, 24133, 25620, 26160, 29336, 29929, 31567,
        ],
        [
            14091, 15406, 16442, 18808, 19136, 19546, 19998, 22096, 24746, 29585, 30958, 32462,
        ],
        [
            12122, 13265, 15603, 16501, 18609, 20033, 22391, 25583, 26437, 30261, 31073, 32475,
        ],
    ],
    [
        [
            10023, 19585, 20848, 21440, 21832, 22760, 23089, 24023, 25381, 29014, 30482, 31436,
        ],
        [
            5983, 24099, 24560, 24886, 25066, 25795, 25913, 26423, 27610, 29905, 31276, 31794,
        ],
        [
            7444, 12781, 20177, 20728, 21077, 21607, 22170, 23405, 24469, 27915, 29090, 30492,
        ],
        [
            8537, 14689, 15432, 17087, 17408, 18172, 18408, 19825, 24649, 29153, 31096, 32210,
        ],
        [
            7543, 14231, 15496, 16195, 17905, 20717, 21984, 24516, 26001, 29675, 30981, 31994,
        ],
    ],
    [
        [
            12613, 13591, 21383, 22004, 22312, 22577, 23401, 25055, 25729, 29538, 30305, 32077,
        ],
        [
            9687, 13470, 18506, 19230, 19604, 20147, 20695, 22062, 23219, 27743, 29211, 30907,
        ],
        [
            6183, 6505, 26024, 26252, 26366, 26434, 27082, 28354, 28555, 30467, 30794, 32086,
        ],
        [
            10718, 11734, 14954, 17224, 17565, 17924, 18561, 21523, 23878, 28975, 30287, 32252,
        ],
        [
            9194, 9858, 16501, 17263, 18424, 19171, 21563, 25961, 26561, 30072, 30737, 32463,
        ],
    ],
    [
        [
            12602, 14399, 15488, 18381, 18778, 19315, 19724, 21419, 25060, 29696, 30917, 32409,
        ],
        [
            8203, 13821, 14524, 17105, 17439, 18131, 18404, 19468, 25225, 29485, 31158, 32342,
        ],
        [
            8451, 9731, 15004, 17643, 18012, 18425, 19070, 21538, 24605, 29118, 30078, 32018,
        ],
        [
            7714, 9048, 9516, 16667, 16817, 16994, 17153, 18767, 26743, 30389, 31536, 32528,
        ],
        [
            8843, 10280, 11496, 15317, 16652, 17943, 19108, 22718, 25769, 29953, 30983, 32485,
        ],
    ],
    [
        [
            12578, 13671, 15979, 16834, 19075, 20913, 22989, 25449, 26219, 30214, 31150, 32477,
        ],
        [
            9563, 13626, 15080, 15892, 17756, 20863, 22207, 24236, 25380, 29653, 31143, 32277,
        ],
        [
            8356, 8901, 17616, 18256, 19350, 20106, 22598, 25947, 26466, 29900, 30523, 32261,
        ],
        [
            10835, 11815, 13124, 16042, 17018, 18039, 18947, 22753, 24615, 29489, 30883, 32482,
        ],
        [
            7618, 8288, 9859, 10509, 15386, 18657, 22903, 28776, 29180, 31355, 31802, 32593,
        ],
    ],
]));
static av1_default_coef_cdf: [CdfCoefContext; 4] = [
    CdfCoefContext {
        eob_bin_16: Align16(cdf2d([
            [[840, 1039, 1980, 4895], [370, 671, 1883, 4471]],
            [[3247, 4950, 9688, 14563], [1904, 3354, 7763, 14647]],
        ])),
        eob_bin_32: Align16(cdf2d([
            [[400, 520, 977, 2102, 6542], [210, 405, 1315, 3326, 7537]],
            [
                [2636, 4273, 7588, 11794, 20401],
                [1786, 3179, 6902, 11357, 19054],
            ],
        ])),
        eob_bin_64: Align16(cdf2d([
            [
                [329, 498, 1101, 1784, 3265, 7758],
                [335, 730, 1459, 5494, 8755, 12997],
            ],
            [
                [3505, 5304, 10086, 13814, 17684, 23370],
                [1563, 2700, 4876, 10911, 14706, 22480],
            ],
        ])),
        eob_bin_128: Align16(cdf2d([
            [
                [219, 482, 1140, 2091, 3680, 6028, 12586],
                [371, 699, 1254, 4830, 9479, 12562, 17497],
            ],
            [
                [5245, 7456, 12880, 15852, 20033, 23932, 27608],
                [2054, 3472, 5869, 14232, 18242, 20590, 26752],
            ],
        ])),
        eob_bin_256: Align32(cdf2d([
            [
                [310, 584, 1887, 3589, 6168, 8611, 11352, 15652],
                [998, 1850, 2998, 5604, 17341, 19888, 22899, 25583],
            ],
            [
                [2520, 3240, 5952, 8870, 12577, 17558, 19954, 24168],
                [2203, 4130, 7435, 10739, 20652, 23681, 25609, 27261],
            ],
        ])),
        eob_bin_512: Align32(cdf1d([
            [641, 983, 3707, 5430, 10234, 14958, 18788, 23412, 26061],
            [5095, 6446, 9996, 13354, 16017, 17986, 20919, 26129, 29140],
        ])),
        eob_bin_1024: Align32(cdf1d([
            [393, 421, 751, 1623, 3160, 6352, 13345, 18047, 22571, 25830],
            [
                1865, 1988, 2930, 4242, 10533, 16538, 21354, 27255, 28546, 31784,
            ],
        ])),
        eob_base_tok: Align8(cdf3d([
            [
                [
                    [17837, 29055],
                    [29600, 31446],
                    [30844, 31878],
                    [24926, 28948],
                ],
                [
                    [21365, 30026],
                    [30512, 32423],
                    [31658, 32621],
                    [29630, 31881],
                ],
            ],
            [
                [
                    [5717, 26477],
                    [30491, 31703],
                    [31550, 32158],
                    [29648, 31491],
                ],
                [
                    [12608, 27820],
                    [30680, 32225],
                    [30809, 32335],
                    [31299, 32423],
                ],
            ],
            [
                [
                    [1786, 12612],
                    [30663, 31625],
                    [32339, 32468],
                    [31148, 31833],
                ],
                [
                    [18857, 23865],
                    [31428, 32428],
                    [31744, 32373],
                    [31775, 32526],
                ],
            ],
            [
                [[1787, 2532], [30832, 31662], [31824, 32682], [32133, 32569]],
                [
                    [13751, 22235],
                    [32089, 32409],
                    [27084, 27920],
                    [29291, 32594],
                ],
            ],
            [
                [[1725, 3449], [31102, 31935], [32457, 32613], [32412, 32649]],
                [
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                ],
            ],
        ])),
        base_tok: Align8(cdf3d([
            [
                [
                    [4034, 8930, 12727],
                    [18082, 29741, 31877],
                    [12596, 26124, 30493],
                    [9446, 21118, 27005],
                    [6308, 15141, 21279],
                    [2463, 6357, 9783],
                    [20667, 30546, 31929],
                    [13043, 26123, 30134],
                    [8151, 18757, 24778],
                    [5255, 12839, 18632],
                    [2820, 7206, 11161],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [15736, 27553, 30604],
                    [11210, 23794, 28787],
                    [5947, 13874, 19701],
                    [4215, 9323, 13891],
                    [2833, 6462, 10059],
                    [19605, 30393, 31582],
                    [13523, 26252, 30248],
                    [8446, 18622, 24512],
                    [3818, 10343, 15974],
                    [1481, 4117, 6796],
                    [22649, 31302, 32190],
                    [14829, 27127, 30449],
                    [8313, 17702, 23304],
                    [3022, 8301, 12786],
                    [1536, 4412, 7184],
                    [22354, 29774, 31372],
                    [14723, 25472, 29214],
                    [6673, 13745, 18662],
                    [2068, 5766, 9322],
                    [8192, 16384, 24576],
                ],
                [
                    [6302, 16444, 21761],
                    [23040, 31538, 32475],
                    [15196, 28452, 31496],
                    [10020, 22946, 28514],
                    [6533, 16862, 23501],
                    [3538, 9816, 15076],
                    [24444, 31875, 32525],
                    [15881, 28924, 31635],
                    [9922, 22873, 28466],
                    [6527, 16966, 23691],
                    [4114, 11303, 17220],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [20201, 30770, 32209],
                    [14754, 28071, 31258],
                    [8378, 20186, 26517],
                    [5916, 15299, 21978],
                    [4268, 11583, 17901],
                    [24361, 32025, 32581],
                    [18673, 30105, 31943],
                    [10196, 22244, 27576],
                    [5495, 14349, 20417],
                    [2676, 7415, 11498],
                    [24678, 31958, 32585],
                    [18629, 29906, 31831],
                    [9364, 20724, 26315],
                    [4641, 12318, 18094],
                    [2758, 7387, 11579],
                    [25433, 31842, 32469],
                    [18795, 29289, 31411],
                    [7644, 17584, 23592],
                    [3408, 9014, 15047],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [4536, 10072, 14001],
                    [25459, 31416, 32206],
                    [16605, 28048, 30818],
                    [11008, 22857, 27719],
                    [6915, 16268, 22315],
                    [2625, 6812, 10537],
                    [24257, 31788, 32499],
                    [16880, 29454, 31879],
                    [11958, 25054, 29778],
                    [7916, 18718, 25084],
                    [3383, 8777, 13446],
                    [22720, 31603, 32393],
                    [14960, 28125, 31335],
                    [9731, 22210, 27928],
                    [6304, 15832, 22277],
                    [2910, 7818, 12166],
                    [20375, 30627, 32131],
                    [13904, 27284, 30887],
                    [9368, 21558, 27144],
                    [5937, 14966, 21119],
                    [2667, 7225, 11319],
                    [23970, 31470, 32378],
                    [17173, 29734, 32018],
                    [12795, 25441, 29965],
                    [8981, 19680, 25893],
                    [4728, 11372, 16902],
                    [24287, 31797, 32439],
                    [16703, 29145, 31696],
                    [10833, 23554, 28725],
                    [6468, 16566, 23057],
                    [2415, 6562, 10278],
                    [26610, 32395, 32659],
                    [18590, 30498, 32117],
                    [12420, 25756, 29950],
                    [7639, 18746, 24710],
                    [3001, 8086, 12347],
                    [25076, 32064, 32580],
                    [17946, 30128, 32028],
                    [12024, 24985, 29378],
                    [7517, 18390, 24304],
                    [3243, 8781, 13331],
                ],
                [
                    [6037, 16771, 21957],
                    [24774, 31704, 32426],
                    [16830, 28589, 31056],
                    [10602, 22828, 27760],
                    [6733, 16829, 23071],
                    [3250, 8914, 13556],
                    [25582, 32220, 32668],
                    [18659, 30342, 32223],
                    [12546, 26149, 30515],
                    [8420, 20451, 26801],
                    [4636, 12420, 18344],
                    [27581, 32362, 32639],
                    [18987, 30083, 31978],
                    [11327, 24248, 29084],
                    [7264, 17719, 24120],
                    [3995, 10768, 16169],
                    [25893, 31831, 32487],
                    [16577, 28587, 31379],
                    [10189, 22748, 28182],
                    [6832, 17094, 23556],
                    [3708, 10110, 15334],
                    [25904, 32282, 32656],
                    [19721, 30792, 32276],
                    [12819, 26243, 30411],
                    [8572, 20614, 26891],
                    [5364, 14059, 20467],
                    [26580, 32438, 32677],
                    [20852, 31225, 32340],
                    [12435, 25700, 29967],
                    [8691, 20825, 26976],
                    [4446, 12209, 17269],
                    [27350, 32429, 32696],
                    [21372, 30977, 32272],
                    [12673, 25270, 29853],
                    [9208, 20925, 26640],
                    [5018, 13351, 18732],
                    [27351, 32479, 32713],
                    [21398, 31209, 32387],
                    [12162, 25047, 29842],
                    [7896, 18691, 25319],
                    [4670, 12882, 18881],
                ],
            ],
            [
                [
                    [5487, 10460, 13708],
                    [21597, 28303, 30674],
                    [11037, 21953, 26476],
                    [8147, 17962, 22952],
                    [5242, 13061, 18532],
                    [1889, 5208, 8182],
                    [26774, 32133, 32590],
                    [17844, 29564, 31767],
                    [11690, 24438, 29171],
                    [7542, 18215, 24459],
                    [2993, 8050, 12319],
                    [28023, 32328, 32591],
                    [18651, 30126, 31954],
                    [12164, 25146, 29589],
                    [7762, 18530, 24771],
                    [3492, 9183, 13920],
                    [27591, 32008, 32491],
                    [17149, 28853, 31510],
                    [11485, 24003, 28860],
                    [7697, 18086, 24210],
                    [3075, 7999, 12218],
                    [28268, 32482, 32654],
                    [19631, 31051, 32404],
                    [13860, 27260, 31020],
                    [9605, 21613, 27594],
                    [4876, 12162, 17908],
                    [27248, 32316, 32576],
                    [18955, 30457, 32075],
                    [11824, 23997, 28795],
                    [7346, 18196, 24647],
                    [3403, 9247, 14111],
                    [29711, 32655, 32735],
                    [21169, 31394, 32417],
                    [13487, 27198, 30957],
                    [8828, 21683, 27614],
                    [4270, 11451, 17038],
                    [28708, 32578, 32731],
                    [20120, 31241, 32482],
                    [13692, 27550, 31321],
                    [9418, 22514, 28439],
                    [4999, 13283, 19462],
                ],
                [
                    [5673, 14302, 19711],
                    [26251, 30701, 31834],
                    [12782, 23783, 27803],
                    [9127, 20657, 25808],
                    [6368, 16208, 21462],
                    [2465, 7177, 10822],
                    [29961, 32563, 32719],
                    [18318, 29891, 31949],
                    [11361, 24514, 29357],
                    [7900, 19603, 25607],
                    [4002, 10590, 15546],
                    [29637, 32310, 32595],
                    [18296, 29913, 31809],
                    [10144, 21515, 26871],
                    [5358, 14322, 20394],
                    [3067, 8362, 13346],
                    [28652, 32470, 32676],
                    [17538, 30771, 32209],
                    [13924, 26882, 30494],
                    [10496, 22837, 27869],
                    [7236, 16396, 21621],
                    [30743, 32687, 32746],
                    [23006, 31676, 32489],
                    [14494, 27828, 31120],
                    [10174, 22801, 28352],
                    [6242, 15281, 21043],
                    [25817, 32243, 32720],
                    [18618, 31367, 32325],
                    [13997, 28318, 31878],
                    [12255, 26534, 31383],
                    [9561, 21588, 28450],
                    [28188, 32635, 32724],
                    [22060, 32365, 32728],
                    [18102, 30690, 32528],
                    [14196, 28864, 31999],
                    [12262, 25792, 30865],
                    [24176, 32109, 32628],
                    [18280, 29681, 31963],
                    [10205, 23703, 29664],
                    [7889, 20025, 27676],
                    [6060, 16743, 23970],
                ],
            ],
            [
                [
                    [5141, 7096, 8260],
                    [27186, 29022, 29789],
                    [6668, 12568, 15682],
                    [2172, 6181, 8638],
                    [1126, 3379, 4531],
                    [443, 1361, 2254],
                    [26083, 31153, 32436],
                    [13486, 24603, 28483],
                    [6508, 14840, 19910],
                    [3386, 8800, 13286],
                    [1530, 4322, 7054],
                    [29639, 32080, 32548],
                    [15897, 27552, 30290],
                    [8588, 20047, 25383],
                    [4889, 13339, 19269],
                    [2240, 6871, 10498],
                    [28165, 32197, 32517],
                    [20735, 30427, 31568],
                    [14325, 24671, 27692],
                    [5119, 12554, 17805],
                    [1810, 5441, 8261],
                    [31212, 32724, 32748],
                    [23352, 31766, 32545],
                    [14669, 27570, 31059],
                    [8492, 20894, 27272],
                    [3644, 10194, 15204],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [2461, 7013, 9371],
                    [24749, 29600, 30986],
                    [9466, 19037, 22417],
                    [3584, 9280, 14400],
                    [1505, 3929, 5433],
                    [677, 1500, 2736],
                    [23987, 30702, 32117],
                    [13554, 24571, 29263],
                    [6211, 14556, 21155],
                    [3135, 10972, 15625],
                    [2435, 7127, 11427],
                    [31300, 32532, 32550],
                    [14757, 30365, 31954],
                    [4405, 11612, 18553],
                    [580, 4132, 7322],
                    [1695, 10169, 14124],
                    [30008, 32282, 32591],
                    [19244, 30108, 31748],
                    [11180, 24158, 29555],
                    [5650, 14972, 19209],
                    [2114, 5109, 8456],
                    [31856, 32716, 32748],
                    [23012, 31664, 32572],
                    [13694, 26656, 30636],
                    [8142, 19508, 26093],
                    [4253, 10955, 16724],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [601, 983, 1311],
                    [18725, 23406, 28087],
                    [5461, 8192, 10923],
                    [3781, 15124, 21425],
                    [2587, 7761, 12072],
                    [106, 458, 810],
                    [22282, 29710, 31894],
                    [8508, 20926, 25984],
                    [3726, 12713, 18083],
                    [1620, 7112, 10893],
                    [729, 2236, 3495],
                    [30163, 32474, 32684],
                    [18304, 30464, 32000],
                    [11443, 26526, 29647],
                    [6007, 15292, 21299],
                    [2234, 6703, 8937],
                    [30954, 32177, 32571],
                    [17363, 29562, 31076],
                    [9686, 22464, 27410],
                    [8192, 16384, 21390],
                    [1755, 8046, 11264],
                    [31168, 32734, 32748],
                    [22486, 31441, 32471],
                    [12833, 25627, 29738],
                    [6980, 17379, 23122],
                    [3111, 8887, 13479],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
        ])),
        br_tok: Align8(cdf3d([
            [
                [
                    [14298, 20718, 24174],
                    [12536, 19601, 23789],
                    [8712, 15051, 19503],
                    [6170, 11327, 15434],
                    [4742, 8926, 12538],
                    [3803, 7317, 10546],
                    [1696, 3317, 4871],
                    [14392, 19951, 22756],
                    [15978, 23218, 26818],
                    [12187, 19474, 23889],
                    [9176, 15640, 20259],
                    [7068, 12655, 17028],
                    [5656, 10442, 14472],
                    [2580, 4992, 7244],
                    [12136, 18049, 21426],
                    [13784, 20721, 24481],
                    [10836, 17621, 21900],
                    [8372, 14444, 18847],
                    [6523, 11779, 16000],
                    [5337, 9898, 13760],
                    [3034, 5860, 8462],
                ],
                [
                    [15967, 22905, 26286],
                    [13534, 20654, 24579],
                    [9504, 16092, 20535],
                    [6975, 12568, 16903],
                    [5364, 10091, 14020],
                    [4357, 8370, 11857],
                    [2506, 4934, 7218],
                    [23032, 28815, 30936],
                    [19540, 26704, 29719],
                    [15158, 22969, 27097],
                    [11408, 18865, 23650],
                    [8885, 15448, 20250],
                    [7108, 12853, 17416],
                    [4231, 8041, 11480],
                    [19823, 26490, 29156],
                    [18890, 25929, 28932],
                    [15660, 23491, 27433],
                    [12147, 19776, 24488],
                    [9728, 16774, 21649],
                    [7919, 14277, 19066],
                    [5440, 10170, 14185],
                ],
            ],
            [
                [
                    [14406, 20862, 24414],
                    [11824, 18907, 23109],
                    [8257, 14393, 18803],
                    [5860, 10747, 14778],
                    [4475, 8486, 11984],
                    [3606, 6954, 10043],
                    [1736, 3410, 5048],
                    [14430, 20046, 22882],
                    [15593, 22899, 26709],
                    [12102, 19368, 23811],
                    [9059, 15584, 20262],
                    [6999, 12603, 17048],
                    [5684, 10497, 14553],
                    [2822, 5438, 7862],
                    [15785, 21585, 24359],
                    [18347, 25229, 28266],
                    [14974, 22487, 26389],
                    [11423, 18681, 23271],
                    [8863, 15350, 20008],
                    [7153, 12852, 17278],
                    [3707, 7036, 9982],
                ],
                [
                    [15460, 21696, 25469],
                    [12170, 19249, 23191],
                    [8723, 15027, 19332],
                    [6428, 11704, 15874],
                    [4922, 9292, 13052],
                    [4139, 7695, 11010],
                    [2291, 4508, 6598],
                    [19856, 26920, 29828],
                    [17923, 25289, 28792],
                    [14278, 21968, 26297],
                    [10910, 18136, 22950],
                    [8423, 14815, 19627],
                    [6771, 12283, 16774],
                    [4074, 7750, 11081],
                    [19852, 26074, 28672],
                    [19371, 26110, 28989],
                    [16265, 23873, 27663],
                    [12758, 20378, 24952],
                    [10095, 17098, 21961],
                    [8250, 14628, 19451],
                    [5205, 9745, 13622],
                ],
            ],
            [
                [
                    [10563, 16233, 19763],
                    [9794, 16022, 19804],
                    [6750, 11945, 15759],
                    [4963, 9186, 12752],
                    [3845, 7435, 10627],
                    [3051, 6085, 8834],
                    [1311, 2596, 3830],
                    [11246, 16404, 19689],
                    [12315, 18911, 22731],
                    [10557, 17095, 21289],
                    [8136, 14006, 18249],
                    [6348, 11474, 15565],
                    [5196, 9655, 13400],
                    [2349, 4526, 6587],
                    [13337, 18730, 21569],
                    [19306, 26071, 28882],
                    [15952, 23540, 27254],
                    [12409, 19934, 24430],
                    [9760, 16706, 21389],
                    [8004, 14220, 18818],
                    [4138, 7794, 10961],
                ],
                [
                    [10870, 16684, 20949],
                    [9664, 15230, 18680],
                    [6886, 12109, 15408],
                    [4825, 8900, 12305],
                    [3630, 7162, 10314],
                    [3036, 6429, 9387],
                    [1671, 3296, 4940],
                    [13819, 19159, 23026],
                    [11984, 19108, 23120],
                    [10690, 17210, 21663],
                    [7984, 14154, 18333],
                    [6868, 12294, 16124],
                    [5274, 8994, 12868],
                    [2988, 5771, 8424],
                    [19736, 26647, 29141],
                    [18933, 26070, 28984],
                    [15779, 23048, 27200],
                    [12638, 20061, 24532],
                    [10692, 17545, 22220],
                    [9217, 15251, 20054],
                    [5078, 9284, 12594],
                ],
            ],
            [
                [
                    [2331, 3662, 5244],
                    [2891, 4771, 6145],
                    [4598, 7623, 9729],
                    [3520, 6845, 9199],
                    [3417, 6119, 9324],
                    [2601, 5412, 7385],
                    [600, 1173, 1744],
                    [7672, 13286, 17469],
                    [4232, 7792, 10793],
                    [2915, 5317, 7397],
                    [2318, 4356, 6152],
                    [2127, 4000, 5554],
                    [1850, 3478, 5275],
                    [977, 1933, 2843],
                    [18280, 24387, 27989],
                    [15852, 22671, 26185],
                    [13845, 20951, 24789],
                    [11055, 17966, 22129],
                    [9138, 15422, 19801],
                    [7454, 13145, 17456],
                    [3370, 6393, 9013],
                ],
                [
                    [5842, 9229, 10838],
                    [2313, 3491, 4276],
                    [2998, 6104, 7496],
                    [2420, 7447, 9868],
                    [3034, 8495, 10923],
                    [4076, 8937, 10975],
                    [1086, 2370, 3299],
                    [9714, 17254, 20444],
                    [8543, 13698, 17123],
                    [4918, 9007, 11910],
                    [4129, 7532, 10553],
                    [2364, 5533, 8058],
                    [1834, 3546, 5563],
                    [1473, 2908, 4133],
                    [15405, 21193, 25619],
                    [15691, 21952, 26561],
                    [12962, 19194, 24165],
                    [10272, 17855, 22129],
                    [8588, 15270, 20718],
                    [8682, 14669, 19500],
                    [4870, 9636, 13205],
                ],
            ],
        ])),
        eob_hi_bit: Align4(cdf3d([
            [
                [
                    [16384],
                    [16384],
                    [16961],
                    [17223],
                    [7621],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [19069],
                    [22525],
                    [13377],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [20401],
                    [17025],
                    [12845],
                    [12873],
                    [14094],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [20681],
                    [20701],
                    [15250],
                    [15017],
                    [14928],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [23905],
                    [17194],
                    [16170],
                    [17695],
                    [13826],
                    [15810],
                    [12036],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [23959],
                    [20799],
                    [19021],
                    [16203],
                    [17886],
                    [14144],
                    [12010],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [27399],
                    [16327],
                    [18071],
                    [19584],
                    [20721],
                    [18432],
                    [19560],
                    [10150],
                    [8805],
                ],
                [
                    [16384],
                    [16384],
                    [24932],
                    [20833],
                    [12027],
                    [16670],
                    [19914],
                    [15106],
                    [17662],
                    [13783],
                    [28756],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [23406],
                    [21845],
                    [18432],
                    [16384],
                    [17096],
                    [12561],
                    [17320],
                    [22395],
                    [21370],
                ],
                [
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
        ])),
        skip: Align4(cdf2d([
            [
                [31849],
                [5892],
                [12112],
                [21935],
                [20289],
                [27473],
                [32487],
                [7654],
                [19473],
                [29984],
                [9961],
                [30242],
                [32117],
            ],
            [
                [31548],
                [1549],
                [10130],
                [16656],
                [18591],
                [26308],
                [32537],
                [5403],
                [18096],
                [30003],
                [16384],
                [16384],
                [16384],
            ],
            [
                [29957],
                [5391],
                [18039],
                [23566],
                [22431],
                [25822],
                [32197],
                [3778],
                [15336],
                [28981],
                [16384],
                [16384],
                [16384],
            ],
            [
                [17920],
                [1818],
                [7282],
                [25273],
                [10923],
                [31554],
                [32624],
                [1366],
                [15628],
                [30462],
                [146],
                [5132],
                [31657],
            ],
            [
                [6308],
                [117],
                [1638],
                [2161],
                [16384],
                [10923],
                [30247],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
            ],
        ])),
        dc_sign: Align4(cdf2d([
            [[16000], [13056], [18816]],
            [[15232], [12928], [17280]],
        ])),
    },
    CdfCoefContext {
        eob_bin_16: Align16(cdf2d([
            [[2125, 2551, 5165, 8946], [513, 765, 1859, 6339]],
            [[7637, 9498, 14259, 19108], [2497, 4096, 8866, 16993]],
        ])),
        eob_bin_32: Align16(cdf2d([
            [[989, 1249, 2019, 4151, 10785], [313, 441, 1099, 2917, 8562]],
            [
                [8394, 10352, 13932, 18855, 26014],
                [2578, 4124, 8181, 13670, 24234],
            ],
        ])),
        eob_bin_64: Align16(cdf2d([
            [
                [1260, 1446, 2253, 3712, 6652, 13369],
                [401, 605, 1029, 2563, 5845, 12626],
            ],
            [
                [8609, 10612, 14624, 18714, 22614, 29024],
                [1923, 3127, 5867, 9703, 14277, 27100],
            ],
        ])),
        eob_bin_128: Align16(cdf2d([
            [
                [685, 933, 1488, 2714, 4766, 8562, 19254],
                [217, 352, 618, 2303, 5261, 9969, 17472],
            ],
            [
                [8045, 11200, 15497, 19595, 23948, 27408, 30938],
                [2310, 4160, 7471, 14997, 17931, 20768, 30240],
            ],
        ])),
        eob_bin_256: Align32(cdf2d([
            [
                [1448, 2109, 4151, 6263, 9329, 13260, 17944, 23300],
                [399, 1019, 1749, 3038, 10444, 15546, 22739, 27294],
            ],
            [
                [6402, 8148, 12623, 15072, 18728, 22847, 26447, 29377],
                [1674, 3252, 5734, 10159, 22397, 23802, 24821, 30940],
            ],
        ])),
        eob_bin_512: Align32(cdf1d([
            [1230, 2278, 5035, 7776, 11871, 15346, 19590, 24584, 28749],
            [7265, 9979, 15819, 19250, 21780, 23846, 26478, 28396, 31811],
        ])),
        eob_bin_1024: Align32(cdf1d([
            [
                696, 948, 3145, 5702, 9706, 13217, 17851, 21856, 25692, 28034,
            ],
            [
                2672, 3591, 9330, 17084, 22725, 24284, 26527, 28027, 28377, 30876,
            ],
        ])),
        eob_base_tok: Align8(cdf3d([
            [
                [
                    [17560, 29888],
                    [29671, 31549],
                    [31007, 32056],
                    [27286, 30006],
                ],
                [
                    [26594, 31212],
                    [31208, 32582],
                    [31835, 32637],
                    [30595, 32206],
                ],
            ],
            [
                [
                    [15239, 29932],
                    [31315, 32095],
                    [32130, 32434],
                    [30864, 31996],
                ],
                [
                    [26279, 30968],
                    [31142, 32495],
                    [31713, 32540],
                    [31929, 32594],
                ],
            ],
            [
                [
                    [2644, 25198],
                    [32038, 32451],
                    [32639, 32695],
                    [32166, 32518],
                ],
                [
                    [17187, 27668],
                    [31714, 32550],
                    [32283, 32678],
                    [31930, 32563],
                ],
            ],
            [
                [[1044, 2257], [30755, 31923], [32208, 32693], [32244, 32615]],
                [
                    [21317, 26207],
                    [29133, 30868],
                    [29311, 31231],
                    [29657, 31087],
                ],
            ],
            [
                [[478, 1834], [31005, 31987], [32317, 32724], [30865, 32648]],
                [
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                ],
            ],
        ])),
        base_tok: Align8(cdf3d([
            [
                [
                    [6041, 11854, 15927],
                    [20326, 30905, 32251],
                    [14164, 26831, 30725],
                    [9760, 20647, 26585],
                    [6416, 14953, 21219],
                    [2966, 7151, 10891],
                    [23567, 31374, 32254],
                    [14978, 27416, 30946],
                    [9434, 20225, 26254],
                    [6658, 14558, 20535],
                    [3916, 8677, 12989],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [18088, 29545, 31587],
                    [13062, 25843, 30073],
                    [8940, 16827, 22251],
                    [7654, 13220, 17973],
                    [5733, 10316, 14456],
                    [22879, 31388, 32114],
                    [15215, 27993, 30955],
                    [9397, 19445, 24978],
                    [3442, 9813, 15344],
                    [1368, 3936, 6532],
                    [25494, 32033, 32406],
                    [16772, 27963, 30718],
                    [9419, 18165, 23260],
                    [2677, 7501, 11797],
                    [1516, 4344, 7170],
                    [26556, 31454, 32101],
                    [17128, 27035, 30108],
                    [8324, 15344, 20249],
                    [1903, 5696, 9469],
                    [8192, 16384, 24576],
                ],
                [
                    [8455, 19003, 24368],
                    [23563, 32021, 32604],
                    [16237, 29446, 31935],
                    [10724, 23999, 29358],
                    [6725, 17528, 24416],
                    [3927, 10927, 16825],
                    [26313, 32288, 32634],
                    [17430, 30095, 32095],
                    [11116, 24606, 29679],
                    [7195, 18384, 25269],
                    [4726, 12852, 19315],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [22822, 31648, 32483],
                    [16724, 29633, 31929],
                    [10261, 23033, 28725],
                    [7029, 17840, 24528],
                    [4867, 13886, 21502],
                    [25298, 31892, 32491],
                    [17809, 29330, 31512],
                    [9668, 21329, 26579],
                    [4774, 12956, 18976],
                    [2322, 7030, 11540],
                    [25472, 31920, 32543],
                    [17957, 29387, 31632],
                    [9196, 20593, 26400],
                    [4680, 12705, 19202],
                    [2917, 8456, 13436],
                    [26471, 32059, 32574],
                    [18458, 29783, 31909],
                    [8400, 19464, 25956],
                    [3812, 10973, 17206],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [6779, 13743, 17678],
                    [24806, 31797, 32457],
                    [17616, 29047, 31372],
                    [11063, 23175, 28003],
                    [6521, 16110, 22324],
                    [2764, 7504, 11654],
                    [25266, 32367, 32637],
                    [19054, 30553, 32175],
                    [12139, 25212, 29807],
                    [7311, 18162, 24704],
                    [3397, 9164, 14074],
                    [25988, 32208, 32522],
                    [16253, 28912, 31526],
                    [9151, 21387, 27372],
                    [5688, 14915, 21496],
                    [2717, 7627, 12004],
                    [23144, 31855, 32443],
                    [16070, 28491, 31325],
                    [8702, 20467, 26517],
                    [5243, 13956, 20367],
                    [2621, 7335, 11567],
                    [26636, 32340, 32630],
                    [19990, 31050, 32341],
                    [13243, 26105, 30315],
                    [8588, 19521, 25918],
                    [4717, 11585, 17304],
                    [25844, 32292, 32582],
                    [19090, 30635, 32097],
                    [11963, 24546, 28939],
                    [6218, 16087, 22354],
                    [2340, 6608, 10426],
                    [28046, 32576, 32694],
                    [21178, 31313, 32296],
                    [13486, 26184, 29870],
                    [7149, 17871, 23723],
                    [2833, 7958, 12259],
                    [27710, 32528, 32686],
                    [20674, 31076, 32268],
                    [12413, 24955, 29243],
                    [6676, 16927, 23097],
                    [2966, 8333, 12919],
                ],
                [
                    [8639, 19339, 24429],
                    [24404, 31837, 32525],
                    [16997, 29425, 31784],
                    [11253, 24234, 29149],
                    [6751, 17394, 24028],
                    [3490, 9830, 15191],
                    [26283, 32471, 32714],
                    [19599, 31168, 32442],
                    [13146, 26954, 30893],
                    [8214, 20588, 26890],
                    [4699, 13081, 19300],
                    [28212, 32458, 32669],
                    [18594, 30316, 32100],
                    [11219, 24408, 29234],
                    [6865, 17656, 24149],
                    [3678, 10362, 16006],
                    [25825, 32136, 32616],
                    [17313, 29853, 32021],
                    [11197, 24471, 29472],
                    [6947, 17781, 24405],
                    [3768, 10660, 16261],
                    [27352, 32500, 32706],
                    [20850, 31468, 32469],
                    [14021, 27707, 31133],
                    [8964, 21748, 27838],
                    [5437, 14665, 21187],
                    [26304, 32492, 32698],
                    [20409, 31380, 32385],
                    [13682, 27222, 30632],
                    [8974, 21236, 26685],
                    [4234, 11665, 16934],
                    [26273, 32357, 32711],
                    [20672, 31242, 32441],
                    [14172, 27254, 30902],
                    [9870, 21898, 27275],
                    [5164, 13506, 19270],
                    [26725, 32459, 32728],
                    [20991, 31442, 32527],
                    [13071, 26434, 30811],
                    [8184, 20090, 26742],
                    [4803, 13255, 19895],
                ],
            ],
            [
                [
                    [7555, 14942, 18501],
                    [24410, 31178, 32287],
                    [14394, 26738, 30253],
                    [8413, 19554, 25195],
                    [4766, 12924, 18785],
                    [2029, 5806, 9207],
                    [26776, 32364, 32663],
                    [18732, 29967, 31931],
                    [11005, 23786, 28852],
                    [6466, 16909, 23510],
                    [3044, 8638, 13419],
                    [29208, 32582, 32704],
                    [20068, 30857, 32208],
                    [12003, 25085, 29595],
                    [6947, 17750, 24189],
                    [3245, 9103, 14007],
                    [27359, 32465, 32669],
                    [19421, 30614, 32174],
                    [11915, 25010, 29579],
                    [6950, 17676, 24074],
                    [3007, 8473, 13096],
                    [29002, 32676, 32735],
                    [22102, 31849, 32576],
                    [14408, 28009, 31405],
                    [9027, 21679, 27931],
                    [4694, 12678, 18748],
                    [28216, 32528, 32682],
                    [20849, 31264, 32318],
                    [12756, 25815, 29751],
                    [7565, 18801, 24923],
                    [3509, 9533, 14477],
                    [30133, 32687, 32739],
                    [23063, 31910, 32515],
                    [14588, 28051, 31132],
                    [9085, 21649, 27457],
                    [4261, 11654, 17264],
                    [29518, 32691, 32748],
                    [22451, 31959, 32613],
                    [14864, 28722, 31700],
                    [9695, 22964, 28716],
                    [4932, 13358, 19502],
                ],
                [
                    [6465, 16958, 21688],
                    [25199, 31514, 32360],
                    [14774, 27149, 30607],
                    [9257, 21438, 26972],
                    [5723, 15183, 21882],
                    [3150, 8879, 13731],
                    [26989, 32262, 32682],
                    [17396, 29937, 32085],
                    [11387, 24901, 29784],
                    [7289, 18821, 25548],
                    [3734, 10577, 16086],
                    [29728, 32501, 32695],
                    [17431, 29701, 31903],
                    [9921, 22826, 28300],
                    [5896, 15434, 22068],
                    [3430, 9646, 14757],
                    [28614, 32511, 32705],
                    [19364, 30638, 32263],
                    [13129, 26254, 30402],
                    [8754, 20484, 26440],
                    [4378, 11607, 17110],
                    [30292, 32671, 32744],
                    [21780, 31603, 32501],
                    [14314, 27829, 31291],
                    [9611, 22327, 28263],
                    [4890, 13087, 19065],
                    [25862, 32567, 32733],
                    [20794, 32050, 32567],
                    [17243, 30625, 32254],
                    [13283, 27628, 31474],
                    [9669, 22532, 28918],
                    [27435, 32697, 32748],
                    [24922, 32390, 32714],
                    [21449, 31504, 32536],
                    [16392, 29729, 31832],
                    [11692, 24884, 29076],
                    [24193, 32290, 32735],
                    [18909, 31104, 32563],
                    [12236, 26841, 31403],
                    [8171, 21840, 29082],
                    [7224, 17280, 25275],
                ],
            ],
            [
                [
                    [3078, 6839, 9890],
                    [13837, 20450, 24479],
                    [5914, 14222, 19328],
                    [3866, 10267, 14762],
                    [2612, 7208, 11042],
                    [1067, 2991, 4776],
                    [25817, 31646, 32529],
                    [13708, 26338, 30385],
                    [7328, 18585, 24870],
                    [4691, 13080, 19276],
                    [1825, 5253, 8352],
                    [29386, 32315, 32624],
                    [17160, 29001, 31360],
                    [9602, 21862, 27396],
                    [5915, 15772, 22148],
                    [2786, 7779, 12047],
                    [29246, 32450, 32663],
                    [18696, 29929, 31818],
                    [10510, 23369, 28560],
                    [6229, 16499, 23125],
                    [2608, 7448, 11705],
                    [30753, 32710, 32748],
                    [21638, 31487, 32503],
                    [12937, 26854, 30870],
                    [8182, 20596, 26970],
                    [3637, 10269, 15497],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [5244, 12150, 16906],
                    [20486, 26858, 29701],
                    [7756, 18317, 23735],
                    [3452, 9256, 13146],
                    [2020, 5206, 8229],
                    [1801, 4993, 7903],
                    [27051, 31858, 32531],
                    [15988, 27531, 30619],
                    [9188, 21484, 26719],
                    [6273, 17186, 23800],
                    [3108, 9355, 14764],
                    [31076, 32520, 32680],
                    [18119, 30037, 31850],
                    [10244, 22969, 27472],
                    [4692, 14077, 19273],
                    [3694, 11677, 17556],
                    [30060, 32581, 32720],
                    [21011, 30775, 32120],
                    [11931, 24820, 29289],
                    [7119, 17662, 24356],
                    [3833, 10706, 16304],
                    [31954, 32731, 32748],
                    [23913, 31724, 32489],
                    [15520, 28060, 31286],
                    [11517, 23008, 28571],
                    [6193, 14508, 20629],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [1035, 2807, 4156],
                    [13162, 18138, 20939],
                    [2696, 6633, 8755],
                    [1373, 4161, 6853],
                    [1099, 2746, 4716],
                    [340, 1021, 1599],
                    [22826, 30419, 32135],
                    [10395, 21762, 26942],
                    [4726, 12407, 17361],
                    [2447, 7080, 10593],
                    [1227, 3717, 6011],
                    [28156, 31424, 31934],
                    [16915, 27754, 30373],
                    [9148, 20990, 26431],
                    [5950, 15515, 21148],
                    [2492, 7327, 11526],
                    [30602, 32477, 32670],
                    [20026, 29955, 31568],
                    [11220, 23628, 28105],
                    [6652, 17019, 22973],
                    [3064, 8536, 13043],
                    [31769, 32724, 32748],
                    [22230, 30887, 32373],
                    [12234, 25079, 29731],
                    [7326, 18816, 25353],
                    [3933, 10907, 16616],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
        ])),
        br_tok: Align8(cdf3d([
            [
                [
                    [14995, 21341, 24749],
                    [13158, 20289, 24601],
                    [8941, 15326, 19876],
                    [6297, 11541, 15807],
                    [4817, 9029, 12776],
                    [3731, 7273, 10627],
                    [1847, 3617, 5354],
                    [14472, 19659, 22343],
                    [16806, 24162, 27533],
                    [12900, 20404, 24713],
                    [9411, 16112, 20797],
                    [7056, 12697, 17148],
                    [5544, 10339, 14460],
                    [2954, 5704, 8319],
                    [12464, 18071, 21354],
                    [15482, 22528, 26034],
                    [12070, 19269, 23624],
                    [8953, 15406, 20106],
                    [7027, 12730, 17220],
                    [5887, 10913, 15140],
                    [3793, 7278, 10447],
                ],
                [
                    [15571, 22232, 25749],
                    [14506, 21575, 25374],
                    [10189, 17089, 21569],
                    [7316, 13301, 17915],
                    [5783, 10912, 15190],
                    [4760, 9155, 13088],
                    [2993, 5966, 8774],
                    [23424, 28903, 30778],
                    [20775, 27666, 30290],
                    [16474, 24410, 28299],
                    [12471, 20180, 24987],
                    [9410, 16487, 21439],
                    [7536, 13614, 18529],
                    [5048, 9586, 13549],
                    [21090, 27290, 29756],
                    [20796, 27402, 30026],
                    [17819, 25485, 28969],
                    [13860, 21909, 26462],
                    [11002, 18494, 23529],
                    [8953, 15929, 20897],
                    [6448, 11918, 16454],
                ],
            ],
            [
                [
                    [15999, 22208, 25449],
                    [13050, 19988, 24122],
                    [8594, 14864, 19378],
                    [6033, 11079, 15238],
                    [4554, 8683, 12347],
                    [3672, 7139, 10337],
                    [1900, 3771, 5576],
                    [15788, 21340, 23949],
                    [16825, 24235, 27758],
                    [12873, 20402, 24810],
                    [9590, 16363, 21094],
                    [7352, 13209, 17733],
                    [5960, 10989, 15184],
                    [3232, 6234, 9007],
                    [15761, 20716, 23224],
                    [19318, 25989, 28759],
                    [15529, 23094, 26929],
                    [11662, 18989, 23641],
                    [8955, 15568, 20366],
                    [7281, 13106, 17708],
                    [4248, 8059, 11440],
                ],
                [
                    [14899, 21217, 24503],
                    [13519, 20283, 24047],
                    [9429, 15966, 20365],
                    [6700, 12355, 16652],
                    [5088, 9704, 13716],
                    [4243, 8154, 11731],
                    [2702, 5364, 7861],
                    [22745, 28388, 30454],
                    [20235, 27146, 29922],
                    [15896, 23715, 27637],
                    [11840, 19350, 24131],
                    [9122, 15932, 20880],
                    [7488, 13581, 18362],
                    [5114, 9568, 13370],
                    [20845, 26553, 28932],
                    [20981, 27372, 29884],
                    [17781, 25335, 28785],
                    [13760, 21708, 26297],
                    [10975, 18415, 23365],
                    [9045, 15789, 20686],
                    [6130, 11199, 15423],
                ],
            ],
            [
                [
                    [13549, 19724, 23158],
                    [11844, 18382, 22246],
                    [7919, 13619, 17773],
                    [5486, 10143, 13946],
                    [4166, 7983, 11324],
                    [3364, 6506, 9427],
                    [1598, 3160, 4674],
                    [15281, 20979, 23781],
                    [14939, 22119, 25952],
                    [11363, 18407, 22812],
                    [8609, 14857, 19370],
                    [6737, 12184, 16480],
                    [5506, 10263, 14262],
                    [2990, 5786, 8380],
                    [20249, 25253, 27417],
                    [21070, 27518, 30001],
                    [16854, 24469, 28074],
                    [12864, 20486, 25000],
                    [9962, 16978, 21778],
                    [8074, 14338, 19048],
                    [4494, 8479, 11906],
                ],
                [
                    [13960, 19617, 22829],
                    [11150, 17341, 21228],
                    [7150, 12964, 17190],
                    [5331, 10002, 13867],
                    [4167, 7744, 11057],
                    [3480, 6629, 9646],
                    [1883, 3784, 5686],
                    [18752, 25660, 28912],
                    [16968, 24586, 28030],
                    [13520, 21055, 25313],
                    [10453, 17626, 22280],
                    [8386, 14505, 19116],
                    [6742, 12595, 17008],
                    [4273, 8140, 11499],
                    [22120, 27827, 30233],
                    [20563, 27358, 29895],
                    [17076, 24644, 28153],
                    [13362, 20942, 25309],
                    [10794, 17965, 22695],
                    [9014, 15652, 20319],
                    [5708, 10512, 14497],
                ],
            ],
            [
                [
                    [5705, 10930, 15725],
                    [7946, 12765, 16115],
                    [6801, 12123, 16226],
                    [5462, 10135, 14200],
                    [4189, 8011, 11507],
                    [3191, 6229, 9408],
                    [1057, 2137, 3212],
                    [10018, 17067, 21491],
                    [7380, 12582, 16453],
                    [6068, 10845, 14339],
                    [5098, 9198, 12555],
                    [4312, 8010, 11119],
                    [3700, 6966, 9781],
                    [1693, 3326, 4887],
                    [18757, 24930, 27774],
                    [17648, 24596, 27817],
                    [14707, 22052, 26026],
                    [11720, 18852, 23292],
                    [9357, 15952, 20525],
                    [7810, 13753, 18210],
                    [3879, 7333, 10328],
                ],
                [
                    [8278, 13242, 15922],
                    [10547, 15867, 18919],
                    [9106, 15842, 20609],
                    [6833, 13007, 17218],
                    [4811, 9712, 13923],
                    [3985, 7352, 11128],
                    [1688, 3458, 5262],
                    [12951, 21861, 26510],
                    [9788, 16044, 20276],
                    [6309, 11244, 14870],
                    [5183, 9349, 12566],
                    [4389, 8229, 11492],
                    [3633, 6945, 10620],
                    [3600, 6847, 9907],
                    [21748, 28137, 30255],
                    [19436, 26581, 29560],
                    [16359, 24201, 27953],
                    [13961, 21693, 25871],
                    [11544, 18686, 23322],
                    [9372, 16462, 20952],
                    [6138, 11210, 15390],
                ],
            ],
        ])),
        eob_hi_bit: Align4(cdf3d([
            [
                [
                    [16384],
                    [16384],
                    [17471],
                    [20223],
                    [11357],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [20335],
                    [21667],
                    [14818],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [20430],
                    [20662],
                    [15367],
                    [16970],
                    [14657],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [22117],
                    [22028],
                    [18650],
                    [16042],
                    [15885],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [22409],
                    [21012],
                    [15650],
                    [17395],
                    [15469],
                    [20205],
                    [19511],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [24220],
                    [22480],
                    [17737],
                    [18916],
                    [19268],
                    [18412],
                    [18844],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [25991],
                    [20314],
                    [17731],
                    [19678],
                    [18649],
                    [17307],
                    [21798],
                    [17549],
                    [15630],
                ],
                [
                    [16384],
                    [16384],
                    [26585],
                    [21469],
                    [20432],
                    [17735],
                    [19280],
                    [15235],
                    [20297],
                    [22471],
                    [28997],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [26605],
                    [11304],
                    [16726],
                    [16560],
                    [20866],
                    [23524],
                    [19878],
                    [13469],
                    [23084],
                ],
                [
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
        ])),
        skip: Align4(cdf2d([
            [
                [30371],
                [7570],
                [13155],
                [20751],
                [20969],
                [27067],
                [32013],
                [5495],
                [17942],
                [28280],
                [16384],
                [16384],
                [16384],
            ],
            [
                [31782],
                [1836],
                [10689],
                [17604],
                [21622],
                [27518],
                [32399],
                [4419],
                [16294],
                [28345],
                [16384],
                [16384],
                [16384],
            ],
            [
                [31901],
                [10311],
                [18047],
                [24806],
                [23288],
                [27914],
                [32296],
                [4215],
                [15756],
                [28341],
                [16384],
                [16384],
                [16384],
            ],
            [
                [26726],
                [1045],
                [11703],
                [20590],
                [18554],
                [25970],
                [31938],
                [5583],
                [21313],
                [29390],
                [641],
                [22265],
                [31452],
            ],
            [
                [26584],
                [188],
                [8847],
                [24519],
                [22938],
                [30583],
                [32608],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
            ],
        ])),
        dc_sign: Align4(cdf2d([
            [[16000], [13056], [18816]],
            [[15232], [12928], [17280]],
        ])),
    },
    CdfCoefContext {
        eob_bin_16: Align16(cdf2d([
            [[4016, 4897, 8881, 14968], [716, 1105, 2646, 10056]],
            [[11139, 13270, 18241, 23566], [3192, 5032, 10297, 19755]],
        ])),
        eob_bin_32: Align16(cdf2d([
            [
                [2515, 3003, 4452, 8162, 16041],
                [574, 821, 1836, 5089, 13128],
            ],
            [
                [13468, 16303, 20361, 25105, 29281],
                [3542, 5502, 10415, 16760, 25644],
            ],
        ])),
        eob_bin_64: Align16(cdf2d([
            [
                [2374, 2772, 4583, 7276, 12288, 19706],
                [497, 810, 1315, 3000, 7004, 15641],
            ],
            [
                [15050, 17126, 21410, 24886, 28156, 30726],
                [4034, 6290, 10235, 14982, 21214, 28491],
            ],
        ])),
        eob_bin_128: Align16(cdf2d([
            [
                [1366, 1738, 2527, 5016, 9355, 15797, 24643],
                [354, 558, 944, 2760, 7287, 14037, 21779],
            ],
            [
                [13627, 16246, 20173, 24429, 27948, 30415, 31863],
                [6275, 9889, 14769, 23164, 27988, 30493, 32272],
            ],
        ])),
        eob_bin_256: Align32(cdf2d([
            [
                [3089, 3920, 6038, 9460, 14266, 19881, 25766, 29176],
                [1084, 2358, 3488, 5122, 11483, 18103, 26023, 29799],
            ],
            [
                [11514, 13794, 17480, 20754, 24361, 27378, 29492, 31277],
                [6571, 9610, 15516, 21826, 29092, 30829, 31842, 32708],
            ],
        ])),
        eob_bin_512: Align32(cdf1d([
            [2624, 3936, 6480, 9686, 13979, 17726, 23267, 28410, 31078],
            [
                12015, 14769, 19588, 22052, 24222, 25812, 27300, 29219, 32114,
            ],
        ])),
        eob_bin_1024: Align32(cdf1d([
            [
                2784, 3831, 7041, 10521, 14847, 18844, 23155, 26682, 29229, 31045,
            ],
            [
                9577, 12466, 17739, 20750, 22061, 23215, 24601, 25483, 25843, 32056,
            ],
        ])),
        eob_base_tok: Align8(cdf3d([
            [
                [
                    [20092, 30774],
                    [30695, 32020],
                    [31131, 32103],
                    [28666, 30870],
                ],
                [
                    [27258, 31095],
                    [31804, 32623],
                    [31763, 32528],
                    [31438, 32506],
                ],
            ],
            [
                [
                    [18049, 30489],
                    [31706, 32286],
                    [32163, 32473],
                    [31550, 32184],
                ],
                [
                    [27116, 30842],
                    [31971, 32598],
                    [32088, 32576],
                    [32067, 32664],
                ],
            ],
            [
                [
                    [12854, 29093],
                    [32272, 32558],
                    [32667, 32729],
                    [32306, 32585],
                ],
                [
                    [25476, 30366],
                    [32169, 32687],
                    [32479, 32689],
                    [31673, 32634],
                ],
            ],
            [
                [
                    [2809, 19301],
                    [32205, 32622],
                    [32338, 32730],
                    [31786, 32616],
                ],
                [
                    [22737, 29105],
                    [30810, 32362],
                    [30014, 32627],
                    [30528, 32574],
                ],
            ],
            [
                [[935, 3382], [30789, 31909], [32466, 32756], [30860, 32513]],
                [
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                ],
            ],
        ])),
        base_tok: Align8(cdf3d([
            [
                [
                    [8896, 16227, 20630],
                    [23629, 31782, 32527],
                    [15173, 27755, 31321],
                    [10158, 21233, 27382],
                    [6420, 14857, 21558],
                    [3269, 8155, 12646],
                    [24835, 32009, 32496],
                    [16509, 28421, 31579],
                    [10957, 21514, 27418],
                    [7881, 15930, 22096],
                    [5388, 10960, 15918],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [20745, 30773, 32093],
                    [15200, 27221, 30861],
                    [13032, 20873, 25667],
                    [12285, 18663, 23494],
                    [11563, 17481, 21489],
                    [26260, 31982, 32320],
                    [15397, 28083, 31100],
                    [9742, 19217, 24824],
                    [3261, 9629, 15362],
                    [1480, 4322, 7499],
                    [27599, 32256, 32460],
                    [16857, 27659, 30774],
                    [9551, 18290, 23748],
                    [3052, 8933, 14103],
                    [2021, 5910, 9787],
                    [29005, 32015, 32392],
                    [17677, 27694, 30863],
                    [9204, 17356, 23219],
                    [2403, 7516, 12814],
                    [8192, 16384, 24576],
                ],
                [
                    [10808, 22056, 26896],
                    [25739, 32313, 32676],
                    [17288, 30203, 32221],
                    [11359, 24878, 29896],
                    [6949, 17767, 24893],
                    [4287, 11796, 18071],
                    [27880, 32521, 32705],
                    [19038, 31004, 32414],
                    [12564, 26345, 30768],
                    [8269, 19947, 26779],
                    [5674, 14657, 21674],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [25742, 32319, 32671],
                    [19557, 31164, 32454],
                    [13381, 26381, 30755],
                    [10101, 21466, 26722],
                    [9209, 19650, 26825],
                    [27107, 31917, 32432],
                    [18056, 28893, 31203],
                    [10200, 21434, 26764],
                    [4660, 12913, 19502],
                    [2368, 6930, 12504],
                    [26960, 32158, 32613],
                    [18628, 30005, 32031],
                    [10233, 22442, 28232],
                    [5471, 14630, 21516],
                    [3235, 10767, 17109],
                    [27696, 32440, 32692],
                    [20032, 31167, 32438],
                    [8700, 21341, 28442],
                    [5662, 14831, 21795],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [9704, 17294, 21132],
                    [26762, 32278, 32633],
                    [18382, 29620, 31819],
                    [10891, 23475, 28723],
                    [6358, 16583, 23309],
                    [3248, 9118, 14141],
                    [27204, 32573, 32699],
                    [19818, 30824, 32329],
                    [11772, 25120, 30041],
                    [6995, 18033, 25039],
                    [3752, 10442, 16098],
                    [27222, 32256, 32559],
                    [15356, 28399, 31475],
                    [8821, 20635, 27057],
                    [5511, 14404, 21239],
                    [2935, 8222, 13051],
                    [24875, 32120, 32529],
                    [15233, 28265, 31445],
                    [8605, 20570, 26932],
                    [5431, 14413, 21196],
                    [2994, 8341, 13223],
                    [28201, 32604, 32700],
                    [21041, 31446, 32456],
                    [13221, 26213, 30475],
                    [8255, 19385, 26037],
                    [4930, 12585, 18830],
                    [28768, 32448, 32627],
                    [19705, 30561, 32021],
                    [11572, 23589, 28220],
                    [5532, 15034, 21446],
                    [2460, 7150, 11456],
                    [29874, 32619, 32699],
                    [21621, 31071, 32201],
                    [12511, 24747, 28992],
                    [6281, 16395, 22748],
                    [3246, 9278, 14497],
                    [29715, 32625, 32712],
                    [20958, 31011, 32283],
                    [11233, 23671, 28806],
                    [6012, 16128, 22868],
                    [3427, 9851, 15414],
                ],
                [
                    [11016, 22111, 26794],
                    [25946, 32357, 32677],
                    [17890, 30452, 32252],
                    [11678, 25142, 29816],
                    [6720, 17534, 24584],
                    [4230, 11665, 17820],
                    [28400, 32623, 32747],
                    [21164, 31668, 32575],
                    [13572, 27388, 31182],
                    [8234, 20750, 27358],
                    [5065, 14055, 20897],
                    [28981, 32547, 32705],
                    [18681, 30543, 32239],
                    [10919, 24075, 29286],
                    [6431, 17199, 24077],
                    [3819, 10464, 16618],
                    [26870, 32467, 32693],
                    [19041, 30831, 32347],
                    [11794, 25211, 30016],
                    [6888, 18019, 24970],
                    [4370, 12363, 18992],
                    [29578, 32670, 32744],
                    [23159, 32007, 32613],
                    [15315, 28669, 31676],
                    [9298, 22607, 28782],
                    [6144, 15913, 22968],
                    [28110, 32499, 32669],
                    [21574, 30937, 32015],
                    [12759, 24818, 28727],
                    [6545, 16761, 23042],
                    [3649, 10597, 16833],
                    [28163, 32552, 32728],
                    [22101, 31469, 32464],
                    [13160, 25472, 30143],
                    [7303, 18684, 25468],
                    [5241, 13975, 20955],
                    [28400, 32631, 32744],
                    [22104, 31793, 32603],
                    [13557, 26571, 30846],
                    [7749, 19861, 26675],
                    [4873, 14030, 21234],
                ],
            ],
            [
                [
                    [9800, 17635, 21073],
                    [26153, 31885, 32527],
                    [15038, 27852, 31006],
                    [8718, 20564, 26486],
                    [5128, 14076, 20514],
                    [2636, 7566, 11925],
                    [27551, 32504, 32701],
                    [18310, 30054, 32100],
                    [10211, 23420, 29082],
                    [6222, 16876, 23916],
                    [3462, 9954, 15498],
                    [29991, 32633, 32721],
                    [19883, 30751, 32201],
                    [11141, 24184, 29285],
                    [6420, 16940, 23774],
                    [3392, 9753, 15118],
                    [28465, 32616, 32712],
                    [19850, 30702, 32244],
                    [10983, 24024, 29223],
                    [6294, 16770, 23582],
                    [3244, 9283, 14509],
                    [30023, 32717, 32748],
                    [22940, 32032, 32626],
                    [14282, 27928, 31473],
                    [8562, 21327, 27914],
                    [4846, 13393, 19919],
                    [29981, 32590, 32695],
                    [20465, 30963, 32166],
                    [11479, 23579, 28195],
                    [5916, 15648, 22073],
                    [3031, 8605, 13398],
                    [31146, 32691, 32739],
                    [23106, 31724, 32444],
                    [13783, 26738, 30439],
                    [7852, 19468, 25807],
                    [3860, 11124, 16853],
                    [31014, 32724, 32748],
                    [23629, 32109, 32628],
                    [14747, 28115, 31403],
                    [8545, 21242, 27478],
                    [4574, 12781, 19067],
                ],
                [
                    [9185, 19694, 24688],
                    [26081, 31985, 32621],
                    [16015, 29000, 31787],
                    [10542, 23690, 29206],
                    [6732, 17945, 24677],
                    [3916, 11039, 16722],
                    [28224, 32566, 32744],
                    [19100, 31138, 32485],
                    [12528, 26620, 30879],
                    [7741, 20277, 26885],
                    [4566, 12845, 18990],
                    [29933, 32593, 32718],
                    [17670, 30333, 32155],
                    [10385, 23600, 28909],
                    [6243, 16236, 22407],
                    [3976, 10389, 16017],
                    [28377, 32561, 32738],
                    [19366, 31175, 32482],
                    [13327, 27175, 31094],
                    [8258, 20769, 27143],
                    [4703, 13198, 19527],
                    [31086, 32706, 32748],
                    [22853, 31902, 32583],
                    [14759, 28186, 31419],
                    [9284, 22382, 28348],
                    [5585, 15192, 21868],
                    [28291, 32652, 32746],
                    [19849, 32107, 32571],
                    [14834, 26818, 29214],
                    [10306, 22594, 28672],
                    [6615, 17384, 23384],
                    [28947, 32604, 32745],
                    [25625, 32289, 32646],
                    [18758, 28672, 31403],
                    [10017, 23430, 28523],
                    [6862, 15269, 22131],
                    [23933, 32509, 32739],
                    [19927, 31495, 32631],
                    [11903, 26023, 30621],
                    [7026, 20094, 27252],
                    [5998, 18106, 24437],
                ],
            ],
            [
                [
                    [4456, 11274, 15533],
                    [21219, 29079, 31616],
                    [11173, 23774, 28567],
                    [7282, 18293, 24263],
                    [4890, 13286, 19115],
                    [1890, 5508, 8659],
                    [26651, 32136, 32647],
                    [14630, 28254, 31455],
                    [8716, 21287, 27395],
                    [5615, 15331, 22008],
                    [2675, 7700, 12150],
                    [29954, 32526, 32690],
                    [16126, 28982, 31633],
                    [9030, 21361, 27352],
                    [5411, 14793, 21271],
                    [2943, 8422, 13163],
                    [29539, 32601, 32730],
                    [18125, 30385, 32201],
                    [10422, 24090, 29468],
                    [6468, 17487, 24438],
                    [2970, 8653, 13531],
                    [30912, 32715, 32748],
                    [20666, 31373, 32497],
                    [12509, 26640, 30917],
                    [8058, 20629, 27290],
                    [4231, 12006, 18052],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [10202, 20633, 25484],
                    [27336, 31445, 32352],
                    [12420, 24384, 28552],
                    [7648, 18115, 23856],
                    [5662, 14341, 19902],
                    [3611, 10328, 15390],
                    [30945, 32616, 32736],
                    [18682, 30505, 32253],
                    [11513, 25336, 30203],
                    [7449, 19452, 26148],
                    [4482, 13051, 18886],
                    [32022, 32690, 32747],
                    [18578, 30501, 32146],
                    [11249, 23368, 28631],
                    [5645, 16958, 22158],
                    [5009, 11444, 16637],
                    [31357, 32710, 32748],
                    [21552, 31494, 32504],
                    [13891, 27677, 31340],
                    [9051, 22098, 28172],
                    [5190, 13377, 19486],
                    [32364, 32740, 32748],
                    [24839, 31907, 32551],
                    [17160, 28779, 31696],
                    [12452, 24137, 29602],
                    [6165, 15389, 22477],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [2575, 7281, 11077],
                    [14002, 20866, 25402],
                    [6343, 15056, 19658],
                    [4474, 11858, 17041],
                    [2865, 8299, 12534],
                    [1344, 3949, 6391],
                    [24720, 31239, 32459],
                    [12585, 25356, 29968],
                    [7181, 18246, 24444],
                    [5025, 13667, 19885],
                    [2521, 7304, 11605],
                    [29908, 32252, 32584],
                    [17421, 29156, 31575],
                    [9889, 22188, 27782],
                    [5878, 15647, 22123],
                    [2814, 8665, 13323],
                    [30183, 32568, 32713],
                    [18528, 30195, 32049],
                    [10982, 24606, 29657],
                    [6957, 18165, 25231],
                    [3508, 10118, 15468],
                    [31761, 32736, 32748],
                    [21041, 31328, 32546],
                    [12568, 26732, 31166],
                    [8052, 20720, 27733],
                    [4336, 12192, 18396],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
        ])),
        br_tok: Align8(cdf3d([
            [
                [
                    [16138, 22223, 25509],
                    [15347, 22430, 26332],
                    [9614, 16736, 21332],
                    [6600, 12275, 16907],
                    [4811, 9424, 13547],
                    [3748, 7809, 11420],
                    [2254, 4587, 6890],
                    [15196, 20284, 23177],
                    [18317, 25469, 28451],
                    [13918, 21651, 25842],
                    [10052, 17150, 21995],
                    [7499, 13630, 18587],
                    [6158, 11417, 16003],
                    [4014, 7785, 11252],
                    [15048, 21067, 24384],
                    [18202, 25346, 28553],
                    [14302, 22019, 26356],
                    [10839, 18139, 23166],
                    [8715, 15744, 20806],
                    [7536, 13576, 18544],
                    [5413, 10335, 14498],
                ],
                [
                    [17394, 24501, 27895],
                    [15889, 23420, 27185],
                    [11561, 19133, 23870],
                    [8285, 14812, 19844],
                    [6496, 12043, 16550],
                    [4771, 9574, 13677],
                    [3603, 6830, 10144],
                    [21656, 27704, 30200],
                    [21324, 27915, 30511],
                    [17327, 25336, 28997],
                    [13417, 21381, 26033],
                    [10132, 17425, 22338],
                    [8580, 15016, 19633],
                    [5694, 11477, 16411],
                    [24116, 29780, 31450],
                    [23853, 29695, 31591],
                    [20085, 27614, 30428],
                    [15326, 24335, 28575],
                    [11814, 19472, 24810],
                    [10221, 18611, 24767],
                    [7689, 14558, 20321],
                ],
            ],
            [
                [
                    [16214, 22380, 25770],
                    [14213, 21304, 25295],
                    [9213, 15823, 20455],
                    [6395, 11758, 16139],
                    [4779, 9187, 13066],
                    [3821, 7501, 10953],
                    [2293, 4567, 6795],
                    [15859, 21283, 23820],
                    [18404, 25602, 28726],
                    [14325, 21980, 26206],
                    [10669, 17937, 22720],
                    [8297, 14642, 19447],
                    [6746, 12389, 16893],
                    [4324, 8251, 11770],
                    [16532, 21631, 24475],
                    [20667, 27150, 29668],
                    [16728, 24510, 28175],
                    [12861, 20645, 25332],
                    [10076, 17361, 22417],
                    [8395, 14940, 19963],
                    [5731, 10683, 14912],
                ],
                [
                    [14433, 21155, 24938],
                    [14658, 21716, 25545],
                    [9923, 16824, 21557],
                    [6982, 13052, 17721],
                    [5419, 10503, 15050],
                    [4852, 9162, 13014],
                    [3271, 6395, 9630],
                    [22210, 27833, 30109],
                    [20750, 27368, 29821],
                    [16894, 24828, 28573],
                    [13247, 21276, 25757],
                    [10038, 17265, 22563],
                    [8587, 14947, 20327],
                    [5645, 11371, 15252],
                    [22027, 27526, 29714],
                    [23098, 29146, 31221],
                    [19886, 27341, 30272],
                    [15609, 23747, 28046],
                    [11993, 20065, 24939],
                    [9637, 18267, 23671],
                    [7625, 13801, 19144],
                ],
            ],
            [
                [
                    [14438, 20798, 24089],
                    [12621, 19203, 23097],
                    [8177, 14125, 18402],
                    [5674, 10501, 14456],
                    [4236, 8239, 11733],
                    [3447, 6750, 9806],
                    [1986, 3950, 5864],
                    [16208, 22099, 24930],
                    [16537, 24025, 27585],
                    [12780, 20381, 24867],
                    [9767, 16612, 21416],
                    [7686, 13738, 18398],
                    [6333, 11614, 15964],
                    [3941, 7571, 10836],
                    [22819, 27422, 29202],
                    [22224, 28514, 30721],
                    [17660, 25433, 28913],
                    [13574, 21482, 26002],
                    [10629, 17977, 22938],
                    [8612, 15298, 20265],
                    [5607, 10491, 14596],
                ],
                [
                    [13569, 19800, 23206],
                    [13128, 19924, 23869],
                    [8329, 14841, 19403],
                    [6130, 10976, 15057],
                    [4682, 8839, 12518],
                    [3656, 7409, 10588],
                    [2577, 5099, 7412],
                    [22427, 28684, 30585],
                    [20913, 27750, 30139],
                    [15840, 24109, 27834],
                    [12308, 20029, 24569],
                    [10216, 16785, 21458],
                    [8309, 14203, 19113],
                    [6043, 11168, 15307],
                    [23166, 28901, 30998],
                    [21899, 28405, 30751],
                    [18413, 26091, 29443],
                    [15233, 23114, 27352],
                    [12683, 20472, 25288],
                    [10702, 18259, 23409],
                    [8125, 14464, 19226],
                ],
            ],
            [
                [
                    [9040, 14786, 18360],
                    [9979, 15718, 19415],
                    [7913, 13918, 18311],
                    [5859, 10889, 15184],
                    [4593, 8677, 12510],
                    [3820, 7396, 10791],
                    [1730, 3471, 5192],
                    [11803, 18365, 22709],
                    [11419, 18058, 22225],
                    [9418, 15774, 20243],
                    [7539, 13325, 17657],
                    [6233, 11317, 15384],
                    [5137, 9656, 13545],
                    [2977, 5774, 8349],
                    [21207, 27246, 29640],
                    [19547, 26578, 29497],
                    [16169, 23871, 27690],
                    [12820, 20458, 25018],
                    [10224, 17332, 22214],
                    [8526, 15048, 19884],
                    [5037, 9410, 13118],
                ],
                [
                    [12339, 17329, 20140],
                    [13505, 19895, 23225],
                    [9847, 16944, 21564],
                    [7280, 13256, 18348],
                    [4712, 10009, 14454],
                    [4361, 7914, 12477],
                    [2870, 5628, 7995],
                    [20061, 25504, 28526],
                    [15235, 22878, 26145],
                    [12985, 19958, 24155],
                    [9782, 16641, 21403],
                    [9456, 16360, 20760],
                    [6855, 12940, 18557],
                    [5661, 10564, 15002],
                    [25656, 30602, 31894],
                    [22570, 29107, 31092],
                    [18917, 26423, 29541],
                    [15940, 23649, 27754],
                    [12803, 20581, 25219],
                    [11082, 18695, 23376],
                    [7939, 14373, 19005],
                ],
            ],
        ])),
        eob_hi_bit: Align4(cdf3d([
            [
                [
                    [16384],
                    [16384],
                    [18983],
                    [20512],
                    [14885],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [20090],
                    [19444],
                    [17286],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [19139],
                    [21487],
                    [18959],
                    [20910],
                    [19089],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [20536],
                    [20664],
                    [20625],
                    [19123],
                    [14862],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [19833],
                    [21502],
                    [17485],
                    [20267],
                    [18353],
                    [23329],
                    [21478],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [22041],
                    [23434],
                    [20001],
                    [20554],
                    [20951],
                    [20145],
                    [15562],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [23312],
                    [21607],
                    [16526],
                    [18957],
                    [18034],
                    [18934],
                    [24247],
                    [16921],
                    [17080],
                ],
                [
                    [16384],
                    [16384],
                    [26579],
                    [24910],
                    [18637],
                    [19800],
                    [20388],
                    [9887],
                    [15642],
                    [30198],
                    [24721],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [26998],
                    [16737],
                    [17838],
                    [18922],
                    [19515],
                    [18636],
                    [17333],
                    [15776],
                    [22658],
                ],
                [
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
        ])),
        skip: Align4(cdf2d([
            [
                [29614],
                [9068],
                [12924],
                [19538],
                [17737],
                [24619],
                [30642],
                [4119],
                [16026],
                [25657],
                [16384],
                [16384],
                [16384],
            ],
            [
                [31957],
                [3230],
                [11153],
                [18123],
                [20143],
                [26536],
                [31986],
                [3050],
                [14603],
                [25155],
                [16384],
                [16384],
                [16384],
            ],
            [
                [32363],
                [10692],
                [19090],
                [24357],
                [24442],
                [28312],
                [32169],
                [3648],
                [15690],
                [26815],
                [16384],
                [16384],
                [16384],
            ],
            [
                [30669],
                [3832],
                [11663],
                [18889],
                [19782],
                [23313],
                [31330],
                [5124],
                [18719],
                [28468],
                [3082],
                [20982],
                [29443],
            ],
            [
                [28573],
                [3183],
                [17802],
                [25977],
                [26677],
                [27832],
                [32387],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
            ],
        ])),
        dc_sign: Align4(cdf2d([
            [[16000], [13056], [18816]],
            [[15232], [12928], [17280]],
        ])),
    },
    CdfCoefContext {
        eob_bin_16: Align16(cdf2d([
            [[6708, 8958, 14746, 22133], [1222, 2074, 4783, 15410]],
            [[19575, 21766, 26044, 29709], [7297, 10767, 19273, 28194]],
        ])),
        eob_bin_32: Align16(cdf2d([
            [
                [4617, 5709, 8446, 13584, 23135],
                [1156, 1702, 3675, 9274, 20539],
            ],
            [
                [22086, 24282, 27010, 29770, 31743],
                [7699, 10897, 20891, 26926, 31628],
            ],
        ])),
        eob_bin_64: Align16(cdf2d([
            [
                [6307, 7541, 12060, 16358, 22553, 27865],
                [1289, 2320, 3971, 7926, 14153, 24291],
            ],
            [
                [24212, 25708, 28268, 30035, 31307, 32049],
                [8726, 12378, 19409, 26450, 30038, 32462],
            ],
        ])),
        eob_bin_128: Align16(cdf2d([
            [
                [3472, 4885, 7489, 12481, 18517, 24536, 29635],
                [886, 1731, 3271, 8469, 15569, 22126, 28383],
            ],
            [
                [24313, 26062, 28385, 30107, 31217, 31898, 32345],
                [9165, 13282, 21150, 30286, 31894, 32571, 32712],
            ],
        ])),
        eob_bin_256: Align32(cdf2d([
            [
                [5348, 7113, 11820, 15924, 22106, 26777, 30334, 31757],
                [2453, 4474, 6307, 8777, 16474, 22975, 29000, 31547],
            ],
            [
                [23110, 24597, 27140, 28894, 30167, 30927, 31392, 32094],
                [9998, 17661, 25178, 28097, 31308, 32038, 32403, 32695],
            ],
        ])),
        eob_bin_512: Align32(cdf1d([
            [5927, 7809, 10923, 14597, 19439, 24135, 28456, 31142, 32060],
            [
                21093, 23043, 25742, 27658, 29097, 29716, 30073, 30820, 31956,
            ],
        ])),
        eob_bin_1024: Align32(cdf1d([
            [
                6698, 8334, 11961, 15762, 20186, 23862, 27434, 29326, 31082, 32050,
            ],
            [
                20569, 22426, 25569, 26859, 28053, 28913, 29486, 29724, 29807, 32570,
            ],
        ])),
        eob_base_tok: Align8(cdf3d([
            [
                [
                    [22497, 31198],
                    [31715, 32495],
                    [31606, 32337],
                    [30388, 31990],
                ],
                [
                    [27877, 31584],
                    [32170, 32728],
                    [32155, 32688],
                    [32219, 32702],
                ],
            ],
            [
                [
                    [21457, 31043],
                    [31951, 32483],
                    [32153, 32562],
                    [31473, 32215],
                ],
                [
                    [27558, 31151],
                    [32020, 32640],
                    [32097, 32575],
                    [32242, 32719],
                ],
            ],
            [
                [
                    [19980, 30591],
                    [32219, 32597],
                    [32581, 32706],
                    [31803, 32287],
                ],
                [
                    [26473, 30507],
                    [32431, 32723],
                    [32196, 32611],
                    [31588, 32528],
                ],
            ],
            [
                [
                    [24647, 30463],
                    [32412, 32695],
                    [32468, 32720],
                    [31269, 32523],
                ],
                [
                    [28482, 31505],
                    [32152, 32701],
                    [31732, 32598],
                    [31767, 32712],
                ],
            ],
            [
                [
                    [12358, 24977],
                    [31331, 32385],
                    [32634, 32756],
                    [30411, 32548],
                ],
                [
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                    [10923, 21845],
                ],
            ],
        ])),
        base_tok: Align8(cdf3d([
            [
                [
                    [7062, 16472, 22319],
                    [24538, 32261, 32674],
                    [13675, 28041, 31779],
                    [8590, 20674, 27631],
                    [5685, 14675, 22013],
                    [3655, 9898, 15731],
                    [26493, 32418, 32658],
                    [16376, 29342, 32090],
                    [10594, 22649, 28970],
                    [8176, 17170, 24303],
                    [5605, 12694, 19139],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [23888, 31902, 32542],
                    [18612, 29687, 31987],
                    [16245, 24852, 29249],
                    [15765, 22608, 27559],
                    [19895, 24699, 27510],
                    [28401, 32212, 32457],
                    [15274, 27825, 30980],
                    [9364, 18128, 24332],
                    [2283, 8193, 15082],
                    [1228, 3972, 7881],
                    [29455, 32469, 32620],
                    [17981, 28245, 31388],
                    [10921, 20098, 26240],
                    [3743, 11829, 18657],
                    [2374, 9593, 15715],
                    [31068, 32466, 32635],
                    [20321, 29572, 31971],
                    [10771, 20255, 27119],
                    [2795, 10410, 17361],
                    [8192, 16384, 24576],
                ],
                [
                    [9320, 22102, 27840],
                    [27057, 32464, 32724],
                    [16331, 30268, 32309],
                    [10319, 23935, 29720],
                    [6189, 16448, 24106],
                    [3589, 10884, 18808],
                    [29026, 32624, 32748],
                    [19226, 31507, 32587],
                    [12692, 26921, 31203],
                    [7049, 19532, 27635],
                    [7727, 15669, 23252],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [28056, 32625, 32748],
                    [22383, 32075, 32669],
                    [15417, 27098, 31749],
                    [18127, 26493, 27190],
                    [5461, 16384, 21845],
                    [27982, 32091, 32584],
                    [19045, 29868, 31972],
                    [10397, 22266, 27932],
                    [5990, 13697, 21500],
                    [1792, 6912, 15104],
                    [28198, 32501, 32718],
                    [21534, 31521, 32569],
                    [11109, 25217, 30017],
                    [5671, 15124, 26151],
                    [4681, 14043, 18725],
                    [28688, 32580, 32741],
                    [22576, 32079, 32661],
                    [10627, 22141, 28340],
                    [9362, 14043, 28087],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [7754, 16948, 22142],
                    [25670, 32330, 32691],
                    [15663, 29225, 31994],
                    [9878, 23288, 29158],
                    [6419, 17088, 24336],
                    [3859, 11003, 17039],
                    [27562, 32595, 32725],
                    [17575, 30588, 32399],
                    [10819, 24838, 30309],
                    [7124, 18686, 25916],
                    [4479, 12688, 19340],
                    [28385, 32476, 32673],
                    [15306, 29005, 31938],
                    [8937, 21615, 28322],
                    [5982, 15603, 22786],
                    [3620, 10267, 16136],
                    [27280, 32464, 32667],
                    [15607, 29160, 32004],
                    [9091, 22135, 28740],
                    [6232, 16632, 24020],
                    [4047, 11377, 17672],
                    [29220, 32630, 32718],
                    [19650, 31220, 32462],
                    [13050, 26312, 30827],
                    [9228, 20870, 27468],
                    [6146, 15149, 21971],
                    [30169, 32481, 32623],
                    [17212, 29311, 31554],
                    [9911, 21311, 26882],
                    [4487, 13314, 20372],
                    [2570, 7772, 12889],
                    [30924, 32613, 32708],
                    [19490, 30206, 32107],
                    [11232, 23998, 29276],
                    [6769, 17955, 25035],
                    [4398, 12623, 19214],
                    [30609, 32627, 32722],
                    [19370, 30582, 32287],
                    [10457, 23619, 29409],
                    [6443, 17637, 24834],
                    [4645, 13236, 20106],
                ],
                [
                    [8626, 20271, 26216],
                    [26707, 32406, 32711],
                    [16999, 30329, 32286],
                    [11445, 25123, 30286],
                    [6411, 18828, 25601],
                    [6801, 12458, 20248],
                    [29918, 32682, 32748],
                    [20649, 31739, 32618],
                    [12879, 27773, 31581],
                    [7896, 21751, 28244],
                    [5260, 14870, 23698],
                    [29252, 32593, 32731],
                    [17072, 30460, 32294],
                    [10653, 24143, 29365],
                    [6536, 17490, 23983],
                    [4929, 13170, 20085],
                    [28137, 32518, 32715],
                    [18171, 30784, 32407],
                    [11437, 25436, 30459],
                    [7252, 18534, 26176],
                    [4126, 13353, 20978],
                    [31162, 32726, 32748],
                    [23017, 32222, 32701],
                    [15629, 29233, 32046],
                    [9387, 22621, 29480],
                    [6922, 17616, 25010],
                    [28838, 32265, 32614],
                    [19701, 30206, 31920],
                    [11214, 22410, 27933],
                    [5320, 14177, 23034],
                    [5049, 12881, 17827],
                    [27484, 32471, 32734],
                    [21076, 31526, 32561],
                    [12707, 26303, 31211],
                    [8169, 21722, 28219],
                    [6045, 19406, 27042],
                    [27753, 32572, 32745],
                    [20832, 31878, 32653],
                    [13250, 27356, 31674],
                    [7718, 21508, 29858],
                    [7209, 18350, 25559],
                ],
            ],
            [
                [
                    [7876, 16901, 21741],
                    [24001, 31898, 32625],
                    [14529, 27959, 31451],
                    [8273, 20818, 27258],
                    [5278, 14673, 21510],
                    [2983, 8843, 14039],
                    [28016, 32574, 32732],
                    [17471, 30306, 32301],
                    [10224, 24063, 29728],
                    [6602, 17954, 25052],
                    [4002, 11585, 17759],
                    [30190, 32634, 32739],
                    [17497, 30282, 32270],
                    [10229, 23729, 29538],
                    [6344, 17211, 24440],
                    [3849, 11189, 17108],
                    [28570, 32583, 32726],
                    [17521, 30161, 32238],
                    [10153, 23565, 29378],
                    [6455, 17341, 24443],
                    [3907, 11042, 17024],
                    [30689, 32715, 32748],
                    [21546, 31840, 32610],
                    [13547, 27581, 31459],
                    [8912, 21757, 28309],
                    [5548, 15080, 22046],
                    [30783, 32540, 32685],
                    [17540, 29528, 31668],
                    [10160, 21468, 26783],
                    [4724, 13393, 20054],
                    [2702, 8174, 13102],
                    [31648, 32686, 32742],
                    [20954, 31094, 32337],
                    [12420, 25698, 30179],
                    [7304, 19320, 26248],
                    [4366, 12261, 18864],
                    [31581, 32723, 32748],
                    [21373, 31586, 32525],
                    [12744, 26625, 30885],
                    [7431, 20322, 26950],
                    [4692, 13323, 20111],
                ],
                [
                    [7833, 18369, 24095],
                    [26650, 32273, 32702],
                    [16371, 29961, 32191],
                    [11055, 24082, 29629],
                    [6892, 18644, 25400],
                    [5006, 13057, 19240],
                    [29834, 32666, 32748],
                    [19577, 31335, 32570],
                    [12253, 26509, 31122],
                    [7991, 20772, 27711],
                    [5677, 15910, 23059],
                    [30109, 32532, 32720],
                    [16747, 30166, 32252],
                    [10134, 23542, 29184],
                    [5791, 16176, 23556],
                    [4362, 10414, 17284],
                    [29492, 32626, 32748],
                    [19894, 31402, 32525],
                    [12942, 27071, 30869],
                    [8346, 21216, 27405],
                    [6572, 17087, 23859],
                    [32035, 32735, 32748],
                    [22957, 31838, 32618],
                    [14724, 28572, 31772],
                    [10364, 23999, 29553],
                    [7004, 18433, 25655],
                    [27528, 32277, 32681],
                    [16959, 31171, 32096],
                    [10486, 23593, 27962],
                    [8192, 16384, 23211],
                    [8937, 17873, 20852],
                    [27715, 32002, 32615],
                    [15073, 29491, 31676],
                    [11264, 24576, 28672],
                    [2341, 18725, 23406],
                    [7282, 18204, 25486],
                    [28547, 32213, 32657],
                    [20788, 29773, 32239],
                    [6780, 21469, 30508],
                    [5958, 14895, 23831],
                    [16384, 21845, 27307],
                ],
            ],
            [
                [
                    [5992, 14304, 19765],
                    [22612, 31238, 32456],
                    [13456, 27162, 31087],
                    [8001, 20062, 26504],
                    [5168, 14105, 20764],
                    [2632, 7771, 12385],
                    [27034, 32344, 32709],
                    [15850, 29415, 31997],
                    [9494, 22776, 28841],
                    [6151, 16830, 23969],
                    [3461, 10039, 15722],
                    [30134, 32569, 32731],
                    [15638, 29422, 31945],
                    [9150, 21865, 28218],
                    [5647, 15719, 22676],
                    [3402, 9772, 15477],
                    [28530, 32586, 32735],
                    [17139, 30298, 32292],
                    [10200, 24039, 29685],
                    [6419, 17674, 24786],
                    [3544, 10225, 15824],
                    [31333, 32726, 32748],
                    [20618, 31487, 32544],
                    [12901, 27217, 31232],
                    [8624, 21734, 28171],
                    [5104, 14191, 20748],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [11206, 21090, 26561],
                    [28759, 32279, 32671],
                    [14171, 27952, 31569],
                    [9743, 22907, 29141],
                    [6871, 17886, 24868],
                    [4960, 13152, 19315],
                    [31077, 32661, 32748],
                    [19400, 31195, 32515],
                    [12752, 26858, 31040],
                    [8370, 22098, 28591],
                    [5457, 15373, 22298],
                    [31697, 32706, 32748],
                    [17860, 30657, 32333],
                    [12510, 24812, 29261],
                    [6180, 19124, 24722],
                    [5041, 13548, 17959],
                    [31552, 32716, 32748],
                    [21908, 31769, 32623],
                    [14470, 28201, 31565],
                    [9493, 22982, 28608],
                    [6858, 17240, 24137],
                    [32543, 32752, 32756],
                    [24286, 32097, 32666],
                    [15958, 29217, 32024],
                    [10207, 24234, 29958],
                    [6929, 18305, 25652],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
            [
                [
                    [4137, 10847, 15682],
                    [17824, 27001, 30058],
                    [10204, 22796, 28291],
                    [6076, 15935, 22125],
                    [3852, 10937, 16816],
                    [2252, 6324, 10131],
                    [25840, 32016, 32662],
                    [15109, 28268, 31531],
                    [9385, 22231, 28340],
                    [6082, 16672, 23479],
                    [3318, 9427, 14681],
                    [30594, 32574, 32718],
                    [16836, 29552, 31859],
                    [9556, 22542, 28356],
                    [6305, 16725, 23540],
                    [3376, 9895, 15184],
                    [29383, 32617, 32745],
                    [18891, 30809, 32401],
                    [11688, 25942, 30687],
                    [7468, 19469, 26651],
                    [3909, 11358, 17012],
                    [31564, 32736, 32748],
                    [20906, 31611, 32600],
                    [13191, 27621, 31537],
                    [8768, 22029, 28676],
                    [5079, 14109, 20906],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
                [
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                    [8192, 16384, 24576],
                ],
            ],
        ])),
        br_tok: Align8(cdf3d([
            [
                [
                    [18315, 24289, 27551],
                    [16854, 24068, 27835],
                    [10140, 17927, 23173],
                    [6722, 12982, 18267],
                    [4661, 9826, 14706],
                    [3832, 8165, 12294],
                    [2795, 6098, 9245],
                    [17145, 23326, 26672],
                    [20733, 27680, 30308],
                    [16032, 24461, 28546],
                    [11653, 20093, 25081],
                    [9290, 16429, 22086],
                    [7796, 14598, 19982],
                    [6502, 12378, 17441],
                    [21681, 27732, 30320],
                    [22389, 29044, 31261],
                    [19027, 26731, 30087],
                    [14739, 23755, 28624],
                    [11358, 20778, 25511],
                    [10995, 18073, 24190],
                    [9162, 14990, 20617],
                ],
                [
                    [21425, 27952, 30388],
                    [18062, 25838, 29034],
                    [11956, 19881, 24808],
                    [7718, 15000, 20980],
                    [5702, 11254, 16143],
                    [4898, 9088, 16864],
                    [3679, 6776, 11907],
                    [23294, 30160, 31663],
                    [24397, 29896, 31836],
                    [19245, 27128, 30593],
                    [13202, 19825, 26404],
                    [11578, 19297, 23957],
                    [8073, 13297, 21370],
                    [5461, 10923, 19745],
                    [27367, 30521, 31934],
                    [24904, 30671, 31940],
                    [23075, 28460, 31299],
                    [14400, 23658, 30417],
                    [13885, 23882, 28325],
                    [14746, 22938, 27853],
                    [5461, 16384, 27307],
                ],
            ],
            [
                [
                    [18274, 24813, 27890],
                    [15537, 23149, 27003],
                    [9449, 16740, 21827],
                    [6700, 12498, 17261],
                    [4988, 9866, 14198],
                    [4236, 8147, 11902],
                    [2867, 5860, 8654],
                    [17124, 23171, 26101],
                    [20396, 27477, 30148],
                    [16573, 24629, 28492],
                    [12749, 20846, 25674],
                    [10233, 17878, 22818],
                    [8525, 15332, 20363],
                    [6283, 11632, 16255],
                    [20466, 26511, 29286],
                    [23059, 29174, 31191],
                    [19481, 27263, 30241],
                    [15458, 23631, 28137],
                    [12416, 20608, 25693],
                    [10261, 18011, 23261],
                    [8016, 14655, 19666],
                ],
                [
                    [17616, 24586, 28112],
                    [15809, 23299, 27155],
                    [10767, 18890, 23793],
                    [7727, 14255, 18865],
                    [6129, 11926, 16882],
                    [4482, 9704, 14861],
                    [3277, 7452, 11522],
                    [22956, 28551, 30730],
                    [22724, 28937, 30961],
                    [18467, 26324, 29580],
                    [13234, 20713, 25649],
                    [11181, 17592, 22481],
                    [8291, 18358, 24576],
                    [7568, 11881, 14984],
                    [24948, 29001, 31147],
                    [25674, 30619, 32151],
                    [20841, 26793, 29603],
                    [14669, 24356, 28666],
                    [11334, 23593, 28219],
                    [8922, 14762, 22873],
                    [8301, 13544, 20535],
                ],
            ],
            [
                [
                    [17113, 23733, 27081],
                    [14139, 21406, 25452],
                    [8552, 15002, 19776],
                    [5871, 11120, 15378],
                    [4455, 8616, 12253],
                    [3469, 6910, 10386],
                    [2255, 4553, 6782],
                    [18224, 24376, 27053],
                    [19290, 26710, 29614],
                    [14936, 22991, 27184],
                    [11238, 18951, 23762],
                    [8786, 15617, 20588],
                    [7317, 13228, 18003],
                    [5101, 9512, 13493],
                    [22639, 28222, 30210],
                    [23216, 29331, 31307],
                    [19075, 26762, 29895],
                    [15014, 23113, 27457],
                    [11938, 19857, 24752],
                    [9942, 17280, 22282],
                    [7167, 13144, 17752],
                ],
                [
                    [15820, 22738, 26488],
                    [13530, 20885, 25216],
                    [8395, 15530, 20452],
                    [6574, 12321, 16380],
                    [5353, 10419, 14568],
                    [4613, 8446, 12381],
                    [3440, 7158, 9903],
                    [24247, 29051, 31224],
                    [22118, 28058, 30369],
                    [16498, 24768, 28389],
                    [12920, 21175, 26137],
                    [10730, 18619, 25352],
                    [10187, 16279, 22791],
                    [9310, 14631, 22127],
                    [24970, 30558, 32057],
                    [24801, 29942, 31698],
                    [22432, 28453, 30855],
                    [19054, 25680, 29580],
                    [14392, 23036, 28109],
                    [12495, 20947, 26650],
                    [12442, 20326, 26214],
                ],
            ],
            [
                [
                    [12162, 18785, 22648],
                    [12749, 19697, 23806],
                    [8580, 15297, 20346],
                    [6169, 11749, 16543],
                    [4836, 9391, 13448],
                    [3821, 7711, 11613],
                    [2228, 4601, 7070],
                    [16319, 24725, 28280],
                    [15698, 23277, 27168],
                    [12726, 20368, 25047],
                    [9912, 17015, 21976],
                    [7888, 14220, 19179],
                    [6777, 12284, 17018],
                    [4492, 8590, 12252],
                    [23249, 28904, 30947],
                    [21050, 27908, 30512],
                    [17440, 25340, 28949],
                    [14059, 22018, 26541],
                    [11288, 18903, 23898],
                    [9411, 16342, 21428],
                    [6278, 11588, 15944],
                ],
                [
                    [13981, 20067, 23226],
                    [16922, 23580, 26783],
                    [11005, 19039, 24487],
                    [7389, 14218, 19798],
                    [5598, 11505, 17206],
                    [6090, 11213, 15659],
                    [3820, 7371, 10119],
                    [21082, 26925, 29675],
                    [21262, 28627, 31128],
                    [18392, 26454, 30437],
                    [14870, 22910, 27096],
                    [12620, 19484, 24908],
                    [9290, 16553, 22802],
                    [6668, 14288, 20004],
                    [27704, 31055, 31949],
                    [24709, 29978, 31788],
                    [21668, 29264, 31657],
                    [18295, 26968, 30074],
                    [16399, 24422, 29313],
                    [14347, 23026, 28104],
                    [12370, 19806, 24477],
                ],
            ],
        ])),
        eob_hi_bit: Align4(cdf3d([
            [
                [
                    [16384],
                    [16384],
                    [20177],
                    [20789],
                    [20262],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [21416],
                    [20855],
                    [23410],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [20238],
                    [21057],
                    [19159],
                    [22337],
                    [20159],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [20125],
                    [20559],
                    [21707],
                    [22296],
                    [17333],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [19941],
                    [20527],
                    [21470],
                    [22487],
                    [19558],
                    [22354],
                    [20331],
                    [16384],
                    [16384],
                ],
                [
                    [16384],
                    [16384],
                    [22752],
                    [25006],
                    [22075],
                    [21576],
                    [17740],
                    [21690],
                    [19211],
                    [16384],
                    [16384],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [21442],
                    [22358],
                    [18503],
                    [20291],
                    [19945],
                    [21294],
                    [21178],
                    [19400],
                    [10556],
                ],
                [
                    [16384],
                    [16384],
                    [24648],
                    [24949],
                    [20708],
                    [23905],
                    [20501],
                    [9558],
                    [9423],
                    [30365],
                    [19253],
                ],
            ],
            [
                [
                    [16384],
                    [16384],
                    [26064],
                    [22098],
                    [19613],
                    [20525],
                    [17595],
                    [16618],
                    [20497],
                    [18989],
                    [15513],
                ],
                [
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                    [16384],
                ],
            ],
        ])),
        skip: Align4(cdf2d([
            [
                [26887],
                [6729],
                [10361],
                [17442],
                [15045],
                [22478],
                [29072],
                [2713],
                [11861],
                [20773],
                [16384],
                [16384],
                [16384],
            ],
            [
                [31903],
                [2044],
                [7528],
                [14618],
                [16182],
                [24168],
                [31037],
                [2786],
                [11194],
                [20155],
                [16384],
                [16384],
                [16384],
            ],
            [
                [32510],
                [8430],
                [17318],
                [24154],
                [23674],
                [28789],
                [32139],
                [3440],
                [13117],
                [22702],
                [16384],
                [16384],
                [16384],
            ],
            [
                [31671],
                [2056],
                [11746],
                [16852],
                [18635],
                [24715],
                [31484],
                [4656],
                [16074],
                [24704],
                [1806],
                [14645],
                [25336],
            ],
            [
                [31539],
                [8433],
                [20576],
                [27904],
                [27852],
                [30026],
                [32441],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
                [16384],
            ],
        ])),
        dc_sign: Align4(cdf2d([
            [[16000], [13056], [18816]],
            [[15232], [12928], [17280]],
        ])),
    },
];
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_update(
    hdr: *const Dav1dFrameHeader,
    dst: *mut CdfContext,
    src: *const CdfContext,
) {
    let mut i = 0;
    while i < N_BS_SIZES as libc::c_int {
        (*dst).m.use_filter_intra[i as usize][0] = (*src).m.use_filter_intra[i as usize][0];
        (*dst).m.use_filter_intra[i as usize][1] = 0 as libc::c_int as uint16_t;
        i += 1;
    }
    memcpy(
        ((*dst).m.filter_intra).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.filter_intra).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
    );
    (*dst).m.filter_intra[4] = 0 as libc::c_int as uint16_t;
    let mut k = 0;
    while k < 2 {
        let mut j = 0;
        while j < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.uv_mode[k as usize][j as usize]).as_mut_ptr() as *mut libc::c_void,
                ((*src).m.uv_mode[k as usize][j as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst).m.uv_mode[k as usize][j as usize]
                [(N_UV_INTRA_PRED_MODES as libc::c_int - 1 - (k == 0) as libc::c_int) as usize] =
                0 as libc::c_int as uint16_t;
            j += 1;
        }
        k += 1;
    }
    let mut j_0 = 0;
    while j_0 < 8 {
        memcpy(
            ((*dst).m.angle_delta[j_0 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.angle_delta[j_0 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst).m.angle_delta[j_0 as usize][6] = 0 as libc::c_int as uint16_t;
        j_0 += 1;
    }
    let mut k_0 = 0;
    while k_0 < N_TX_SIZES as libc::c_int - 1 {
        let mut j_1 = 0;
        while j_1 < 3 {
            memcpy(
                ((*dst).m.txsz[k_0 as usize][j_1 as usize]).as_mut_ptr() as *mut libc::c_void,
                ((*src).m.txsz[k_0 as usize][j_1 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst).m.txsz[k_0 as usize][j_1 as usize][imin(k_0 + 1, 2 as libc::c_int) as usize] =
                0 as libc::c_int as uint16_t;
            j_1 += 1;
        }
        k_0 += 1;
    }
    let mut k_1 = 0;
    while k_1 < 2 {
        let mut j_2 = 0;
        while j_2 < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.txtp_intra1[k_1 as usize][j_2 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.txtp_intra1[k_1 as usize][j_2 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).m.txtp_intra1[k_1 as usize][j_2 as usize][6] = 0 as libc::c_int as uint16_t;
            j_2 += 1;
        }
        k_1 += 1;
    }
    let mut k_2 = 0;
    while k_2 < 3 {
        let mut j_3 = 0;
        while j_3 < N_INTRA_PRED_MODES as libc::c_int {
            memcpy(
                ((*dst).m.txtp_intra2[k_2 as usize][j_3 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).m.txtp_intra2[k_2 as usize][j_3 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).m.txtp_intra2[k_2 as usize][j_3 as usize][4] = 0 as libc::c_int as uint16_t;
            j_3 += 1;
        }
        k_2 += 1;
    }
    let mut i_0 = 0;
    while i_0 < 3 {
        (*dst).m.skip[i_0 as usize][0] = (*src).m.skip[i_0 as usize][0];
        (*dst).m.skip[i_0 as usize][1] = 0 as libc::c_int as uint16_t;
        i_0 += 1;
    }
    let mut k_3 = 0;
    while k_3 < N_BL_LEVELS as libc::c_int {
        let mut j_4 = 0;
        while j_4 < 4 {
            memcpy(
                ((*dst).m.partition[k_3 as usize][j_4 as usize]).as_mut_ptr() as *mut libc::c_void,
                ((*src).m.partition[k_3 as usize][j_4 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst).m.partition[k_3 as usize][j_4 as usize]
                [dav1d_partition_type_count[k_3 as usize] as usize] = 0 as libc::c_int as uint16_t;
            j_4 += 1;
        }
        k_3 += 1;
    }
    let mut j_5 = 0;
    while j_5 < N_TX_SIZES as libc::c_int {
        let mut i_1 = 0;
        while i_1 < 13 {
            (*dst).coef.skip[j_5 as usize][i_1 as usize][0] =
                (*src).coef.skip[j_5 as usize][i_1 as usize][0];
            (*dst).coef.skip[j_5 as usize][i_1 as usize][1] = 0 as libc::c_int as uint16_t;
            i_1 += 1;
        }
        j_5 += 1;
    }
    let mut k_4 = 0;
    while k_4 < 2 {
        let mut j_6 = 0;
        while j_6 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_16[k_4 as usize][j_6 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_16[k_4 as usize][j_6 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).coef.eob_bin_16[k_4 as usize][j_6 as usize][4] = 0 as libc::c_int as uint16_t;
            j_6 += 1;
        }
        k_4 += 1;
    }
    let mut k_5 = 0;
    while k_5 < 2 {
        let mut j_7 = 0;
        while j_7 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_32[k_5 as usize][j_7 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_32[k_5 as usize][j_7 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).coef.eob_bin_32[k_5 as usize][j_7 as usize][5] = 0 as libc::c_int as uint16_t;
            j_7 += 1;
        }
        k_5 += 1;
    }
    let mut k_6 = 0;
    while k_6 < 2 {
        let mut j_8 = 0;
        while j_8 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_64[k_6 as usize][j_8 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_64[k_6 as usize][j_8 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).coef.eob_bin_64[k_6 as usize][j_8 as usize][6] = 0 as libc::c_int as uint16_t;
            j_8 += 1;
        }
        k_6 += 1;
    }
    let mut k_7 = 0;
    while k_7 < 2 {
        let mut j_9 = 0;
        while j_9 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_128[k_7 as usize][j_9 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_128[k_7 as usize][j_9 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).coef.eob_bin_128[k_7 as usize][j_9 as usize][7] = 0 as libc::c_int as uint16_t;
            j_9 += 1;
        }
        k_7 += 1;
    }
    let mut k_8 = 0;
    while k_8 < 2 {
        let mut j_10 = 0;
        while j_10 < 2 {
            memcpy(
                ((*dst).coef.eob_bin_256[k_8 as usize][j_10 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).coef.eob_bin_256[k_8 as usize][j_10 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst).coef.eob_bin_256[k_8 as usize][j_10 as usize][8] = 0 as libc::c_int as uint16_t;
            j_10 += 1;
        }
        k_8 += 1;
    }
    let mut j_11 = 0;
    while j_11 < 2 {
        memcpy(
            ((*dst).coef.eob_bin_512[j_11 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).coef.eob_bin_512[j_11 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).coef.eob_bin_512[j_11 as usize][9] = 0 as libc::c_int as uint16_t;
        j_11 += 1;
    }
    let mut j_12 = 0;
    while j_12 < 2 {
        memcpy(
            ((*dst).coef.eob_bin_1024[j_12 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).coef.eob_bin_1024[j_12 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).coef.eob_bin_1024[j_12 as usize][10] = 0 as libc::c_int as uint16_t;
        j_12 += 1;
    }
    let mut k_9 = 0;
    while k_9 < N_TX_SIZES as libc::c_int {
        let mut j_13 = 0;
        while j_13 < 2 {
            let mut i_2 = 0;
            while i_2 < 11 {
                (*dst).coef.eob_hi_bit[k_9 as usize][j_13 as usize][i_2 as usize][0] =
                    (*src).coef.eob_hi_bit[k_9 as usize][j_13 as usize][i_2 as usize][0];
                (*dst).coef.eob_hi_bit[k_9 as usize][j_13 as usize][i_2 as usize][1] =
                    0 as libc::c_int as uint16_t;
                i_2 += 1;
            }
            j_13 += 1;
        }
        k_9 += 1;
    }
    let mut l = 0;
    while l < N_TX_SIZES as libc::c_int {
        let mut k_10 = 0;
        while k_10 < 2 {
            let mut j_14 = 0;
            while j_14 < 4 {
                memcpy(
                    ((*dst).coef.eob_base_tok[l as usize][k_10 as usize][j_14 as usize])
                        .as_mut_ptr() as *mut libc::c_void,
                    ((*src).coef.eob_base_tok[l as usize][k_10 as usize][j_14 as usize]).as_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst).coef.eob_base_tok[l as usize][k_10 as usize][j_14 as usize][2] =
                    0 as libc::c_int as uint16_t;
                j_14 += 1;
            }
            k_10 += 1;
        }
        l += 1;
    }
    let mut l_0 = 0;
    while l_0 < N_TX_SIZES as libc::c_int {
        let mut k_11 = 0;
        while k_11 < 2 {
            let mut j_15 = 0;
            while j_15 < 41 {
                memcpy(
                    ((*dst).coef.base_tok[l_0 as usize][k_11 as usize][j_15 as usize]).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*src).coef.base_tok[l_0 as usize][k_11 as usize][j_15 as usize]).as_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst).coef.base_tok[l_0 as usize][k_11 as usize][j_15 as usize][3] =
                    0 as libc::c_int as uint16_t;
                j_15 += 1;
            }
            k_11 += 1;
        }
        l_0 += 1;
    }
    let mut j_16 = 0;
    while j_16 < 2 {
        let mut i_3 = 0;
        while i_3 < 3 {
            (*dst).coef.dc_sign[j_16 as usize][i_3 as usize][0] =
                (*src).coef.dc_sign[j_16 as usize][i_3 as usize][0];
            (*dst).coef.dc_sign[j_16 as usize][i_3 as usize][1] = 0 as libc::c_int as uint16_t;
            i_3 += 1;
        }
        j_16 += 1;
    }
    let mut l_1 = 0;
    while l_1 < 4 {
        let mut k_12 = 0;
        while k_12 < 2 {
            let mut j_17 = 0;
            while j_17 < 21 {
                memcpy(
                    ((*dst).coef.br_tok[l_1 as usize][k_12 as usize][j_17 as usize]).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*src).coef.br_tok[l_1 as usize][k_12 as usize][j_17 as usize]).as_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
                );
                (*dst).coef.br_tok[l_1 as usize][k_12 as usize][j_17 as usize][3] =
                    0 as libc::c_int as uint16_t;
                j_17 += 1;
            }
            k_12 += 1;
        }
        l_1 += 1;
    }
    let mut j_18 = 0;
    while j_18 < 3 {
        memcpy(
            ((*dst).m.seg_id[j_18 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.seg_id[j_18 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst).m.seg_id[j_18 as usize][(8 - 1) as usize] = 0 as libc::c_int as uint16_t;
        j_18 += 1;
    }
    memcpy(
        ((*dst).m.cfl_sign).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.cfl_sign).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
    );
    (*dst).m.cfl_sign[7] = 0 as libc::c_int as uint16_t;
    let mut j_19 = 0;
    while j_19 < 6 {
        memcpy(
            ((*dst).m.cfl_alpha[j_19 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.cfl_alpha[j_19 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).m.cfl_alpha[j_19 as usize][15] = 0 as libc::c_int as uint16_t;
        j_19 += 1;
    }
    (*dst).m.restore_wiener[0] = (*src).m.restore_wiener[0];
    (*dst).m.restore_wiener[1] = 0 as libc::c_int as uint16_t;
    (*dst).m.restore_sgrproj[0] = (*src).m.restore_sgrproj[0];
    (*dst).m.restore_sgrproj[1] = 0 as libc::c_int as uint16_t;
    memcpy(
        ((*dst).m.restore_switchable).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.restore_switchable).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst).m.restore_switchable[2] = 0 as libc::c_int as uint16_t;
    memcpy(
        ((*dst).m.delta_q).0.as_mut_ptr() as *mut libc::c_void,
        ((*src).m.delta_q).0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst).m.delta_q[3] = 0 as libc::c_int as uint16_t;
    let mut j_20 = 0;
    while j_20 < 5 {
        memcpy(
            ((*dst).m.delta_lf[j_20 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.delta_lf[j_20 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).m.delta_lf[j_20 as usize][3] = 0 as libc::c_int as uint16_t;
        j_20 += 1;
    }
    let mut j_21 = 0;
    while j_21 < 7 {
        let mut i_4 = 0;
        while i_4 < 3 {
            (*dst).m.pal_y[j_21 as usize][i_4 as usize][0] =
                (*src).m.pal_y[j_21 as usize][i_4 as usize][0];
            (*dst).m.pal_y[j_21 as usize][i_4 as usize][1] = 0 as libc::c_int as uint16_t;
            i_4 += 1;
        }
        j_21 += 1;
    }
    let mut i_5 = 0;
    while i_5 < 2 {
        (*dst).m.pal_uv[i_5 as usize][0] = (*src).m.pal_uv[i_5 as usize][0];
        (*dst).m.pal_uv[i_5 as usize][1] = 0 as libc::c_int as uint16_t;
        i_5 += 1;
    }
    let mut k_13 = 0;
    while k_13 < 2 {
        let mut j_22 = 0;
        while j_22 < 7 {
            memcpy(
                ((*dst).m.pal_sz[k_13 as usize][j_22 as usize]).as_mut_ptr() as *mut libc::c_void,
                ((*src).m.pal_sz[k_13 as usize][j_22 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
            );
            (*dst).m.pal_sz[k_13 as usize][j_22 as usize][6] = 0 as libc::c_int as uint16_t;
            j_22 += 1;
        }
        k_13 += 1;
    }
    let mut l_2 = 0;
    while l_2 < 2 {
        let mut k_14 = 0;
        while k_14 < 7 {
            let mut j_23 = 0;
            while j_23 < 5 {
                memcpy(
                    ((*dst).m.color_map[l_2 as usize][k_14 as usize][j_23 as usize]).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*src).m.color_map[l_2 as usize][k_14 as usize][j_23 as usize]).as_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
                );
                (*dst).m.color_map[l_2 as usize][k_14 as usize][j_23 as usize]
                    [(k_14 + 1) as usize] = 0 as libc::c_int as uint16_t;
                j_23 += 1;
            }
            k_14 += 1;
        }
        l_2 += 1;
    }
    let mut j_24 = 0;
    while j_24 < 7 {
        let mut i_6 = 0;
        while i_6 < 3 {
            (*dst).m.txpart[j_24 as usize][i_6 as usize][0] =
                (*src).m.txpart[j_24 as usize][i_6 as usize][0];
            (*dst).m.txpart[j_24 as usize][i_6 as usize][1] = 0 as libc::c_int as uint16_t;
            i_6 += 1;
        }
        j_24 += 1;
    }
    let mut j_25 = 0;
    while j_25 < 2 {
        memcpy(
            ((*dst).m.txtp_inter1[j_25 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.txtp_inter1[j_25 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).m.txtp_inter1[j_25 as usize][15] = 0 as libc::c_int as uint16_t;
        j_25 += 1;
    }
    memcpy(
        ((*dst).m.txtp_inter2.0).as_mut_ptr() as *mut libc::c_void,
        ((*src).m.txtp_inter2.0).as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
    );
    (*dst).m.txtp_inter2[11] = 0 as libc::c_int as uint16_t;
    let mut i_7 = 0;
    while i_7 < 4 {
        (*dst).m.txtp_inter3[i_7 as usize][0] = (*src).m.txtp_inter3[i_7 as usize][0];
        (*dst).m.txtp_inter3[i_7 as usize][1] = 0 as libc::c_int as uint16_t;
        i_7 += 1;
    }
    if (*hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0 {
        (*dst).m.intrabc[0] = (*src).m.intrabc[0];
        (*dst).m.intrabc[1] = 0 as libc::c_int as uint16_t;
        memcpy(
            ((*dst).dmv.joint.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).dmv.joint.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).dmv.joint[(N_MV_JOINTS as libc::c_int - 1) as usize] = 0 as libc::c_int as uint16_t;
        let mut k_15 = 0;
        while k_15 < 2 {
            memcpy(
                ((*dst).dmv.comp[k_15 as usize].classes.0).as_mut_ptr() as *mut libc::c_void,
                ((*src).dmv.comp[k_15 as usize].classes.0).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
            );
            (*dst).dmv.comp[k_15 as usize].classes[10] = 0 as libc::c_int as uint16_t;
            (*dst).dmv.comp[k_15 as usize].class0[0] = (*src).dmv.comp[k_15 as usize].class0[0];
            (*dst).dmv.comp[k_15 as usize].class0[1] = 0 as libc::c_int as uint16_t;
            let mut i_8 = 0;
            while i_8 < 10 {
                (*dst).dmv.comp[k_15 as usize].classN[i_8 as usize][0] =
                    (*src).dmv.comp[k_15 as usize].classN[i_8 as usize][0];
                (*dst).dmv.comp[k_15 as usize].classN[i_8 as usize][1] =
                    0 as libc::c_int as uint16_t;
                i_8 += 1;
            }
            (*dst).dmv.comp[k_15 as usize].sign[0] = (*src).dmv.comp[k_15 as usize].sign[0];
            (*dst).dmv.comp[k_15 as usize].sign[1] = 0 as libc::c_int as uint16_t;
            k_15 += 1;
        }
        return;
    }
    let mut i_9 = 0;
    while i_9 < 3 {
        (*dst).m.skip_mode.0[i_9 as usize][0] = (*src).m.skip_mode.0[i_9 as usize][0];
        (*dst).m.skip_mode.0[i_9 as usize][1] = 0 as libc::c_int as uint16_t;
        i_9 += 1;
    }
    let mut j_26 = 0;
    while j_26 < 4 {
        memcpy(
            ((*dst).m.y_mode.0[j_26 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.y_mode.0[j_26 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).m.y_mode.0[j_26 as usize][(N_INTRA_PRED_MODES as libc::c_int - 1) as usize] =
            0 as libc::c_int as uint16_t;
        j_26 += 1;
    }
    let mut k_16 = 0;
    while k_16 < 2 {
        let mut j_27 = 0;
        while j_27 < 8 {
            memcpy(
                ((*dst).m.filter.0[k_16 as usize][j_27 as usize]).as_mut_ptr() as *mut libc::c_void,
                ((*src).m.filter.0[k_16 as usize][j_27 as usize]).as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst).m.filter.0[k_16 as usize][j_27 as usize]
                [(DAV1D_N_SWITCHABLE_FILTERS as libc::c_int - 1) as usize] =
                0 as libc::c_int as uint16_t;
            j_27 += 1;
        }
        k_16 += 1;
    }
    let mut i_10 = 0;
    while i_10 < 6 {
        (*dst).m.newmv_mode.0[i_10 as usize][0] = (*src).m.newmv_mode.0[i_10 as usize][0];
        (*dst).m.newmv_mode.0[i_10 as usize][1] = 0 as libc::c_int as uint16_t;
        i_10 += 1;
    }
    let mut i_11 = 0;
    while i_11 < 2 {
        (*dst).m.globalmv_mode.0[i_11 as usize][0] = (*src).m.globalmv_mode.0[i_11 as usize][0];
        (*dst).m.globalmv_mode.0[i_11 as usize][1] = 0 as libc::c_int as uint16_t;
        i_11 += 1;
    }
    let mut i_12 = 0;
    while i_12 < 6 {
        (*dst).m.refmv_mode.0[i_12 as usize][0] = (*src).m.refmv_mode.0[i_12 as usize][0];
        (*dst).m.refmv_mode.0[i_12 as usize][1] = 0 as libc::c_int as uint16_t;
        i_12 += 1;
    }
    let mut i_13 = 0;
    while i_13 < 3 {
        (*dst).m.drl_bit.0[i_13 as usize][0] = (*src).m.drl_bit.0[i_13 as usize][0];
        (*dst).m.drl_bit.0[i_13 as usize][1] = 0 as libc::c_int as uint16_t;
        i_13 += 1;
    }
    let mut j_28 = 0;
    while j_28 < 8 {
        memcpy(
            ((*dst).m.comp_inter_mode.0[j_28 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.comp_inter_mode.0[j_28 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 8]>() as libc::c_ulong,
        );
        (*dst).m.comp_inter_mode.0[j_28 as usize]
            [(N_COMP_INTER_PRED_MODES as libc::c_int - 1) as usize] = 0 as libc::c_int as uint16_t;
        j_28 += 1;
    }
    let mut i_14 = 0;
    while i_14 < 4 {
        (*dst).m.intra.0[i_14 as usize][0] = (*src).m.intra.0[i_14 as usize][0];
        (*dst).m.intra.0[i_14 as usize][1] = 0 as libc::c_int as uint16_t;
        i_14 += 1;
    }
    let mut i_15 = 0;
    while i_15 < 5 {
        (*dst).m.comp.0[i_15 as usize][0] = (*src).m.comp.0[i_15 as usize][0];
        (*dst).m.comp.0[i_15 as usize][1] = 0 as libc::c_int as uint16_t;
        i_15 += 1;
    }
    let mut i_16 = 0;
    while i_16 < 5 {
        (*dst).m.comp_dir.0[i_16 as usize][0] = (*src).m.comp_dir.0[i_16 as usize][0];
        (*dst).m.comp_dir.0[i_16 as usize][1] = 0 as libc::c_int as uint16_t;
        i_16 += 1;
    }
    let mut i_17 = 0;
    while i_17 < 6 {
        (*dst).m.jnt_comp.0[i_17 as usize][0] = (*src).m.jnt_comp.0[i_17 as usize][0];
        (*dst).m.jnt_comp.0[i_17 as usize][1] = 0 as libc::c_int as uint16_t;
        i_17 += 1;
    }
    let mut i_18 = 0;
    while i_18 < 6 {
        (*dst).m.mask_comp.0[i_18 as usize][0] = (*src).m.mask_comp.0[i_18 as usize][0];
        (*dst).m.mask_comp.0[i_18 as usize][1] = 0 as libc::c_int as uint16_t;
        i_18 += 1;
    }
    let mut i_19 = 0;
    while i_19 < 9 {
        (*dst).m.wedge_comp.0[i_19 as usize][0] = (*src).m.wedge_comp.0[i_19 as usize][0];
        (*dst).m.wedge_comp.0[i_19 as usize][1] = 0 as libc::c_int as uint16_t;
        i_19 += 1;
    }
    let mut j_29 = 0;
    while j_29 < 9 {
        memcpy(
            ((*dst).m.wedge_idx.0[j_29 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.wedge_idx.0[j_29 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).m.wedge_idx[j_29 as usize][15] = 0 as libc::c_int as uint16_t;
        j_29 += 1;
    }
    let mut j_30 = 0;
    while j_30 < 6 {
        let mut i_20 = 0;
        while i_20 < 3 {
            (*dst).m.r#ref[j_30 as usize][i_20 as usize][0] =
                (*src).m.r#ref[j_30 as usize][i_20 as usize][0];
            (*dst).m.r#ref[j_30 as usize][i_20 as usize][1] = 0 as libc::c_int as uint16_t;
            i_20 += 1;
        }
        j_30 += 1;
    }
    let mut j_31 = 0;
    while j_31 < 3 {
        let mut i_21 = 0;
        while i_21 < 3 {
            (*dst).m.comp_fwd_ref[j_31 as usize][i_21 as usize][0] =
                (*src).m.comp_fwd_ref[j_31 as usize][i_21 as usize][0];
            (*dst).m.comp_fwd_ref[j_31 as usize][i_21 as usize][1] = 0 as libc::c_int as uint16_t;
            i_21 += 1;
        }
        j_31 += 1;
    }
    let mut j_32 = 0;
    while j_32 < 2 {
        let mut i_22 = 0;
        while i_22 < 3 {
            (*dst).m.comp_bwd_ref[j_32 as usize][i_22 as usize][0] =
                (*src).m.comp_bwd_ref[j_32 as usize][i_22 as usize][0];
            (*dst).m.comp_bwd_ref[j_32 as usize][i_22 as usize][1] = 0 as libc::c_int as uint16_t;
            i_22 += 1;
        }
        j_32 += 1;
    }
    let mut j_33 = 0;
    while j_33 < 3 {
        let mut i_23 = 0;
        while i_23 < 3 {
            (*dst).m.comp_uni_ref[j_33 as usize][i_23 as usize][0] =
                (*src).m.comp_uni_ref[j_33 as usize][i_23 as usize][0];
            (*dst).m.comp_uni_ref[j_33 as usize][i_23 as usize][1] = 0 as libc::c_int as uint16_t;
            i_23 += 1;
        }
        j_33 += 1;
    }
    let mut i_24 = 0;
    while i_24 < 3 {
        (*dst).m.seg_pred[i_24 as usize][0] = (*src).m.seg_pred[i_24 as usize][0];
        (*dst).m.seg_pred[i_24 as usize][1] = 0 as libc::c_int as uint16_t;
        i_24 += 1;
    }
    let mut i_25 = 0;
    while i_25 < 4 {
        (*dst).m.interintra[i_25 as usize][0] = (*src).m.interintra[i_25 as usize][0];
        (*dst).m.interintra[i_25 as usize][1] = 0 as libc::c_int as uint16_t;
        i_25 += 1;
    }
    let mut i_26 = 0;
    while i_26 < 7 {
        (*dst).m.interintra_wedge[i_26 as usize][0] = (*src).m.interintra_wedge[i_26 as usize][0];
        (*dst).m.interintra_wedge[i_26 as usize][1] = 0 as libc::c_int as uint16_t;
        i_26 += 1;
    }
    let mut j_34 = 0;
    while j_34 < 4 {
        memcpy(
            ((*dst).m.interintra_mode[j_34 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.interintra_mode[j_34 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).m.interintra_mode[j_34 as usize][3] = 0 as libc::c_int as uint16_t;
        j_34 += 1;
    }
    let mut j_35 = 0;
    while j_35 < N_BS_SIZES as libc::c_int {
        memcpy(
            ((*dst).m.motion_mode[j_35 as usize]).as_mut_ptr() as *mut libc::c_void,
            ((*src).m.motion_mode[j_35 as usize]).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).m.motion_mode[j_35 as usize][2] = 0 as libc::c_int as uint16_t;
        j_35 += 1;
    }
    let mut i_27 = 0;
    while i_27 < N_BS_SIZES as libc::c_int {
        (*dst).m.obmc[i_27 as usize][0] = (*src).m.obmc[i_27 as usize][0];
        (*dst).m.obmc[i_27 as usize][1] = 0 as libc::c_int as uint16_t;
        i_27 += 1;
    }
    memcpy(
        ((*dst).mv.joint.0).as_mut_ptr() as *mut libc::c_void,
        ((*src).mv.joint.0).as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
    );
    (*dst).mv.joint[(N_MV_JOINTS as libc::c_int - 1) as usize] = 0 as libc::c_int as uint16_t;
    let mut k_17 = 0;
    while k_17 < 2 {
        memcpy(
            ((*dst).mv.comp[k_17 as usize].classes.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).mv.comp[k_17 as usize].classes.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 16]>() as libc::c_ulong,
        );
        (*dst).mv.comp[k_17 as usize].classes[10] = 0 as libc::c_int as uint16_t;
        (*dst).mv.comp[k_17 as usize].class0[0] = (*src).mv.comp[k_17 as usize].class0[0];
        (*dst).mv.comp[k_17 as usize].class0[1] = 0 as libc::c_int as uint16_t;
        let mut i_28 = 0;
        while i_28 < 10 {
            (*dst).mv.comp[k_17 as usize].classN[i_28 as usize][0] =
                (*src).mv.comp[k_17 as usize].classN[i_28 as usize][0];
            (*dst).mv.comp[k_17 as usize].classN[i_28 as usize][1] = 0 as libc::c_int as uint16_t;
            i_28 += 1;
        }
        let mut j_36 = 0;
        while j_36 < 2 {
            memcpy(
                ((*dst).mv.comp[k_17 as usize].class0_fp[j_36 as usize]).as_mut_ptr()
                    as *mut libc::c_void,
                ((*src).mv.comp[k_17 as usize].class0_fp[j_36 as usize]).as_ptr()
                    as *const libc::c_void,
                ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
            );
            (*dst).mv.comp[k_17 as usize].class0_fp[j_36 as usize][3] =
                0 as libc::c_int as uint16_t;
            j_36 += 1;
        }
        memcpy(
            ((*dst).mv.comp[k_17 as usize].classN_fp.0).as_mut_ptr() as *mut libc::c_void,
            ((*src).mv.comp[k_17 as usize].classN_fp.0).as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).mv.comp[k_17 as usize].classN_fp[3] = 0 as libc::c_int as uint16_t;
        (*dst).mv.comp[k_17 as usize].class0_hp[0] = (*src).mv.comp[k_17 as usize].class0_hp[0];
        (*dst).mv.comp[k_17 as usize].class0_hp[1] = 0 as libc::c_int as uint16_t;
        (*dst).mv.comp[k_17 as usize].classN_hp[0] = (*src).mv.comp[k_17 as usize].classN_hp[0];
        (*dst).mv.comp[k_17 as usize].classN_hp[1] = 0 as libc::c_int as uint16_t;
        (*dst).mv.comp[k_17 as usize].sign[0] = (*src).mv.comp[k_17 as usize].sign[0];
        (*dst).mv.comp[k_17 as usize].sign[1] = 0 as libc::c_int as uint16_t;
        k_17 += 1;
    }
}
#[inline]
unsafe extern "C" fn get_qcat_idx(q: libc::c_int) -> libc::c_int {
    if q <= 20 {
        return 0 as libc::c_int;
    }
    if q <= 60 {
        return 1 as libc::c_int;
    }
    if q <= 120 {
        return 2 as libc::c_int;
    }
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_init_static(
    cdf: *mut CdfThreadContext,
    qidx: libc::c_int,
) {
    (*cdf).r#ref = 0 as *mut Dav1dRef;
    (*cdf).data.qcat = get_qcat_idx(qidx) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_copy(dst: *mut CdfContext, src: *const CdfThreadContext) {
    if !((*src).r#ref).is_null() {
        memcpy(
            dst as *mut libc::c_void,
            (*src).data.cdf as *const libc::c_void,
            ::core::mem::size_of::<CdfContext>() as libc::c_ulong,
        );
    } else {
        (*dst).m = av1_default_cdf;
        memcpy(
            ((*dst).kfym.0).as_mut_ptr() as *mut libc::c_void,
            default_kf_y_mode_cdf.0.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[[[uint16_t; 16]; 5]; 5]>() as libc::c_ulong,
        );
        (*dst).coef = av1_default_coef_cdf[(*src).data.qcat as usize];
        memcpy(
            ((*dst).mv.joint.0).as_mut_ptr() as *mut libc::c_void,
            default_mv_joint_cdf.0.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        memcpy(
            ((*dst).dmv.joint.0).as_mut_ptr() as *mut libc::c_void,
            default_mv_joint_cdf.0.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint16_t; 4]>() as libc::c_ulong,
        );
        (*dst).dmv.comp[1] = default_mv_component_cdf;
        (*dst).dmv.comp[0] = (*dst).dmv.comp[1];
        (*dst).mv.comp[1] = (*dst).dmv.comp[0];
        (*dst).mv.comp[0] = (*dst).mv.comp[1];
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_alloc(
    c: *mut Dav1dContext,
    cdf: *mut CdfThreadContext,
    have_frame_mt: libc::c_int,
) -> libc::c_int {
    (*cdf).r#ref = dav1d_ref_create_using_pool(
        (*c).cdf_pool,
        (::core::mem::size_of::<CdfContext>()).wrapping_add(::core::mem::size_of::<atomic_uint>()),
    );
    if ((*cdf).r#ref).is_null() {
        return -(12 as libc::c_int);
    }
    (*cdf).data.cdf = (*(*cdf).r#ref).data as *mut CdfContext;
    if have_frame_mt != 0 {
        (*cdf).progress = &mut *((*cdf).data.cdf).offset(1) as *mut CdfContext as *mut atomic_uint;
        *(*cdf).progress = 0 as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_ref(
    dst: *mut CdfThreadContext,
    src: *mut CdfThreadContext,
) {
    *dst = *src;
    if !((*src).r#ref).is_null() {
        dav1d_ref_inc((*src).r#ref);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext) {
    memset(
        &mut (*cdf).data as *mut CdfThreadContext_data as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<CdfThreadContext>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_ulong),
    );
    dav1d_ref_dec(&mut (*cdf).r#ref);
}
