use std::fmt::Debug;

pub trait Scalar: Clone + Copy + Default + Debug + PartialEq + PartialOrd {}

impl<T: Clone + Copy + Default + Debug + PartialEq + PartialOrd> Scalar for T {}
