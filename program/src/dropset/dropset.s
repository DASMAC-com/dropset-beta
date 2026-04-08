.globl entrypoint

# Included in order of dependency.
.include "error.s"
.include "common/account.s"
.include "common/memory.s"
.include "common/pubkey.s"
.include "common/token.s"
.include "entrypoint.s"
.include "market/market.s"
.include "market/register.s"
.include "market/market_pda_prelude.s"
.include "market/init_market_pda.s"
.include "market/init_vault.s"
