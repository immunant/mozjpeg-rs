use libc::c_double;use crate::jmorecfg_h::JSAMPLE;
#[inline(always)]
pub unsafe extern "C" fn rgb_to_cmyk(
    mut r: JSAMPLE,
    mut g: JSAMPLE,
    mut b: JSAMPLE,
    mut c: *mut JSAMPLE,
    mut m: *mut JSAMPLE,
    mut y: *mut JSAMPLE,
    mut k: *mut JSAMPLE,
) {
    let mut ctmp: c_double = 1.0f64 - r as c_double / 255.0f64;
    let mut mtmp: c_double = 1.0f64 - g as c_double / 255.0f64;
    let mut ytmp: c_double = 1.0f64 - b as c_double / 255.0f64;
    let mut ktmp: c_double = if (if ctmp < mtmp { ctmp } else { mtmp }) < ytmp {
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
    *c = (255.0f64 - ctmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *m = (255.0f64 - mtmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *y = (255.0f64 - ytmp * 255.0f64 + 0.5f64) as JSAMPLE;
    *k = (255.0f64 - ktmp * 255.0f64 + 0.5f64) as JSAMPLE;
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
    mut c: JSAMPLE,
    mut m: JSAMPLE,
    mut y: JSAMPLE,
    mut k: JSAMPLE,
    mut r: *mut JSAMPLE,
    mut g: *mut JSAMPLE,
    mut b: *mut JSAMPLE,
) {
    *r = (c as c_double * k as c_double / 255.0f64 + 0.5f64)
        as JSAMPLE;
    *g = (m as c_double * k as c_double / 255.0f64 + 0.5f64)
        as JSAMPLE;
    *b = (y as c_double * k as c_double / 255.0f64 + 0.5f64)
        as JSAMPLE;
}
