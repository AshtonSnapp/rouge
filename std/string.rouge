impl string:
end

impl Index<uint> for string:
end

impl IndexMut<uint> for string:
end

# convert strings into stuff
pub trait FromString:
	type Error
	func from_str(string str) Result<Self, Self::Error>
end

# convert stuff into strings
pub trait ToString(FromString):
	func to_str(self) Result<string, Self::Error>
end