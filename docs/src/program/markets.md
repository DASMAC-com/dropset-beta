# Markets

Because the market account is at a fixed position in the
[input buffer], its data begins at a compile-time-known
offset from `r1`. The `MarketHeader` sits at the start of market account data
and stores absolute pointers into the market's memory map:

<Include rs="interface::market#market_header"/>

These absolute pointers are initialized during market registration, eliminating
input buffer pointer arithmetic overhead when accessing market data structures.

## Registration

Market registration accepts only the [`RegisterMarket`](inputs.md#discriminants)
discriminant byte as instruction data:

<Include rs="interface::market::register#register_market_data"/>

All relevant information is derived from the accounts:

<Include rs="interface::market::register#register_market_accounts"/>

The entrypoint dispatches to
[REGISTER-MARKET](#register-market), which validates the provided accounts,
derives and creates the market PDA, then initializes the base and quote
token vaults.

## REGISTER-MARKET

REGISTER-MARKET is the top-level orchestrator for market creation. It sequences
four procedures: [MARKET-PDA-PRELUDE](#market-pda-prelude) validates all
accounts, [INIT-MARKET-PDA](#init-market-pda) derives and creates the market
PDA, then [INIT-BASE-VAULT](#init-base-vault) and
[INIT-QUOTE-VAULT](#init-quote-vault) each initialize their respective token
vaults.

<AlgorithmIndex root="REGISTER-MARKET"/>

<Algorithm id="REGISTER-MARKET"/>

### MARKET-PDA-PRELUDE

Validates account counts, instruction data length, and account constraints
(no duplicates, no pre-existing data). Locates the [System Program] and [Rent]
sysvar, then computes shifted input buffer offsets for downstream account
lookups.

<Algorithm id="MARKET-PDA-PRELUDE"/>

### INIT-MARKET-PDA

Derives the market PDA from the base and quote mint addresses via
[`sol_try_find_program_address`], verifies the provided market account matches,
then delegates to [CREATE-MARKET-ACCOUNT](#create-market-account).

<AlgorithmIndex root="INIT-MARKET-PDA"/>

<Algorithm id="INIT-MARKET-PDA"/>

#### CREATE-MARKET-ACCOUNT

Issues a [`system_program::CreateAccount`] CPI with the derived PDA
signer seeds, then initializes the market header's `next` pointer and
stores the bump seed.

<Algorithm id="CREATE-MARKET-ACCOUNT"/>

### INIT-BASE-VAULT

Locates and validates the base token program account
([Token Program] or [Token 2022]), verifies it owns the base mint,
then delegates to [INIT-VAULT](#init-vault). Stores the derived base
vault bump in the market header.

<Algorithm id="INIT-BASE-VAULT"/>

### INIT-QUOTE-VAULT

Handles both distinct and duplicate quote token program accounts. When the quote
token program matches the base token program, the duplicate path verifies both
mints are owned by the same program. Delegates to
[INIT-VAULT](#init-vault) and stores the quote vault bump.

<Algorithm id="INIT-QUOTE-VAULT"/>

### INIT-VAULT

Derives the vault PDA from the market address and a vault index (base or quote),
verifies the provided vault account matches, then calls
[GET-VAULT-SIZE](#get-vault-size),
[CREATE-VAULT-ACCOUNT](#create-vault-account), and
[INIT-VAULT-TOKEN-ACCOUNT](#init-vault-token-account) in sequence.

<AlgorithmIndex root="INIT-VAULT"/>

<Algorithm id="INIT-VAULT"/>

#### GET-VAULT-SIZE

For Token 2022, invokes [`spl_token_2022::GetAccountDataSize`] and
reads the return data via [`sol_get_return_data`]. For the standard
Token Program, uses the fixed `ACCOUNT_SIZE` constant.

<Algorithm id="GET-VAULT-SIZE"/>

#### CREATE-VAULT-ACCOUNT

Issues a [`system_program::CreateAccount`] CPI with the vault PDA signer seeds,
setting the token program as the owner.

<Algorithm id="CREATE-VAULT-ACCOUNT"/>

#### INIT-VAULT-TOKEN-ACCOUNT

Invokes [`spl_token::InitializeAccount2`] to initialize the vault as a token
account for the given mint, with the market as the account owner.

<Algorithm id="INIT-VAULT-TOKEN-ACCOUNT"/>

[input buffer]: inputs#input-buffer
[System Program]: https://solana.com/docs/core/programs/builtin-programs#the-system-program
[`sol_try_find_program_address`]: https://github.com/anza-xyz/agave/blob/v4.0.0-beta.5/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L74-L83
[`system_program::CreateAccount`]: https://github.com/anza-xyz/solana-sdk/blob/sdk@v4.0.1/system-interface/src/instruction.rs#L93-L107
[`spl_token_2022::GetAccountDataSize`]: https://github.com/solana-program/token-2022/blob/interface@v2.1.0/interface/src/instruction.rs#L487-L499
[`sol_get_return_data`]: https://github.com/anza-xyz/agave/blob/v4.0.0-beta.5/platform-tools-sdk/sbf/c/inc/sol/inc/return_data.inc#L27-L35
[`spl_token::InitializeAccount2`]: https://github.com/solana-program/token/blob/program@v9.0.0/interface/src/instruction.rs#L352-L365
[Rent]: https://docs.rs/pinocchio/0.11.0/pinocchio/sysvars/rent/struct.Rent.html
[Token Program]: https://github.com/solana-program/token
[Token 2022]: https://github.com/solana-program/token-2022
