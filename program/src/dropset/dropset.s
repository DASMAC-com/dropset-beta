.globl entrypoint

# Included in order of dependency.
.include "error.s"
.include "svm/account.s"
.include "svm/memory.s"
.include "svm/pubkey.s"
.include "svm/token.s"
.include "entrypoint.s"
.include "market/register.s"
.include "market/init_market_pda.s"
.include "market/init_vault.s"
