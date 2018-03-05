use std::iter::Iterator;

use super::{Component, ComponentCollection};

pub trait ComponentJoin
where
	Self: Sized,
{
	type Join;

	fn at(&self, entity_index: usize) -> Option<Self::Join>;

	fn iter(self, entity_count: usize) -> ComponentIter<Self> {
		ComponentIter {
			entity_index: 0,
			entity_count: entity_count,
			join: self,
		}
	}
}

pub struct ComponentIter<J: ComponentJoin> {
	entity_index: usize,
	entity_count: usize,
	join: J,
}

impl<J: ComponentJoin> Iterator for ComponentIter<J> {
	type Item = J::Join;

	fn next(&mut self) -> Option<Self::Item> {
		while self.entity_index < self.entity_count {
			let index = self.entity_index;
			self.entity_index += 1;

			let j = self.join.at(index);
			if j.is_some() {
				return j;
			}
		}

		None
	}
}

impl<'a, A: 'a + Component> ComponentJoin for &'a ComponentCollection<A> {
	type Join = &'a A;

	fn at(&self, i: usize) -> Option<Self::Join> {
		if let Some(a) = ComponentCollection::at(self, i) {
			Some(a)
		} else {
			None
		}
	}
}

macro_rules! impl_component_join {
	() => ();
	($($x:ident,)+) => (
		impl<'a, $($x: 'a + Component),*> ComponentJoin
			for ($(&'a ComponentCollection<$x>,)*)
		{
			type Join = ($(&'a $x,)*);

			#[allow(non_snake_case)]
			fn at(&self, i: usize) -> Option<Self::Join> {
				let &($($x,)*) = self;
				if let ($(Some($x),)*) = ($($x.at(i),)*) {
					Some(($($x,)*))
				} else {
					None
				}
			}
		}
	)
}

impl_component_join!(A,);
impl_component_join!(A, B,);
impl_component_join!(A, B, C,);
impl_component_join!(A, B, C, D,);
impl_component_join!(A, B, C, D, E,);
impl_component_join!(A, B, C, D, E, F,);
impl_component_join!(A, B, C, D, E, F, G,);
impl_component_join!(A, B, C, D, E, F, G, H,);
impl_component_join!(A, B, C, D, E, F, G, H, I,);
impl_component_join!(A, B, C, D, E, F, G, H, I, J,);
