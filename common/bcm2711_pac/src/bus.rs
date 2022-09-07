/// VideoCore physical address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vpa(pub u64);

impl Vpa {
    /// Map a given VC address to a low-peripheral ARM physical address.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bcm2711_pac::Vpa;
    /// let gpio = Vpa(0x4_7e20_0000);
    /// assert_eq!(gpio.to_arm_pa(), Some(0xfe20_0000));
    /// ```
    #[inline]
    pub const fn to_arm_pa(self) -> Option<u64> {
        match self.0 {
            0x4_7c00_0000..=0x4_7fff_ffff => Some(self.0 - 0x4_7c00_0000 + 0xfc00_0000),
            _ => None,
        }
    }
}
