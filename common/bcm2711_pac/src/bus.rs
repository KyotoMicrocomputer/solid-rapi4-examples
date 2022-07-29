/// VideoCore physical address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vpa(pub u64);

impl Vpa {
    /// Map this address to a low-peripheral ARM physical address.
    pub const fn to_arm_pa(self) -> Option<u64> {
        match self.0 {
            0x4_7c00_0000..=0x4_7fff_ffff => Some(self.0 - 0x4_7c00_0000 + 0xfc00_0000),
            _ => None,
        }
    }
}
