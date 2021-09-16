use std::path::Path;

fn main() {
    let mut cc = cc::Build::new();
    let jp2dir = Path::new("vendor/src/lib/openjp2");

    let target = std::env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY");
    if target == "windows" {
        cc.define("OPJ_HAVE__ALIGNED_MALLOC", Some("1"));
    } else {
        cc.define("OPJ_HAVE_POSIX_MEMALIGN", Some("1"));
    }
    cc.include(jp2dir);
    cc.include("config");
    cc.define("OPJ_STATIC", Some("1"));
    cc.define("MUTEX_stub", Some("1")); // FIXME define MUTEX_win32 or MUTEX_pthread
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH");
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
    cc.compile("openjp2");
}
