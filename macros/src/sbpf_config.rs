/// Maximum size of a single SBPF stack frame in bytes.
pub fn stack_frame_size() -> usize {
    solana_sbpf::vm::Config::default().stack_frame_size
}

/// Required alignment for stack frame fields (BPF_ALIGN_OF_U128).
pub const FRAME_ALIGN: i64 = 8;
