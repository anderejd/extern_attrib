# extern_attrib
Rust automatic ABI attribute macro: #[extern_auto]

Currenly only handles target_os = "windows" / "macos" / "linux".
The purpose of this library is to replace or insert the ABI name at compile time for functions.
This is useful for hiding the calling convention for FFI callbacks while avoiding code duplication.

Please see the tests/tests.rs for a usage example.
Pull Requests Welcome!
