pub mod merge_sort;
pub mod quick_sort;


#[cfg(test)]
mod test {
	use rand;
	use sorting::merge_sort;
	use sorting::quick_sort;

	fn test_sort(function: &Fn(&mut [u8])) {
		for size in 0..128 {
			let mut data1 = Vec::with_capacity(size);

			for _ in 0..128 {
				data1.push(rand::random::<u8>());
			}

			let mut data2 = data1.clone();
			
			function(&mut data1);

			data2.sort_unstable();

			assert_eq!(data1, data2);
		}
	}

	#[test]
	fn merge_sort() {
		test_sort(&merge_sort::sort);
	}

	#[test]
	fn quick_sort() {
		test_sort(&quick_sort::sort);
	}
}