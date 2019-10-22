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
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]

pub mod src {
    pub mod cdjpeg;
    
    
    pub mod jcapimin;
    pub mod jcapistd;
    pub mod jccoefct;
    pub mod jccolor;
    pub mod jcdctmgr;
    pub mod jcext;
    pub mod jchuff;
    pub mod jcicc;
    pub mod jcinit;
    pub mod jcmainct;
    pub mod jcmarker;
    pub mod jcmaster;
    pub mod jcomapi;
    pub mod jcparam;
    pub mod jcphuff;
    pub mod jcprepct;
    pub mod jcsample;
    
    pub mod jctrans;
    pub mod jdapimin;
    pub mod jdapistd;
    pub mod jdatadst;
    pub mod jdatadst_tj;
    pub mod jdatasrc;
    pub mod jdatasrc_tj;
    pub mod jdcoefct;
    pub mod jdcolor;
    pub mod jddctmgr;
    pub mod jdhuff;
    pub mod jdicc;
    pub mod jdinput;
    pub mod jdmainct;
    pub mod jdmarker;
    pub mod jdmaster;
    pub mod jdmerge;
    pub mod jdphuff;
    pub mod jdpostct;
    pub mod jdsample;
    pub mod jdtrans;
    pub mod jerror;
    pub mod jfdctflt;
    pub mod jfdctfst;
    pub mod jfdctint;
    pub mod jidctflt;
    pub mod jidctfst;
    pub mod jidctint;
    pub mod jidctred;
    pub mod jmemmgr;
    pub mod jmemnobs;
    
    pub mod jquant1;
    pub mod jquant2;
    pub mod jutils;
    pub mod md5 {
        pub mod md5;
        
        pub mod md5hl;
    } // mod md5
    pub mod rdbmp;
    pub mod rdcolmap;
    pub mod rdgif;
    pub mod rdjpeg;
    
    pub mod rdppm;
    pub mod rdswitch;
    pub mod rdtarga;
    pub mod simd {
        pub mod x86_64 {
            pub mod jsimd;
        } // mod x86_64
    } // mod simd
    
    
    
    pub mod tjutil;
    pub mod transupp;
    pub mod turbojpeg;
    pub mod wrbmp;
    pub mod wrgif;
    
    pub mod wrppm;
    pub mod wrtarga;
}
#[path = "src/cderror_h.rs"]
pub mod cderror_h;
#[path = "src/cmyk_h.rs"]
pub mod cmyk_h;
#[path = "src/internal.rs"]
pub mod internal;
#[path = "src/jccolext_c.rs"]
pub mod jccolext_c;
#[path = "src/jconfig_h.rs"]
pub mod jconfig_h;
#[path = "src/jconfigint_h.rs"]
pub mod jconfigint_h;
#[path = "src/jdcol565_c.rs"]
pub mod jdcol565_c;
#[path = "src/jdcolext_c.rs"]
pub mod jdcolext_c;
#[path = "src/jdct_h.rs"]
pub mod jdct_h;
#[path = "src/jdmrg565_c.rs"]
pub mod jdmrg565_c;
#[path = "src/jdmrgext_c.rs"]
pub mod jdmrgext_c;
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
#[path = "src/jstdhuff_c.rs"]
pub mod jstdhuff_c;
#[path = "src/jversion_h.rs"]
pub mod jversion_h;
#[path = "src/limits_h.rs"]
pub mod limits_h;
#[path = "src/stddef_h.rs"]
pub mod stddef_h;
#[path = "src/stdlib.rs"]
pub mod stdlib; // mod src
