pub fn sort<T : PartialOrd + Clone>(data: &mut [T]) {
	let mut buf = Vec::with_capacity(data.len());

	sort_internal(data, &mut buf);
}

fn sort_internal<T: PartialOrd + Clone>(data: &mut [T], buf: &mut Vec<T>){;

	if data.len() <= 1 {
		return;
	}

	let len = data.len();

	sort_internal(&mut data[0 .. len / 2], buf);
	sort_internal(&mut data[len / 2 .. len], buf);

	merge(data[0 .. len / 2].iter().peekable(), data[len / 2 .. len].iter().peekable(), buf);

	data.clone_from_slice(&buf[0 .. len]);
}

use std::iter::Peekable;
use std::slice::Iter;
fn merge<T: PartialOrd + Clone>(mut a: Peekable<Iter<T>>, mut b: Peekable<Iter<T>>, buf: &mut Vec<T>) {
	buf.clear();

	loop {
		match (a.peek(), b.peek()) {
			(Some(&x), Some(&y)) if (x <= y) => { buf.push(x.clone()); a.next(); },
			(Some(&x), None) => { buf.push(x.clone()); a.next(); },
			(_, Some(&y)) => { buf.push(y.clone()); b.next(); },
			(None, None) => return,
		}
	}
}
