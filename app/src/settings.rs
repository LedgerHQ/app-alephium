use ledger_device_sdk::nvm::{AtomicStorage, SingleStorage};
use ledger_device_sdk::NVMData;

// Keep the size consistent with the settings defined in the ledger sdk
pub const SETTINGS_SIZE: usize = 10;
#[link_section = ".nvm_data"]
pub static mut SETTINGS_DATA: NVMData<AtomicStorage<[u8; SETTINGS_SIZE]>> =
    NVMData::new(AtomicStorage::new(&[0u8; SETTINGS_SIZE]));

#[allow(static_mut_refs)]
pub fn is_blind_signing_enabled() -> bool {
    let settings = unsafe { SETTINGS_DATA.get_mut() };
    settings.get_ref()[0] != 0
}

#[allow(static_mut_refs)]
#[cfg(any(target_os = "nanosplus", target_os = "nanox"))]
pub fn toggle_blind_signing_setting() {
    let settings = unsafe { SETTINGS_DATA.get_mut() };
    let mut updated_data: [u8; SETTINGS_SIZE] = unsafe { *SETTINGS_DATA.get_mut().get_ref() };
    updated_data[0] = if settings.get_ref()[0] != 0 { 0 } else { 1 };
    unsafe { SETTINGS_DATA.get_mut().update(&updated_data) }
}
