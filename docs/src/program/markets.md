# Markets

Because the market account is at a fixed position in the
[input buffer], its data begins at a compile-time-known
offset from `r1`. The `MarketHeader` sits at the start of market account data
and stores absolute pointers into the market's memory map:

<Include rs="interface::market#market_header" collapsible/>

These absolute pointers are initialized during market registration, eliminating
input buffer pointer arithmetic overhead when accessing market data structures.

## Registration

Market registration accepts only the discriminant byte as instruction data:

<Include rs="interface::market#register_market_data" collapsible/>

The instruction requires the following accounts:

<Include rs="interface::market#register_market_accounts" collapsed/>

<Algorithm tex="REGISTER-MARKET" asm="market/register"/>

## Helpers

### Market PDA initialization

<Algorithm tex="INIT-MARKET-PDA" asm="market/init_market_pda"/>

### Vault initialization

<Algorithm tex="INIT-VAULT" asm="market/init_vault"/>

[input buffer]: inputs#input-buffer
