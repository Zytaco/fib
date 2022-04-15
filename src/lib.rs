use std::ops::Add;

pub trait Fibonacci: Clone + Add<Output = Self> {
	fn decrement(self) -> Self;
	fn is_zero(&self) -> bool;
	fn zero() -> Self;
	fn one() -> Self;
}

pub fn fib<T: Fibonacci>(n: T) -> T {
	fn recursive<U: Fibonacci>(n: U, at_n_plus_one: U, at_n: U) -> U {
		if n.is_zero() {
			at_n_plus_one
		} else {
			recursive(
				Fibonacci::decrement(n),
				at_n + at_n_plus_one.clone(),
				at_n_plus_one
			)
		}
	}
	recursive(
		n,
		Fibonacci::zero(),
		Fibonacci::one()
	)
}

pub mod implementations {
	use super::Fibonacci;
	impl Fibonacci for i8 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for u8 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}	
	impl Fibonacci for i16 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for u16 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}	
	impl Fibonacci for i32 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for u32 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}	
	impl Fibonacci for i64 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for u64 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}	
	impl Fibonacci for i128 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for u128 {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for isize {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}
	impl Fibonacci for usize {
		fn decrement(self) -> Self { self - 1 }
		fn is_zero(&self) -> bool { *self == 0 }
		fn zero() -> Self { 0 }
		fn one() -> Self { 1 }
	}	
}

#[cfg(test)]
mod tests {
	use super::fib;

	#[test]
	fn zero_test() {
		assert_eq!(fib(0), 0);
	}

	#[test]
	fn one_test() {
		assert_eq!(fib(1), 1);
	}

	#[test]
	fn sequence_test() {
		let answers = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811];
		assert_eq!((0i32 ..= 28).map(|n| fib(n)).collect::<Vec<i32>>(), answers);
	}
}
