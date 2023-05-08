use crate::prelude::*;

mod config;
mod error;
mod prelude;
mod utils;

const LUA_DEFAULT_CONFIG_PATH: &str = r"lua/init.lua";

fn main() -> Result<()> {
    let _path_to_config = config::get_lua_config(LUA_DEFAULT_CONFIG_PATH)?;

    return Ok(());
}
