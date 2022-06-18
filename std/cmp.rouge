# x == y, x != y
pub trait Equatable:
	# equal ==
	func eq(self, Self rhs) bool

	# not equal !=
	func ne(self, Self rhs) bool:
		return not self.eq(rhs)
	end
end

pub enum Ordering:
	# LHS < RHS
	Less,
	# LHS = RHS
	Equal,
	# LHS > RHS
	Greater
end

# x < y, x > y, x <= y, x >= y
pub trait Comparable(Equatable):
	func cmp(self, Self rhs) Ordering

	# less than <
	func lt(self, Self rhs) bool:
		if self.cmp(rhs) is Ordering::Less:
			return true
		else:
			return false
		end
	end

	# greater than >
	func gt(self, Self rhs) bool:
		if self.cmp(rhs) is Ordering::Greater:
			return true
		else:
			return false
		end
	end

	# less than or equal to <=
	func le(self, Self rhs) bool:
		if self.cmp(rhs) is not Ordering::Greater:
			return true
		else:
			return false
		end
	end

	# greater than or equal to >=
	func ge(self, Self rhs) bool:
		if self.cmp(rhs) is not Ordering::Less:
			return true
		else:
			return false
		end
	end
end