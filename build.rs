use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

type Error = Box<dyn std::error::Error>;

static BINARY_CFGS: &[&'static str] = &[
    // TODO
];

fn cfgs_to_defines() -> HashMap<String, String> {
    let cfgs = BINARY_CFGS.iter();

    let cargo_env_vars = cfgs
        .clone()
        .map(|cfg| format!("CARGO_CFG_{}", cfg.to_uppercase()));
    let defines = cfgs.map(|cfg| cfg.to_uppercase());
    cargo_env_vars.zip(defines).collect()
}

fn set_cfg_defines(bld: &mut cc::Build) {
    let cfgs_to_defines = cfgs_to_defines();
    for (cfg, _) in env::vars() {
        if let Some(define) = cfgs_to_defines.get(&cfg) {
            bld.define(define, None);
        }
    }
}

fn main() -> Result<(), Error> {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo::warning=Hello docs.rs! Skipping FileX (cross) compile");
        return Ok(());
    }

    let port = env::var("DEP_THREADX_PORT")?;
    let threadx_common_inc = env::var("DEP_THREADX_COMMON_INCLUDE")?;
    let threadx_port_inc = env::var("DEP_THREADX_PORT_INCLUDE")?;

    let mut bld = cc::Build::new();
    set_cfg_defines(&mut bld);
    bld.include(threadx_common_inc);
    bld.include(threadx_port_inc);

    bld.include("filex/common/inc");
    for src in fs::read_dir("filex/common/src")? {
        bld.file(src?.path());
    }

    // The Cortex M ports are simply including the generic header. It's easier
    // to point include paths there, rather than point include paths to the
    // cortex_mX path. We'd still need a way to set up the generic include
    // path if we _didn't_ do this.
    let port_inc = if port.starts_with("cortex") {
        let port_inc = Path::new("filex/ports/generic/inc");
        bld.include(port_inc);
        port_inc
    } else {
        panic!("Unsupported FileX port {port}");
    };

    let out = PathBuf::from(env::var("OUT_DIR")?);
    let common = out.join("common");
    fs::create_dir_all(&common)?;

    for inc in fs::read_dir("filex/common/inc")? {
        let from = inc?.path();
        let to = common.join(from.file_name().unwrap());
        fs::copy(&from, &to)?;
    }

    let ports = out.join("ports");
    fs::create_dir_all(&ports)?;

    fs::copy(port_inc.join("fx_port.h"), ports.join("fx_port.h"))?;

    println!("cargo::metadata=common_include={}", common.display());
    println!("cargo::metadata=port_include={}", ports.display());

    bld.compile("filex");
    Ok(())
}
