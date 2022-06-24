# Functions

A lot of the time you'll have some operation that you're doing a lot. I'm not talking about sequentially as that can be handled in a loop. More like you find yourself doing the same thing at different times but with different values and different. In programming, it's generally bad practice to repeat yourself as this makes your code more prone to errors and bugs. The solution is to take that code you're repeating and group it together into a _function_. Functions in Rouge are declared using the `func` keyword, like so:

```rouge
func name(args) return:
	# code
end
```

The first line is the _function signature_. This tells the runtime:

 - what the function is called,
 - what data the function needs,
 - and what data the function returns

For example, we might want a function that adds a bunch of numbers together and returns whether those numbers meet or exceed some threshold. In fact, let's make this function in the REPL:

```
main>=> func sum_threshold([uint] nums, uint threshold) bool:
sum_threshold>=> mut uint sum = 0
sum_threshold>=> for num in nums:
		... sum += num
		... end
sum_threshold>=> return sum >= threshold
sum_threshold>=> end
<=< func([uint], uint) bool
main>=> [uint] nums = [21, 42, 84]
<=< ()
main>=> sum_threshold(nums, 128)
<=< true
main>=> sum_threshold(nums, 160)
<=< false
```

> **Wait, why did the prompt change?** The main prompt includes the name of the function you are currently in. `main` is a function too, just a special one. When you start writing a new function, the prompt changes to indicate that you are in a different function. The REPL will only automatically run your code if it is in the `main` function, so keep that in mind.

## Recursion

Let's say that, for one reason or another, you want to compute factorials. One interesting thing about factorials, is that 5! = 5 * 4!. And 4! = 4 * 3!. And so on. So, following that logic, you can implement factorials as follows:

```rouge
func factorial(uint num) uint:
	if num <= 1 then:
		return 1
	else:
		return num * factorial(num - 1)
	end
end
```

_Recursion_ is whenever you have a function that calls itself, and it can be very useful. You do have to be careful with it, however: just like with a loop, if you don't have a condition, it will call itself over and over and over again. And then the runtime (or your computer) will crash because you ran out of memory. Like with all tools, use responsibly!

## Variadic Functions

Sometimes you want a function to be able to take an arbitrary number of arguments. You can't really do that with individual specifying argument names. However, Rouge supports _variadic functions_. These are functions where you specify the final argument as `...Type` where Type is just the name of the type of argument. Like so:

```rouge
func something(...string):
	# code
end
```

All of the extra arguments at the end (or in this case, all of the arguments) are collected into a list with the given type called `vargs`. This has several uses. For example, we can redo our `sum_threshold()` function from earlier:

```rouge
func sum_threshold(uint threshold, ...uint) bool:
	mut uint sum = 0

	for varg in vargs:
		sum += varg
	end

	return sum >= threshold
end
```

And then, instead of having to put all of our numbers into a list before calling the function, we can just list them out as separate arguments:

```rouge
sum_threshold(25, 1, 2, 4, 8, 16)
```

> **Why use variadic functions?** Certain kinds of operations can be more naturally implemented as variadic functions, such as summing numbers and concatenating strings. Also, using variadic functions means you don't have to create list variables that are only used for a single function call.
>
> On the lower level of things, variadic functions work by having the runtime collect the arguments into a list which the function is given a reference to. This does mean a slight performance hit as this collection process has to occur, but it also means the list will likely be wiped from the heap as the reference is invalidated when the function returns.
>
> In short, whether you use variadic functions depends entirely on what's more important to your use case: performance, or memory efficiency.

[<-prev](4_flow.md) | [next->](6_structs.md)