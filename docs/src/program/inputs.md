# Inputs

The entrypoint reads instruction data via the [SIMD-0321] `r2` pointer, which
provides the instruction data address directly in a register. It dispatches on
the instruction discriminant to the appropriate handler.

## Entrypoint

<Algorithm tex="ENTRYPOINT" asm="entrypoint"/>

## Algorithm conventions

- `procedure`: a label that does not return (no stack push); control flow
  exits via `exit` or jumps to another procedure.
- `function`: a label that pushes onto the call stack and returns to the
  caller.

## Input buffer

The Solana runtime serializes accounts and instruction data into a contiguous
input buffer pointed to by `r1`. Dropset enables direct pointer addressing by
requiring the user (signer) account first with no data, followed by the market
account.

Through this design, all user and market account metadata can be accessed
at compile-time-known offsets from `r1`, and more importantly, market account
data offsets are persisted across transactions. The `InputBufferHeader` layout
captures this design:

<Include rs="interface::memory#input_buffer_header" collapsible/>

Account data regions in the Solana input buffer are variable-length and would
normally prevent static offset computation. `FullRuntimeAccount` solves this by
extending [`RuntimeAccount`] with a compile-time-sized data buffer and trailing
rent epoch:

<Include rs="interface::memory#full_runtime_account" collapsible/>

Because the user data length is zero, `FullRuntimeAccount` has a fixed size and
the market account begins at a deterministic offset. This makes every account
metadata field in the header addressable as an immediate offset from `r1`.

[SIMD-0321]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0321-vm-r2-instruction-data-pointer.md
[`RuntimeAccount`]: https://docs.rs/solana-account-view/solana_account_view/struct.RuntimeAccount.html
