impl [T]:
end

impl Index<uint> for [T]:
	type Output = T

	func index(self, uint index) T:
	end
end

impl IndexMut<uint> for [T]:
	func index_mut(mut self, uint index) T:
	end
end

impl [T; const uint N]:
end

impl Index<uint> for [T; const uint N]:
	type Output = T

	func index(self, uint index) T:
	end
end

impl IndexMut<uint> for [T; const uint N]:
	func index_mut(mut self, uint index) T:
	end
end

impl [K: V]:
end

impl Index<K> for [K: V]:
	type Output = V

	func index(self, K index) V:
	end
end

impl IndexMut<K> for [K: V]:
	func index_mut(mut self, K index) V:
	end
end