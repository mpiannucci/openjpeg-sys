use std::path::Path;

fn main() {
    let mut cc = cc::Build::new();
    let jp2dir = Path::new("vendor/src/lib/openjp2");

    let target = std::env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY");
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");

    if target == "windows" {
        cc.define("OPJ_HAVE__ALIGNED_MALLOC", Some("1"));
    } else if target_arch == "wasm32" {
        //cc.define("OPJ_HAVE__ALIGNED_MALLOC", Some("0"));
        //cc.define("OPJ_HAVE_POSIX_MEMALIGN", Some("0"));
    } else {
        cc.define("OPJ_HAVE_POSIX_MEMALIGN", Some("1"));
    }

    cc.include(jp2dir);
    cc.include("config");
    cc.define("OPJ_STATIC", Some("1"));
    cc.define("MUTEX_stub", Some("1")); // FIXME define MUTEX_win32 or MUTEX_pthread
    if target_arch == "wasm32" {
        cc.define("_FILE_OFFSET_BITS", Some("64"));
    }

    let files = [
        "thread.c",
        "bio.c",
        "cio.c",
        "dwt.c",
        "event.c",
        "image.c",
        "invert.c",
        "j2k.c",
        "jp2.c",
        "mct.c",
        "mqc.c",
        "openjpeg.c",
        "opj_clock.c",
        "pi.c",
        "t1.c",
        "t2.c",
        "tcd.c",
        "tgt.c",
        "function_list.c",
        "opj_malloc.c",
        "sparse_array.c",
    ];
    for file in files.iter() {
        cc.file(jp2dir.join(file));
    }

    if target_arch == "wasm32" {
        cc.define("_FILE_OFFSET_BITS", Some("64"));

        let walloc_dir = Path::new("walloc");
        cc.include(walloc_dir);
        cc.file(walloc_dir.join("walloc.c"));

        let libc_wasm_dir = Path::new("libc-wasm");
        cc.include(libc_wasm_dir);
        cc.file(libc_wasm_dir.join("assert.c"));
        cc.file(libc_wasm_dir.join("calloc.c"));
        cc.file(libc_wasm_dir.join("errno.c"));
        cc.file(libc_wasm_dir.join("getenv.c"));
        //cc.file(libc_wasm_dir.join("memalign.c"));
        //cc.file(libc_wasm_dir.join("posix_memalign.c"));
    }

    cc.compile("openjp2");
}
