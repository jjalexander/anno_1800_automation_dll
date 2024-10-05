use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

unsafe extern "system" fn injection(_lp_parameter: *mut c_void) -> u32 {
    1
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            CloseHandle(CreateThread(
                null_mut(),
                0,
                Some(injection),
                null_mut(),
                0,
                null_mut(),
            ));
        },
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    }
    1
}

#[ctor::ctor]
fn ctor() {}
