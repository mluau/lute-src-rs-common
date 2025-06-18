use crate::LConfig;

/// Finalizes the build process by linking all the necessary libraries
/// in the right order (GNU ld needs the libraries to be linked in exact
/// dependency order).
///
/// Prior to calling this, it is the job of the caller to set the linker 
/// search path.
pub fn finalize_build(lcfg: LConfig, prebuilt: bool) {
    if prebuilt {
        // Link in Luau.LuteExt and Luau.Custom
        println!("cargo:rustc-link-lib=static=Luau.Custom");
        println!("cargo:rustc-link-lib=static=Luau.LuteExt");
    }

    println!("cargo:rustc-link-lib=static=Lute.Luau");
    println!("cargo:rustc-link-lib=static=Luau.Compiler");
    println!("cargo:rustc-link-lib=static=Luau.Analysis");
    println!("cargo:rustc-link-lib=static=Luau.Ast");
    println!("cargo:rustc-link-lib=static=Luau.CodeGen");
    println!("cargo:rustc-link-lib=static=Luau.Config");
    println!("cargo:rustc-link-lib=static=Luau.EqSat");
    println!("cargo:rustc-link-lib=static=Luau.VM");
    if !lcfg.disable_crypto {
        println!("cargo:rustc-link-lib=static=Lute.Crypto");
    }
    println!("cargo:rustc-link-lib=static=Lute.Fs");
    if !lcfg.disable_net {
        println!("cargo:rustc-link-lib=static=Lute.Net");
    }
    println!("cargo:rustc-link-lib=static=Lute.Process");
    println!("cargo:rustc-link-lib=static=Lute.System");
    println!("cargo:rustc-link-lib=static=Lute.Task");
    println!("cargo:rustc-link-lib=static=Lute.Time");
    println!("cargo:rustc-link-lib=static=Lute.VM");
    println!("cargo:rustc-link-lib=static=Lute.Require");
    println!("cargo:rustc-link-lib=static=Lute.Std");
    println!("cargo:rustc-link-lib=static=Lute.Runtime");
    println!("cargo:rustc-link-lib=static=Luau.Require");
    println!("cargo:rustc-link-lib=static=Luau.RequireNavigator"); 
    println!("cargo:rustc-link-lib=static=Luau.CLI.lib");
    if !lcfg.disable_net {
        println!("cargo:rustc-link-lib=static=uSockets");
    }

    if !lcfg.disable_net || !lcfg.disable_crypto {
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=decrepit");
        println!("cargo:rustc-link-lib=static=pki");
        println!("cargo:rustc-link-lib=static=ssl");
    }

    if !lcfg.disable_crypto {
        // libsodium
        println!("cargo:rustc-link-lib=static=sodium");
    }
    
    if !lcfg.disable_net {
        println!("cargo:rustc-link-lib=static=curl");
    }

    // libuv
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=static=uv");
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=User32"); // Solves the __imp_TranslateMessage error
        println!("cargo:rustc-link-lib=Ws2_32"); // For sockets
        println!("cargo:rustc-link-lib=Iphlpapi");
        println!("cargo:rustc-link-lib=Psapi");
        println!("cargo:rustc-link-lib=Userenv");
        println!("cargo:rustc-link-lib=Advapi32");
        println!("cargo:rustc-link-lib=Ole32");
        println!("cargo:rustc-link-lib=Shell32");

        println!("cargo:rustc-link-lib=static=libuv");
    }

    // zlib (system)
    if !lcfg.disable_net {
        println!("cargo:rustc-link-lib=static=z"); 
    }

    if prebuilt {
        // Configure C++ here (todo: determine if its useful for non-prebuilt as well) 
        if let Some(ref cpp_stdlib) = get_cpp_link_stdlib(&std::env::var("TARGET").unwrap(), &std::env::var("HOST").unwrap()) {
            println!("cargo:rustc-link-lib={cpp_stdlib}");
        }
    }
}

/// From mlua (https://github.com/mlua-rs/luau-src-rs/blob/7c89c42b25ce45dec72a15c4f430a0aa1a999897/src/lib.rs#L238C1-L269C1)
/// Returns the C++ standard library:
/// 1) Uses `CXXSTDLIB` environment variable if set
/// 2) The default `c++` for OS X and BSDs
/// 3) `c++_shared` for Android
/// 4) `None` for MSVC
/// 5) `stdc++` for anything else.
///
/// Inspired by the `cc` crate.
fn get_cpp_link_stdlib(target: &str, host: &str) -> Option<String> {
    // Try to get value from the `CXXSTDLIB` env variable
    let kind = if host == target { "HOST" } else { "TARGET" };
    let res = std::env::var(format!("CXXSTDLIB_{target}"))
        .or_else(|_| std::env::var(format!("CXXSTDLIB_{}", target.replace('-', "_"))))
        .or_else(|_| std::env::var(format!("{kind}_CXXSTDLIB")))
        .or_else(|_| std::env::var("CXXSTDLIB"))
        .ok();
    if res.is_some() {
        return res;
    }

    if target.contains("msvc") {
        None
    } else if target.contains("apple") | target.contains("freebsd") | target.contains("openbsd")
    {
        Some("c++".to_string())
    } else if target.contains("android") {
        Some("c++_shared".to_string())
    } else {
        Some("stdc++".to_string())
    }
}
