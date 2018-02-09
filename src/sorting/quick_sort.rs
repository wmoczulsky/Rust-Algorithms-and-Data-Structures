use std::mem::swap;

pub fn sort<T: PartialOrd + Clone>(data: &mut [T]) {

	if data.len() <= 1 {
		return;
	}

	let len = data.len();

	let p = good_pivot_idx(data);
	data.swap(p, len - 1);

	let mut cur = 0;
	
	for i in 0..len-1 {
		if data[i] < data[len - 1] {
			data.swap(i, cur);
			cur += 1;
		}
	}

	data.swap(cur, len - 1);

	sort(&mut data[0 .. cur]);
	sort(&mut data[cur + 1 .. len]);
}

fn good_pivot_idx<T: Clone>(data: &[T]) -> usize {
	data.len() / 2
}

//TODO HORE
