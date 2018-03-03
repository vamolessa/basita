use std::iter::Iterator;

use super::{Component, ComponentCollection};

trait ComponentJoin {
	type Type;
	type Collection;

	fn get(collections: &Self::Collection, index: usize) -> Self::Type;
}

pub struct ComponentJoinIter<T: Join> {
	collections: T::Collection,
}

impl<T: Join> ComponentJoinIter<T> {}

impl<T: Join> Iterator for JoinIter<T> {
	type Item = T::Type;

	fn next(&mut self) -> Option<T::Type> {
		None
	}
}
