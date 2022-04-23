# scylla-macros-flex

This crate defines two derive macros:
- FromRowFlex:
    - Any NULL value is converted to the default value (default for option is none, don't use this derive macro if one of the fields doesn't implement default).

- FromUserTypeFlex:
    - A field not defined in the struct will be ignore without throwing an error. Warning: this could lead to a dataloss if the udt is overwritten with this partial udt.
    - Any NULL value is converted to the default value (default for option is none, don't use this derive macro if one of the fields doesn't implement default). 