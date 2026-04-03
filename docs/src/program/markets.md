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

## Algorithms

<AlgorithmIndex root="REGISTER-MARKET"/>

### Dispatcher

<Algorithm id="REGISTER-MARKET"/>

### Market PDA initialization

<Algorithm id="INIT-MARKET-PDA"/>

### Vault initialization

<Algorithm id="INIT-VAULT"/>

[input buffer]: inputs#input-buffer
