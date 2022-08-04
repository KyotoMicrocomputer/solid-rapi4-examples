//! High-level binding for SOLID Loader API
use core::{ffi::CStr, mem::MaybeUninit};

use std::num::NonZeroUsize;

use crate::{abi, error::Error as SolidError, fs::ToSolidPath};

/// Represents a loader instance.
pub struct Loader(());

impl Loader {
    /// Get the global instance of the loader.
    ///
    /// # Safety
    ///
    /// - All API calls to the loader must be externally synchronized. (The
    ///   loader is not thread-safe.)
    pub unsafe fn global_unchecked() -> LoaderRef<'static> {
        LoaderRef([])
    }
}

/// An error type indicating that a specified object name was not found in a
/// loader database.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct NotFoundError;

/// The error type for [`LoaderRef::load_object`]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum LoadError {
    /// The specified object name is already in use.
    ObjectNameInUse,
    Other(SolidError),
}

/// The error type for [`LoaderRef::register_symbol`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum RegisterSymbolError {
    OutOfMemory,
}

/// A reference to a loader instance.
///
/// Use [`Loader::global_unchecked`] to get a reference to the global loader
/// instance.
pub struct LoaderRef<'a>([&'a (); 0]);

impl LoaderRef<'static> {
    /// Look up a symbol and get its value (address).
    pub fn symbol_value(&mut self, sym_name: &CStr) -> Result<usize, NotFoundError> {
        unsafe {
            let mut addr = MaybeUninit::uninit();
            match abi::SOLID_LDR_GetAddr(sym_name.as_ptr(), addr.as_mut_ptr()) {
                abi::SOLID_ERR_OK => Ok(addr.assume_init() as _),
                abi::SOLID_ERR_NOTFOUND => Err(NotFoundError),
                abi::c_int(e) => panic!("SOLID_LDR_GetAddr failed: {e}"),
            }
        }
    }

    /// Look up a symbol in the specified loaded object and get its value (address).
    pub fn symbol_value_in_object(
        &mut self,
        obj_name: &CStr,
        sym_name: &CStr,
    ) -> Result<usize, NotFoundError> {
        unsafe {
            let mut addr = MaybeUninit::uninit();
            match abi::SOLID_LDR_GetDllAddr(obj_name.as_ptr(), sym_name.as_ptr(), addr.as_mut_ptr())
            {
                abi::SOLID_ERR_OK => Ok(addr.assume_init() as _),
                abi::SOLID_ERR_NOTFOUND | abi::SOLID_ERR_PAR => Err(NotFoundError),
                abi::c_int(e) => panic!("SOLID_LDR_GetDllAddr failed: {e}"),
            }
        }
    }

    /// Load an object file (DLL or SLO file) from a file.
    ///
    /// # Safety
    ///
    /// TODO
    pub unsafe fn load_object(
        &mut self,
        _obj_name: &CStr,
        _path: impl ToSolidPath,
        _offset: usize,
        _max_size: Option<NonZeroUsize>,
    ) -> Result<(), LoadError> {
        // TODO: Needs a newer SDK providing the definition of this function
        // unsafe {
        //     SolidError::err_if_negative(abi::SOLID_LDR_Load(
        //         obj_name.as_ptr(),
        //         path.to_solid_path().ok_or(SolidError::PAR)?.borrow().as_ptr(),
        //         offset,
        //         max_size.map_or(0, NonZeroUsize::get),
        //     ))?;
        //     Ok(())
        // }
        todo!()
    }

    /// Get the entry point of the specified object file.
    pub fn object_entry_addr(&mut self, obj_name: &CStr) -> Result<usize, NotFoundError> {
        unsafe {
            let mut addr = MaybeUninit::uninit();
            match abi::SOLID_LDR_CanExec(obj_name.as_ptr(), addr.as_mut_ptr()) {
                abi::c_int(can_exec) if can_exec >= 0 => Ok(addr.assume_init() as _),
                abi::SOLID_ERR_NOTFOUND => Err(NotFoundError),
                abi::c_int(e) => panic!("SOLID_LDR_CanExec failed: {e}"),
            }
        }
    }

    /// Check if a loaded object is fully linked and executable.
    ///
    /// The memory sections of a loaded object is marked as non-executable until
    /// all relocations are resolved, and the dynamic linking is complete.
    pub fn is_object_ready(&mut self, obj_name: &CStr) -> Result<bool, NotFoundError> {
        unsafe {
            match abi::SOLID_LDR_CanExec(obj_name.as_ptr(), core::ptr::null_mut()) {
                abi::c_int(can_exec) if can_exec >= 0 => Ok(can_exec != 0),
                abi::SOLID_ERR_NOTFOUND => Err(NotFoundError),
                abi::c_int(e) => panic!("SOLID_LDR_CanExec failed: {e}"),
            }
        }
    }

    /// Unload the specified loaded object.
    pub fn unload_object(&mut self, obj_name: &CStr) -> Result<(), NotFoundError> {
        unsafe {
            match abi::SOLID_LDR_UnLoad(obj_name.as_ptr()) {
                abi::SOLID_ERR_OK => Ok(()),
                abi::SOLID_ERR_NOTFOUND => Err(NotFoundError),
                abi::c_int(e) => panic!("SOLID_LDR_CanExec failed: {e}"),
            }
        }
    }

    /// Register a symbol.
    pub fn register_symbol(
        &mut self,
        sym_name: &CStr,
        sym_value: usize,
    ) -> Result<(), RegisterSymbolError> {
        unsafe {
            match abi::SOLID_LDR_RegisterSymbol(sym_name.as_ptr(), sym_value as _) {
                abi::SOLID_ERR_OK => Ok(()),
                abi::SOLID_ERR_NOMEM => Err(RegisterSymbolError::OutOfMemory),
                abi::c_int(e) => panic!("SOLID_LDR_CanExec failed: {e}"),
            }
        }
    }
}
