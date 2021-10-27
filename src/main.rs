#![allow(dead_code)]
#![allow(unused_variables)]

struct Context {
    //...
}

#[derive(Clone, Copy)]
enum Variant {
    Var1,
}

struct VariantWithContext<'a> {
    ctx: &'a Context,
    variant: Variant,
}

trait GenericVariant<'a> {
    //...
}
impl<'a> GenericVariant<'a> for VariantWithContext<'a> {}

struct Variants {
    inner: Vec<Variant>,
}

impl Variants {
    fn iter_with_context<'slf, 'ctx>(
        &'slf self,
        ctx: &'ctx Context,
    ) -> Box<dyn Iterator<Item = Box<dyn GenericVariant + 'ctx>> + 'slf>
    where
        'ctx: 'slf,
    {
        Box::new(self.inner.iter().cloned().map(|variant| {
            let abstraction: VariantWithContext<'ctx> = variant.with_context(ctx);
            let box_step: Box<VariantWithContext<'ctx>> = Box::new(abstraction);
            let dyn_box: Box<dyn GenericVariant + 'ctx> = box_step;
            dyn_box
        }))
    }
}

impl Variant {
    pub fn with_context<'slf, 'ctx>(&'slf self, ctx: &'ctx Context) -> VariantWithContext<'ctx> {
        VariantWithContext {
            ctx,
            variant: *self,
        }
    }
}

fn main() {
    todo!()
}
