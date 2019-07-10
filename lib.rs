#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(custom_attribute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]

extern crate libc;

#[path = "src/cderror_h.rs"]
pub mod cderror_h;
#[path = "src/cdjpeg.rs"]
pub mod cdjpeg;

#[path = "src/cmyk_h.rs"]
pub mod cmyk_h;

#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/jcapimin.rs"]
pub mod jcapimin;
#[path = "src/jcapistd.rs"]
pub mod jcapistd;
#[path = "src/jccoefct.rs"]
pub mod jccoefct;
#[path = "src/jccolext_c.rs"]
pub mod jccolext_c;
#[path = "src/jccolor.rs"]
pub mod jccolor;
#[path = "src/jcdctmgr.rs"]
pub mod jcdctmgr;
#[path = "src/jcext.rs"]
pub mod jcext;
#[path = "src/jchuff.rs"]
pub mod jchuff;
#[path = "src/jcicc.rs"]
pub mod jcicc;
#[path = "src/jcinit.rs"]
pub mod jcinit;
#[path = "src/jcmainct.rs"]
pub mod jcmainct;
#[path = "src/jcmarker.rs"]
pub mod jcmarker;
#[path = "src/jcmaster.rs"]
pub mod jcmaster;
#[path = "src/jcomapi.rs"]
pub mod jcomapi;
#[path = "src/jconfig_h.rs"]
pub mod jconfig_h;
#[path = "src/jconfigint_h.rs"]
pub mod jconfigint_h;
#[path = "src/jcparam.rs"]
pub mod jcparam;
#[path = "src/jcphuff.rs"]
pub mod jcphuff;
#[path = "src/jcprepct.rs"]
pub mod jcprepct;
#[path = "src/jcsample.rs"]
pub mod jcsample;

#[path = "src/jctrans.rs"]
pub mod jctrans;
#[path = "src/jdapimin.rs"]
pub mod jdapimin;
#[path = "src/jdapistd.rs"]
pub mod jdapistd;
#[path = "src/jdatadst.rs"]
pub mod jdatadst;
#[path = "src/jdatadst_tj.rs"]
pub mod jdatadst_tj;
#[path = "src/jdatasrc.rs"]
pub mod jdatasrc;
#[path = "src/jdatasrc_tj.rs"]
pub mod jdatasrc_tj;
#[path = "src/jdcoefct.rs"]
pub mod jdcoefct;
#[path = "src/jdcol565_c.rs"]
pub mod jdcol565_c;
#[path = "src/jdcolext_c.rs"]
pub mod jdcolext_c;
#[path = "src/jdcolor.rs"]
pub mod jdcolor;
#[path = "src/jdct_h.rs"]
pub mod jdct_h;
#[path = "src/jddctmgr.rs"]
pub mod jddctmgr;
#[path = "src/jdhuff.rs"]
pub mod jdhuff;
#[path = "src/jdicc.rs"]
pub mod jdicc;
#[path = "src/jdinput.rs"]
pub mod jdinput;
#[path = "src/jdmainct.rs"]
pub mod jdmainct;
#[path = "src/jdmarker.rs"]
pub mod jdmarker;
#[path = "src/jdmaster.rs"]
pub mod jdmaster;
#[path = "src/jdmerge.rs"]
pub mod jdmerge;
#[path = "src/jdmrg565_c.rs"]
pub mod jdmrg565_c;
#[path = "src/jdmrgext_c.rs"]
pub mod jdmrgext_c;
#[path = "src/jdphuff.rs"]
pub mod jdphuff;
#[path = "src/jdpostct.rs"]
pub mod jdpostct;
#[path = "src/jdsample.rs"]
pub mod jdsample;
#[path = "src/jdtrans.rs"]
pub mod jdtrans;
#[path = "src/jerror.rs"]
pub mod jerror;
#[path = "src/jfdctflt.rs"]
pub mod jfdctflt;
#[path = "src/jfdctfst.rs"]
pub mod jfdctfst;
#[path = "src/jfdctint.rs"]
pub mod jfdctint;
#[path = "src/jidctflt.rs"]
pub mod jidctflt;
#[path = "src/jidctfst.rs"]
pub mod jidctfst;
#[path = "src/jidctint.rs"]
pub mod jidctint;
#[path = "src/jidctred.rs"]
pub mod jidctred;
#[path = "src/jmemmgr.rs"]
pub mod jmemmgr;
#[path = "src/jmemnobs.rs"]
pub mod jmemnobs;
#[path = "src/jmemsys_h.rs"]
pub mod jmemsys_h;
#[path = "src/jmorecfg_h.rs"]
pub mod jmorecfg_h;
#[path = "src/jpeg_nbits_table_h.rs"]
pub mod jpeg_nbits_table_h;
#[path = "src/jpegint_h.rs"]
pub mod jpegint_h;
#[path = "src/jpeglib_h.rs"]
pub mod jpeglib_h;

#[path = "src/jquant1.rs"]
pub mod jquant1;
#[path = "src/jquant2.rs"]
pub mod jquant2;
#[path = "src/jsimd.rs"]
pub mod jsimd;
#[path = "src/jstdhuff_c.rs"]
pub mod jstdhuff_c;
#[path = "src/jutils.rs"]
pub mod jutils;
#[path = "src/jversion_h.rs"]
pub mod jversion_h;
#[path = "src/limits_h.rs"]
pub mod limits_h;
#[path = "src/md5.rs"]
pub mod md5;

#[path = "src/md5hl.rs"]
pub mod md5hl;
#[path = "src/rdbmp.rs"]
pub mod rdbmp;
#[path = "src/rdcolmap.rs"]
pub mod rdcolmap;
#[path = "src/rdgif.rs"]
pub mod rdgif;
#[path = "src/rdjpeg.rs"]
pub mod rdjpeg;

#[path = "src/rdppm.rs"]
pub mod rdppm;
#[path = "src/rdswitch.rs"]
pub mod rdswitch;
#[path = "src/rdtarga.rs"]
pub mod rdtarga;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib;



#[path = "src/tjutil.rs"]
pub mod tjutil;
#[path = "src/transupp.rs"]
pub mod transupp;
#[path = "src/turbojpeg.rs"]
pub mod turbojpeg;
#[path = "src/wrbmp.rs"]
pub mod wrbmp;
#[path = "src/wrgif.rs"]
pub mod wrgif;

#[path = "src/wrppm.rs"]
pub mod wrppm;
#[path = "src/wrtarga.rs"]
pub mod wrtarga;
