/// Telegram Bot API.
pub mod bot;
/// Available methods
pub mod methods;
/// Available types
pub mod types;

#[cfg(all(feature = "non-wasm", feature = "wasm"))]
compile_error!("You cannot enable non-wasm and wasm feature at the same time.");

#[cfg(not(any(feature = "non-wasm", feature = "wasm")))]
compile_error!("You need enable non-wasm or wasm feature.");
