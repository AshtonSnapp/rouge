# https://projecteuler.net/problem=2

func fibonacci(uint one, uint two, uint max) [uint]:
	mut var sequence = [one, two]

	loop:
		var next = sequence[-1] + sequence[-2]
		if next > max:
			break
		else:
			sequence += next
		end
	end

	return sequence
end

pub func main():
	outl!("{}", fibonacci(1, 2, 4000000).iter.filter(|x| x % 2 == 0).sum())
ends