# Rouge

Rouge (pronounced 'rooj', like the Louisiana state capital of Baton Rouge) is a statically-typed programming language designed for two primary uses: applications (graphical and command-line), and embedding into native programs (plugins, config files). To be suitable for both use cases, Rouge aims to have the following feature set:

 - A memory management model that aims to be intuitive and fast but with at least some guarantees towards memory and thread safety.
	- Current thoughts: reference counting with copy-on-write
 - A (mostly) simple syntax. Originally it was based on a combination of Ruby, Python, and Rust, but it evolved over time.
 - Interpreted for development and use in config files, bytecode-compiled for distribution.
 - Extreme flexibility provided to the programmer.

The custom runtime environment (RTE) for the Rouge programming language will be provided as a Rust library (and someone can probably work on making a wrapping cdylib for interfacing with other languages) for embedding, and as a standalone utility for applications. Both will simply be called `rouge` and contain everything necessary to run and compile Rouge code.

Rouge is currently licensed under MIT.

For the project's Code of Conflict, please click [here](./CONFLICT.md).

## Current TODO List

### High Priority

 - [ ] Create a functioning RTE and compiler.
	- [ ] Design the RTE's instruction set and the bytecode file format.
	- [ ] Design an intuitive interface for communication between a native program and an embedded Rouge runtime.
	- [ ] Design a read-eval-print-loop (REPL), or interactive session, for the RTE.
 - [ ] Create the standard library.
 - [ ] Create documentation.
	- [ ] (Optional) Specifications (at least semi-formal) for the language and related things.
 - [ ] Create a functioning command-line utility.

### Medium Priority

 - [ ] Create a toolchain (e.g. dependency/project manager, doctool, language server)

### Low Priority

 - [ ] Branding.

## A Quick Tour of What's to Come

Rouge isn't really implemented yet, and a lot of things are probably going to change (especially if people other than me start contributing to the project), but I'll try and give some examples that give a general feel of what the language should be like.

### "Hello, world!" but in French

```rouge
pub func main() do:
	outl!("Bonjour le monde!")
end
```

(I wanted to go with Louisiana Creole since it's an endangered language, but because it is an endangered language I haven't been able to find much in the way of resources I could use to put together a translation.)

### Variables and Primitive Data Types

```rouge
pub func main() do:
	# An `int` can contain a positive or negative whole number.
	int year = 2022
	
	# A `nat` can only contain a positive whole number.
	nat day = 252
	
	# A `float` can contain a whole or non-whole number.
	float hour = 11.916

	# A `byte` is similar to a `nat`, except it is limited to values in the range of 0 to 255. It is meant for representing binary data - bytes.
	byte meaning_of_life = b'*'

	# A `bool` holds a true or false value.
	bool is_finished = false

	# A `char` holds any single Unicode scalar value. This may not exactly match your view of what a character is at any given time - some things that appear as a single character are actually multiple. It's weird.
	char english_favorite_vowel = 'ə'

	# A `tuple` holds some set of related information.
	(float, float) position = (1.0, 2.0)

	# Tuple members are accessed using dot syntax with a number starting from 0.
	outl!("{}, {}", position.0, position.1)

	# The elements of a tuple don't have to be of the same type.
	(byte, char) byte_and_char = (42, '*')

	# A `list` contains some unspecified number of some type of item.
	[nat] fibonacci = [1, 1, 2, 3, 5, 8, 13, 21, 34]

	# List elements are accessed using bracket syntax with a number representing the index into the list. The index starts from 0.
	outl!("Element {} of the fibonacci sequence is {}.", 7, fibonacci[6])

	# A `string` is just a list of characters.
	string capital = "Baton Rouge"

	# A list of bytes can be represented with a byte string.
	[byte] linux_binary_magic = b"ELF"

	# A `map` allows you to use one type to get another type. The first type is called the key, the second type is the value.
	[string: float] classes = ["English 101": 3.0, "Calculus 101": 2.5, "Computer Science 101": 4.0]

	# Map entries are accessed using bracket syntax with the key you want to get the value of.
	outl!("Your GPA in Computer Science 101 is {}", classes["Computer Science 101"])

	# The `mut` keyword goes before a variable's type to specify that changing (or mutating) the variable's value in your code is allowed.
	# The `var` keyword is used to indicate that the type of the variable should be inferred.
	mut var my_age = 22
	my_age += 1 # this is allowed
	day += 2 # this is not allowed, day is not declared as mutable
end
```

### Control Flow

```rouge
pub func main() do:
	var name = prompt!("What's your name? ")

	# Simple control flow using `if`, `elif` (else if), and `else`.
	if name == "Rouge" then:
		outl!("Hey, that's MY name!")
	elif name == "Ashton" then:
		outl!("Isn't that the name of the guy who created me?")
	else outl!("Nice to meet you, {}.", name)

	var num = prompt!("What's your favorite number? ")

	# Using the `is` keyword, you can check if whatever's on the left matches some pattern on the right. Variables will be bound if possible.
	if num.parse::<nat>() is Ok(n) then:
		# You can also do this using `is` - it's like a Rust `match` block. Using it like this means you have to handle any possible case - hence the else branch.
		if n is:
			42 then outl!("I see you're a fan of Douglas Adams. Did you bring a towel?")
			0..=9 then outl!("Single digit club, huh?")
			100.. then outl!("I mean, who doesn't like big numbers?")
			else outl!("Double digit club, let's goooooooooo")
		end
	else errl!("I don't think that was a number, so we'll just skip over this.")

	mut nat count = 5

	# The `loop` keyword creates an infinite loop. It will continue running forever, unless you stop it yourself or add code to break out of the loop.
	loop:
		outl!("This will print forever!")
		count -= 1
		if count == 0 then:
			outl!("Okay forever sounds boring, let's stop.")
			break
		end
	end

	# `while` will loop while some condition is true.
	count = 10
	while count != 0 do:
		outl!("One hop this time!")
		count -= 1
	end

	# It's equivalent to the following simple loop:
	count = 10
	loop:
		if not count != 0 then break # `if count == 0 then break` would be more concise, but this line is more clear as to how `while` works.
		outl!("One hop this time!")
		count -= 1
	end

	# `until` is like while, but it loops until some condition is true
	count = 10
	until count == 0 do:
		outl!("!emit siht poh enO") # "One hop this time!" but reversed
		count -= 1
	end

	# It's equivalent to the following simple loop:
	count = 10
	loop:
		outl!("!emit siht poh enO")
		count -= 1
		if count == 0 then break
	end

	# A `for` loop is used for looping through the members of some collection. The `while` loop above can be simplified to the following one-line `for` loop.
	for _ in 0..10 do outl!("One hop this time!")

	# To be more clear, the above `for` loop is exactly equivalent to the following `while` loop:
	mut var range = 0..10
	while range.next() is Some(_) do outl!("One hop this time!")

	# And is therefore equivalent to the following simple loop:
	range = 0..10
	loop:
		if range.next() is Some(_) then outl!("One hop this time!")
		else break
	end
end
```

### Functions

```rouge
# The most basic function takes no arguments and returns nothing.
func do_something() do outl!("Did something!")

# Sometimes you want to pass data into a function. For this, you need to specify what arguments you want.
func double_it(float number) do:
	float doubled = number * 2
	outl!("{} doubled is {}", number, doubled)
end

# And you'll often want your functions to give you some data. So you'll need to specify the type of data that the function returns.
# The `return` keyword is used to return data from a function.
func multiply_case(int case, float number) float do:
	if case is:
		0..10 then return number * 2
		10..100 then return number * 3
		100..1_000 then return number * 4
		1_000..10_000 then return number * 5
		10_000..100_000 then return number * 6
		else return number * 7
	end
end

# Functions can call themselves. This is called recursion.
func factorial(int number) int do:
	if number == 2 then return number # optimization: short-circuiting the base case, look it up on the Wikipedia page for recursion

	return number * factorial(number - 1)
end

# Any function named 'main' is considered an entrypoint function. The standalone version of the Rouge runtime expects this function to have no arguments, and return either nothing, an `int`, or a class that implements `Try`.
pub func main() do:
	do_something()
	double_it(42.0)
	outl!("{}", multiply_case(999, 6.9))
	outl!("{}! = {}", 8, factorial(8))

	# A closure is an anonymous, unnamed function (usually called a lambda in other languages) that can use variables from the environment it was defined in.
	mut nat test_num = 64
	var closure = func() do:
		nat old_test_num = test_num
		test_num *= 2
		outl!("{}", old_test_num)
	end

	closure()
	closure()
end
```

### Classes

```rouge
# There are a few different kinds of classes.

# Empty, or unit, classes contain no data. They are useful in situations where you want to implement some common behavior but don't have any data you need to deal with in that implementation.
class Empty

# Unstructured, or tuple, classes contain unnamed data. They essentially act as named tuples.
class Point2D is (float, float)

# Structured, or normal, classes contain multiple named data fields.
class Person is:
	string name,
	nat age,
end

# Enumerated classes contain multiple variants. Each variant may be empty, or contain unstructured or structured data. They are defined using the `enum` keyword instead of the `class` keyword.
enum Status is:
	Healthy,
	Unhealthy:
		[string] diseases,
		[string] wounds,
	end,
	Down(string),
end

pub func main() do:
	# This is one way you can instantiate a normal class.
	var me = Person:
		name = "Ashton",
		age = 22,
	end

	# Here's another way: (This is the only place curly braces are ever used)
	var ranodm_dude = Person { name = "Bob", age = 33 }

	# Members of a class are accessed using dot syntax with the name of the member.
	outl!("{} is {} years old", me.name, me.age)

	# Tuple classes are instantiated like a tuple, but with the name of the class prefixed.
	var pos = Point2D(42.0, 6.9)

	# I think you can figure out how members of tuple classes are accessed...
	outl!("({}, {})", pos.0, pos.1)

	# Unit classes are instantiated by writing the name of the class. Simple, really.
	var blankity = Empty

	# Variants of enumerated classes are accessed using double-colon syntax.
	mut var state = Status::Healthy
end
```

### Implementing on Classes

```rouge
class Person is:
	string name,
	nat age,
end

# Items can be associated with a class by placing them inside an implementation block, like this.
impl for Person is:
	# An associated function like this is usually used as a constructor. It can be more complicated for certain classes.
	pub func new(string name, nat age) Person do return Person { name, age }

	# An associated function with a `self` argument is called a method, and operates on a specific instance of the class.
	# They are called on an instance using dot syntax, as if they were a member.
	pub func name(self) string do return self.name

	pub func age(self) nat do return self.age

	# Some methods can change an instance's data. This is explicitly marked, and these methods can't be called on immutable (unchanging) instances.
	pub func birthday(mut self) do self.age += 1
end

pub func main():
	# Calling an associated function, such as our constructor, is done using double-colon syntax.
	var me = Person::new("Ashton", 22)

	# Calling a method on a class is done using dot syntax.
	outl!("{} is {} years old.", me.name, me.age)

	# The following line is commented out as it would fail, because I defined `me` as an immutable value.
	#me.birthay()

	mut var you = Person::new("Your Name Here", 42)
	# Now we can run the birthday method without raising any errors.
	you.birthday()
end
```

### Inheritance

```rouge
class Vec2 is (float, float)

# Inheritance is done by putting the name of the class(es) being inherited from (the super-class) in parentheses before the `is` keyword, usually connected to the class name.
# When inheriting from a tuple class, ONLY the additional members of the class have their types declared.
class Vec3(Vec2) is (float)

class Person is:
	string name,
	nat age,
end

impl for Person is:
	# -- snip --
	# Pretend the impl block from the previous example is here. I don't feel like re-writing it.
end

# Similarly, when inheriting from a normal class, you only need to declare the additional class members.
class Student(Person) is:
	float gpa,
	[string: float] classes,
end

# By default, only data (members of most classes, variants of enumerated classes) is inherited. Implemented items can be inherited using `from`.
impl for Student from Person is:
	pub func name(self) string

	pub func age(self) nat

	pub func birthday(mut self)
end

impl for Student is:
	pub func new(string name, nat age, [string: float] classes) Student do:
		mut float grade_sum = 0.0
		for (_, grade) in classes do grade_sum += grade
		return Student:
			name,
			age,
			gpa = grade_sum / classes.len(),
			classes
		end
	end

	pub func gpa(self) float do return self.gpa

	pub func get_grade(self, string class_name) float do return self.classes[class_name]

	pub func set_grade(mut self, string class_name, float grade) do:
		self.classes.insert(class_name, grade)
		self.calculate_gpa()
	end

	func calculate_gpa(mut self) float do:
		mut float sum = 0.0
		for (_, grade) in self.classes do sum += grade
		self.gpa = sum / self.classes.len()
	end
end

# This function obviously won't take an instance of Person.
func determine_eligibility(Student student) bool do return student.gpa() >= 2.5

# This function won't take a Student though. This is necessary due to the fact that only data is inherited by default - you can't rely on a class having all the behavior of a class it inherits from.
func can_vote(Person person) bool do person.age() >= 18

# One option for remedying this is function overloading.
func can_vote(Student person) bool do person.age() >= 18

pub func main() do:
	mut var person = Person::new("Ashton", 22)

	mut var student = Student::new("Alice", 17, ["ENG101": 3.2, "MATH101": 3.5, "CS101": 4.0])

	outl!("{} can vote: {}", person.name(), can_vote(person))
	
	outl!("{} can vote: {}", student.name(), can_vote(student))
	
	outl!("{} has scholarship: {}", student.name(), determine_eligibility(student))

	# This line will fail, because there isn't a version of determine_eligibility() that can take a Person argument.
	#outl!("{} has scholarship: {}", person.name(), determine_eligibility(person))
end
```

### Generics and Traits

```rouge
# Generics are a way to automatically generate multiple versions of a function.
# It's essentially a placeholder type. Here, T is a generic. Generics are declared between < angle brackets >.
func sum<T>([T] items) T do:
	# Within the context of the function, T is used as if it was any other type.
	mut T sum = 0
	# When you try to run this code, it will make sure that whatever type you try to use can be added.
	for item in items do sum += item
	return sum
end

# You can also use generics to create classes that can contain any other type.
pub class Vec2<T> is (T, T)

# A trait is a way to implement shared behavior on multiple types or classes.
# An example is the Add trait, used to implement addition.
impl Add for Vec2<T> is:
	func add(self, Vec2<T> other) Vec2<T> do:
		return Vec2(self.0 + other.0, self.1 + other.1)
	end
end

pub func main() do:
	[Vec2<float>] list = [Vec2(1.0, 2.0), Vec2(3.5, 6.9), Vec2(10.4, 42.0)]

	outl!("{}", sum(list))
end
```