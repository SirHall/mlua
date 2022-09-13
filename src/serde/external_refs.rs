use crate::{lua::ExtraData, types::Callback, Lua};
use linkme::distributed_slice;
use serde::{Deserialize, Serialize};

#[distributed_slice]
pub static LUA_FUNCTIONS: [(&'static str, Callback)] = [..];

// #[distributed_slice]
// pub static LUA_LIGHT_USER_DATA_TYPES: [(&'static str,)] = [..];

#[distributed_slice]
pub static LUA_USER_DATA_TYPES: [(&'static str, &'dyn UserData)] = [..];
