#![no_std]
#![no_main]

#[cfg(not(any(target_os = "stax", target_os = "flex")))]
use crate::ui::display::MainPages;
use crate::ui::tx_reviewer::TxReviewer;
use handler::{handle_apdu, Ins};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use include_gif::include_gif;
use ledger_device_sdk::io;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{init_comm, NbglGlyph, NbglHomeAndSettings};
#[cfg(any(target_os = "stax", target_os = "flex"))]
use settings::SETTINGS_DATA;
use sign_tx_context::SignTxContext;

mod blake2b_hasher;
mod debug;
mod error_code;
mod handler;
mod ledger_sdk_stub;
mod public_key;
mod settings;
mod sign_tx_context;
mod token_verifier;
mod ui;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

// This function is the app entry point
#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    // Initialize the sign tx context and tx reviewer
    let mut sign_tx_context: SignTxContext = SignTxContext::new();
    let mut tx_reviewer: TxReviewer = TxReviewer::new();

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        let mut main_pages = MainPages::new();
        loop {
            // Wait for either a specific button push to exit the app
            // or an APDU command
            if let io::Event::Command(ins) = main_pages.show(&mut comm) {
                match handle_apdu(&mut comm, ins, &mut sign_tx_context, &mut tx_reviewer) {
                    Ok(()) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }
                main_pages.show_ui();
            }
        }
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        init_comm(&mut comm);
        let settings_strings = [["Blind signing", "Enable blind signing"]];
        const APP_ICON: NbglGlyph = NbglGlyph::from_include(include_gif!("alph_64x64.gif", NBGL));
        loop {
            let event = NbglHomeAndSettings::new()
                .glyph(&APP_ICON)
                .settings(unsafe { SETTINGS_DATA.get_mut() }, &settings_strings)
                .infos(
                    "Alephium",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS"),
                )
                .show::<Ins>();
            if let io::Event::Command(ins) = event {
                match handle_apdu(&mut comm, ins, &mut sign_tx_context, &mut tx_reviewer) {
                    Ok(_) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }
            }
        }
    }
}
