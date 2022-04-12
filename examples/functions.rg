func factorial(uint n) uint:
	if n == 1:
		return 1
	else:
		return n * factorial(n - 1)
	end
end

pub func main() ubyte:
	match uint::from_str(prompt!("Give me a positive whole number: ")):
		Ok(num) then:
			outl!("{}! = {}", num, factorial(num))
			return 0
		end
		Err(_) then:
			errl!("{} isn't a positive whole number!", num)
			return 1
		end
	end
end