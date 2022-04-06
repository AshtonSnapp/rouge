func factorial(ulong n) ulong:
	if n == 1:
		return 1
	else:
		return n * factorial(n - 1)
	end
end

pub func main() ubyte:
	var num_text = inl("Give me a positive whole number: ")
	match ulong::from_str(num_text):
		Ok(num) then:
			outl("{}! = {}", num, factorial(num))
			return 0
		end
		Err(_) then:
			errl("{} isn't a positive whole number!", num_text)
			return 1
		end
	end
end