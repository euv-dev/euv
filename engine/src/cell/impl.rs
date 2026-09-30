use super::*;

// ===================================================================
// Accessor impls (the `#[derive(Data)]` Lombok derive is intentionally
// not applied to either struct - see `struct.rs` for the rationale,
// chiefly that Lombok requires `T: Sized` while our cells expose the
// `T: ?Sized` bound).
//
// These hand-written accessors mirror the contract Lombok's `Data`
// derive would have produced:
//
//   `pub fn get_inner(&self) -> &UnsafeCell<T>`               (both)
//   `pub fn set_inner(&self, val: UnsafeCell<Option<T>>) -> UnsafeCell<Option<T>>`
//                                                            (MaybeEngineCell only)
//
// `EngineCell` deliberately does NOT expose `set_inner` because
// `T: ?Sized` rules out `mem::replace` of the inner UnsafeCell; in
// practice the public surface of `EngineCell` never needs to swap
// the whole backing storage.
//
// All other impls in this file MUST go through these accessors
// rather than touching `self.inner` directly.
// ===================================================================

/// Accessor implementations for [`EngineCell`].
impl<T: ?Sized> EngineCell<T> {
    /// Returns a shared reference to the backing `UnsafeCell<T>`.
    ///
    /// # Returns
    ///
    /// - `&UnsafeCell<T>` - The backing storage of the cell, borrowed
    ///   for the lifetime of the cell.
    pub fn get_inner(&self) -> &UnsafeCell<T> {
        &self.inner
    }
}

/// Accessor implementations for [`MaybeEngineCell`].
impl<T> MaybeEngineCell<T> {
    /// Returns a shared reference to the backing `UnsafeCell<Option<T>>`.
    ///
    /// # Returns
    ///
    /// - `&UnsafeCell<Option<T>>` - A shared reference to the backing
    ///   `UnsafeCell<Option<T>>` storage.
    pub fn get_inner(&self) -> &UnsafeCell<Option<T>> {
        &self.inner
    }

    /// Replaces the backing storage with `val`, returning the previous
    /// `UnsafeCell<Option<T>>`.
    ///
    /// # Arguments
    ///
    /// - `UnsafeCell<Option<T>>` - The new backing storage to
    ///   install.
    ///
    /// # Returns
    ///
    /// - `UnsafeCell<Option<T>>` - The previous backing storage.
    ///
    /// Implemented via `core::mem::replace`. `MaybeEngineCell<T>`
    /// carries the implicit `Sized` bound (it is `T: Sized` here
    /// because `Option<T>` is `Sized` only when `T` is); `mem::replace`
    /// therefore applies. The returned previous backing storage is
    /// the caller's responsibility to drop.
    ///
    /// Note: `set_inner` is currently unused by the rest of the
    /// module (the `try_*` paths go straight through raw pointer
    /// reads/writes). It is kept here as the Lombok-shaped
    /// counterpart for parity with `EngineCell::get_inner`, in case
    /// future code wants to swap the whole backing storage.
    pub fn set_inner(&mut self, val: UnsafeCell<Option<T>>) -> UnsafeCell<Option<T>> {
        // `mem::replace` requires `T: Sized` - which holds for
        // `MaybeEngineCell<T>` because `Option<T>` is only `Sized`
        // when `T` is. The borrow of `self.inner` is the single
        // mutable access point under the cell's single-threaded
        // contract.
        mem::replace(&mut self.inner, val)
    }
}

// ===================================================================
// `Sync` impls (blanket markers, run as the first impl block per §9).
//
// SAFETY: see `struct.rs` doc comments. The engine runs only on the
// single wasm thread; concurrent access from multiple threads is
// undefined. Aligns with the `Sync` newtype shape used by
// `core::reactive::hook::impl::HookContext::current`.
// ===================================================================

/// Marker that `EngineCell<T>` is safe to share across the wasm main
/// thread under single-threaded access.
unsafe impl<T: ?Sized> Sync for EngineCell<T> {}

/// Marker that `MaybeEngineCell<T>` is safe to share across the wasm
/// main thread under single-threaded access.
unsafe impl<T> Sync for MaybeEngineCell<T> {}

// ===================================================================
// `Default` impls (run before body impls per §9).
// ===================================================================

/// `Default` impl for [`EngineCell`].
///
/// Only available when `T: Default + Sized`. The `?Sized` bound on
/// the cell prevents adding `Default` blanket-style because trait
/// `Default` cannot be implemented for unsized types.
impl<T: Default> Default for EngineCell<T> {
    /// Creates a default cell by installing `T::default()` as the
    /// initial value.
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// `Default` impl for [`MaybeEngineCell`].
impl<T> Default for MaybeEngineCell<T> {
    /// Constructs a default [`MaybeEngineCell`] value.
    fn default() -> Self {
        Self::new()
    }
}

// ===================================================================
// Body impls (constructor + accessors).
//
// All read sites go through the `get_inner` accessor and then
// dereference via the standard `UnsafeCell::get()` raw-pointer
// escape. Write sites that need to replace the whole backing
// storage use `set_inner` (MaybeEngineCell) or none at all
// (EngineCell, which is never required to swap its backing storage).
// There are NO direct field accesses in this block - the
// field-access rule in `struct.rs` is the single source of truth.
// ===================================================================

/// Constructor + read accessors for [`EngineCell`].
impl<T: ?Sized> EngineCell<T> {
    /// Creates a new cell with the given initial value.
    ///
    /// Construction is the one place where direct field initialisation
    /// is permitted (see field-access rule in `struct.rs`); all other
    /// sites must go through the accessors.
    ///
    /// # Arguments
    ///
    /// - `T` - The `value` to store in the new cell.
    pub fn new(value: T) -> Self
    where
        T: Sized,
    {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Returns a mutable reference to the contained value.
    ///
    /// # Safety
    ///
    /// The borrow MUST be exclusive - no other `get`, `get_mut`,
    /// `try_get`, or `try_get_mut` on the same cell may be alive when
    /// the returned reference is used. Two concurrent mutable borrows on
    /// a wasm single-threaded runtime are well-defined in practice only
    /// because wasm has no data-race detection; the `&'static mut`
    /// return type tells the borrow checker you promise exclusivity.
    ///
    /// Reads via the `get_inner` accessor; the resulting
    /// `&UnsafeCell<T>` is then turned into a raw pointer by
    /// `UnsafeCell::get()` so we can hand the caller the
    /// `&'static mut T` the rest of the engine expects.
    ///
    /// # Returns
    ///
    /// - `&'static mut T` - An exclusive reference to the contained
    ///   value, extended to the `'static` lifetime.
    pub fn get_mut(&self) -> &'static mut T {
        let inner: &UnsafeCell<T> = self.get_inner();
        unsafe { &mut *inner.get() }
    }

    /// Returns a shared reference to the contained value.
    ///
    /// # Safety
    ///
    /// The lifetime of `&'static T` extends beyond what the borrow
    /// checker can prove. Callers MUST NOT use this while a mutable
    /// borrow on the same cell is alive. Use [`Self::get_mut`]
    /// exclusively for write access.
    ///
    /// Reads via the `get_inner` accessor + `UnsafeCell::get`.
    ///
    /// # Returns
    ///
    /// - `&'static T` - The current value (or a snapshot thereof), as a
    ///   shared reference extended to the `'static` lifetime.
    pub fn get(&self) -> &'static T {
        let inner: &UnsafeCell<T> = self.get_inner();
        unsafe { &*inner.get() }
    }
}

/// Constructor + accessors for [`MaybeEngineCell`].
impl<T> MaybeEngineCell<T> {
    /// Creates an empty cell.
    ///
    /// Struct literal is permitted by the field-access rule for the
    /// constructor only.
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    /// If the cell contains a value, returns a shared reference to it.
    ///
    /// # Returns
    ///
    /// - `Option<&'static T>` - Optional shared reference to the inner
    ///   value, or `None` when the cell is empty.
    pub fn try_get(&self) -> Option<&'static T> {
        let inner: &UnsafeCell<Option<T>> = self.get_inner();
        let slot: &Option<T> = unsafe { &*inner.get() };
        slot.as_ref()
    }

    /// If the cell contains a value, returns a mutable reference to it.
    ///
    /// # Safety
    ///
    /// Exclusivity rules from [`EngineCell::get_mut`] apply - no other
    /// borrow on the same cell may be alive.
    ///
    /// # Returns
    ///
    /// - `Option<&'static mut T>` - Optional mutable reference to the
    ///   inner value, or `None` when the cell is empty.
    pub fn try_get_mut(&self) -> Option<&'static mut T> {
        let inner: &UnsafeCell<Option<T>> = self.get_inner();
        let slot: &mut Option<T> = unsafe { &mut *inner.get() };
        slot.as_mut()
    }

    /// Installs `value` into the cell. Returns `Err(value)` if the cell
    /// is already populated; the caller may then retry or drop it.
    ///
    /// Reads through `get_inner` to inspect the current state without
    /// aliasing, then writes through `UnsafeCell::get` raw pointer.
    ///
    /// # Arguments
    ///
    /// - `T` - Value to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), T>` - `Ok(())` on success, or `Err(value)` if the cell was occupied.
    pub fn try_set(&self, value: T) -> Result<(), T> {
        let inner: &UnsafeCell<Option<T>> = self.get_inner();
        let slot: *mut Option<T> = inner.get();
        unsafe {
            if (*slot).is_some() {
                return Err(value);
            }
            *slot = Some(value);
        }
        Ok(())
    }

    /// Removes and returns the contained value, leaving the cell empty.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The taken value, or `None` if the cell was empty.
    pub fn try_take(&self) -> Option<T> {
        let inner: &UnsafeCell<Option<T>> = self.get_inner();
        unsafe { (*inner.get()).take() }
    }

    /// Replaces the contained value, returning the old one.
    ///
    /// # Arguments
    ///
    /// - `T` - Replacement value.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The previously stored value, or `None`.
    pub fn try_replace(&self, value: T) -> Option<T> {
        let inner: &UnsafeCell<Option<T>> = self.get_inner();
        let slot: *mut Option<T> = inner.get();
        unsafe { (*slot).replace(value) }
    }
}
