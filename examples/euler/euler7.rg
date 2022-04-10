# https://projecteuler.net/problem=7

func xPrimes(uint x) List<uint>:
	mut var primes = List::new()
	primes.push(2)
	mut uint y = 2

	`outer loop:
		y += 1
		for prime in primes:
			if y % prime == 0:
				continue outer
			end
		end
		primes.push(y)
		if primes.len() == x + 1:
			break
		end
	end

	return primes
end

pub func main():
	outl!("{}", xPrimes(10001)[-1])
end