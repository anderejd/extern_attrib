#![feature(proc_macro)]
extern crate extern_attrib;

#[cfg(test)]
mod tests {
    use extern_attrib::extern_auto;

    #[test]
    fn extern_auto_sets_the_target_os_ffi_abi() {
        let i = 1234;

        /// This function should be modified by the extern_auto
        /// macro, to the appropriate FFI ABI for the target_os.
        #[extern_auto]
        #[cfg(not(extra_attrib_to_make_things_harder = "for the procedural macro"))]
        fn modified_by_attrib(i: i32) -> i32 {
            i * i
        }

        /// This should cause build failure if extern_auto is incorrect.
        #[cfg(target_os = "windows")]
        let f: extern "stdcall" fn(i: i32) -> i32 = modified_by_attrib;

        /// This should cause build failure if extern_auto is incorrect.
        #[cfg(not(target_os = "windows"))]
        let f: extern "C" fn(i: i32) -> i32 = modified_by_attrib;

        assert!(f(i) == i * i)
    }
}
