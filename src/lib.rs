use std::{fmt::Debug, pin::Pin, ptr};

/// Raw representation of Union-Find classes
/// This structure is self-referential and should never
/// be used with unpinned pointers!
#[derive(Debug)]
pub struct Repr<T> {
    #[allow(unused)]
    pub tag: T,
    rank: u32,
    parent: *mut Repr<T>,
}

pub type PinnedRepr<T> = Pin<Box<Repr<T>>>;

impl<T> Repr<T> {
    pub fn new(tag: T) -> PinnedRepr<T> {
        let boxed_repr = Box::new(Self {
            tag,
            rank: 0,
            parent: ptr::null_mut(),
        });

        unsafe {
            let repr = Box::into_raw(boxed_repr);
            (*repr).parent = repr;
            Box::into_pin(Box::from_raw(repr))
        }
    }

    /// Find the root/representative of the class of the Repr
    pub fn find(&mut self) -> *mut Repr<T> {
        if self.parent != self as *mut Repr<T> {
            unsafe {
                let next = &mut *self.parent;
                let parent = next.find();
                self.parent = parent;
            }
        }
        self.parent
    }

    /// Union two classes
    pub fn union(&mut self, other: &mut Repr<T>) {
        let lhs = self.find();
        let rhs = other.find();
        unsafe {
            let lhs_rank = (*lhs).rank;
            let rhs_rank = (*rhs).rank;
            if lhs != rhs {
                if lhs_rank < rhs_rank {
                    (*lhs).parent = rhs;
                } else {
                    (*rhs).parent = lhs;
                    if lhs_rank == rhs_rank {
                        (*lhs).rank += 1;
                    }
                }
            }
        }
    }

    /// Check if a given Repr is the representative of is class
    pub fn is_root(&mut self) -> bool {
        self.parent == self as *mut Repr<T>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_null_parent() {
        let x = Repr::new(());
        assert!(!x.parent.is_null());
    }

    #[test]
    fn rank_aware_insertion() {
        let mut x = Repr::new(());
        let mut y = Repr::new(());
        let mut z = Repr::new(());
        x.union(&mut y);
        assert_eq!(x.rank, 1);
        assert_eq!(y.rank, 0);
        y.union(&mut z);
        assert_eq!(x.rank, 1);
        assert_eq!(y.rank, 0);
        assert_eq!(z.rank, 0);
    }

    #[test]
    fn correct_incrementation() {
        let mut x = Repr::new(());
        let mut y = Repr::new(());
        let mut z = Repr::new(());
        let mut w = Repr::new(());
        x.union(&mut y);
        w.union(&mut z);
        w.union(&mut x);
        assert_eq!(w.rank, 2);
    }

    #[test]
    fn no_cycle() {
        let mut x = Repr::new(());
        let mut y = Repr::new(());
        x.union(&mut y);
        y.union(&mut x);
        assert_eq!(x.find(), y.find());
    }
}
