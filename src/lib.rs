use std::ffi::{CString, CStr};
use libc::{dlopen, dlinfo, dlerror, dlclose, RTLD_NOW, RTLD_DI_LINKMAP, c_void};
use std::ptr::null_mut;
use std::mem::transmute;
use std::path::Path;
use neon::prelude::*;

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

fn get_dlerror<'a>() -> &'a str 
{
    unsafe 
    {
        let err = dlerror();
        if err.is_null()
        {
            return "No Error";
        }
        return CStr::from_ptr(err).to_str().expect("failed to convert to str")
    }
}

fn get_nvidia_glx_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let nvngx_lib = CString::new("libGLX_nvidia.so.0").expect("failed to create CString");
    let nvngx = unsafe { dlopen(nvngx_lib.as_ptr(), RTLD_NOW) };

    if nvngx.is_null()
    {
        panic!("dlopen failed {}", get_dlerror());
    }

    let mut info: *mut LinkMap = null_mut();
    let ret = unsafe { dlinfo(nvngx, RTLD_DI_LINKMAP, transmute(&mut info)) };

    if ret != 0
    {
        panic!("dlinfo failed with ret {:?} {}", ret, get_dlerror());
    }

    let mut path = unsafe { Path::new(CStr::from_ptr((*info).l_name).to_str().expect("failed to convert to str")) };
    path = path.parent().expect("failed to get parent directory");

    unsafe { dlclose(nvngx); }

    // this returns the path as jsstring
    Ok(cx.string(path.to_str().expect("")))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getNvngxPath", get_nvidia_glx_path)?;
    Ok(())
}


