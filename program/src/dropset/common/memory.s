.equ SIZE_OF_U8, 1 # Size of u8 in bytes.
.equ SIZE_OF_U64, 8 # Size of u64 in bytes.
.equ SIZE_OF_PUBKEY, 32 # Size of Pubkey in bytes.
.equ SIZE_OF_EMPTY_ACCOUNT, 10336 # Size of EmptyAccount in bytes.
.equ SIZE_OF_MARKET_HEADER, 43 # Size of MarketHeader in bytes.
.equ SIZE_OF_CREATE_ACCOUNT_DATA, 52 # Size of CreateAccountData in bytes.
.equ SIZE_OF_INITIALIZE_ACCOUNT2, 33 # Size of InitializeAccount2 in bytes.
.equ SIZE_OF_SECTOR, 161 # Size of Sector in bytes.

# Common data-related constants.
# -------------------------------------------------------------------------
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_BPF_ALIGN_OF_U128, 8 # Data alignment during runtime.
# Maximum possible data length padding for a runtime account.
.equ DATA_LEN_MAX_PAD, 7
.equ DATA_LEN_AND_MASK, -8 # And mask for data length alignment.
.equ DATA_BOOL_FALSE, 0 # Boolean false value.
.equ DATA_BOOL_TRUE, 1 # Boolean true value.
# -------------------------------------------------------------------------

# Discriminant tag for nodes in the market memory map.
# -------------------------------------------------------------------------
.equ NODE_TAG_SEAT, 0 # Seat node.
.equ NODE_TAG_ORDER, 1 # Order node.
.equ NODE_TAG_STACK, 2 # Stack node.
# -------------------------------------------------------------------------
