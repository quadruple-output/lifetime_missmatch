type Fraction = f32;

#[derive(Clone, Copy)]
enum Variant {
    Velocity {
        v_ref: usize,
    },
    AccelerationDt {
        factor: f32,
        a_ref: usize,
        dt_fraction: Fraction,
    },
}

impl Variant {
    pub fn abstraction_scaled_for<'variant, 'step>(
        &'variant self,
        step: &'step Step,
        fraction: Fraction,
    ) -> Abstraction<'step> {
        todo!()
    }
}

struct Abstraction<'a> {
    step: &'a Step,
    variant: Variant,
}

impl<'a> Contribution<'a> for Abstraction<'a> {
    fn contributions_iter(&'a self) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a> {
        todo!()
    }
}

struct Step {
    dt: f32,
}

trait Contribution<'a> {
    // fn sampling_position(&self) -> Position;

    // fn kind(&self) -> PhysicalQuantityKind;

    // fn vector(&self) -> Option<Vec3>;

    // fn contributions_factor(&self) -> f32;

    fn contributions_iter(&'a self) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a>;
}

fn main() {
    println!("Hello, world!");
}

struct Generic {
    fraction: Fraction,
    inner: Vec<Variant>,
}

impl Generic {
    fn abstraction_iter_for<'collection, 'step>(
        &'collection self,
        step: &'step Step,
    ) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'step>> + 'collection>
    where
        'step: 'collection,
    {
        let f = self.fraction;
        Box::new(self.inner.iter().cloned().map(|variant| {
            let abstraction: Abstraction<'step> = variant.abstraction_scaled_for(step, f);
            let box_step: Box<Abstraction<'step>> = Box::new(abstraction);
            let dyn_box: Box<dyn Contribution + 'step> = box_step;
            dyn_box
        }))
    }
}
