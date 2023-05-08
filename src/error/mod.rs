#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LuaError(#[from] rlua::prelude::LuaError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
