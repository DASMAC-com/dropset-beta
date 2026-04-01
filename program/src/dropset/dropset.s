.globl entrypoint

# Included in order of dependency.
.include "common/discriminant.s"
.include "common/error.s"
.include "common/memory.s"
.include "common/pubkey.s"
.include "entrypoint.s"
.include "market/register.s"
.include "market/init_market_pda.s"
.include "market/init_vault.s"
