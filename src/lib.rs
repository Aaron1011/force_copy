#![feature(const_panic)]
#![feature(inline_const)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(incomplete_features)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::mem::MaybeUninit;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

pub struct ForceCopy<T> where [(); std::mem::size_of::<T>() ]: {
    raw: [MaybeUninit<u8>; std::mem::size_of::<T>() ],
    _phantom: PhantomData<T>
}

impl<T> Copy for ForceCopy<T> where [(); std::mem::size_of::<T>() ]: {}
impl<T> Clone for ForceCopy<T> where [(); std::mem::size_of::<T>() ]: {
    fn clone(&self) -> Self { *self }
}

impl<T> ForceCopy<T> where [(); std::mem::size_of::<T>() ]: {
    const NO_DTOR: () = {
        if std::mem::needs_drop::<T>() {
            panic!("Cannot have destructor!");
        }
    };
    
    pub unsafe fn new(val: T) -> Self {
        let _assert = Self::NO_DTOR;
        let mut val = ManuallyDrop::new(val);
        ForceCopy {
            raw: unsafe { std::ptr::read(&mut val as *mut ManuallyDrop<T> as *mut _) },
            _phantom: PhantomData
        }
    }
    
    pub unsafe fn into_inner(mut self) -> T {
        unsafe { std::ptr::read(&mut self.raw as *mut [_] as *mut _) }
    }
    
    pub unsafe fn as_ref(&self) -> &T {
        unsafe { std::mem::transmute(&self.raw) }
    }
    
    pub unsafe fn as_mut(&mut self) -> &mut T {
        unsafe { std::mem::transmute(&mut self.raw) }
    }
}
