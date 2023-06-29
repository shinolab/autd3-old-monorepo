#[macro_export]
macro_rules! impl_holo {
    ($backend:tt, $t:ty) => {
        impl<$backend> $t
        where
            $backend: $crate::Backend,
        {
            pub fn add_focus(self, focus: Vector3, amp: float) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                foci.push(focus);
                amps.push(amp);
                Self { foci, amps, ..self }
            }

            pub fn with_constraint(self, constraint: Constraint) -> Self {
                Self { constraint, ..self }
            }

            pub fn add_foci_from_iter<I: IntoIterator<Item = (Vector3, float)>>(
                self,
                iter: I,
            ) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                for (focus, amp) in iter {
                    foci.push(focus);
                    amps.push(amp);
                }
                Self { foci, amps, ..self }
            }

            pub fn foci(
                &self,
            ) -> std::iter::Zip<std::slice::Iter<'_, Vector3>, std::slice::Iter<'_, float>> {
                self.foci.iter().zip(self.amps.iter())
            }

            pub fn constraint(&self) -> &Constraint {
                &self.constraint
            }
        }
    };

    ($t:ty) => {
        impl $t {
            pub fn add_focus(self, focus: Vector3, amp: float) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                foci.push(focus);
                amps.push(amp);
                Self { foci, amps, ..self }
            }

            pub fn with_constraint(self, constraint: Constraint) -> Self {
                Self { constraint, ..self }
            }

            pub fn add_foci_from_iter<I: IntoIterator<Item = (Vector3, float)>>(
                self,
                iter: I,
            ) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                for (focus, amp) in iter {
                    foci.push(focus);
                    amps.push(amp);
                }
                Self { foci, amps, ..self }
            }

            pub fn foci(
                &self,
            ) -> std::iter::Zip<std::slice::Iter<'_, Vector3>, std::slice::Iter<'_, float>> {
                self.foci.iter().zip(self.amps.iter())
            }

            pub fn constraint(&self) -> &Constraint {
                &self.constraint
            }
        }
    };
}
