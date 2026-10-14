use std::{env, fs, path::PathBuf};

fn main() {
    const ENV_VAR: &str = "EMBASSY_HA_ENTITY_LIMIT";

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={ENV_VAR}");

    let entity_limit = match env::var(ENV_VAR) {
        Ok(value) => value
            .parse::<usize>()
            .ok()
            .filter(|&value| value > 0)
            .unwrap_or_else(|| panic!("{ENV_VAR} must be a positive integer, got {value:?}")),
        Err(env::VarError::NotPresent) => 16,
        Err(error) => panic!("could not read {ENV_VAR}: {error}"),
    };

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set by Cargo"));
    fs::write(
        out_dir.join("config.rs"),
        format!(
            "/// Maximum number of entities per device (default: 16).\n\
             pub const ENTITY_LIMIT: usize = {entity_limit};\n"
        ),
    )
    .expect("failed to write build-time configuration");
}
