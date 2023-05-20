#[macro_export]
macro_rules! impl_holo {
    ($backend:tt, $t:ty) => {
        impl<$backend> $crate::Holo for $t
        where
            $backend: $crate::Backend,
        {
            fn add_focus(&mut self, focus: Vector3, amp: float) {
                self.foci.push(focus);
                self.amps.push(amp);
            }

            fn set_constraint(&mut self, constraint: Constraint) {
                self.constraint = constraint;
            }
        }
    };

    ($t:ty) => {
        impl $crate::Holo for $t {
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
