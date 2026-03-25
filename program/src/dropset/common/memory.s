# Common data-related constants.
# -------------------------------------------------------------------------
.equ DATA_DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_BPF_ALIGN_OF_U128, 8 # Data alignment during runtime.
# Maximum possible data length padding for a runtime account.
.equ DATA_MAX_DATA_PAD, 7
.equ DATA_DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
# -------------------------------------------------------------------------

# Input buffer constants for static header.
# -------------------------------------------------------------------------
.equ IB_NON_DUP_MARKER, 255 # Non-dup marker for accounts.
.equ IB_USER_DATA_LEN_OFF, 88 # From input buffer to user data length.
# From input buffer to market duplicate flag.
.equ IB_MARKET_DUPLICATE_OFF, 10344
# From input buffer to market data length.
.equ IB_MARKET_DATA_LEN_OFF, 10424
# -------------------------------------------------------------------------

.equ SIZE_OF_ADDRESS, 32 # Size of Address in bytes.
