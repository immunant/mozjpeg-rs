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
extern crate mozjpeg;

pub use mozjpeg::cdjpeg;
pub use mozjpeg::jcapimin;
pub use mozjpeg::jcapistd;
pub use mozjpeg::jccoefct;
pub use mozjpeg::jccolor;
pub use mozjpeg::jcdctmgr;
pub use mozjpeg::jcext;
pub use mozjpeg::jchuff;
pub use mozjpeg::jcicc;
pub use mozjpeg::jcinit;
pub use mozjpeg::jcmainct;
pub use mozjpeg::jcmarker;
pub use mozjpeg::jcmaster;
pub use mozjpeg::jcomapi;
pub use mozjpeg::jcparam;
pub use mozjpeg::jcphuff;
pub use mozjpeg::jcprepct;
pub use mozjpeg::jcsample;
pub use mozjpeg::jctrans;
pub use mozjpeg::jdapimin;
pub use mozjpeg::jdapistd;
pub use mozjpeg::jdatadst;
pub use mozjpeg::jdatasrc;
pub use mozjpeg::jdcoefct;
pub use mozjpeg::jdcolor;
pub use mozjpeg::jddctmgr;
pub use mozjpeg::jdhuff;
pub use mozjpeg::jdicc;
pub use mozjpeg::jdinput;
pub use mozjpeg::jdmainct;
pub use mozjpeg::jdmarker;
pub use mozjpeg::jdmaster;
pub use mozjpeg::jdmerge;
pub use mozjpeg::jdphuff;
pub use mozjpeg::jdpostct;
pub use mozjpeg::jdsample;
pub use mozjpeg::jdtrans;
pub use mozjpeg::jerror;
pub use mozjpeg::jfdctflt;
pub use mozjpeg::jfdctfst;
pub use mozjpeg::jfdctint;
pub use mozjpeg::jidctflt;
pub use mozjpeg::jidctfst;
pub use mozjpeg::jidctint;
pub use mozjpeg::jidctred;
pub use mozjpeg::jmemmgr;
pub use mozjpeg::jmemnobs;
pub use mozjpeg::jquant1;
pub use mozjpeg::jquant2;
pub use mozjpeg::jsimd;
pub use mozjpeg::jutils;
