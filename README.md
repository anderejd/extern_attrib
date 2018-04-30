This is an archived experiment, please don't use it.
====================================================



# extern_attrib
Rust automatic ABI attribute macro: #[extern_auto]

The purpose of this library is to replace or insert the ABI name at compile time for functions.
This is useful for hiding the calling convention for FFI callbacks while avoiding code duplication.

- __Requires nightly rust__, depends on #[proc_macro_attribute].
- Currenly only handles target_os = "windows" / "macos" / "linux".
- See the tests/tests.rs for a usage example.
- Pull Requests Welcome!
