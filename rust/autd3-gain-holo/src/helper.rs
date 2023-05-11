#[macro_export]
macro_rules! impl_holo {
    ($t:ty) => {
        impl<B: Backend> $t {
            pub fn add_focus(&mut self, focus: Vector3, amp: float) {
                self.foci.push(focus);
                self.amps.push(amp);
            }
        }
    };
}
