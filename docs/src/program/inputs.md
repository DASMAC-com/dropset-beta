# Inputs

The entrypoint reads instruction data via the [SIMD-0321] `r2` pointer, which
provides the instruction data address directly in a register. It dispatches on
the instruction discriminant to the appropriate handler.

## Entrypoint

<Algorithm id="ENTRYPOINT"/>

## Discriminants

Each instruction is identified by a `u8` discriminant, the first byte of the
instruction data buffer. The `Discriminant` enum defines the mapping:

<Include rs="interface::entrypoint#discriminant_enum"/>

## Input buffer

The Solana runtime serializes accounts and instruction data into a contiguous
input buffer pointed to by `r1`. Dropset enables direct pointer addressing by
requiring the user (signer) account first with no data, followed by the market
account.

Through this design, all user and market account metadata can be accessed
at compile-time-known offsets from `r1`, and more importantly, market account
data offsets are persisted across transactions. The header also overlays a
`MarketHeader` directly after the `RuntimeAccount` so that market data fields
(pointers, bump seeds) are addressable at static offsets. A trailing
`market_data_bytes` field marks the first allocatable byte after the header,
giving `MarketHeader.next` its initialization target. The `InputBufferHeader`
layout captures this design:

<Include rs="interface::entrypoint#input_buffer_header"/>

Account data regions in the Solana input buffer are variable-length and would
normally prevent static offset computation. `FullRuntimeAccount` solves this by
extending [`RuntimeAccount`] with a compile-time-sized data buffer and trailing
rent epoch:

<Include rs="interface::common::account#full_runtime_account"/>

The type alias `EmptyAccount` specializes `FullRuntimeAccount` with zero-length
data, which is the common case for accounts whose data is not statically known.
Because the user data length is zero, `EmptyAccount` has a fixed size and the
market account begins at a deterministic offset. This makes every account
metadata field in the header addressable as an immediate offset from `r1`.

[SIMD-0321]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md
[`RuntimeAccount`]: https://docs.rs/solana-account-view/2.0.0/solana_account_view/struct.RuntimeAccount.html
