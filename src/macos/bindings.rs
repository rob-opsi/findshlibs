// automatically generated by rust-bindgen

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub const MH_MAGIC: ::std::os::raw::c_uint = 4277009102;
pub const MH_MAGIC_64: ::std::os::raw::c_uint = 4277009103;
pub const LC_SEGMENT: ::std::os::raw::c_uint = 1;
pub const LC_SEGMENT_64: ::std::os::raw::c_uint = 25;
pub const LC_SEGMENT_SPLIT_INFO: ::std::os::raw::c_uint = 30;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type integer_t = ::std::os::raw::c_int;
pub type cpu_type_t = integer_t;
pub type cpu_subtype_t = integer_t;
pub type vm_prot_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct mach_header {
    pub magic: u32,
    pub cputype: cpu_type_t,
    pub cpusubtype: cpu_subtype_t,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
}
#[test]
fn bindgen_test_layout_mach_header() {
    assert_eq!(::std::mem::size_of::<mach_header>(), 28usize);
    assert_eq!(::std::mem::align_of::<mach_header>(), 4usize);
}
impl Clone for mach_header {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct mach_header_64 {
    pub magic: u32,
    pub cputype: cpu_type_t,
    pub cpusubtype: cpu_subtype_t,
    pub filetype: u32,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: u32,
    pub reserved: u32,
}
#[test]
fn bindgen_test_layout_mach_header_64() {
    assert_eq!(::std::mem::size_of::<mach_header_64>(), 32usize);
    assert_eq!(::std::mem::align_of::<mach_header_64>(), 4usize);
}
impl Clone for mach_header_64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct load_command {
    pub cmd: u32,
    pub cmdsize: u32,
}
#[test]
fn bindgen_test_layout_load_command() {
    assert_eq!(::std::mem::size_of::<load_command>(), 8usize);
    assert_eq!(::std::mem::align_of::<load_command>(), 4usize);
}
impl Clone for load_command {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct segment_command {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [::std::os::raw::c_char; 16usize],
    pub vmaddr: u32,
    pub vmsize: u32,
    pub fileoff: u32,
    pub filesize: u32,
    pub maxprot: vm_prot_t,
    pub initprot: vm_prot_t,
    pub nsects: u32,
    pub flags: u32,
}
#[test]
fn bindgen_test_layout_segment_command() {
    assert_eq!(::std::mem::size_of::<segment_command>(), 56usize);
    assert_eq!(::std::mem::align_of::<segment_command>(), 4usize);
}
impl Clone for segment_command {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct segment_command_64 {
    pub cmd: u32,
    pub cmdsize: u32,
    pub segname: [::std::os::raw::c_char; 16usize],
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: vm_prot_t,
    pub initprot: vm_prot_t,
    pub nsects: u32,
    pub flags: u32,
}
#[test]
fn bindgen_test_layout_segment_command_64() {
    assert_eq!(::std::mem::size_of::<segment_command_64>(), 72usize);
    assert_eq!(::std::mem::align_of::<segment_command_64>(), 8usize);
}
impl Clone for segment_command_64 {
    fn clone(&self) -> Self {
        *self
    }
}
extern "C" {
    pub fn _dyld_image_count() -> u32;
}
extern "C" {
    pub fn _dyld_get_image_header(image_index: u32) -> *const mach_header;
}
extern "C" {
    pub fn _dyld_get_image_vmaddr_slide(image_index: u32) -> isize;
}
extern "C" {
    pub fn _dyld_get_image_name(image_index: u32) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn _dyld_register_func_for_add_image(func:
                                                 ::std::option::Option<unsafe extern "C" fn(mh:
                                                                                                *const mach_header,
                                                                                            vmaddr_slide:
                                                                                                isize)>);
}
extern "C" {
    pub fn _dyld_register_func_for_remove_image(func:
                                                    ::std::option::Option<unsafe extern "C" fn(mh:
                                                                                                   *const mach_header,
                                                                                               vmaddr_slide:
                                                                                                   isize)>);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __NSModule {
    pub _address: u8,
}
impl Clone for __NSModule {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NSModule = *mut __NSModule;
extern "C" {
    pub fn _dyld_present() -> bool;
}
extern "C" {
    pub fn _dyld_launched_prebound() -> bool;
}
extern "C" {
    pub fn _dyld_all_twolevel_modules_prebound() -> bool;
}
extern "C" {
    pub fn _dyld_bind_objc_module(objc_module: *const ::std::os::raw::c_void);
}
extern "C" {
    pub fn _dyld_bind_fully_image_containing_address(address: *const ::std::os::raw::c_void)
                                                     -> bool;
}
extern "C" {
    pub fn _dyld_image_containing_address(address: *const ::std::os::raw::c_void) -> bool;
}
extern "C" {
    pub fn _dyld_lookup_and_bind(symbol_name: *const ::std::os::raw::c_char,
                                 address: *mut *mut ::std::os::raw::c_void,
                                 module: *mut NSModule);
}
extern "C" {
    pub fn _dyld_lookup_and_bind_with_hint(symbol_name: *const ::std::os::raw::c_char,
                                           library_name_hint: *const ::std::os::raw::c_char,
                                           address: *mut *mut ::std::os::raw::c_void,
                                           module: *mut NSModule);
}
extern "C" {
    pub fn _dyld_lookup_and_bind_fully(symbol_name: *const ::std::os::raw::c_char,
                                       address: *mut *mut ::std::os::raw::c_void,
                                       module: *mut NSModule);
}
extern "C" {
    pub fn _dyld_get_image_header_containing_address(address: *const ::std::os::raw::c_void)
                                                     -> *const mach_header;
}
