#![feature(specialization)]

pub trait Trait<T> { }

default impl<T, U> Trait<T> for U { }

impl<T> Trait<<T as Iterator>::Item> for T { }

fn main() { }
