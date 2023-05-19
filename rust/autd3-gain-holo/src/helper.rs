#[macro_export]
macro_rules! impl_holo {
    ($t:ty) => {
        impl<T: Transducer, B: Backend> crate::Holo<T> for $t {
            fn add_focus(&mut self, focus: Vector3, amp: float) {
                self.foci.push(focus);
                self.amps.push(amp);
            }

            fn set_constraint(&mut self, constraint: Constraint) {
                self.constraint = constraint;
            }
        }
    };
}
