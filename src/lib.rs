use solana_program::declare_id;

pub mod consts;
pub mod instructions;
pub mod state;

#[cfg(feature = "entrypoint")]
pub mod entrypoint;
#[cfg(feature = "entrypoint")]
mod processors;

declare_id!("91awjYRxwaJNdSXPsNijTDRAj11HSWRnBH9EH4d8JiYx");

#[cfg(all(feature = "entrypoint", feature = "security-txt"))]
security_txt::security_txt! {
    name: "MagicBlock Domain Registration Program",
    project_url: "https://magicblock.gg",
    contacts: "email:dev@magicblock.gg,twitter:@magicblock",
    policy: "https://github.com/magicblock-labs/magic-domain-program/blob/master/LICENSE.md",
    preferred_languages: "en",
    source_code: "https://github.com/magicblock-labs/magic-domain-program"
}
