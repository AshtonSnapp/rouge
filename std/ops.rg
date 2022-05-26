# Arithmetic and Logic Operators

# x + y, x += y
pub trait Add<Rhs = Self>:
	type Output
	func add(self, Rhs rhs) Self::Output
	func add_assign(mut self, Rhs rhs)
end

# x + y, x -= y
pub trait Subtract<Rhs = Self>:
	type Output
	func sub(self, Rhs rhs) Self::Output
	func sub_assign(mut self, Rhs rhs)
end

# x * y, x *= y
pub trait Multiply<Rhs = Self>:
	type Output
	func mul(self, Rhs rhs) Self::Output
	func mul_assign(mut self, Rhs rhs)
end

# x / y, x /= y
pub trait Divide<Rhs = Self>:
	type Output
	func div(self, Rhs rhs) Self::Output
	func div_assign(mut self, Rhs rhs)
end

# x % y, x %= y
pub trait Remainder<Rhs = Self>:
	type Output
	func rem(self, Rhs rhs) Self::Output
	func rem_assign(mut self, Rhs rhs)
end

# x & y, x &= y
pub trait BitAnd<Rhs = Self>:
	type Output
	func and(self, Rhs rhs) Self::Output
	func and_assign(mut self, Rhs rhs)
end

# x | y, x |= y
pub trait BitOr<Rhs = Self>:
	type Output
	func or(self, Rhs rhs) Self::Output
	func or_assign(mut self, Rhs rhs)
end

# x ^ y, x ^= y
pub trait BitXor<Rhs = Self>:
	type Output
	func xor(self, Rhs rhs) Self::Output
	func xor_assign(mut self, Rhs rhs)
end

# x << y, x <<= y
pub trait ShiftLeft<Rhs = Self>:
	type Output
	func shl(self, Rhs rhs) Self::Output
	func shl_assign(mut self, Rhs rhs)
end

# x >> y, x >>= y
pub trait ShiftRight<Rhs = Self>:
	type Output
	func shr(self, Rhs rhs) Self::Output
	func shr_assign(mut self, Rhs rhs)
end

# !x
pub trait Not:
	type Output
	func not(self) Self::Output
end

# -x
pub trait Negate:
	type Output
	func neg(self) Self::Output
end

# All Things Ranges

pub enum Bound<T>:
	Included(T)
	Excluded(T)
	Unbounded
end

pub trait RangeBounds<?Sized T>:
	func start_bound(self) Bound<T>

	func stop_bound(self) Bound<T>

	func contains<Comparable T>(self, item) bool:
		return (match self.start_bound():
			Included(start) then start <= item
			Excluded(start) then start < item
			Unbounded then true
		end) and (match self.stop_bound():
			Included(stop) then item <= stop
			Excluded(stop) then item < stop
			Unbounded then true
		end)
	end
end

# x..y
pub struct Range<Idx>:
	pub Idx start
	pub Idx stop
end
impl RangeBounds<T> for Range<T>:
	func start_bound(self) Bound<T>:
		return Bound::Included(self.start)
	end

	func stop_bound(self) Bound<T>:
		return Bound::Excluded(self.stop)
	end
end

# x..
pub struct RangeFrom<Idx>:
	pub Idx start
end

impl RangeBounds<T> for RangeFrom<T>:
	func start_bound(self) Bound<T>:
		return Bound::Included(self.start)
	end

	func stop_bound(self) Bound<T>:
		return Bound::Unbounded
	end
end

# ..
pub struct RangeFull

impl RangeBounds<T> for RangeFull:
	func start_bound(self) Bound<T>:
		return Bound::Unbounded
	end

	func stop_bound(self) Bound<T>:
		return Bound::Unbounded
	end
end

# x..=y
pub struct RangeInclusive<Idx>:
	pub Idx start
	pub Idx stop
end

impl RangeBounds<T> for RangeInclusive<T>:
	func start_bound(self) Bound<T>:
		return Bound::Included(self.start)
	end

	func stop_bound(self) Bound<T>:
		return Bound::Included(self.stop)
	end
end

# ..y
pub struct RangeTo<Idx>:
	pub Idx stop
end

impl RangeBounds<T> for RangeTo<T>:
	func start_bound(self) Bound<T>:
		return Bound::Unbounded
	end

	func stop_bound(self) Bound<T>:
		return Bound::Excluded(self.stop)
	end
end

# ..=y
pub struct RangeToInclusive<Idx>:
	pub Idx stop
end

impl RangeBounds<T> for RangeToInclusive<T>:
	func start_bound(self) Bound<T>:
		return Bound::Unbounded
	end

	func stop_bound(self) Bound<T>:
		return Bound::Included(self.stop)
	end
end

# Error Propagation

pub enum ControlFlow<C = (), B>:
	Continue(C)
	Break(B)
end

# error propagation operator ? and try_* methods require something that implements this trait
pub trait Try:
	type Output
	type Residual

	func from_output(Self::Output output) Self

	func branch(self) ControlFlow<Self::Output, Self::Residual>
end

# Utility Traits

# x[y]
pub trait Index<?Sized Idx>:
	type ?Sized Output

	func index(self, Idx index) Self::Output
end

# x[y] but mutable
pub trait IndexMut(Index)<?Sized Idx>:
	func index_mut(mut self, Idx index) Self::Output
end

# functions
pub trait Func(FuncMut)<Args>:
	func call(self, args: Args) Self::Output
end

# functions that mutate stuff
pub trait FuncMut<Args>:
	type Output

	func call_mut(mut self, args: Args) Self::Output
end

# Custom destructors
pub trait Drop:
	func drop(mut self)
end