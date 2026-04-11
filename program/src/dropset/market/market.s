# Market-level constants. Assumes user has no data for static addressing.
# -------------------------------------------------------------------------
# Vault index for base token in PDA derivation and vault creation.
.equ MKT_VAULT_INDEX_BASE, 0
# Vault index for quote token in PDA derivation and vault creation.
.equ MKT_VAULT_INDEX_QUOTE, 1
.equ MKT_NEXT_OFF, 10464 # From input buffer to MarketHeader.next.
# From input buffer to MarketHeader.base_mint.
.equ MKT_BASE_MINT_OFF, 10472
# From input buffer to MarketHeader.base_mint (chunk 0).
.equ MKT_BASE_MINT_CHUNK_0_OFF, 10472
# From input buffer to MarketHeader.base_mint (chunk 1).
.equ MKT_BASE_MINT_CHUNK_1_OFF, 10480
# From input buffer to MarketHeader.base_mint (chunk 2).
.equ MKT_BASE_MINT_CHUNK_2_OFF, 10488
# From input buffer to MarketHeader.base_mint (chunk 3).
.equ MKT_BASE_MINT_CHUNK_3_OFF, 10496
# From input buffer to MarketHeader.quote_mint.
.equ MKT_QUOTE_MINT_OFF, 10504
# From input buffer to MarketHeader.quote_mint (chunk 0).
.equ MKT_QUOTE_MINT_CHUNK_0_OFF, 10504
# From input buffer to MarketHeader.quote_mint (chunk 1).
.equ MKT_QUOTE_MINT_CHUNK_1_OFF, 10512
# From input buffer to MarketHeader.quote_mint (chunk 2).
.equ MKT_QUOTE_MINT_CHUNK_2_OFF, 10520
# From input buffer to MarketHeader.quote_mint (chunk 3).
.equ MKT_QUOTE_MINT_CHUNK_3_OFF, 10528
.equ MKT_BUMP_OFF, 10536 # From input buffer to MarketHeader.bump.
# From input buffer to MarketHeader.base_vault.
.equ MKT_BASE_VAULT_OFF, 10537
# From input buffer to MarketHeader.base_vault (chunk 0).
.equ MKT_BASE_VAULT_CHUNK_0_OFF, 10537
# From input buffer to MarketHeader.base_vault (chunk 1).
.equ MKT_BASE_VAULT_CHUNK_1_OFF, 10545
# From input buffer to MarketHeader.base_vault (chunk 2).
.equ MKT_BASE_VAULT_CHUNK_2_OFF, 10553
# From input buffer to MarketHeader.base_vault (chunk 3).
.equ MKT_BASE_VAULT_CHUNK_3_OFF, 10561
# From input buffer to MarketHeader.base_vault_bump.
.equ MKT_BASE_VAULT_BUMP_OFF, 10569
# From input buffer to MarketHeader.quote_vault.
.equ MKT_QUOTE_VAULT_OFF, 10570
# From input buffer to MarketHeader.quote_vault (chunk 0).
.equ MKT_QUOTE_VAULT_CHUNK_0_OFF, 10570
# From input buffer to MarketHeader.quote_vault (chunk 1).
.equ MKT_QUOTE_VAULT_CHUNK_1_OFF, 10578
# From input buffer to MarketHeader.quote_vault (chunk 2).
.equ MKT_QUOTE_VAULT_CHUNK_2_OFF, 10586
# From input buffer to MarketHeader.quote_vault (chunk 3).
.equ MKT_QUOTE_VAULT_CHUNK_3_OFF, 10594
# From input buffer to MarketHeader.quote_vault_bump.
.equ MKT_QUOTE_VAULT_BUMP_OFF, 10602
# From input buffer to MarketHeader.base_total.
.equ MKT_BASE_TOTAL_OFF, 10603
# From input buffer to MarketHeader.quote_total.
.equ MKT_QUOTE_TOTAL_OFF, 10611
# From input buffer to MarketHeader.lamports_total.
.equ MKT_LAMPORTS_TOTAL_OFF, 10619
# From input buffer to first sector in market memory map.
.equ MKT_SECTORS_START_OFF, 10627
# Absolute SBPF pointer to first sector in market memory map.
.equ MKT_SECTORS_START_PTR_WD, 17179879811
# -------------------------------------------------------------------------
