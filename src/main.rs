use std::ffi::{CString, CStr};
use libc::{dlopen, dlinfo, dlerror, RTLD_NOW, RTLD_DI_LINKMAP, c_void};
use std::ptr::null_mut;
use std::mem::transmute;
use std::path::Path;

#[allow(dead_code)]
#[repr(C)]
struct LinkMap
{
    l_addr: *mut c_void,
    l_name: *mut i8,
    l_ld: *mut c_void,
    l_next: *mut LinkMap,
    l_prev: *mut LinkMap,
}

fn main() {
    let nvngx_lib = CString::new("libGLX_nvidia.so.0").expect("failed to create CString");
    let nvngx: *mut c_void = unsafe { dlopen(nvngx_lib.as_ptr(), RTLD_NOW) };

    if nvngx.is_null()
    {
        unsafe { panic!("dlopen failed {}", CStr::from_ptr(dlerror()).to_str().expect("failed to convert to str")) };
    }

    let mut info: *mut c_void = null_mut();
    let ret: i32 = unsafe { dlinfo(nvngx, RTLD_DI_LINKMAP, transmute(&mut info)) };

    if ret != 0
    {
        panic!("dlinfo failed with ret {:?}", ret);
    }

    let link_map: *mut LinkMap =  info as *mut LinkMap;

    let mut path = unsafe { Path::new(CStr::from_ptr((*link_map).l_name).to_str().expect("failed to convert to str")) };
    path = path.parent().expect("failed to get parent directory");

    println!("{}", path.to_str().expect("failed to unwrap path"));
}