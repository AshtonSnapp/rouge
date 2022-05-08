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

pub struct Range<Idx>:
	pub Idx start
	pub Idx stop
end

pub struct RangeFrom<Idx>:
	pub Idx start
end

pub struct RangeFull

pub struct RangeInclusive<Idx>:
	pub Idx start
	pub Idx stop
end

pub struct RangeTo<Idx>:
	pub Idx stop
end

pub struct RangeToInclusive<Idx>:
	pub Idx stop
end

# Utility Traits

pub trait Func(FuncMut)<Args>:
	func call(self, args: Args) Self::Output
end

pub trait FuncMut<Args>:
	type Output

	func call_mut(mut self, args: Args) Self::Output
end

pub trait Drop:
	func drop(mut self)
end