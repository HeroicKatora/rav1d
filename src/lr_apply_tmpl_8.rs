use std::cmp;

use crate::include::stddef::*;
use crate::include::stdint::*;

use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::src::tables::dav1d_sgr_params;

pub type pixel = uint8_t;

use crate::src::internal::Dav1dFrameContext;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;

use crate::include::dav1d::headers::DAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::DAV1D_RESTORATION_WIENER;

use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;

use crate::src::align::Align16;

use crate::src::lf_mask::Av1RestorationUnit;

use crate::src::internal::Dav1dDSPContext;

use crate::src::looprestoration::looprestorationfilter_fn;

use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;
use crate::src::looprestoration::LR_HAVE_BOTTOM;
use crate::src::looprestoration::LR_HAVE_LEFT;
use crate::src::looprestoration::LR_HAVE_RIGHT;
use crate::src::looprestoration::LR_HAVE_TOP;

use crate::src::lr_apply::LR_RESTORE_U;
use crate::src::lr_apply::LR_RESTORE_V;
use crate::src::lr_apply::LR_RESTORE_Y;
unsafe extern "C" fn lr_stripe(
    f: *const Dav1dFrameContext,
    mut p: *mut pixel,
    mut left: *const [pixel; 4],
    x: libc::c_int,
    mut y: libc::c_int,
    plane: libc::c_int,
    unit_w: libc::c_int,
    row_h: libc::c_int,
    lr: *const Av1RestorationUnit,
    mut edges: LrEdgeFlags,
) {
    let dsp: *const Dav1dDSPContext = (*f).dsp;
    let chroma = (plane != 0) as libc::c_int;
    let ss_ver = chroma
        & ((*f).sr_cur.p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let sby =
        y + (if y != 0 {
            (8 as libc::c_int) << ss_ver
        } else {
            0 as libc::c_int
        }) >> 6 - ss_ver + (*(*f).seq_hdr).sb128;
    let have_tt = ((*(*f).c).n_tc > 1 as libc::c_uint) as libc::c_int;
    let mut lpf: *const pixel = ((*f).lf.lr_lpf_line[plane as usize] as *mut pixel)
        .offset(
            ((have_tt * (sby * ((4 as libc::c_int) << (*(*f).seq_hdr).sb128) - 4)) as isize
                * stride) as isize,
        )
        .offset(x as isize);
    let mut stripe_h = cmp::min(64 - 8 * (y == 0) as libc::c_int >> ss_ver, row_h - y);
    let lr_fn: looprestorationfilter_fn;
    let mut params: LooprestorationParams = LooprestorationParams {
        filter: [[0; 8]; 2].into(),
    };
    if (*lr).r#type as libc::c_int == DAV1D_RESTORATION_WIENER as libc::c_int {
        let filter: *mut [int16_t; 8] = (params.filter.0).as_mut_ptr();
        let ref mut fresh0 = (*filter.offset(0))[6];
        *fresh0 = (*lr).filter_h[0] as int16_t;
        (*filter.offset(0))[0] = *fresh0;
        let ref mut fresh1 = (*filter.offset(0))[5];
        *fresh1 = (*lr).filter_h[1] as int16_t;
        (*filter.offset(0))[1] = *fresh1;
        let ref mut fresh2 = (*filter.offset(0))[4];
        *fresh2 = (*lr).filter_h[2] as int16_t;
        (*filter.offset(0))[2] = *fresh2;
        (*filter.offset(0))[3] = (-((*filter.offset(0))[0] as libc::c_int
            + (*filter.offset(0))[1] as libc::c_int
            + (*filter.offset(0))[2] as libc::c_int)
            * 2) as int16_t;
        let ref mut fresh3 = (*filter.offset(1))[6];
        *fresh3 = (*lr).filter_v[0] as int16_t;
        (*filter.offset(1))[0] = *fresh3;
        let ref mut fresh4 = (*filter.offset(1))[5];
        *fresh4 = (*lr).filter_v[1] as int16_t;
        (*filter.offset(1))[1] = *fresh4;
        let ref mut fresh5 = (*filter.offset(1))[4];
        *fresh5 = (*lr).filter_v[2] as int16_t;
        (*filter.offset(1))[2] = *fresh5;
        (*filter.offset(1))[3] = (128 as libc::c_int
            - ((*filter.offset(1))[0] as libc::c_int
                + (*filter.offset(1))[1] as libc::c_int
                + (*filter.offset(1))[2] as libc::c_int)
                * 2) as int16_t;
        lr_fn = (*dsp).lr.wiener[((*filter.offset(0))[0] as libc::c_int
            | (*filter.offset(1))[0] as libc::c_int
            == 0) as libc::c_int as usize];
    } else {
        if !((*lr).r#type as libc::c_int == DAV1D_RESTORATION_SGRPROJ as libc::c_int) {
            unreachable!();
        }
        let sgr_params: *const uint16_t = (dav1d_sgr_params[(*lr).sgr_idx as usize]).as_ptr();
        params.sgr.s0 = *sgr_params.offset(0) as uint32_t;
        params.sgr.s1 = *sgr_params.offset(1) as uint32_t;
        params.sgr.w0 = (*lr).sgr_weights[0] as int16_t;
        params.sgr.w1 = (128 as libc::c_int
            - ((*lr).sgr_weights[0] as libc::c_int + (*lr).sgr_weights[1] as libc::c_int))
            as int16_t;
        lr_fn = (*dsp).lr.sgr[((*sgr_params.offset(0) != 0) as libc::c_int
            + (*sgr_params.offset(1) != 0) as libc::c_int * 2
            - 1) as usize];
    }
    while y + stripe_h <= row_h {
        edges = ::core::mem::transmute::<libc::c_uint, LrEdgeFlags>(
            edges as libc::c_uint
                ^ (-((sby + 1 != (*f).sbh || y + stripe_h != row_h) as libc::c_int)
                    as libc::c_uint
                    ^ edges as libc::c_uint)
                    & LR_HAVE_BOTTOM as libc::c_int as libc::c_uint,
        );
        lr_fn(
            p.cast(),
            stride,
            left.cast(),
            lpf.cast(),
            unit_w,
            stripe_h,
            &mut params,
            edges,
            8,
        );
        left = left.offset(stripe_h as isize);
        y += stripe_h;
        p = p.offset((stripe_h as isize * stride) as isize);
        edges = ::core::mem::transmute::<libc::c_uint, LrEdgeFlags>(
            edges as libc::c_uint | LR_HAVE_TOP as libc::c_int as libc::c_uint,
        );
        stripe_h = cmp::min(64 >> ss_ver, row_h - y);
        if stripe_h == 0 {
            break;
        }
        lpf = lpf.offset((4 * stride) as isize);
    }
}
unsafe extern "C" fn backup4xU(
    mut dst: *mut [pixel; 4],
    mut src: *const pixel,
    src_stride: ptrdiff_t,
    mut u: libc::c_int,
) {
    while u > 0 {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        u -= 1;
        dst = dst.offset(1);
        src = src.offset(src_stride as isize);
    }
}
unsafe extern "C" fn lr_sbrow(
    f: *const Dav1dFrameContext,
    mut p: *mut pixel,
    y: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    row_h: libc::c_int,
    plane: libc::c_int,
) {
    let chroma = (plane != 0) as libc::c_int;
    let ss_ver = chroma
        & ((*f).sr_cur.p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint) as libc::c_int;
    let ss_hor = chroma
        & ((*f).sr_cur.p.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint) as libc::c_int;
    let p_stride: ptrdiff_t = (*f).sr_cur.p.stride[chroma as usize];
    let unit_size_log2 =
        (*(*f).frame_hdr).restoration.unit_size[(plane != 0) as libc::c_int as usize];
    let unit_size = (1 as libc::c_int) << unit_size_log2;
    let half_unit_size = unit_size >> 1;
    let max_unit_size = unit_size + half_unit_size;
    let row_y = y + (8 >> ss_ver) * (y != 0) as libc::c_int;
    let shift_hor = 7 - ss_hor;
    let mut pre_lr_border: Align16<[[[pixel; 4]; 136]; 2]> = Align16([[[0; 4]; 136]; 2]);
    let mut lr: [*const Av1RestorationUnit; 2] = [0 as *const Av1RestorationUnit; 2];
    let mut edges: LrEdgeFlags = ((if y > 0 {
        LR_HAVE_TOP as libc::c_int
    } else {
        0 as libc::c_int
    }) | LR_HAVE_RIGHT as libc::c_int) as LrEdgeFlags;
    let mut aligned_unit_pos = row_y & !(unit_size - 1);
    if aligned_unit_pos != 0 && aligned_unit_pos + half_unit_size > h {
        aligned_unit_pos -= unit_size;
    }
    aligned_unit_pos <<= ss_ver;
    let sb_idx = (aligned_unit_pos >> 7) * (*f).sr_sb128w;
    let unit_idx = (aligned_unit_pos >> 6 & 1) << 1;
    lr[0] = &mut *(*((*((*f).lf.lr_mask).offset(sb_idx as isize)).lr)
        .as_mut_ptr()
        .offset(plane as isize))
    .as_mut_ptr()
    .offset(unit_idx as isize) as *mut Av1RestorationUnit;
    let mut restore =
        ((*lr[0]).r#type as libc::c_int != DAV1D_RESTORATION_NONE as libc::c_int) as libc::c_int;
    let mut x = 0;
    let mut bit = 0;
    while x + max_unit_size <= w {
        let next_x = x + unit_size;
        let next_u_idx = unit_idx + (next_x >> shift_hor - 1 & 1);
        lr[(bit == 0) as libc::c_int as usize] =
            &mut *(*((*((*f).lf.lr_mask).offset((sb_idx + (next_x >> shift_hor)) as isize)).lr)
                .as_mut_ptr()
                .offset(plane as isize))
            .as_mut_ptr()
            .offset(next_u_idx as isize) as *mut Av1RestorationUnit;
        let restore_next = ((*lr[(bit == 0) as libc::c_int as usize]).r#type as libc::c_int
            != DAV1D_RESTORATION_NONE as libc::c_int) as libc::c_int;
        if restore_next != 0 {
            backup4xU(
                (pre_lr_border[bit as usize]).as_mut_ptr(),
                p.offset(unit_size as isize)
                    .offset(-(4 as libc::c_int as isize)),
                p_stride,
                row_h - y,
            );
        }
        if restore != 0 {
            lr_stripe(
                f,
                p,
                (pre_lr_border[(bit == 0) as libc::c_int as usize]).as_mut_ptr()
                    as *const [pixel; 4],
                x,
                y,
                plane,
                unit_size,
                row_h,
                lr[bit as usize],
                edges,
            );
        }
        x = next_x;
        restore = restore_next;
        p = p.offset(unit_size as isize);
        edges = ::core::mem::transmute::<libc::c_uint, LrEdgeFlags>(
            edges as libc::c_uint | LR_HAVE_LEFT as libc::c_int as libc::c_uint,
        );
        bit ^= 1 as libc::c_int;
    }
    if restore != 0 {
        edges = ::core::mem::transmute::<libc::c_uint, LrEdgeFlags>(
            edges as libc::c_uint & !(LR_HAVE_RIGHT as libc::c_int) as libc::c_uint,
        );
        let unit_w = w - x;
        lr_stripe(
            f,
            p,
            (pre_lr_border[(bit == 0) as libc::c_int as usize]).as_mut_ptr() as *const [pixel; 4],
            x,
            y,
            plane,
            unit_w,
            row_h,
            lr[bit as usize],
            edges,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_lr_sbrow_8bpc(
    f: *mut Dav1dFrameContext,
    dst: *const *mut pixel,
    sby: libc::c_int,
) {
    let offset_y = 8 * (sby != 0) as libc::c_int;
    let dst_stride: *const ptrdiff_t = ((*f).sr_cur.p.stride).as_mut_ptr();
    let restore_planes = (*f).lf.restore_planes;
    let not_last = ((sby + 1) < (*f).sbh) as libc::c_int;
    if restore_planes & LR_RESTORE_Y as libc::c_int != 0 {
        let h = (*f).sr_cur.p.p.h;
        let w = (*f).sr_cur.p.p.w;
        let next_row_y = (sby + 1) << 6 + (*(*f).seq_hdr).sb128;
        let row_h = cmp::min(next_row_y - 8 * not_last, h);
        let y_stripe = (sby << 6 + (*(*f).seq_hdr).sb128) - offset_y;
        lr_sbrow(
            f,
            (*dst.offset(0)).offset(-((offset_y as isize * *dst_stride.offset(0)) as isize)),
            y_stripe,
            w,
            h,
            row_h,
            0 as libc::c_int,
        );
    }
    if restore_planes & (LR_RESTORE_U as libc::c_int | LR_RESTORE_V as libc::c_int) != 0 {
        let ss_ver = ((*f).sr_cur.p.p.layout as libc::c_uint
            == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let ss_hor = ((*f).sr_cur.p.p.layout as libc::c_uint
            != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
            as libc::c_int;
        let h_0 = (*f).sr_cur.p.p.h + ss_ver >> ss_ver;
        let w_0 = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let next_row_y_0 = (sby + 1) << 6 - ss_ver + (*(*f).seq_hdr).sb128;
        let row_h_0 = cmp::min(next_row_y_0 - (8 >> ss_ver) * not_last, h_0);
        let offset_uv = offset_y >> ss_ver;
        let y_stripe_0 = (sby << 6 - ss_ver + (*(*f).seq_hdr).sb128) - offset_uv;
        if restore_planes & LR_RESTORE_U as libc::c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(1)).offset(-((offset_uv as isize * *dst_stride.offset(1)) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                1 as libc::c_int,
            );
        }
        if restore_planes & LR_RESTORE_V as libc::c_int != 0 {
            lr_sbrow(
                f,
                (*dst.offset(2)).offset(-((offset_uv as isize * *dst_stride.offset(1)) as isize)),
                y_stripe_0,
                w_0,
                h_0,
                row_h_0,
                2 as libc::c_int,
            );
        }
    }
}
