# Assorted runtime account constants.
# -------------------------------------------------------------------------
.equ ACCT_DUPLICATE_OFF, 0 # Borrow state / duplicate marker.
.equ ACCT_IS_SIGNER_OFF, 1 # Whether the account is a signer.
.equ ACCT_IS_WRITABLE_OFF, 2 # Whether the account is writable.
.equ ACCT_EXECUTABLE_OFF, 3 # Whether the account is executable.
.equ ACCT_RESIZE_DELTA_OFF, 4 # Resize delta.
.equ ACCT_ADDRESS_OFF, 8 # Account address field.
.equ ACCT_ADDRESS_CHUNK_0_OFF, 8 # Account address field (chunk 0).
.equ ACCT_ADDRESS_CHUNK_1_OFF, 16 # Account address field (chunk 1).
.equ ACCT_ADDRESS_CHUNK_2_OFF, 24 # Account address field (chunk 2).
.equ ACCT_ADDRESS_CHUNK_3_OFF, 32 # Account address field (chunk 3).
.equ ACCT_OWNER_OFF, 40 # Account owner.
.equ ACCT_OWNER_CHUNK_0_OFF, 40 # Account owner (chunk 0).
.equ ACCT_OWNER_CHUNK_1_OFF, 48 # Account owner (chunk 1).
.equ ACCT_OWNER_CHUNK_2_OFF, 56 # Account owner (chunk 2).
.equ ACCT_OWNER_CHUNK_3_OFF, 64 # Account owner (chunk 3).
.equ ACCT_DATA_LEN_OFF, 80 # Account data length.
.equ ACCT_DATA_OFF, 88 # Account data start.
# From address to owner in a runtime account.
.equ ACCT_ADDRESS_TO_OWNER_REL_OFF_IMM, 32
# From owner to lamports in a runtime account.
.equ ACCT_OWNER_TO_LAMPORTS_REL_OFF_IMM, 32
# From lamports to data start in a runtime account.
.equ ACCT_LAMPORTS_TO_DATA_REL_OFF_IMM, 16
.equ ACCT_NON_DUP_MARKER, 255 # Non-dup marker for accounts.
# Account storage overhead for rent calculation.
.equ ACCT_STORAGE_OVERHEAD, 128
# -------------------------------------------------------------------------

# CPI-related account constants.
# -------------------------------------------------------------------------
# Mask for writable signer (is_writable | is_signer).
.equ CPI_WRITABLE_SIGNER, 257
.equ CPI_READONLY_NON_SIGNER, 0 # Mask for readonly non-signer.
# -------------------------------------------------------------------------
