#![no_std]

use core::ptr;

use panic_never as _;
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{
        libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread},
        processthreadsapi::CreateThread,
        synchapi::Sleep,
        winnt::DLL_PROCESS_ATTACH,
    },
};

// Pick up _DllMainCRTStartup.
#[link(name = "msvcrt")]
extern {}

extern "system" fn on_attach(dll: LPVOID) -> DWORD {
    unsafe {
        Sleep(1000);
        FreeLibraryAndExitThread(dll.cast(), 0);
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            DisableThreadLibraryCalls(dll);
            CreateThread(ptr::null_mut(), 0, Some(on_attach), dll.cast(), 0, ptr::null_mut());
        }
    }

    TRUE
}
