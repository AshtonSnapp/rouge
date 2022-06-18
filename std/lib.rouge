impl bool:
end

impl char:
end

pub enum Option<T>:
	Some(T)
	None
end

impl Option<T>:
end

impl Try for Option<T>:
	type Output = T
	type Residual = Option<!>

	func from_output(T output) Self:
		return Option::Some(output)
	end

	func branch(self) ControlFlow<Self::Output, Self::Residual>:
		match self:
			Option::Some(t) then return ControlFlow::Continue(t)
			Option::None then return ControlFlow::Break(Option::None)
		end
	end
end

pub enum Result<T, E>:
	Ok(T)
	Err(E)
end

impl Result<T, E>:
end

impl Try for Result<T, E>:
	type Output = T
	type Residual = Result<!, E>

	func from_output(T output) Self:
		return Result::Ok(output)
	end

	func branch(self) ControlFlow<Self::Output, Self::Residual>:
		match self:
			Result::Ok(t) then return ControlFlow::Continue(t)
			Result::Err(e) then return ControlFlow::Break(Result::Err(e))
		end
	end
end