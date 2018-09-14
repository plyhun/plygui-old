extern crate cc;
extern crate pkg_config;

fn main() {
    let gtk_probe = pkg_config::Config::new().atleast_version("3.0").probe("gtk+-3.0").unwrap();
    let glib_probe = pkg_config::Config::new().atleast_version("2.0").probe("glib-2.0").unwrap();

    let mut cc_build = cc::Build::new();

    for lib in gtk_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    for lib in glib_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }

    cc_build.include("ffi").define("STATIC_BUILD", None).opt_level(3).warnings(false).file("ffi/reckless_fixed.c").compile("gtk_reckless_fixed");
}
