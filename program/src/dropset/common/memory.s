# Assorted runtime account constants.
# -------------------------------------------------------------------------
.equ ACCT_DUPLICATE_OFF, 0 # Borrow state / duplicate marker.
.equ ACCT_IS_SIGNER_OFF, 1 # Whether the account is a signer.
.equ ACCT_IS_WRITABLE_OFF, 2 # Whether the account is writable.
.equ ACCT_EXECUTABLE_OFF, 3 # Whether the account is executable.
.equ ACCT_RESIZE_DELTA_OFF, 4 # Resize delta.
.equ ACCT_ADDRESS_OFF, 8 # Account address.
.equ ACCT_ADDRESS_CHUNK_0_OFF, 8 # Account address (chunk 0).
.equ ACCT_ADDRESS_CHUNK_1_OFF, 16 # Account address (chunk 1).
.equ ACCT_ADDRESS_CHUNK_2_OFF, 24 # Account address (chunk 2).
.equ ACCT_ADDRESS_CHUNK_3_OFF, 32 # Account address (chunk 3).
.equ ACCT_OWNER_OFF, 40 # Account owner.
.equ ACCT_OWNER_CHUNK_0_OFF, 40 # Account owner (chunk 0).
.equ ACCT_OWNER_CHUNK_1_OFF, 48 # Account owner (chunk 1).
.equ ACCT_OWNER_CHUNK_2_OFF, 56 # Account owner (chunk 2).
.equ ACCT_OWNER_CHUNK_3_OFF, 64 # Account owner (chunk 3).
.equ ACCT_DATA_LEN_OFF, 80 # Account data length.
.equ ACCT_DATA_OFF, 88 # Account data start.
.equ ACCT_NON_DUP_MARKER, 255 # Non-dup marker for accounts.
# Account storage overhead for rent calculation.
.equ ACCT_STORAGE_OVERHEAD, 128
# -------------------------------------------------------------------------

# CPI-related constants.
# -------------------------------------------------------------------------
# Mask for writable signer (is_writable | is_signer).
.equ CPI_WRITABLE_SIGNER, 257
# -------------------------------------------------------------------------

# Common data-related constants.
# -------------------------------------------------------------------------
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_BPF_ALIGN_OF_U128, 8 # Data alignment during runtime.
# Maximum possible data length padding for a runtime account.
.equ DATA_LEN_MAX_PAD, 7
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
# -------------------------------------------------------------------------

# Input buffer constants for static header.
# -------------------------------------------------------------------------
.equ IB_USER_DATA_LEN_OFF, 88 # From input buffer to user data length.
.equ IB_USER_PUBKEY_OFF, 16 # From input buffer to user pubkey.
# From input buffer to user pubkey (chunk 0).
.equ IB_USER_PUBKEY_CHUNK_0_OFF, 16
# From input buffer to user pubkey (chunk 1).
.equ IB_USER_PUBKEY_CHUNK_1_OFF, 24
# From input buffer to user pubkey (chunk 2).
.equ IB_USER_PUBKEY_CHUNK_2_OFF, 32
# From input buffer to user pubkey (chunk 3).
.equ IB_USER_PUBKEY_CHUNK_3_OFF, 40
# From input buffer to market duplicate flag.
.equ IB_MARKET_DUPLICATE_OFF, 10344
# From input buffer to market data length.
.equ IB_MARKET_DATA_LEN_OFF, 10424
.equ IB_MARKET_PUBKEY_OFF, 10352 # From input buffer to market address.
# From input buffer to market address (chunk 0).
.equ IB_MARKET_PUBKEY_CHUNK_0_OFF, 10352
# From input buffer to market address (chunk 1).
.equ IB_MARKET_PUBKEY_CHUNK_1_OFF, 10360
# From input buffer to market address (chunk 2).
.equ IB_MARKET_PUBKEY_CHUNK_2_OFF, 10368
# From input buffer to market address (chunk 3).
.equ IB_MARKET_PUBKEY_CHUNK_3_OFF, 10376
# From address to owner in a runtime account.
.equ IB_ADDRESS_TO_OWNER_REL_OFF_IMM, 32
# From owner to lamports in a runtime account.
.equ IB_OWNER_TO_LAMPORTS_REL_OFF_IMM, 32
# From lamports to data start in a runtime account.
.equ IB_LAMPORTS_TO_DATA_REL_OFF_IMM, 16
# From user data to market address in the input buffer.
.equ IB_USER_DATA_TO_MARKET_ADDRESS_REL_OFF_IMM, 10256
# -------------------------------------------------------------------------

.equ SIZE_OF_U8, 1 # Size of u8 in bytes.
.equ SIZE_OF_ADDRESS, 32 # Size of Address in bytes.
.equ SIZE_OF_EMPTY_ACCOUNT, 10336 # Size of EmptyAccount in bytes.
.equ SIZE_OF_MARKET_HEADER, 40 # Size of MarketHeader in bytes.
