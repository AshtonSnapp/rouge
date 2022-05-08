# Control Flow

In most programs, you'll want to make decisions on what code to run, or run the same code over and over. These are done using control flow expressions.

## Branching Paths

It's a common thing to only run certain code so long as a given condition is met. To do this, you use an `if` expression. You can probably guess how it works: if a given condition is true, run some code. As an example, let's check if the age from the last chapter is old enough to vote. Assuming your age is at least 18:

```
main>=> age = 21
<=< ()
main>=> if age >= 18:
		... outl!("You can vote!")
		... end
You can vote!
<=< ()
```

> **Notice that `...`?** The REPL will generally recognize things like `if` as starts of multi-line code blocks - the `...` is the secondary prompt (like Bash's PS2 prompt).

Optionally, you can include an `else` expression at the end to execute different code if the condition is false.

```
main>=> age = 17
<=< ()
main>=> if age >= 18:
		... outl!("You can vote!")
		... else
		... outl!("You can't vote.")
		... end
You can't vote.
<=< ()
```

If you want to chain checking conditions, you can use `elif` expressions: they're like `if` expressions in that they take a condition, but they're like `else` expressions in that they go after an existing `if` expression. Note that, if you have `if`, `elif`, _and_ `else` expressions put together, the `else` block _has_ to come last. Generally, the syntax is as follows:

```
if condition:
	# code
elif condition:
	# code
else:
	# code
end
```

One important note is that conditions given to `if` and `elif` expressions must evaluate to booleans. Unlike Ruby, JavaScript, or PHP, Rouge isn't going to try and turn whatever you give it into a boolean value. You have to explicitly give it a boolean somehow, like the `>=` operator which compares two comparable things and returns `true` if the first thing is greater or equal to the second thing, returning `false` otherwise.

### Using `if`/`elif`/`else` in variable assignment.

Because `if`/`elif`/`else` are expressions, they can be used to assign a value to a variable. The only difference is that each branch has to end with a `return` statement to return some value to the variable. All branches have to return the same type of value too - and if you aren't using type inferrence, or the variable was already declared, that type has to match the variable.

## Repitition with Loops

A lot of programs will want to execute some code multiple times in a row. To do this, you use a _loop_ expression. The simplest of these is the `loop` expression - yes, it is literally just the keyword `loop`. The general syntax for a loop is as follows:

```rouge
loop:
	# code
end
```

However, if you create a basic loop like this, you'll probably notice something: it repeats that code, and it doesn't _stop_ repeating. Let's try this out in the REPL:

```
main>=> loop:
		... outl!("One hop this time!")
		... end
One hop this time!
One hop this time!
One hop this time!
```

And it's just going to print 'One hop this time!' until the program is shut down - which you will have to do forcefully using Control+C, or by killing the REPL in your task manager of choice. That's not desirable in most situations, however. To remedy this, the `break` keyword eists to allow you to, well, _break_ out of the loop and continue regular code execution. You'll usually want to place it inside of an `if` expression, since if it always breaks you don't really have a loop. Let's try this again in the REPL:

```
main>=> mut var i = 0
<=< ()
main>=> loop:
		... if i >= 5:
			... break
			... end
		... outl!("One hop this time!")
		... i += 1
		... end
One hop this time!
One hop this time!
One hop this time!
One hop this time!
One hop this time!
<=< ()
```

### `while` and `until` Loops

Since breaking out of a loop when a certain condition is met is extremely common, most programming languages (including Rouge) have an easier way to do this: `while` loops. The name hints as to how it works: while some condition is true, repeat this code. The syntax is nearly identical to that of an `if` expression, but with the `while` keyword. Let's rework our code from before:

```
main>=> i = 0
<=< ()
main>=> while i < 5:
		... outl!("One hop this time!")
		... i += 1
		... end
*output omitted*
<=< ()
```

Rouge also has `until` loops. These are essentially inverse `while` loops: until some condition is true, repeat this code. Syntax is basically identical. One other difference between `while` and `until` is when the condition is checked. A `while` loop checks the condition at the start of each iteration, while an `until` loop checks the condition at the end of each iteration. This means that an `until` loop will always run at least once. An easier way to look at this is by showing them in terms of simple loops:

```rouge
# while loop
loop:
	if not condition:
		break
	end
	# code
end

# until loop
loop:
	# code
	if condition:
		break
	end
end
```

### `for` Loops

There are also situations where you'll want to run some code on every element of a list, or every key-value pair of a map. An easy way to do this is a `for` loop. As an example, let's create a list. Let's say it's a list of teachers' names at a school.

```
main>=> var teachers = ["Campbell", "Harris", "Grey", "Woods"] # [string]
<=< ()
```

Now let's create a for loop that prints out each teacher's name.

```
main>=> for teacher in teachers:
		... outl!("{} works here.", teacher)
		... end
Campbell works here.
Harris works here.
Grey works here.
Woods works here.
<=< ()
```

As another example, let's create a map that uses the name of a student as the key and has their GPA as the value.

```
main>=> var gpas = ["John": 3.2, "Mary": 4.0, "Alice": 2.0, "Bob": 1.2] # [string: flt]
<=< ()
```

Now let's print out each student and their GPA. Let's also say whether they'll need to repeat the school year (which we'll say is any student with a GPA below 2).

```
main>=> for (student, gpa) in gpas:
		... outl!("{} has a GPA of {}.", student, gpa)
		... if gpa < 2.0:
			... outl!("{} will have to repeat the school year.", student)
			... end
		... end
John has a GPA of 3.2.
Mary has a GPA of 4.0.
Alice has a GPA of 2.0.
Bob has a GPA of 1.2.
Bob will have to repeat the school year.
<=< ()
```

Notice that I used parentheses to define two variables in that example: one for the key and one for the value. This is because, when iterating through a map, each iteration yields a tuple of the key and value. What I did here was _destructure_ the tuple - pulling the different fields out of it and making them into independent variables.

[<-prev](3_variables.md) | [next->](5_something.md)