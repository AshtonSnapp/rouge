enum Status:
	Healthy
	Unhealthy(List<String>)
	Dead(String)
end

struct Person:
	String name
	uint age
	Status state
end

impl Person:
	pub func new(String name, uint age) Person:
		return Person { name, age, state: Status::Healthy }
	end
end