#![no_std]

use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self, Debug, Display, Formatter};
use core::future::Future;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::task::{Context, Poll};

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NoInlineDrop<T>
where
    T: ?Sized,
{
    value: ManuallyDrop<T>,
}

impl<T> NoInlineDrop<T>
where
    T: ?Sized,
{
    pub const fn new(value: T) -> Self
    where
        T: Sized,
    {
        Self {
            value: ManuallyDrop::new(value),
        }
    }

    pub fn as_pin_ref(this: Pin<&Self>) -> Pin<&T> {
        unsafe { this.map_unchecked(|this| this.as_ref()) }
    }

    pub fn as_pin_mut(this: Pin<&mut Self>) -> Pin<&mut T> {
        unsafe { this.map_unchecked_mut(|this| this.as_mut()) }
    }

    pub fn into_inner(mut this: Self) -> T
    where
        T: Sized,
    {
        unsafe {
            let value = ManuallyDrop::take(&mut this.value);

            mem::forget(this);

            value
        }
    }

    unsafe fn drop_value(&mut self) {
        ManuallyDrop::drop(&mut self.value)
    }
}

impl<T> AsRef<T> for NoInlineDrop<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for NoInlineDrop<T>
where
    T: ?Sized,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> Borrow<T> for NoInlineDrop<T>
where
    T: ?Sized,
{
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T> BorrowMut<T> for NoInlineDrop<T>
where
    T: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<T> Debug for NoInlineDrop<T>
where
    T: Debug + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        T::fmt(self.as_ref(), f)
    }
}

impl<T> Display for NoInlineDrop<T>
where
    T: Display + ?Sized,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        T::fmt(self.as_ref(), f)
    }
}

impl<T> Deref for NoInlineDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for NoInlineDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> Drop for NoInlineDrop<T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        unsafe { self.drop_value() }
    }
}

impl<T> From<T> for NoInlineDrop<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Future for NoInlineDrop<T>
where
    T: Future + ?Sized,
{
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        NoInlineDrop::as_pin_mut(self).poll(cx)
    }
}
