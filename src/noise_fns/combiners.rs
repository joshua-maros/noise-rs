use crate::{NoiseFn, SamplePoint};

macro_rules! combiner {
    ($vis:vis $name:ident($combine_fn:expr)) => {
        /// Noise function that outputs the combination of the two output values from two source
        /// functions using a math operation.
        $vis struct $name<A, B> {
            /// Outputs a value.
            pub source1: A,
            /// Outputs a value.
            pub source2: B,
        }

        impl<A, B> $name<A, B>
        {
            pub fn new(source1: A, source2: B) -> Self {
                Self { source1, source2 }
            }

            with!(pub source1: A);
            with!(pub source2: B);
        }

        impl<A, B, P: SamplePoint + Clone> NoiseFn<P> for $name<A, B>
        where
            A: NoiseFn<P>,
            B: NoiseFn<P>,
        {
            fn get(&self, point: P) -> f64 {
                $combine_fn(self.source1.get(point.clone()), self.source2.get(point))
            }
        }
    };
}

combiner! { pub Add(std::ops::Add::add) }
combiner! { pub Multiply(std::ops::Mul::mul) }
combiner! { pub Power(f64::powf) }
combiner! { pub Min(f64::min) }
combiner! { pub Max(f64::max) }
