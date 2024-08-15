pub mod poop {
	macro_rules! oop {
		(
			public class $class_name:ident {
				$($field:ident: $field_type:ty),*
				$(,)?
			}
			$($method:item)*
		) => {
			pub struct $class_name {
				$($field: $field_type),*
			}

			impl $class_name {
				fn _custom($($field: $field_type),*) -> Self {
					Self {
						$($field),*
					}
				}

				$($method)*
			}
		};
	}

	pub(crate) use oop;
}