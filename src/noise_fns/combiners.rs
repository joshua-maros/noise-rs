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

        impl<P: SamplePoint, A, B> Add<A, B>
        where
            A: NoiseFn<P>,
            B: NoiseFn<P>,
        {
            pub fn new(source1: A, source2: B) -> Self {
                Self { source1, source2 }
            }

            with!(source1: A);
            with!(source2: A);
        }

        impl<A, B, P: SamplePoint> NoiseFn<P> for Add<A, B>
        where
            A: NoiseFn<P>,
            B: NoiseFn<P>,
        {
            fn get(&self, point: P) -> f64 {
                $combine_fn(self.source1.get(point), self.source2.get(point))
            }
        }
    };
}

combiner! { pub Add(f64::add) }
combiner! { pub Multiply(f64::mul) }
combiner! { pub Power(f64::powf) }
combiner! { pub Min(f64::min) }
combiner! { pub Max(f64::max) }
