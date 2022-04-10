# https://projecteuler.net/problem=1

func sumMultiplesBelow(&[uint] factors, uint max) uint:
	mut var sum = 0

	for i in 0..=max:
		for factor in factors:
			if i % factor == 0:
				sum += i
				break
			end
		end
	end

	return sum
end

pub func main():
	outl!("{}", sumMultiplesBelow([3, 5], 1000))
end