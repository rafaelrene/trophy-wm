mod structs;

use crate::config::structs::*;
use crate::prelude::*;

use rlua::{Lua, Table};

use std::fs::read_to_string;

pub fn get_lua_config(path: &str) -> Result<&str> {
    let lua_config_as_string = read_to_string(path)?;
    println!("{:?}", lua_config_as_string);

    let lua_config_module = LuaConfigModule {
        config: LuaConfig {},
    };

    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let lua_config_as_str = lua_config_as_string.as_str();

        let config_module: Table = lua_ctx
            .load(lua_config_as_str)
            .set_name("lua config module")?
            .eval()?;

        let config: Table = config_module.get("config")?;
        let config_color: String = config.get("color")?;

        println!("Config module: {:?}", config_color);

        return Ok::<(), Error>(());
    })?;

    return Ok(path);
}
