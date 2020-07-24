#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn lief_elf_example(){
        unsafe {
            let filename = CString::new("../hello").unwrap(); 
            let output = String::new();
            let input = include_str!("../hello.sections");
            let elf = elf_parse(filename.as_ptr());

            if elf.is_null() {
                panic!("unable to parse elf file.")
            }
            let _sections = (*elf).sections;

            elf_binary_destroy(elf);

            assert_eq!(input, output);
        }
    }
}
