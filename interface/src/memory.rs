#[repr(C, packed)]
pub struct StackNode {
    pub next: *mut StackNode,
}
