use core::{error::Error, fmt};

pub fn is_aligned(value: usize, align: usize) -> bool {
    value % align == 0
}

/// align should be power of 2.
pub fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

/// align should be power of 2.
pub fn align_down(value: usize, align: usize) -> usize {
    (value) & !(align - 1)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrKind {
    NoMemory = 1,
    TooManyTasks,
    PteNotFound,
    OutOfMemory,
    ProcessNotFound,
    MessageBoxIsFull,
    InvalidUserAddress,
    UnknownCapType,
    UnexpectedCapType,
    CanNotNewFromDeviceMemory,
    NoEnoughSlot,
    NotEmptySlot,
    SlotIsEmpty,
    VaddressAlreadyMapped,
    PageTableAlreadyMapped,
    PageAlreadyMapped,
    PageTableNotMappedYet,
    UnknownInvocation,
    CanNotDerivable,
    InvalidOperation,
}

pub type KernelResult<T> = Result<T, KernelError>;

//TODO: thiserror and anyhow
#[derive(Debug)]
pub struct KernelError {
    pub e_kind: ErrKind,
    pub e_val: u16,
    #[cfg(debug_assertions)]
    pub e_place: EPlace,
}

#[macro_export]
macro_rules! kerr {
    ($ekind:expr) => {
        $crate::common::KernelError {
            e_kind: $ekind,
            e_val: 0,
            #[cfg(debug_assertions)]
            e_place: $crate::common::EPlace {
                e_line: line!(),
                e_column: column!(),
                e_file: file!(),
            },
        }
    };
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for KernelError {}

#[cfg(debug_assertions)]
#[derive(Debug)]
pub struct EPlace {
    pub e_line: u32,
    pub e_column: u32,
    pub e_file: &'static str,
}
