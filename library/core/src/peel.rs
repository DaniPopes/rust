#![doc(hidden)]

use crate::intrinsics::transmute_unchecked;

/// Peel the references of a type.
#[inline(always)]
#[rustc_const_unstable(feature = "peel", issue = "none")]
pub const fn peel<T: ?Sized>(t: &T) -> &<T as Peel>::Peeled {
    Peel::peel(t)
}

/// Peel the references of a type.
#[const_trait]
#[rustc_const_unstable(feature = "peel", issue = "none")]
pub trait Peel {
    /// The type after peeling all references.
    type Peeled: ?Sized;
    /// Peel the references of a type.
    fn peel(&self) -> &Self::Peeled;
}

#[rustc_const_unstable(feature = "peel", issue = "none")]
impl<T: ?Sized> const Peel for T {
    default type Peeled = T;
    #[inline(always)]
    default fn peel(&self) -> &Self::Peeled {
        const {
            assert!(type_eq::<T, Self::Peeled>(), "T and Peeled are not the same type");
        }
        unsafe { transmute_unchecked(self) }
    }
}

#[rustc_const_unstable(feature = "peel", issue = "none")]
impl<T: [const] Peel + ?Sized> const Peel for &T {
    type Peeled = T::Peeled;
    #[inline(always)]
    fn peel(&self) -> &Self::Peeled {
        T::peel(self)
    }
}
#[rustc_const_unstable(feature = "peel", issue = "none")]
impl<T: [const] Peel + ?Sized> const Peel for &mut T {
    type Peeled = T::Peeled;
    #[inline(always)]
    fn peel(&self) -> &Self::Peeled {
        T::peel(self)
    }
}

const fn type_eq<T: ?Sized, U: ?Sized>() -> bool {
    pub trait MaybeSame<T: ?Sized> {
        const IS_SAME: bool;
    }
    impl<T: ?Sized, U: ?Sized> MaybeSame<U> for T {
        default const IS_SAME: bool = false;
    }
    impl<T: ?Sized> MaybeSame<T> for T {
        const IS_SAME: bool = true;
    }

    const {
        assert!(<T as MaybeSame<U>>::IS_SAME == <U as MaybeSame<T>>::IS_SAME);
        <T as MaybeSame<U>>::IS_SAME
    }
}
