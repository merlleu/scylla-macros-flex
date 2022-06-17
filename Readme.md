# scylla-macros-flex

This crate defines two derive macros:
- FromRowFlex:
    - Any NULL value is converted to the default value (default for option is none, don't use this derive macro if one of the fields doesn't implement default).

- FromUserTypeFlex:
    - A field not defined in the struct will be ignore without throwing an error. Warning: this could lead to a dataloss if the udt is overwritten with this partial udt.
    - Any NULL value is converted to the default value (default for option is none, don't use this derive macro if one of the fields doesn't implement default). 

## serde_json support: 
Allows you to use arbitrary structs serialized with serde_json, stored as text in Scylla/Cassandra. 

They have approximately the same behavior as UDT.

Requires "serde_json" feature.
### Derive Macros: 
    - FromJson
    - IntoJson
### Pros:
    - Way more flexible than CQL **FROZEN** UDTs thanks to Rust enums
    - Schema can be changed a lot without any issue.
### Cons:
    - Can take more space than UDT
    - Can be slower than UDTs

## rmp-serde support: 
Allows you to use arbitrary structs serialized with MessagePack, stored as Blobs in Scylla/Cassandra. 

They have approximately the same behavior as UDT.

Requires "rmp-serde" feature.
### Derive Macros: 
    - FromMessagePack
    - IntoMessagePack
### Pros:
    - Way more flexible than CQL **FROZEN** UDTs thanks to Rust enums
    - Schema can be changed a lot without any issue.
    - Most of the time uses less space than UDT
### Cons:
    - Can be slower than UDTs


## speedy support: 
Allows you to use arbitrary structs serialized with speedy, stored as Blobs in Scylla/Cassandra. 

Use this only if you won't change your schema later.
They have approximately the same behavior as UDT.

Requires "speedy" feature.
### Derive Macros: 
    - FromSpeedy
    - IntoSpeedy
### Pros:
    - Way more flexible than CQL **FROZEN** UDTs thanks to Rust enums
    - Less storage required than UDTs because udt include fields names & udt names
    - Faster than UDT (because speedy is quite fast)
### Cons:
    - You can't update a field in an atomic manner (like frozen UDTs)
    - Changing your schema (eg. adding a field to a struct inside a Vec) will break everything.
