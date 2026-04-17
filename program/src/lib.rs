#![cfg_attr(target_os = "solana", no_std)]
#![cfg_attr(target_os = "solana", no_main)]
#![cfg_attr(target_os = "solana", feature(asm_experimental_arch))]

#[cfg(target_os = "solana")]
core::arch::global_asm!(include_str!(concat!(env!("OUT_DIR"), "/combined.s")));

#[cfg(target_os = "solana")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }
