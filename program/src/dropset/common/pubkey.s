# Pubkey constants.
# -------------------------------------------------------------------------
.equ PUBKEY_CHUNK_OFF_0, 0 # Offset for the first 8 bytes.
.equ PUBKEY_CHUNK_OFF_1, 8 # Offset for the second 8 bytes.
.equ PUBKEY_CHUNK_OFF_2, 16 # Offset for the third 8 bytes.
.equ PUBKEY_CHUNK_OFF_3, 24 # Offset for the fourth 8 bytes.
.equ PUBKEY_RENT_CHUNK_0_LO, 399877894 # Rent sysvar ID (chunk 0 lo).
.equ PUBKEY_RENT_CHUNK_0_HI, 1364995097 # Rent sysvar ID (chunk 0 hi).
.equ PUBKEY_RENT_CHUNK_1_LO, 1288277025 # Rent sysvar ID (chunk 1 lo).
.equ PUBKEY_RENT_CHUNK_1_HI, 2146519613 # Rent sysvar ID (chunk 1 hi).
.equ PUBKEY_RENT_CHUNK_2_LO, 149871192 # Rent sysvar ID (chunk 2 lo).
.equ PUBKEY_RENT_CHUNK_2_HI, 1157472667 # Rent sysvar ID (chunk 2 hi).
.equ PUBKEY_RENT_CHUNK_3_LO, -1965433885 # Rent sysvar ID (chunk 3 lo).
.equ PUBKEY_RENT_CHUNK_3_HI, 0 # Rent sysvar ID (chunk 3 hi).
# -------------------------------------------------------------------------

pubkey_eq:
    exit
