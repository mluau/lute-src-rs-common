use crate::{LConfig, cmake::Config};

pub fn setup_lute_cmake(lcfg: LConfig) -> std::path::PathBuf {
    let mut config = cc::Build::new();

    let target = std::env::var("TARGET").unwrap();

    if target.ends_with("emscripten") {
        // Enable c++ exceptions for emscripten (it's disabled by default)
        // Later we should switch to wasm exceptions
        config.flag_if_supported("-fexceptions");
    }

    config
        .warnings(false)
        .cargo_metadata(true)
        .std("c++20")
        .cpp(true)
        .static_crt(true);

    Config::new("lute")
        .profile("Release") // Debug builds tend to be extremely slow and nearly unusable in practice
        .define("LUAU_EXTERN_C", "ON") // Provides DLUA_USE_LONGJMP, DLUA_API, LUACODE_API, LUACODEGEN_API
        .define("LUAU_STATIC_CRT", "ON")
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreaded$<$<CONFIG:Debug>:Debug>") // Use static CRT for MSVC
        .define("LUAU_BUILD_STATIC", "ON")
        .define("LUTE_DISABLE_NET", if lcfg.disable_net { "ON" } else { "OFF" } )
        .define("LUTE_DISABLE_CRYPTO", if lcfg.disable_crypto { "ON" } else { "OFF" }  )
        .cxxflag("-DLUAI_MAXCSTACK=1000000")
        .cxxflag("-DLUA_UTAG_LIMIT=128")
        .cxxflag("-DLUA_LUTAG_LIMIT=128") 
        .cxxflag("-DLUA_USE_LONGJMP=1") // Use longjmp for error handling
        .cxxflag(
            "-fexceptions" // Enable C++ exceptions on non-Windows
        )
        .init_cxx_cfg(config)
        .no_build_target(true)
        .static_crt(true)
        .build()
}

pub fn build_cc_lute_lib(lcfg: LConfig, lib_name: &str, files: Vec<String>) {
    let mut build = cc::Build::new();

    /*
    let mut build = cc::Build::new();

    build
        .host(&host)
        .target(&target)
        .cpp(true)
	    .std("c++20")
        .file("LuteExt/src/lopen.cpp")
        .include("lute/lute/cli/include")
        .include("lute/lute/crypto/include")
        .include("lute/lute/fs/include")
        .include("lute/lute/luau/include")
        .include("lute/lute/net/include")
        .include("lute/lute/process/include")
        .include("lute/lute/system/include")
        .include("lute/lute/vm/include")
        .include("lute/lute/task/include")
        .include("lute/lute/time/include")
        .include("lute/lute/runtime/include")
        .include("lute/extern/luau/VM/include")
        .include("lute/extern/luau/VM/src")
        .include("lute/extern/luau/Common/include")
        .include("lute/extern/luau/Compiler/include")
        .include("lute/extern/libuv/include")
        .flag("-DLUA_USE_LONGJMP=1")
        .flag("-DLUA_API=extern \"C\"")
        .flag("-DLUACODE_API=extern \"C\"")
        .flag("-DLUACODEGEN_API=extern \"C\"")
        .flag("-DLUAI_MAXCSTACK=1000000")
        .flag("-DLUA_UTAG_LIMIT=128")
        .flag("-DLUA_LUTAG_LIMIT=128")
        .flag_if_supported(
            "-fexceptions" // Enable C++ exceptions on non-Windows
        ); 
        
    if lcfg.disable_net {
        build.flag("-DLUTE_DISABLE_NET=1");
    }

    if lcfg.disable_crypto {
        build.flag("-DLUTE_DISABLE_CRYPTO=1");
    }

    build
        .static_crt(true)
        .out_dir(&prebuilts_dir)
        .compile("Luau.LuteExt");
    */

    build
        .cpp(true)
	    .std("c++20")
        .files(files)
        .flag("-DLUA_USE_LONGJMP=1")
        .flag("-DLUA_API=extern \"C\"")
        .flag("-DLUACODE_API=extern \"C\"")
        .flag("-DLUACODEGEN_API=extern \"C\"")
        .flag("-DLUAI_MAXCSTACK=1000000")
        .flag("-DLUA_UTAG_LIMIT=128") 
        .flag("-DLUA_LUTAG_LIMIT=128") 
        .flag_if_supported(
            "-fexceptions" // Enable C++ exceptions on non-Windows
        )
        .include("lute/lute/cli/include")
        .include("lute/lute/crypto/include")
        .include("lute/lute/fs/include")
        .include("lute/lute/luau/include")
        .include("lute/lute/net/include")
        .include("lute/lute/process/include")
        .include("lute/lute/system/include")
        .include("lute/lute/vm/include")
        .include("lute/lute/task/include")
        .include("lute/lute/time/include")
        .include("lute/lute/runtime/include")
        .include("lute/extern/luau/VM/include")
        .include("lute/extern/luau/VM/src")
        .include("lute/extern/luau/Common/include")
        .include("lute/extern/luau/Compiler/include")
        .include("lute/extern/libuv/include")
        .static_crt(true);

    if lcfg.disable_net {
        build.flag("-DLUTE_DISABLE_NET=1");
    }

    if lcfg.disable_crypto {
        build.flag("-DLUTE_DISABLE_CRYPTO=1");
    }

    build
        .compile(lib_name);
}