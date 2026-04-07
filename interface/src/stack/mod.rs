use dropset_macros::svm_data;

#[svm_data]
pub struct StackNode {
    pub next: *mut StackNode,
}
