use bindgen::Builder;
use std::env;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=php_wrapper.h");
    println!("cargo:rerun-if-env-changed=PHP_CONFIG");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let php_config = env::var("PHP_CONFIG").unwrap_or("php-config".to_string());

    // Generate php const.

    let php_bin = execute_command(&[php_config.as_str(), "--php-binary"]);
    let php_info = execute_command(&[php_bin.as_str(), "-i"]);

    let php_extension_build = php_info
        .lines()
        .find_map(|line| {
            if line.starts_with("PHP Extension Build") {
                Some(
                    line.chars()
                        .skip_while(|c| *c != 'A')
                        .collect::<String>()
                        .trim()
                        .to_owned(),
                )
            } else {
                None
            }
        })
        .expect("Can't found the field `PHP Extension Build`");

    println!(
        "cargo:rustc-env=PHP_EXTENSION_BUILD={}",
        php_extension_build
    );

    // Generate bindgen file.
    let includes = execute_command(&[php_config.as_str(), "--includes"]);
    let includes = includes.split(' ').collect::<Vec<_>>();

    includes.iter().for_each(|include| {
        let include = &include[2..];
        println!("cargo:include={}", include);
    });

    let bindings = Builder::default()
        .header("php_wrapper.h")
        .clang_args(&includes)
        .blacklist_function("__acosf64x")
        .blacklist_function("__acosf64x")
        .blacklist_function("__acoshf64x")
        .blacklist_function("__acoshf64x")
        .blacklist_function("__acoshl")
        .blacklist_function("__acoshl")
        .blacklist_function("__acosl")
        .blacklist_function("__acosl")
        .blacklist_function("__asinf64x")
        .blacklist_function("__asinf64x")
        .blacklist_function("__asinhf64x")
        .blacklist_function("__asinhf64x")
        .blacklist_function("__asinhl")
        .blacklist_function("__asinhl")
        .blacklist_function("__asinl")
        .blacklist_function("__asinl")
        .blacklist_function("__atan2f64x")
        .blacklist_function("__atan2f64x")
        .blacklist_function("__atan2f64x")
        .blacklist_function("__atan2l")
        .blacklist_function("__atan2l")
        .blacklist_function("__atan2l")
        .blacklist_function("__atanf64x")
        .blacklist_function("__atanf64x")
        .blacklist_function("__atanhf64x")
        .blacklist_function("__atanhf64x")
        .blacklist_function("__atanhl")
        .blacklist_function("__atanhl")
        .blacklist_function("__atanl")
        .blacklist_function("__atanl")
        .blacklist_function("__cbrtf64x")
        .blacklist_function("__cbrtf64x")
        .blacklist_function("__cbrtl")
        .blacklist_function("__cbrtl")
        .blacklist_function("__ceilf64x")
        .blacklist_function("__ceilf64x")
        .blacklist_function("__ceill")
        .blacklist_function("__ceill")
        .blacklist_function("__copysignf64x")
        .blacklist_function("__copysignf64x")
        .blacklist_function("__copysignf64x")
        .blacklist_function("__copysignl")
        .blacklist_function("__copysignl")
        .blacklist_function("__copysignl")
        .blacklist_function("__cosf64x")
        .blacklist_function("__cosf64x")
        .blacklist_function("__coshf64x")
        .blacklist_function("__coshf64x")
        .blacklist_function("__coshl")
        .blacklist_function("__coshl")
        .blacklist_function("__cosl")
        .blacklist_function("__cosl")
        .blacklist_function("__dreml")
        .blacklist_function("__dreml")
        .blacklist_function("__dreml")
        .blacklist_function("__erfcf64x")
        .blacklist_function("__erfcf64x")
        .blacklist_function("__erfcl")
        .blacklist_function("__erfcl")
        .blacklist_function("__erff64x")
        .blacklist_function("__erff64x")
        .blacklist_function("__erfl")
        .blacklist_function("__erfl")
        .blacklist_function("__exp10f64x")
        .blacklist_function("__exp10f64x")
        .blacklist_function("__exp10l")
        .blacklist_function("__exp10l")
        .blacklist_function("__exp2f64x")
        .blacklist_function("__exp2f64x")
        .blacklist_function("__exp2l")
        .blacklist_function("__exp2l")
        .blacklist_function("__expf64x")
        .blacklist_function("__expf64x")
        .blacklist_function("__expl")
        .blacklist_function("__expl")
        .blacklist_function("__expm1f64x")
        .blacklist_function("__expm1f64x")
        .blacklist_function("__expm1l")
        .blacklist_function("__expm1l")
        .blacklist_function("__fabsf64x")
        .blacklist_function("__fabsf64x")
        .blacklist_function("__fabsl")
        .blacklist_function("__fabsl")
        .blacklist_function("__fdimf64x")
        .blacklist_function("__fdimf64x")
        .blacklist_function("__fdimf64x")
        .blacklist_function("__fdiml")
        .blacklist_function("__fdiml")
        .blacklist_function("__fdiml")
        .blacklist_function("__finitel")
        .blacklist_function("__floorf64x")
        .blacklist_function("__floorf64x")
        .blacklist_function("__floorl")
        .blacklist_function("__floorl")
        .blacklist_function("__fmaf64x")
        .blacklist_function("__fmaf64x")
        .blacklist_function("__fmaf64x")
        .blacklist_function("__fmaf64x")
        .blacklist_function("__fmal")
        .blacklist_function("__fmal")
        .blacklist_function("__fmal")
        .blacklist_function("__fmal")
        .blacklist_function("__fmaxf64x")
        .blacklist_function("__fmaxf64x")
        .blacklist_function("__fmaxf64x")
        .blacklist_function("__fmaxl")
        .blacklist_function("__fmaxl")
        .blacklist_function("__fmaxl")
        .blacklist_function("__fmaxmagf64x")
        .blacklist_function("__fmaxmagf64x")
        .blacklist_function("__fmaxmagf64x")
        .blacklist_function("__fmaxmagl")
        .blacklist_function("__fminf64x")
        .blacklist_function("__fminf64x")
        .blacklist_function("__fminf64x")
        .blacklist_function("__fminl")
        .blacklist_function("__fminl")
        .blacklist_function("__fminl")
        .blacklist_function("__fminmagf64x")
        .blacklist_function("__fminmagf64x")
        .blacklist_function("__fminmagf64x")
        .blacklist_function("__fminmagl")
        .blacklist_function("__fmodf64x")
        .blacklist_function("__fmodf64x")
        .blacklist_function("__fmodf64x")
        .blacklist_function("__fmodl")
        .blacklist_function("__fmodl")
        .blacklist_function("__fmodl")
        .blacklist_function("__fpclassifyl")
        .blacklist_function("__frexpf64x")
        .blacklist_function("__frexpf64x")
        .blacklist_function("__frexpl")
        .blacklist_function("__frexpl")
        .blacklist_function("__fromfpf64x")
        .blacklist_function("__fromfpl")
        .blacklist_function("__fromfpxf64x")
        .blacklist_function("__fromfpxl")
        .blacklist_function("__gammal")
        .blacklist_function("__gammal")
        .blacklist_function("__getpayloadf64x")
        .blacklist_function("__getpayloadf64x")
        .blacklist_function("__getpayloadl")
        .blacklist_function("__hypotf64x")
        .blacklist_function("__hypotf64x")
        .blacklist_function("__hypotf64x")
        .blacklist_function("__hypotl")
        .blacklist_function("__hypotl")
        .blacklist_function("__hypotl")
        .blacklist_function("__ilogbf64x")
        .blacklist_function("__ilogbl")
        .blacklist_function("__iscanonicall")
        .blacklist_function("__iseqsigl")
        .blacklist_function("__iseqsigl")
        .blacklist_function("__isinfl")
        .blacklist_function("__isnanl")
        .blacklist_function("__issignalingl")
        .blacklist_function("__j0f64x")
        .blacklist_function("__j0f64x")
        .blacklist_function("__j0l")
        .blacklist_function("__j0l")
        .blacklist_function("__j1f64x")
        .blacklist_function("__j1f64x")
        .blacklist_function("__j1l")
        .blacklist_function("__j1l")
        .blacklist_function("__jnf64x")
        .blacklist_function("__jnf64x")
        .blacklist_function("__jnl")
        .blacklist_function("__jnl")
        .blacklist_function("__ldexpf64x")
        .blacklist_function("__ldexpf64x")
        .blacklist_function("__ldexpl")
        .blacklist_function("__ldexpl")
        .blacklist_function("__lgammaf64x")
        .blacklist_function("__lgammaf64x")
        .blacklist_function("__lgammaf64x_r")
        .blacklist_function("__lgammaf64x_r")
        .blacklist_function("__lgammal")
        .blacklist_function("__lgammal")
        .blacklist_function("__lgammal_r")
        .blacklist_function("__lgammal_r")
        .blacklist_function("__llogbf64x")
        .blacklist_function("__llogbl")
        .blacklist_function("__llrintf64x")
        .blacklist_function("__llrintl")
        .blacklist_function("__llroundf64x")
        .blacklist_function("__llroundl")
        .blacklist_function("__log10f64x")
        .blacklist_function("__log10f64x")
        .blacklist_function("__log10l")
        .blacklist_function("__log10l")
        .blacklist_function("__log1pf64x")
        .blacklist_function("__log1pf64x")
        .blacklist_function("__log1pl")
        .blacklist_function("__log1pl")
        .blacklist_function("__log2f64x")
        .blacklist_function("__log2f64x")
        .blacklist_function("__log2l")
        .blacklist_function("__log2l")
        .blacklist_function("__logbf64x")
        .blacklist_function("__logbf64x")
        .blacklist_function("__logbl")
        .blacklist_function("__logbl")
        .blacklist_function("__logf64x")
        .blacklist_function("__logf64x")
        .blacklist_function("__logl")
        .blacklist_function("__logl")
        .blacklist_function("__lrintf64x")
        .blacklist_function("__lrintl")
        .blacklist_function("__lroundf64x")
        .blacklist_function("__lroundl")
        .blacklist_function("__modff64x")
        .blacklist_function("__modff64x")
        .blacklist_function("__modff64x")
        .blacklist_function("__modfl")
        .blacklist_function("__modfl")
        .blacklist_function("__modfl")
        .blacklist_function("__nanf64x")
        .blacklist_function("__nanl")
        .blacklist_function("__nearbyintf64x")
        .blacklist_function("__nearbyintf64x")
        .blacklist_function("__nearbyintl")
        .blacklist_function("__nearbyintl")
        .blacklist_function("__nextafterf64x")
        .blacklist_function("__nextafterf64x")
        .blacklist_function("__nextafterf64x")
        .blacklist_function("__nextafterl")
        .blacklist_function("__nextafterl")
        .blacklist_function("__nextafterl")
        .blacklist_function("__nextdownf64x")
        .blacklist_function("__nextdownf64x")
        .blacklist_function("__nextdownl")
        .blacklist_function("__nextdownl")
        .blacklist_function("__nexttoward")
        .blacklist_function("__nexttowardf")
        .blacklist_function("__nexttowardl")
        .blacklist_function("__nexttowardl")
        .blacklist_function("__nexttowardl")
        .blacklist_function("__nextupf64x")
        .blacklist_function("__nextupf64x")
        .blacklist_function("__nextupl")
        .blacklist_function("__nextupl")
        .blacklist_function("__powf64x")
        .blacklist_function("__powf64x")
        .blacklist_function("__powf64x")
        .blacklist_function("__powl")
        .blacklist_function("__powl")
        .blacklist_function("__powl")
        .blacklist_function("__remainderf64x")
        .blacklist_function("__remainderf64x")
        .blacklist_function("__remainderf64x")
        .blacklist_function("__remainderl")
        .blacklist_function("__remainderl")
        .blacklist_function("__remainderl")
        .blacklist_function("__remquof64x")
        .blacklist_function("__remquol")
        .blacklist_function("__remquol")
        .blacklist_function("__remquol")
        .blacklist_function("__rintf64x")
        .blacklist_function("__rintf64x")
        .blacklist_function("__rintl")
        .blacklist_function("__rintl")
        .blacklist_function("__roundevenf64x")
        .blacklist_function("__roundevenf64x")
        .blacklist_function("__roundevenl")
        .blacklist_function("__roundevenl")
        .blacklist_function("__roundf64x")
        .blacklist_function("__roundf64x")
        .blacklist_function("__roundl")
        .blacklist_function("__roundl")
        .blacklist_function("__scalbl")
        .blacklist_function("__scalblnf64x")
        .blacklist_function("__scalblnf64x")
        .blacklist_function("__scalblnl")
        .blacklist_function("__scalblnl")
        .blacklist_function("__scalbnf64x")
        .blacklist_function("__scalbnf64x")
        .blacklist_function("__scalbnl")
        .blacklist_function("__scalbnl")
        .blacklist_function("__signbitl")
        .blacklist_function("__significandl")
        .blacklist_function("__significandl")
        .blacklist_function("__sincosf64x")
        .blacklist_function("__sincosf64x")
        .blacklist_function("__sincosf64x")
        .blacklist_function("__sincosl")
        .blacklist_function("__sincosl")
        .blacklist_function("__sincosl")
        .blacklist_function("__sinf64x")
        .blacklist_function("__sinf64x")
        .blacklist_function("__sinhf64x")
        .blacklist_function("__sinhf64x")
        .blacklist_function("__sinhl")
        .blacklist_function("__sinhl")
        .blacklist_function("__sinl")
        .blacklist_function("__sinl")
        .blacklist_function("__sqrtf64x")
        .blacklist_function("__sqrtf64x")
        .blacklist_function("__sqrtl")
        .blacklist_function("__sqrtl")
        .blacklist_function("__tanf64x")
        .blacklist_function("__tanf64x")
        .blacklist_function("__tanhf64x")
        .blacklist_function("__tanhf64x")
        .blacklist_function("__tanhl")
        .blacklist_function("__tanhl")
        .blacklist_function("__tanl")
        .blacklist_function("__tanl")
        .blacklist_function("__tgammaf64x")
        .blacklist_function("__tgammaf64x")
        .blacklist_function("__tgammal")
        .blacklist_function("__tgammal")
        .blacklist_function("__truncf64x")
        .blacklist_function("__truncf64x")
        .blacklist_function("__truncl")
        .blacklist_function("__truncl")
        .blacklist_function("__ufromfpf64x")
        .blacklist_function("__ufromfpl")
        .blacklist_function("__ufromfpxf64x")
        .blacklist_function("__ufromfpxl")
        .blacklist_function("__y0f64x")
        .blacklist_function("__y0f64x")
        .blacklist_function("__y0l")
        .blacklist_function("__y0l")
        .blacklist_function("__y1f64x")
        .blacklist_function("__y1f64x")
        .blacklist_function("__y1l")
        .blacklist_function("__y1l")
        .blacklist_function("__ynf64x")
        .blacklist_function("__ynf64x")
        .blacklist_function("__ynl")
        .blacklist_function("__ynl")
        .blacklist_function("acosf64x")
        .blacklist_function("acosf64x")
        .blacklist_function("acoshf64x")
        .blacklist_function("acoshf64x")
        .blacklist_function("acoshl")
        .blacklist_function("acoshl")
        .blacklist_function("acosl")
        .blacklist_function("acosl")
        .blacklist_function("asinf64x")
        .blacklist_function("asinf64x")
        .blacklist_function("asinhf64x")
        .blacklist_function("asinhf64x")
        .blacklist_function("asinhl")
        .blacklist_function("asinhl")
        .blacklist_function("asinl")
        .blacklist_function("asinl")
        .blacklist_function("atan2f64x")
        .blacklist_function("atan2f64x")
        .blacklist_function("atan2f64x")
        .blacklist_function("atan2l")
        .blacklist_function("atan2l")
        .blacklist_function("atan2l")
        .blacklist_function("atanf64x")
        .blacklist_function("atanf64x")
        .blacklist_function("atanhf64x")
        .blacklist_function("atanhf64x")
        .blacklist_function("atanhl")
        .blacklist_function("atanhl")
        .blacklist_function("atanl")
        .blacklist_function("atanl")
        .blacklist_function("canonicalizef64x")
        .blacklist_function("canonicalizef64x")
        .blacklist_function("canonicalizel")
        .blacklist_function("cbrtf64x")
        .blacklist_function("cbrtf64x")
        .blacklist_function("cbrtl")
        .blacklist_function("cbrtl")
        .blacklist_function("ceilf64x")
        .blacklist_function("ceilf64x")
        .blacklist_function("ceill")
        .blacklist_function("ceill")
        .blacklist_function("clock_adjtime")
        .blacklist_function("copysignf64x")
        .blacklist_function("copysignf64x")
        .blacklist_function("copysignf64x")
        .blacklist_function("copysignl")
        .blacklist_function("copysignl")
        .blacklist_function("copysignl")
        .blacklist_function("cosf64x")
        .blacklist_function("cosf64x")
        .blacklist_function("coshf64x")
        .blacklist_function("coshf64x")
        .blacklist_function("coshl")
        .blacklist_function("coshl")
        .blacklist_function("cosl")
        .blacklist_function("cosl")
        .blacklist_function("dreml")
        .blacklist_function("dreml")
        .blacklist_function("dreml")
        .blacklist_function("erfcf64x")
        .blacklist_function("erfcf64x")
        .blacklist_function("erfcl")
        .blacklist_function("erfcl")
        .blacklist_function("erff64x")
        .blacklist_function("erff64x")
        .blacklist_function("erfl")
        .blacklist_function("erfl")
        .blacklist_function("exp10f64x")
        .blacklist_function("exp10f64x")
        .blacklist_function("exp10l")
        .blacklist_function("exp10l")
        .blacklist_function("exp2f64x")
        .blacklist_function("exp2f64x")
        .blacklist_function("exp2l")
        .blacklist_function("exp2l")
        .blacklist_function("expf64x")
        .blacklist_function("expf64x")
        .blacklist_function("expl")
        .blacklist_function("expl")
        .blacklist_function("expm1f64x")
        .blacklist_function("expm1f64x")
        .blacklist_function("expm1l")
        .blacklist_function("expm1l")
        .blacklist_function("exttoward")
        .blacklist_function("fabsf64x")
        .blacklist_function("fabsf64x")
        .blacklist_function("fabsl")
        .blacklist_function("fabsl")
        .blacklist_function("fdimf64x")
        .blacklist_function("fdimf64x")
        .blacklist_function("fdimf64x")
        .blacklist_function("fdiml")
        .blacklist_function("fdiml")
        .blacklist_function("fdiml")
        .blacklist_function("finitel")
        .blacklist_function("floorf64x")
        .blacklist_function("floorf64x")
        .blacklist_function("floorl")
        .blacklist_function("floorl")
        .blacklist_function("fmaf64x")
        .blacklist_function("fmaf64x")
        .blacklist_function("fmaf64x")
        .blacklist_function("fmaf64x")
        .blacklist_function("fmal")
        .blacklist_function("fmal")
        .blacklist_function("fmal")
        .blacklist_function("fmal")
        .blacklist_function("fmaxf64x")
        .blacklist_function("fmaxf64x")
        .blacklist_function("fmaxf64x")
        .blacklist_function("fmaxl")
        .blacklist_function("fmaxl")
        .blacklist_function("fmaxl")
        .blacklist_function("fmaxmagf64x")
        .blacklist_function("fmaxmagf64x")
        .blacklist_function("fmaxmagf64x")
        .blacklist_function("fmaxmagl")
        .blacklist_function("fminf64x")
        .blacklist_function("fminf64x")
        .blacklist_function("fminf64x")
        .blacklist_function("fminl")
        .blacklist_function("fminl")
        .blacklist_function("fminl")
        .blacklist_function("fminmagf64x")
        .blacklist_function("fminmagf64x")
        .blacklist_function("fminmagf64x")
        .blacklist_function("fminmagl")
        .blacklist_function("fmodf64x")
        .blacklist_function("fmodf64x")
        .blacklist_function("fmodf64x")
        .blacklist_function("fmodl")
        .blacklist_function("fmodl")
        .blacklist_function("fmodl")
        .blacklist_function("frexpf64x")
        .blacklist_function("frexpf64x")
        .blacklist_function("frexpl")
        .blacklist_function("frexpl")
        .blacklist_function("fromfpf64x")
        .blacklist_function("fromfpl")
        .blacklist_function("fromfpxf64x")
        .blacklist_function("fromfpxl")
        .blacklist_function("gammal")
        .blacklist_function("gammal")
        .blacklist_function("getpayloadf64x")
        .blacklist_function("getpayloadf64x")
        .blacklist_function("getpayloadl")
        .blacklist_function("hypotf64x")
        .blacklist_function("hypotf64x")
        .blacklist_function("hypotf64x")
        .blacklist_function("hypotl")
        .blacklist_function("hypotl")
        .blacklist_function("hypotl")
        .blacklist_function("ilogbf64x")
        .blacklist_function("ilogbl")
        .blacklist_function("isinfl")
        .blacklist_function("isnanl")
        .blacklist_function("j0f64x")
        .blacklist_function("j0f64x")
        .blacklist_function("j0l")
        .blacklist_function("j0l")
        .blacklist_function("j1f64x")
        .blacklist_function("j1f64x")
        .blacklist_function("j1l")
        .blacklist_function("j1l")
        .blacklist_function("jnf64x")
        .blacklist_function("jnf64x")
        .blacklist_function("jnl")
        .blacklist_function("jnl")
        .blacklist_function("ldexpf64x")
        .blacklist_function("ldexpf64x")
        .blacklist_function("ldexpl")
        .blacklist_function("ldexpl")
        .blacklist_function("lgammaf64x")
        .blacklist_function("lgammaf64x")
        .blacklist_function("lgammaf64x_r")
        .blacklist_function("lgammaf64x_r")
        .blacklist_function("lgammal")
        .blacklist_function("lgammal")
        .blacklist_function("lgammal_r")
        .blacklist_function("lgammal_r")
        .blacklist_function("llogbf64x")
        .blacklist_function("llogbl")
        .blacklist_function("llrintf64x")
        .blacklist_function("llrintl")
        .blacklist_function("llroundf64x")
        .blacklist_function("llroundl")
        .blacklist_function("log10f64x")
        .blacklist_function("log10f64x")
        .blacklist_function("log10l")
        .blacklist_function("log10l")
        .blacklist_function("log1pf64x")
        .blacklist_function("log1pf64x")
        .blacklist_function("log1pl")
        .blacklist_function("log1pl")
        .blacklist_function("log2f64x")
        .blacklist_function("log2f64x")
        .blacklist_function("log2l")
        .blacklist_function("log2l")
        .blacklist_function("logbf64x")
        .blacklist_function("logbf64x")
        .blacklist_function("logbl")
        .blacklist_function("logbl")
        .blacklist_function("logf64x")
        .blacklist_function("logf64x")
        .blacklist_function("logl")
        .blacklist_function("logl")
        .blacklist_function("lrintf64x")
        .blacklist_function("lrintl")
        .blacklist_function("lroundf64x")
        .blacklist_function("lroundl")
        .blacklist_function("modff64x")
        .blacklist_function("modff64x")
        .blacklist_function("modff64x")
        .blacklist_function("modfl")
        .blacklist_function("modfl")
        .blacklist_function("modfl")
        .blacklist_function("nanf64x")
        .blacklist_function("nanl")
        .blacklist_function("nearbyintf64x")
        .blacklist_function("nearbyintf64x")
        .blacklist_function("nearbyintl")
        .blacklist_function("nearbyintl")
        .blacklist_function("nextafterf64x")
        .blacklist_function("nextafterf64x")
        .blacklist_function("nextafterf64x")
        .blacklist_function("nextafterl")
        .blacklist_function("nextafterl")
        .blacklist_function("nextafterl")
        .blacklist_function("nextdownf64x")
        .blacklist_function("nextdownf64x")
        .blacklist_function("nextdownl")
        .blacklist_function("nextdownl")
        .blacklist_function("nexttoward")
        .blacklist_function("nexttowardf")
        .blacklist_function("nexttowardl")
        .blacklist_function("nexttowardl")
        .blacklist_function("nexttowardl")
        .blacklist_function("nextupf64x")
        .blacklist_function("nextupf64x")
        .blacklist_function("nextupl")
        .blacklist_function("nextupl")
        .blacklist_function("powf64x")
        .blacklist_function("powf64x")
        .blacklist_function("powf64x")
        .blacklist_function("powl")
        .blacklist_function("powl")
        .blacklist_function("powl")
        .blacklist_function("qecvt")
        .blacklist_function("qecvt_r")
        .blacklist_function("qfcvt")
        .blacklist_function("qfcvt_r")
        .blacklist_function("qgcvt")
        .blacklist_function("remainderf64x")
        .blacklist_function("remainderf64x")
        .blacklist_function("remainderf64x")
        .blacklist_function("remainderl")
        .blacklist_function("remainderl")
        .blacklist_function("remainderl")
        .blacklist_function("remquof64x")
        .blacklist_function("remquol")
        .blacklist_function("remquol")
        .blacklist_function("remquol")
        .blacklist_function("rintf64x")
        .blacklist_function("rintf64x")
        .blacklist_function("rintl")
        .blacklist_function("rintl")
        .blacklist_function("roundevenf64x")
        .blacklist_function("roundevenf64x")
        .blacklist_function("roundevenl")
        .blacklist_function("roundevenl")
        .blacklist_function("roundf64x")
        .blacklist_function("roundf64x")
        .blacklist_function("roundl")
        .blacklist_function("roundl")
        .blacklist_function("scalbl")
        .blacklist_function("scalblnf64x")
        .blacklist_function("scalblnf64x")
        .blacklist_function("scalblnl")
        .blacklist_function("scalblnl")
        .blacklist_function("scalbnf64x")
        .blacklist_function("scalbnf64x")
        .blacklist_function("scalbnl")
        .blacklist_function("scalbnl")
        .blacklist_function("setpayloadf64x")
        .blacklist_function("setpayloadf64x")
        .blacklist_function("setpayloadl")
        .blacklist_function("setpayloadsigf64x")
        .blacklist_function("setpayloadsigf64x")
        .blacklist_function("setpayloadsigl")
        .blacklist_function("significandl")
        .blacklist_function("significandl")
        .blacklist_function("sincosf64x")
        .blacklist_function("sincosf64x")
        .blacklist_function("sincosf64x")
        .blacklist_function("sincosl")
        .blacklist_function("sincosl")
        .blacklist_function("sincosl")
        .blacklist_function("sinf64x")
        .blacklist_function("sinf64x")
        .blacklist_function("sinhf64x")
        .blacklist_function("sinhf64x")
        .blacklist_function("sinhl")
        .blacklist_function("sinhl")
        .blacklist_function("sinl")
        .blacklist_function("sinl")
        .blacklist_function("sqrtf64x")
        .blacklist_function("sqrtf64x")
        .blacklist_function("sqrtl")
        .blacklist_function("sqrtl")
        .blacklist_function("strfromf64x")
        .blacklist_function("strfroml")
        .blacklist_function("strtof64x")
        .blacklist_function("strtof64x_l")
        .blacklist_function("strtold")
        .blacklist_function("strtold_l")
        .blacklist_function("tanf64x")
        .blacklist_function("tanf64x")
        .blacklist_function("tanhf64x")
        .blacklist_function("tanhf64x")
        .blacklist_function("tanhl")
        .blacklist_function("tanhl")
        .blacklist_function("tanl")
        .blacklist_function("tanl")
        .blacklist_function("tgammaf64x")
        .blacklist_function("tgammaf64x")
        .blacklist_function("tgammal")
        .blacklist_function("tgammal")
        .blacklist_function("totalorderf64x")
        .blacklist_function("totalorderf64x")
        .blacklist_function("totalorderl")
        .blacklist_function("totalordermagf64x")
        .blacklist_function("totalordermagf64x")
        .blacklist_function("totalordermagl")
        .blacklist_function("truncf64x")
        .blacklist_function("truncf64x")
        .blacklist_function("truncl")
        .blacklist_function("truncl")
        .blacklist_function("ufromfpf64x")
        .blacklist_function("ufromfpl")
        .blacklist_function("ufromfpxf64x")
        .blacklist_function("ufromfpxl")
        .blacklist_function("y0f64x")
        .blacklist_function("y0f64x")
        .blacklist_function("y0l")
        .blacklist_function("y0l")
        .blacklist_function("y1f64x")
        .blacklist_function("y1f64x")
        .blacklist_function("y1l")
        .blacklist_function("y1l")
        .blacklist_function("ynf64x")
        .blacklist_function("ynf64x")
        .blacklist_function("ynl")
        .blacklist_function("ynl")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_INT_DOWNWARD")
        .blacklist_item("FP_INT_TONEAREST")
        .blacklist_item("FP_INT_TONEARESTFROMZERO")
        .blacklist_item("FP_INT_TOWARDZERO")
        .blacklist_item("FP_INT_UPWARD")
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_NORMAL")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_ZERO")
        .blacklist_type("_Float64x")
        .blacklist_type("timex")
        .generate()
        .expect("Unable to generate bindings");

    let generated_path = out_path.join("php_bindings.rs");
    bindings
        .write_to_file(&generated_path)
        .expect("Unable to write output file");
}

fn execute_command<S: AsRef<OsStr> + Debug>(argv: &[S]) -> String {
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]);
    let output = command
        .output()
        .expect(&format!("Execute command {:?} failed", &argv))
        .stdout;
    String::from_utf8(output).unwrap().trim().to_owned()
}
