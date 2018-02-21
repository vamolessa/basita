macro_rules! declare_handle {
	($name:ident) => {
		#[derive(Serialize, Deserialize)]
		pub struct $name<T> {
			index: usize,
			_phantom: ::std::marker::PhantomData<T>,
		}

		impl<T> $name<T> {
			fn new(index: usize) -> Self {
				$name {
					index: index,
					_phantom: ::std::marker::PhantomData,
				}
			}
		}

		impl<T> Clone for $name<T> {
			fn clone(&self) -> Self {
				$name::new(self.index)
			}
		}

		impl<T> Copy for $name<T> {}

		impl<T> Default for $name<T> {
			fn default() -> Self {
				$name {
					index: Default::default(),
					_phantom: ::std::marker::PhantomData,
				}
			}
		}

		impl<T> ::std::fmt::Debug for $name<T> {
			fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(formatter, "{} [{}]", stringify!($name), self.index)
			}
		}
	};
}
