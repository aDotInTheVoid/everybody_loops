use anyhow::Result;
use fs_err as fs;
use syn::visit_mut::VisitMut;
use syn::Block;
use syn::ImplItemMethod;
use syn::ItemFn;

fn main() -> Result<()> {
    for f in std::env::args().skip(1) {
        do_on_file(&f)?;
    }

    Ok(())
}

fn do_on_file(path: &str) -> Result<()> {
    let src = fs::read_to_string(&path)?;
    let mut tree: syn::File = syn::parse_str(&src)?;

    let mut looper = Looper {
        loop_block: syn::parse_str("{ loop {} }").unwrap(),
    };

    looper.visit_file_mut(&mut tree);

    let out = prettyplease::unparse(&tree);

    fs::write(path, out)?;
    Ok(())
}

struct Looper {
    loop_block: Block,
}

impl VisitMut for Looper {
    fn visit_item_fn_mut(&mut self, func: &mut ItemFn) {
        func.block = Box::new(self.loop_block.clone())
    }

    fn visit_impl_item_method_mut(&mut self, method: &mut ImplItemMethod) {
        method.block = self.loop_block.clone()
    }
}
