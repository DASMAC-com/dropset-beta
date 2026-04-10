use dropset_macros::svm_data;

#[svm_data]
pub struct StackNode {
    pub tag: u8,
    pub next: *mut StackNode,
}
