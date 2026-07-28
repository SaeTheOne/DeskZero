#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 单实例检查：使用 Windows 命名互斥体防止多开
    #[cfg(target_os = "windows")]
    {
        use std::ffi::c_void;
        type HANDLE = *mut c_void;
        type BOOL = i32;
        type DWORD = u32;
        type LPCSTR = *const u8;

        const ERROR_ALREADY_EXISTS: DWORD = 183;

        extern "system" {
            fn CreateMutexA(
                lpMutexAttributes: *const c_void,
                bInitialOwner: BOOL,
                lpName: LPCSTR,
            ) -> HANDLE;
            fn GetLastError() -> DWORD;
            fn CloseHandle(hObject: HANDLE) -> BOOL;
        }

        let name = b"DeskZero_SingleInstance\0";
        let handle = unsafe { CreateMutexA(std::ptr::null(), 0, name.as_ptr()) };
        if !handle.is_null() && unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
            eprintln!("[DeskZero] 检测到已有实例在运行，退出");
            unsafe {
                CloseHandle(handle);
            }
            return;
        }
    }

    deskzero_lib::run()
}
