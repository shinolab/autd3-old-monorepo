#[macro_export]
macro_rules! impl_holo {
    ($backend:tt, $t:ty) => {
        impl<$backend> $crate::HoloProps for $t
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

        impl<$backend, T: Transducer> $crate::Holo<T> for $t where $backend: $crate::Backend {}
    };

    ($t:ty) => {
        impl $crate::HoloProps for $t {
            fn add_focus(&mut self, focus: Vector3, amp: float) {
                self.foci.push(focus);
                self.amps.push(amp);
            }

            fn set_constraint(&mut self, constraint: Constraint) {
                self.constraint = constraint;
            }
        }

        impl<T: Transducer> $crate::Holo<T> for $t {}
    };
}
