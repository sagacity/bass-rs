use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
struct BASS_DeviceInfo {
    name: *const c_char,
    driver: *const c_char,
    flags: u32,
}

extern {
    pub fn BASS_Init(device: i32, freq: u32, flags: u32, win: libc::uintptr_t, dsguid: libc::uintptr_t) -> i32;
    pub fn BASS_Free() -> i32;
    pub fn BASS_StreamCreateFile(mem: i32, file: *const u8, offset: u64, length: u64, flags: u32) -> u32;
    pub fn BASS_StreamFree(handle: u32) -> i32;
    pub fn BASS_ChannelPlay(handle: u32, restart: i32) -> i32;
    pub fn BASS_ChannelGetPosition(handle: u32, mode: u32) -> u64;
    pub fn BASS_ChannelSetPosition(handle: u32, pos: u64, mode: u32) -> i32;
    pub fn BASS_ChannelSeconds2Bytes(handle: u32, pos: f64) -> u64;
    pub fn BASS_ChannelBytes2Seconds(handle: u32, pos: u64) -> f64;

    fn BASS_GetDeviceInfo(device: u32, info: *mut BASS_DeviceInfo) -> i32;
}

#[derive(Debug)]
pub struct DeviceInfo {
    name: String,
    driver: String
}

pub fn get_device_info() -> Vec<DeviceInfo> {
    use std::mem::MaybeUninit;

    let mut devices = Vec::new();
    let mut device_index = 1;
    loop {
        let mut info: BASS_DeviceInfo = unsafe { MaybeUninit::zeroed().assume_init() };
        let result = unsafe {BASS_GetDeviceInfo(device_index, &mut info) };
        if result == 0 {
            break;
        }

        unsafe {
            let di = DeviceInfo {
                name: CStr::from_ptr(info.name).to_str().unwrap().to_owned(),
                driver: CStr::from_ptr(info.driver).to_str().unwrap().to_owned()
            };
            devices.push(di);
        }

        device_index = device_index + 1;
    }

    devices
}
