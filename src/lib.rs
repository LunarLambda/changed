#![doc = include_str!("../README.md")]

use std::ops::{Deref, DerefMut};

/// Cd: Change Detection
///
/// Start by creating one with [`new()`](Cd::new()).
#[derive(Debug, Default)]
pub struct Cd<T> {
    data: T,
    changed: bool,
}

impl<T> Cd<T> {
    /// Create a new Cd with data.
    /// It is initialized to false for change detection.
    /// Same as [`Cd::default()`].
    ///
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new(5);
    /// ```
    pub fn new(data: T) -> Cd<T> {
        Cd {
            data,
            changed: false,
        }
    }

    /// Create a new Cd with data.
    /// It is initialized to true for change detection.
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new_true(5);
    /// assert!(cd.changed());
    /// ```
    pub fn new_true(data: T) -> Cd<T> {
        Cd {
            data,
            changed: true,
        }
    }

    /// Reset the change tracking to false.
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new_true(5);
    /// cd.reset();
    /// assert!(!cd.changed());
    /// ```
    pub fn reset(&mut self) {
        self.changed = false;
    }

    /// Take the data out of the Cd.
    /// Consumes self and returns data.
    /// ```
    /// use changed::Cd;
    /// let cd = Cd::new(5);
    /// let data = cd.take();
    /// // Error: cd has been moved.
    /// // cd.changed();
    /// ```
    pub fn take(self) -> T {
        self.data
    }

    /// Check if the Cd has been changed since the last call to reset (or created.)
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// assert!(!cd.changed());
    /// *cd += 5;
    /// assert!(cd.changed());
    /// ```
    pub fn changed(&self) -> bool {
        self.changed
    }

    /// Mutate the Cd without tripping change detection.
    ///
    /// ```
    /// use changed::Cd;
    /// let mut cd = Cd::new(5);
    /// *cd.mutate_silently() += 5;
    /// assert!(!cd.changed());
    /// ```
    pub fn mutate_silently(&mut self) -> &mut T {
        &mut self.data
    }
}

/// `deref()` does not trip change detection.
/// ```
/// use changed::Cd;
/// let cd = Cd::new(5);
/// assert_eq!(*cd, 5); // deref for == 5
/// assert!(!cd.changed()); // .changed() is false
/// ```
impl<T> Deref for Cd<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// `deref_mut()` trips change detection.
/// ```
/// use changed::Cd;
/// let mut cd = Cd::new(5);
/// *cd += 5; // deref_mut for add assign
/// assert_eq!(*cd, 10);
/// assert!(cd.changed()); // .changed() is true
/// ```
impl<T> DerefMut for Cd<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.changed = true;
        &mut self.data
    }
}

/// Implement [`PartialEq<T>`] if `T` does.
///
/// Allows checking `Cd<T> == T`.
impl<T: PartialEq> PartialEq<T> for Cd<T> {
    fn eq(&self, other: &T) -> bool {
        &self.data == other
    }
}

#[cfg(test)]
mod tests {
    use crate::Cd;

    #[test]
    fn it_works() {
        let mut changed = Cd::new(15);
        *changed += 5;
        assert!(changed.changed);
        changed.reset();
        assert_eq!(*changed, 20);
        assert!(!changed.changed);
    }
}
