# Pubkeys

Pubkey operations in SBPF work on 32-byte addresses split into four 8-byte
chunks. The [`interface`][interface] crate's [`pubkey`][pubkey-mod] module
defines chunk offsets and known address immediates that are injected into
[`common/pubkey.s`][layout-pubkey].

## Chunk offsets

Each 32-byte pubkey is accessed as four 8-byte (`u64`) chunks at offsets 0, 8,
16, and 24. These are emitted as `PUBKEY_CHUNK_{0..3}_OFF` immediates.

## Known addresses

Known addresses (such as the rent sysvar ID) are split into `_CHUNK_{0..3}_LO`
and `_CHUNK_{0..3}_HI` `i32` immediates using [`address!`][bs-address] so they
can be loaded with `mov32` / `lsh64` pairs without runtime memory access.

## Pubkey offset constants

When a struct field holds a 32-byte pubkey that needs per-chunk access,
[`pubkey_offsets!`][bs-pubkey-offsets] generates a base `_OFF` plus four
`_CHUNK_{0..3}_OFF` constants. This is used for input buffer fields
(e.g. `IB_MARKET_PUBKEY_CHUNK_{0..3}_OFF`) and frame-relative fields
(e.g. `RM_FM_PDA_CHUNK_{0..3}_OFF`).

<Include rs="interface::pubkey#pubkey_constants" collapsible/>

[interface]: https://github.com/DASMAC-com/dropset-beta/tree/main/interface
[pubkey-mod]: https://github.com/DASMAC-com/dropset-beta/blob/main/interface/src/pubkey.rs
[layout-pubkey]: layout#pubkeys
[bs-address]: ../development/build-scaffolding#constant_group
[bs-pubkey-offsets]: ../development/build-scaffolding#constant_group
