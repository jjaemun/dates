use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize
)]
pub enum Units {
    Days,
    BusinessDays,
    Weeks,
    Months,
    Quarters,
    Trimesters,
    Semesters,
    Years,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize
)]
#[repr(transparent)]
pub struct Period<const U: Units> {
    pub count: i32,
}

impl<const U: Units> Period<U> {
    pub const UNITS: Units = U;

    #[inline]
    pub const fn units(&self) -> Units {
        Self::UNITS
    }

    #[inline]
    pub const fn new(count: i32) -> Self {
        Self { count }
    }

    #[inline]
    pub const fn get_count(&self) -> i32 {
        self.count
    }

   #[inline]
    pub const fn is_zero(&self) -> bool {
        self.count == 0
    }

    #[inline]
    pub const fn is_negative(&self) -> bool {
        self.count < 0
    }

    #[inline]
    pub const fn is_positive(&self) -> bool {
        self.count > 0
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self::new(self.count.abs())
    }

    #[inline]
    pub fn negate(&self) -> Self {
        Self::new(-self.count)
    }
}


impl<const U: Units> Default for Period<U> {
    #[inline]
    fn default() -> Self {
        Self::new(0)
    }
}
