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

<Include rs="interface::market::register#register_market_data" collapsible/>

The instruction requires the following accounts:

<Include rs="interface::market::register#register_market_accounts" collapsed/>

## Market registration algorithms

<AlgorithmIndex root="REGISTER-MARKET"/>

### REGISTER-MARKET

<Algorithm id="REGISTER-MARKET"/>

### MARKET-PDA-PRELUDE

<Algorithm id="MARKET-PDA-PRELUDE"/>

### INIT-MARKET-PDA

<Algorithm id="INIT-MARKET-PDA"/>

### INIT-BASE-VAULT

<Algorithm id="INIT-BASE-VAULT"/>

### INIT-QUOTE-VAULT

<Algorithm id="INIT-QUOTE-VAULT"/>

### INIT-VAULT

<Algorithm id="INIT-VAULT"/>

[input buffer]: inputs#input-buffer
