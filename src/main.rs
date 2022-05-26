use anyhow::Result;
use fs_err as fs;
use syn::visit_mut::VisitMut;
use syn::Block;
use syn::ImplItemMethod;
use syn::ItemFn;
fn main() -> Result<()> {
    loop {}
}
fn do_on_file(path: &str) -> Result<()> {
    loop {}
}
struct Looper {
    loop_block: Block,
}
impl VisitMut for Looper {
    fn visit_item_fn_mut(&mut self, func: &mut ItemFn) {
        loop {}
    }
    fn visit_impl_item_method_mut(&mut self, method: &mut ImplItemMethod) {
        loop {}
    }
}
