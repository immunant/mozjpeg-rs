use crate::jmorecfg_h::JSAMPLE;
#[inline(always)]
pub unsafe extern "C" fn rgb_to_cmyk(
    mut r: crate::jmorecfg_h::JSAMPLE,
    mut g: crate::jmorecfg_h::JSAMPLE,
    mut b: crate::jmorecfg_h::JSAMPLE,
    mut c: *mut crate::jmorecfg_h::JSAMPLE,
    mut m: *mut crate::jmorecfg_h::JSAMPLE,
    mut y: *mut crate::jmorecfg_h::JSAMPLE,
    mut k: *mut crate::jmorecfg_h::JSAMPLE,
) {
    let mut ctmp: libc::c_double = 1.0f64 - r as libc::c_double / 255.0f64;
    let mut mtmp: libc::c_double = 1.0f64 - g as libc::c_double / 255.0f64;
    let mut ytmp: libc::c_double = 1.0f64 - b as libc::c_double / 255.0f64;
    let mut ktmp: libc::c_double = if (if ctmp < mtmp { ctmp } else { mtmp }) < ytmp {
        if ctmp < mtmp {
            ctmp
        } else {
            mtmp
        }
    } else {
        ytmp
    };
    if ktmp == 1.0f64 {
        ytmp = 0.0f64;
        mtmp = ytmp;
        ctmp = mtmp
    } else {
        ctmp = (ctmp - ktmp) / (1.0f64 - ktmp);
        mtmp = (mtmp - ktmp) / (1.0f64 - ktmp);
        ytmp = (ytmp - ktmp) / (1.0f64 - ktmp)
    }
    *c = (255.0f64 - ctmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
    *m = (255.0f64 - mtmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
    *y = (255.0f64 - ytmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
    *k = (255.0f64 - ktmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
}
/*
 * cmyk.h
 *
 * Copyright (C) 2017-2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains convenience functions for performing quick & dirty
 * CMYK<->RGB conversion.  This algorithm is suitable for testing purposes
 * only.  Properly converting between CMYK and RGB requires a color management
 * system.
 */

/* Fully reversible */

/* Fully reversible only for C/M/Y/K values generated with rgb_to_cmyk() */
#[inline(always)]
pub unsafe extern "C" fn cmyk_to_rgb(
    mut c: crate::jmorecfg_h::JSAMPLE,
    mut m: crate::jmorecfg_h::JSAMPLE,
    mut y: crate::jmorecfg_h::JSAMPLE,
    mut k: crate::jmorecfg_h::JSAMPLE,
    mut r: *mut crate::jmorecfg_h::JSAMPLE,
    mut g: *mut crate::jmorecfg_h::JSAMPLE,
    mut b: *mut crate::jmorecfg_h::JSAMPLE,
) {
    *r = (c as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
        as crate::jmorecfg_h::JSAMPLE;
    *g = (m as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
        as crate::jmorecfg_h::JSAMPLE;
    *b = (y as libc::c_double * k as libc::c_double / 255.0f64 + 0.5f64)
        as crate::jmorecfg_h::JSAMPLE;
}
