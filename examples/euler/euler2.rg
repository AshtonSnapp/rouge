# https://projecteuler.net/problem=2

func fibonacci(uint one, uint two, uint max) List<uint>:
	mut var sequence = List::new()

	sequence.push(one)
	sequence.push(two)

	loop:
		var next = sequence[-1] + sequence[-2]
		if next > max:
			break
		else:
			sequence.push(next)
		end
	end

	return sequence
end

pub func main():
	outl!("{}", fibonacci(1, 2, 4000000).iter.filter(|x| x % 2 == 0).sum())
ends