use serde::{Deserialize, Serialize};

use crate::{lua::ExtraData, Lua};

// | For serialization:
// + Each function must have a unique id that is specific to it
// + Each (light) user data need to have a unique id for Lua to serialize it, and store a type-reference name

// This aims to serialize the Lua state into a binary stream, rather than passing values to/from the Lua instance

struct LuaStateSerde<'lua> {
    lua: &'lua Lua,
}

impl Serialize for Lua {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 1. Generate the function/object reference map

        // 2. Serialize all other Lua member fields

        // 3. Write table of all serialized subfields

        todo!()
    }
}

pub fn persist_lua_instance() -> Option<Vec<u8>> {
    None
}

impl Serialize for ExtraData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
