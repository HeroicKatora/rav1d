use crate::include::common::attributes::clz;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::LeftPixelRow2px;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::common::intops::ulog2;
use crate::src::tables::dav1d_cdef_directions;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type CdefEdgeFlags = c_uint;
pub const CDEF_HAVE_BOTTOM: CdefEdgeFlags = 8;
pub const CDEF_HAVE_TOP: CdefEdgeFlags = 4;
pub const CDEF_HAVE_RIGHT: CdefEdgeFlags = 2;
pub const CDEF_HAVE_LEFT: CdefEdgeFlags = 1;

pub type cdef_fn = unsafe extern "C" fn(
    *mut DynPixel,
    ptrdiff_t,
    *const LeftPixelRow2px<DynPixel>,
    *const DynPixel,
    *const DynPixel,
    c_int,
    c_int,
    c_int,
    c_int,
    CdefEdgeFlags,
    c_int,
) -> ();

pub type cdef_dir_fn =
    unsafe extern "C" fn(*const DynPixel, ptrdiff_t, *mut c_uint, c_int) -> c_int;

#[repr(C)]
pub struct Rav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse4(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_8bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_sse2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_8", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_8bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_8bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_8",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_8bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_8bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
    pub(crate) fn dav1d_cdef_filter8_8bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
    );
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_dir_16bpc_sse4(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_ssse3(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_ssse3(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(feature = "asm", feature = "bitdepth_16", target_arch = "x86_64",))]
extern "C" {
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx512icl(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x4_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_4x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter_8x8_16bpc_avx2(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        edges: CdefEdgeFlags,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_dir_16bpc_avx2(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
}

// TODO(legare): Temporarily pub until init fns are deduplicated.
#[cfg(all(
    feature = "asm",
    feature = "bitdepth_16",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    pub(crate) fn dav1d_cdef_find_dir_16bpc_neon(
        dst: *const DynPixel,
        dst_stride: ptrdiff_t,
        var: *mut c_uint,
        bitdepth_max: c_int,
    ) -> c_int;
    pub(crate) fn dav1d_cdef_padding4_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_padding8_16bpc_neon(
        tmp: *mut u16,
        src: *const DynPixel,
        src_stride: ptrdiff_t,
        left: *const LeftPixelRow2px<DynPixel>,
        top: *const DynPixel,
        bottom: *const DynPixel,
        h: c_int,
        edges: CdefEdgeFlags,
    );
    pub(crate) fn dav1d_cdef_filter4_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
    pub(crate) fn dav1d_cdef_filter8_16bpc_neon(
        dst: *mut DynPixel,
        dst_stride: ptrdiff_t,
        tmp: *const u16,
        pri_strength: c_int,
        sec_strength: c_int,
        dir: c_int,
        damping: c_int,
        h: c_int,
        edges: usize,
        bitdepth_max: c_int,
    );
}

#[inline]
pub unsafe fn constrain(diff: c_int, threshold: c_int, shift: c_int) -> c_int {
    let adiff = diff.abs();
    return apply_sign(
        cmp::min(adiff, cmp::max(0 as c_int, threshold - (adiff >> shift))),
        diff,
    );
}

#[inline]
pub unsafe fn fill(mut tmp: *mut i16, stride: ptrdiff_t, w: c_int, h: c_int) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *tmp.offset(x as isize) = i16::MIN;
            x += 1;
        }
        tmp = tmp.offset(stride as isize);
        y += 1;
    }
}

// TODO(perl): Temporarily pub until mod is deduplicated
pub(crate) unsafe fn padding<BD: BitDepth>(
    mut tmp: *mut i16,
    tmp_stride: ptrdiff_t,
    mut src: *const BD::Pixel,
    src_stride: ptrdiff_t,
    left: *const [BD::Pixel; 2],
    mut top: *const BD::Pixel,
    mut bottom: *const BD::Pixel,
    w: c_int,
    h: c_int,
    edges: CdefEdgeFlags,
) {
    let mut x_start = -(2 as c_int);
    let mut x_end = w + 2;
    let mut y_start = -(2 as c_int);
    let mut y_end = h + 2;
    if edges as c_uint & CDEF_HAVE_TOP as c_int as c_uint == 0 {
        fill(
            tmp.offset(-2).offset(-((2 * tmp_stride) as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_start = 0 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_BOTTOM as c_int as c_uint == 0 {
        fill(
            tmp.offset((h as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            w + 4,
            2 as c_int,
        );
        y_end -= 2 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_LEFT as c_int as c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(-(2 as c_int as isize)),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_start = 0 as c_int;
    }
    if edges as c_uint & CDEF_HAVE_RIGHT as c_int as c_uint == 0 {
        fill(
            tmp.offset((y_start as isize * tmp_stride) as isize)
                .offset(w as isize),
            tmp_stride,
            2 as c_int,
            y_end - y_start,
        );
        x_end -= 2 as c_int;
    }
    let mut y = y_start;
    while y < 0 {
        let mut x = x_start;
        while x < x_end {
            *tmp.offset((x as isize + y as isize * tmp_stride) as isize) =
                (*top.offset(x as isize)).as_::<i16>();
            x += 1;
        }
        top = top.offset(BD::pxstride(src_stride as usize) as isize);
        y += 1;
    }
    let mut y_0 = 0;
    while y_0 < h {
        let mut x_0 = x_start;
        while x_0 < 0 {
            *tmp.offset((x_0 as isize + y_0 as isize * tmp_stride) as isize) =
                (*left.offset(y_0 as isize))[(2 + x_0) as usize].as_::<i16>();
            x_0 += 1;
        }
        y_0 += 1;
    }
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_1 = if y_1 < h { 0 as c_int } else { x_start };
        while x_1 < x_end {
            *tmp.offset(x_1 as isize) = (*src.offset(x_1 as isize)).as_::<i16>();
            x_1 += 1;
        }
        src = src.offset(BD::pxstride(src_stride as usize) as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_1 += 1;
    }
    let mut y_2 = h;
    while y_2 < y_end {
        let mut x_2 = x_start;
        while x_2 < x_end {
            *tmp.offset(x_2 as isize) = (*bottom.offset(x_2 as isize)).as_::<i16>();
            x_2 += 1;
        }
        bottom = bottom.offset(BD::pxstride(src_stride as usize) as isize);
        tmp = tmp.offset(tmp_stride as isize);
        y_2 += 1;
    }
}

// TODO(perl): Temporarily pub until mod is deduplicated
#[inline(never)]
pub(crate) unsafe fn cdef_filter_block_c<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    dst_stride: ptrdiff_t,
    left: *const [BD::Pixel; 2],
    top: *const BD::Pixel,
    bottom: *const BD::Pixel,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    w: c_int,
    mut h: c_int,
    edges: CdefEdgeFlags,
    bd: BD,
) {
    let tmp_stride: ptrdiff_t = 12 as c_int as ptrdiff_t;
    if !((w == 4 || w == 8) && (h == 4 || h == 8)) {
        unreachable!();
    }
    let mut tmp_buf: [i16; 144] = [0; 144];
    let mut tmp: *mut i16 = tmp_buf
        .as_mut_ptr()
        .offset((2 * tmp_stride) as isize)
        .offset(2);
    padding::<BD>(
        tmp, tmp_stride, dst, dst_stride, left, top, bottom, w, h, edges,
    );
    if pri_strength != 0 {
        let bitdepth_min_8 = 32 - clz(bd.bitdepth_max().as_::<c_uint>()) - 8;
        let pri_tap = 4 - (pri_strength >> bitdepth_min_8 & 1);
        let pri_shift = cmp::max(0 as c_int, damping - ulog2(pri_strength as c_uint));
        if sec_strength != 0 {
            let sec_shift = damping - ulog2(sec_strength as c_uint);
            loop {
                let mut x = 0;
                while x < w {
                    let px = (*dst.offset(x as isize)).as_::<c_int>();
                    let mut sum = 0;
                    let mut max = px;
                    let mut min = px;
                    let mut pri_tap_k = pri_tap;
                    let mut k = 0;
                    while k < 2 {
                        let off1 = dav1d_cdef_directions[(dir + 2) as usize][k as usize] as c_int;
                        let p0 = *tmp.offset((x + off1) as isize) as c_int;
                        let p1 = *tmp.offset((x - off1) as isize) as c_int;
                        sum += pri_tap_k * constrain(p0 - px, pri_strength, pri_shift);
                        sum += pri_tap_k * constrain(p1 - px, pri_strength, pri_shift);
                        pri_tap_k = pri_tap_k & 3 | 2;
                        min = cmp::min(p0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p0, max);
                        min = cmp::min(p1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(p1, max);
                        let off2 = dav1d_cdef_directions[(dir + 4) as usize][k as usize] as c_int;
                        let off3 = dav1d_cdef_directions[(dir + 0) as usize][k as usize] as c_int;
                        let s0 = *tmp.offset((x + off2) as isize) as c_int;
                        let s1 = *tmp.offset((x - off2) as isize) as c_int;
                        let s2 = *tmp.offset((x + off3) as isize) as c_int;
                        let s3 = *tmp.offset((x - off3) as isize) as c_int;
                        let sec_tap = 2 - k;
                        sum += sec_tap * constrain(s0 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s1 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s2 - px, sec_strength, sec_shift);
                        sum += sec_tap * constrain(s3 - px, sec_strength, sec_shift);
                        min = cmp::min(s0 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s0, max);
                        min = cmp::min(s1 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s1, max);
                        min = cmp::min(s2 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s2, max);
                        min = cmp::min(s3 as c_uint, min as c_uint) as c_int;
                        max = cmp::max(s3, max);
                        k += 1;
                    }
                    *dst.offset(x as isize) =
                        iclip(px + (sum - (sum < 0) as c_int + 8 >> 4), min, max)
                            .as_::<BD::Pixel>();
                    x += 1;
                }
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        } else {
            loop {
                let mut x_0 = 0;
                while x_0 < w {
                    let px_0 = (*dst.offset(x_0 as isize)).as_::<c_int>();
                    let mut sum_0 = 0;
                    let mut pri_tap_k_0 = pri_tap;
                    let mut k_0 = 0;
                    while k_0 < 2 {
                        let off = dav1d_cdef_directions[(dir + 2) as usize][k_0 as usize] as c_int;
                        let p0_0 = *tmp.offset((x_0 + off) as isize) as c_int;
                        let p1_0 = *tmp.offset((x_0 - off) as isize) as c_int;
                        sum_0 += pri_tap_k_0 * constrain(p0_0 - px_0, pri_strength, pri_shift);
                        sum_0 += pri_tap_k_0 * constrain(p1_0 - px_0, pri_strength, pri_shift);
                        pri_tap_k_0 = pri_tap_k_0 & 3 | 2;
                        k_0 += 1;
                    }
                    *dst.offset(x_0 as isize) =
                        (px_0 + (sum_0 - (sum_0 < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                    x_0 += 1;
                }
                dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
                tmp = tmp.offset(tmp_stride as isize);
                h -= 1;
                if !(h != 0) {
                    break;
                }
            }
        }
    } else {
        if sec_strength == 0 {
            unreachable!();
        }
        let sec_shift_0 = damping - ulog2(sec_strength as c_uint);
        loop {
            let mut x_1 = 0;
            while x_1 < w {
                let px_1 = (*dst.offset(x_1 as isize)).as_::<c_int>();
                let mut sum_1 = 0;
                let mut k_1 = 0;
                while k_1 < 2 {
                    let off1_0 = dav1d_cdef_directions[(dir + 4) as usize][k_1 as usize] as c_int;
                    let off2_0 = dav1d_cdef_directions[(dir + 0) as usize][k_1 as usize] as c_int;
                    let s0_0 = *tmp.offset((x_1 + off1_0) as isize) as c_int;
                    let s1_0 = *tmp.offset((x_1 - off1_0) as isize) as c_int;
                    let s2_0 = *tmp.offset((x_1 + off2_0) as isize) as c_int;
                    let s3_0 = *tmp.offset((x_1 - off2_0) as isize) as c_int;
                    let sec_tap_0 = 2 - k_1;
                    sum_1 += sec_tap_0 * constrain(s0_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s1_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s2_0 - px_1, sec_strength, sec_shift_0);
                    sum_1 += sec_tap_0 * constrain(s3_0 - px_1, sec_strength, sec_shift_0);
                    k_1 += 1;
                }
                *dst.offset(x_1 as isize) =
                    (px_1 + (sum_1 - (sum_1 < 0) as c_int + 8 >> 4)).as_::<BD::Pixel>();
                x_1 += 1;
            }
            dst = dst.offset(BD::pxstride(dst_stride as usize) as isize);
            tmp = tmp.offset(tmp_stride as isize);
            h -= 1;
            if !(h != 0) {
                break;
            }
        }
    };
}
