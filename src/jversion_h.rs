use libc::c_char;
pub const JVERSION: [c_char; 16] =
    unsafe { *::std::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"6b  27-Mar-1998\x00") };
/*
 * NOTE: It is our convention to place the authors in the following order:
 * - libjpeg-turbo authors (2009-) in descending order of the date of their
 *   most recent contribution to the project, then in ascending order of the
 *   date of their first contribution to the project
 * - Upstream authors in descending order of the date of the first inclusion of
 *   their code
 */
pub const JCOPYRIGHT: [c_char; 533] = unsafe {
    *::std::mem::transmute::<&[u8; 533],
                                     &[c_char; 533]>(b"Copyright (C) 2009-2018 D. R. Commander\nCopyright (C) 2011-2016 Siarhei Siamashka\nCopyright (C) 2015-2016, 2018 Matthieu Darbois\nCopyright (C) 2015 Intel Corporation\nCopyright (C) 2015 Google, Inc.\nCopyright (C) 2014 Mozilla Corporation\nCopyright (C) 2013-2014 MIPS Technologies, Inc.\nCopyright (C) 2013 Linaro Limited\nCopyright (C) 2009-2011 Nokia Corporation and/or its subsidiary(-ies)\nCopyright (C) 2009 Pierre Ossman for Cendio AB\nCopyright (C) 1999-2006 MIYASAKA Masaru\nCopyright (C) 1991-2016 Thomas G. Lane, Guido Vollbeding\x00")
};
