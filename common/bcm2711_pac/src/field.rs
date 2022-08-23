use core::{fmt, marker::PhantomData};
use tock_registers::{fields::FieldValue, interfaces::Readable, RegisterLongName, UIntLike};

/// An on-memory value that can be read and written through the interface of
/// `tock_registers` and follows inherited mutability.
#[repr(transparent)]
pub struct MemoryField<T: UIntLike, R: RegisterLongName = ()> {
    pub value: T,
    _associated_register: PhantomData<R>,
}

impl<T: UIntLike, R: RegisterLongName> MemoryField<T, R> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _associated_register: PhantomData,
        }
    }

    #[inline]
    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    #[inline]
    pub fn write(&mut self, field: FieldValue<T, R>) {
        self.value = field.value;
    }

    #[inline]
    pub fn modify(&mut self, field: FieldValue<T, R>) {
        self.value = field.modify(self.value);
    }
}

impl<T: UIntLike, R: RegisterLongName> Copy for MemoryField<T, R> {}

impl<T: UIntLike, R: RegisterLongName> Clone for MemoryField<T, R> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: UIntLike + Default, R: RegisterLongName> Default for MemoryField<T, R> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: UIntLike + fmt::Debug, R: RegisterLongName> fmt::Debug for MemoryField<T, R> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.extract().fmt(f)
    }
}

impl<T: UIntLike, R: RegisterLongName> Readable for MemoryField<T, R> {
    type T = T;
    type R = R;

    #[inline]
    fn get(&self) -> Self::T {
        self.value
    }
}

impl<T: UIntLike, R: RegisterLongName> From<FieldValue<T, R>> for MemoryField<T, R> {
    #[inline]
    fn from(field: FieldValue<T, R>) -> Self {
        Self::new(field.value)
    }
}
