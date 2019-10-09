#[cfg(workaround_build)]
fn main() {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    use cmake;
    use cmake::Config;

    let dst = Config::new("longfi-device")
                 .define("BUILD_TESTING", "OFF")
                 .define("CMAKE_C_COMPILER_WORKS", "1")
                 .define("CMAKE_CXX_COMPILER_WORKS", "1")
                 .define("CMAKE_TOOLCHAIN_FILE", "toolchain-gcc-arm-none-eabi.cmake")
                 .pic(false)
                 .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=longfi");
    println!("cargo:rustc-link-lib=static=sx12xx");

   // make the bindings
   let bindings = bindgen::Builder::default()
       .raw_line("use cty;")
       .use_core()
       .ctypes_prefix("cty")
       .header("longfi-device/board.h")
       .header("longfi-device/longfi.h")
       .header("longfi-device/radio/radio.h")
       .header("longfi-device/radio/sx1276/sx1276.h")
       .whitelist_var("XTAL_FREQ")
       .whitelist_var("FREQ_STEP")
       .whitelist_var("RX_BUFFER_SIZE")
       .whitelist_type("RfEvent_t")
       .whitelist_type("RadioState_t")
       .whitelist_type("RadioModems_t")
       .whitelist_type("ClientEvent_t")
       .whitelist_type("QualityOfService_t")
       .whitelist_type("RfConfig_t")
       .whitelist_type("RxPacket_t")
       .whitelist_type("LF_Gpio_t")
       .whitelist_type("LF_Spi_t")
       .whitelist_type("AntPinsMode_t")
       .whitelist_function("longfi_init")
       .whitelist_function("longfi_new_handle")
       .whitelist_function("longfi_handle_event")
       .whitelist_function("longfi_send")
       .whitelist_function("longfi_get_rx")
       .whitelist_function("longfi_set_buf")
       .whitelist_function("longfi_rf_test")
       .whitelist_function("longfi_get_random")
       .whitelist_function("longfi_enable_tcxo")
       .whitelist_function("board_set_bindings")
       .whitelist_function("SX1276RadioNew")
       .trust_clang_mangling(false)
       .rustfmt_bindings(true)
       .rustified_enum("ClientEvent_t")
       .rustified_enum("RfEvent_t")
       .rustified_enum("QualityOfService_t")
       .rustified_enum("AntPinsMode_t")
       .derive_copy(false)
       .derive_debug(false)
       .layout_tests(false)
       .generate()
       .expect("Failed to generate sx1276 bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(workaround_build))]
fn main() {
   cargo_5730::run_build_script();
}
