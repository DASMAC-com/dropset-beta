use dropset_macros::svm_data;

// region: stack_node
#[svm_data]
pub struct StackNode {
    pub tag: u8,
    pub next: *mut StackNode,
}
// endregion: stack_node
