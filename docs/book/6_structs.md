# Structs

In programming, you'll often find yourself using multiple different kinds of data to represent pieces of one larger bit of data. Rather than passing these values around individually, which can be error prone, you can bundle them together into a _struct_, short for structure.

A struct is defined using the `struct` keyword, as you may expect. When defining a struct, you specify the names and types of its different _fields_. For example, a struct that contains information about a student at a school may look like this:

```rouge
struct Student:
	string name
	flt gpa
	uint year
	[string] classes
end
```

That struct definition itself doesn't do much though. It's just a blueprint of sorts - you need to create an _instance_ of the struct based on that blueprint. This is done with curly brace syntax, as follows:

```rouge
var bob = Student {
	name: "Bob Baker"
	gpa: 1.2
	year: 10
	classes: [
		"English",
		"Algebra",
		"World Geography",
		"Physical Science",
		"PE",
		"Spanish",
		"Computer Science"
	]
}
```

You can access the different fields of the struct using the dot syntax, like with tuples but using the name of the field: `bob.gpa`. Also, when the instance is mutable, any of the fields can be modified.

```rouge
mut var alice = Student {
	name: "Alice Anderson"
	gpa: 2.0
	year: 11
	classes: [
		"English",
		"Algebra",
		"World History",
		"Physics",
		"Spanish",
		"PE",
		"Art"
	]
}

alice.gpa = 2.5
```

## Struct Update

Sometimes you'll want to create a new instance of a struct that's slightly modified from an existing instance. You can do this using the struct update syntax, as follows:

```rouge
mut var mary = Student {
	name: "Mary Miller"
	gpa: 4.0
	..alice
}
```

## Tuple Structs

You can also create structs that work like tuples - the fields don't have names, but they are typed. This is useful for when you have a tuple with a specific meaning attached, or if naming the fields would be verbose or redundant.

```rouge
struct Color(ubyte, ubyte, ubyte)
```

These are then instantiated like a tuple with a word attached to the opening parentheses:

```rouge
var lime = Color(0x55, 0xff, 0x00)
```

## Unit Structs

One kind of struct that has its uses, even though it seems like it shouldn't, is unit structs. These are structs that hold - guess what - no data! They're basically the unit type `()` but with a special name, and they can have methods defined on them which we will discuss shortly.

```rouge
struct Thing

# ...

var item = Thing
```

## Methods

_Methods_ are essentially functions defined on a struct. Their first argument is always `self`, which refers to the particular instance of the struct that the function was called on using the dot syntax. There are two main ways of creating methods: either within an `impl` (short for _implementation_) block, or standalone. You'll usually want to use `impl` so we'll discuss that first.

An `impl` block associates whatever it contains with a particular type. This is generally used for methods, but can also be used for constants - for example, the primitive `flt` and `dbl` types both have associated constants for `PI`, `TAU`, and `E` among others. The general `impl` syntax is as follows:

```rouge
impl Type:
	# items
end
```

Inside of this block, you can define your methods. For example, let's take our `Color` struct and define a method that returns the color as a single number.

```rouge
impl Color:
	pub func as_num(self) uword:
		return ((self.0 << 16) | (self.1 << 8) | (self.2)) as uword
	end
end
```

You can also define _associated functions_, which are like methods but they don't have self as an argument. They are commonly used for constructors - functions which are used to more easily create an instance of a given type.

```rouge
impl Color:
	pub func new(ubyte red, ubyte green, ubyte blue) Color:
		return Color(red, green, blue)
	end
end
```

Two final things to notice: first, a given type can always have new functions implemented on it. If you've ever used Ruby before, you know that classes in that languages are open - meaning they can always be modified. In Rouge, types are semi-open - you can always add new associated functions, constants, and methods, but you can't modify the fields. Note that if there are multiple conflicting implementations of a given function or constant on a given type, the runtime or compiler will produce an error.

Second, you don't necessarily need an `impl` block to define a function on a type. You can also write a function normally, but prepend the function's name with the type and two colons like so:

```rouge
func Type::function(args) return:
	# code
end
```

[<-prev](5_functions.md) | [next->](7_enums.md)