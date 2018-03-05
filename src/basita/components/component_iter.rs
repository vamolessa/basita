use std::iter::Iterator;

use super::{Component, ComponentCollection};

pub trait ComponentJoin
where Self: Sized {
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

impl<'a, A: 'a + Component, B: 'a + Component> ComponentJoin for (&'a ComponentCollection<A>, &'a ComponentCollection<B>) {
	type Join = (&'a A, &'a B);

	fn at(&self, i: usize) -> Option<Self::Join> {
		if let (Some(a), Some(b)) = (self.0.at(i), self.1.at(i)) {
			Some((a, b))
		} else {
			None
		}
	}
}

fn test()
{
	use super::Transform;
	use super::Collider;

	let a = ComponentCollection::<Transform>::default();
	let b = ComponentCollection::<Collider>::default();

	for (x,y) in (&a,&b).iter(0) {

	}
}