extern crate nasm_rs;

const SIMD_PATH: &'static str = "mozjpeg-c/simd";

fn main() {
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "x86") {
        "i386"
    } else {
        panic!("Unsupported target architecture");
    };
    let sources = if cfg!(target_arch = "x86_64") {
        vec![
            "jsimdcpu.asm",
            "jfdctflt-sse.asm",
            "jccolor-sse2.asm",
            "jcgray-sse2.asm",
            "jchuff-sse2.asm",
            "jcphuff-sse2.asm",
            "jcsample-sse2.asm",
            "jdcolor-sse2.asm",
            "jdmerge-sse2.asm",
            "jdsample-sse2.asm",
            "jfdctfst-sse2.asm",
            "jfdctint-sse2.asm",
            "jidctflt-sse2.asm",
            "jidctfst-sse2.asm",
            "jidctint-sse2.asm",
            "jidctred-sse2.asm",
            "jquantf-sse2.asm",
            "jquanti-sse2.asm",
            "jccolor-avx2.asm",
            "jcgray-avx2.asm",
            "jcsample-avx2.asm",
            "jdcolor-avx2.asm",
            "jdmerge-avx2.asm",
            "jdsample-avx2.asm",
            "jfdctint-avx2.asm",
            "jidctint-avx2.asm",
            "jquanti-avx2.asm",
        ]
    } else if cfg!(target_arch = "x86") {
        vec![
            "jsimdcpu.asm",
            "jfdctflt-3dn.asm",
            "jidctflt-3dn.asm",
            "jquant-3dn.asm",
            "jccolor-mmx.asm",
            "jcgray-mmx.asm",
            "jcsample-mmx.asm",
            "jdcolor-mmx.asm",
            "jdmerge-mmx.asm",
            "jdsample-mmx.asm",
            "jfdctfst-mmx.asm",
            "jfdctint-mmx.asm",
            "jidctfst-mmx.asm",
            "jidctint-mmx.asm",
            "jidctred-mmx.asm",
            "jquant-mmx.asm",
            "jfdctflt-sse.asm",
            "jidctflt-sse.asm",
            "jquant-sse.asm",
            "jccolor-sse2.asm",
            "jcgray-sse2.asm",
            "jchuff-sse2.asm",
            "jcphuff-sse2.asm",
            "jcsample-sse2.asm",
            "jdcolor-sse2.asm",
            "jdmerge-sse2.asm",
            "jdsample-sse2.asm",
            "jfdctfst-sse2.asm",
            "jfdctint-sse2.asm",
            "jidctflt-sse2.asm",
            "jidctfst-sse2.asm",
            "jidctint-sse2.asm",
            "jidctred-sse2.asm",
            "jquantf-sse2.asm",
            "jquanti-sse2.asm",
            "jccolor-avx2.asm",
            "jcgray-avx2.asm",
            "jcsample-avx2.asm",
            "jdcolor-avx2.asm",
            "jdmerge-avx2.asm",
            "jdsample-avx2.asm",
            "jfdctint-avx2.asm",
            "jidctint-avx2.asm",
            "jquanti-avx2.asm",
        ]
    } else {
        panic!("Unsupported target architecture");
    };

    let sources = sources
        .into_iter()
        .map(|src| format!("{}/{}/{}", SIMD_PATH, arch, src))
        .collect::<Vec<_>>();

    let mut args = vec![format!("-I{}/nasm/", SIMD_PATH)];

    if cfg!(target_os = "macos") {
        args.push("-DMACHO".to_string());
    } else if cfg!(unix) {
        args.push("-DELF".to_string());
    }

    if cfg!(target_arch = "x86_64") {
        if cfg!(windows) {
            args.push("-DWIN64".to_string());
        }
        args.push("-D__x86_64__".to_string());
        args.push(format!("-I{}/x86_64/", SIMD_PATH));
    } else if cfg!(target_arch = "x86") {
        if cfg!(windows) {
            args.push("-DWIN32".to_string());
        }
        args.push(format!("-I{}/i386/", SIMD_PATH));
    }

    if cfg!(not(windows)) {
        // Needed when building a shared library, which is all we support for
        // now
        args.push("-DPIC".to_string());
    }

    if cfg!(debug_assertions) {
        args.push("-g".to_string());
    }

    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    nasm_rs::compile_library_args("libsimd.a", &sources, &args);
    println!("cargo:rustc-link-lib=static=simd");
}
