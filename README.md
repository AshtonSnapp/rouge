![Rouge Logo](./rouge.svg)

# Rouge (Work in Progress)

[![Rust Continuous Integration](https://github.com/AshtonSnapp/rouge/actions/workflows/rust.yml/badge.svg)](https://github.com/AshtonSnapp/rouge/actions/workflows/rust.yml)

Rouge (pronounced 'rooj', like the Louisiana state capital of Baton Rouge) is a statically-typed programming language being written in Rust.

Rust is an awesome programming language, with a lot of very useful features. However, there is a steep hill to climb whenever you start learning it. It ends up being a plateau later on - steady progress instead of feeling like an uphill battle - but that initial steep climb is a big turn off for some people trying to learn it. Plus, while Rust generally manages memory automatically in a way that doesn't require a garbage collector, you still have to make a lot of decisions about memory. If you want a type to be able to contain itself, you need to put the child instances inside of `Box`es. If you want multiple ownership instead of single ownership, you need to manually wrap your types in `Rc`, or `Arc` for multithreaded situations. It gets really complicated, and for a lot of applications you really don't need or want to decide all of this yourself. Plus, due to Rust being compiled to native, it really isn't suitable for use as a scripting engine inside of another program.

Rouge aims to do something about both of those issues. By compiling to bytecode which is executed inside of a custom interpreter, it can be more easily embedded into another program and used for scripting, plugins, or configuration. Examples of where Rouge may be useful in this way would be programs like [Neovim](https://neovim.io) or [Awesome](https://awesomewm.org/), or a game engine like [Godot](https://godotengine.org) or [Bevy](https://bevyengine.org). And, since the interpreter would be able to run by itself, it should also be useful for writing standalone programs or scripts. Hopefully I could even write an interactive interpreter, or REPL.

Of course, Rouge won't _just_ do the same things as Rust. To try and make things a bit easier, Rouge plans to adopt [mutable value semantics](https://www.jot.fm/issues/issue_2022_02/article2.pdf) as an alternative to borrow checking - references are not a language-level feature, just an implementation detail. In addition, for a number of reasons that I won't get into right now, Rouge also plans to adopt [algebraic effects](https://overreacted.io/algebraic-effects-for-the-rest-of-us/).

Rouge is planned to use the also work-in-progress [`baton`](https://github.com/AshtonSnapp/baton) runtime, which is essentially designed and named to go with Rouge (though it can be used with other languages).

Rouge is currently licensed under MIT.

For the project's Code of Conflict, please click [here](./CONFLICT.md).

(Current probably-temporary logo derived from ["claw scratch" by Feri Saputra, from Noun Project](https://thenounproject.com/icon/claw-scratch-4766678/). Literally all I did was download it and change the color to a red gradient. I don't have to attribute, since I paid for it thinking it was necessary to be allowed to edit, but not doing so would be a jerk move in my opinion so here we are.)

> **Note:** Rouge is in an extremely early alpha! The embedding and native-function interfaces are NOT STABLE! Hell, the syntax can still change on a dime!

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
pub func main() do
	outl("Bonjour le monde!")
end
```

(I wanted to go with Louisiana Creole since it's an endangered language, but because it is an endangered language I haven't been able to find much in the way of resources I could use to put together a translation.)

### Variables and Builtin Data Types

Rouge is statically typed with type inference, meaning that each variable may be of only one data type BUT the compiler can generally figure out what type it is supposed to be.

If you're new to programming, think of it like this: variables are like buckets, but each bucket can only hold one kind of thing. Sometimes you need to write down what the bucket can hold, but other times the computer can figure it out for you.

The _basic_ types of things your variable buckets can hold are as follows:

| Type         | Description                                                                                                           |
| ------------ | --------------------------------------------------------------------------------------------------------------------- |
| `bool`       | True or false, short for 'boolean'                                                                                    |
| `char`       | UTF-8 scalar, short for 'character'                                                                                   |
| `byte`       | 8-bit unsigned integer, primarily for representing binary data                                                        |
| `nat`        | 64-bit unsigned integer, short for 'natural'                                                                          |
| `int`        | 64-bit signed integer, short for 'integer'                                                                            |
| `flo`        | 64-bit floating-point number, short for 'float'                                                                       |
| `[T]`        | Dynamically-sized list containing items of type `T`                                                                   |
| `[T; N]`     | Statically-sized list containing `N` items of type `T`                                                                |
| `[K: V]`     | Map between values of type `K` to values of type `V`                                                                  |
| `str`        | A 'string' of text, equivalent to `[char]`                                                                            |

To define a variable in Rouge, state its name and value separated by the walrus operator. If you're also specifying the type, split the walrus operator and place the type between the colon and equal sign.

```rouge
name := "Ashton"
age: nat = 22

job := (
	employer: "Banana Incorporated" # this isn't a real company as far as I know, so don't try looking to see if I'm employed there
	title: "Developer"
	annual_salary: 86_215
)
```

If you need to be able to change a variable's value (yes, variables can't be varied by default, I am aware of the irony), add the word `mut` in front of it. `mut` is short for 'mutable', which means "able to be mutated" (or, in normal people terms, "able to be changed").

```rouge
# It makes sense for age to be mutable.
mut age: nat = 22

age += 1 # run this line of code on August 30th :3
```

Alternatively, if you _really_ want to make sure a value never gets changed, you can use the word `const` instead of `mut`. `const` is short for 'constant', and indicates that a variable has to have a value **before your code ever actually runs**, and this value can never be changed.

```rouge
# I doubt I'd ever change my name.
const name := "Ashton"
```

### Control Flow

Anything to do with the control flow is going to have the keyword `do`. But, there's a few different classes of control flow structure.

#### Basic Branching

Basic branching - where you check a condition to decide between two pieces of code to run - is achieved using `if`, `elif`, and `else`. If you're curious, `elif` is shorthand for `else if`. When doing basic branching, there will be exactly one `if` at the start, any number of `elif`s in the middle, and optionally an `else` at the very end. Each `if` or `elif` will take a condition, which will generally be some expression that returns a `bool`. If the condition comes out `true`, that branch's code will be ran. Otherwise, if there is a following `elif`, that branch's condition will be checked, and for an `else`, that branch will be taken. If there is no other branch to take, then we jump out of the branch expression and continue normal execution. Examples:

```rouge
if person.age >= 21 then
	outl("You can drink alcohol.")
elif person.age >= 18 then
	outl("You can vote.")
else
	outl("You can't do anything. :<")
end
```

If you really want to, you can generalize this as follows:

```
if condition_0 then code_0 (elif condition_n then code_n)* (else code_last)?
```

And for a multi-line version, add line breaks and an `end` keyword.

#### Pattern Matching

There's another kind of branching - pattern matching. Here, instead of having a condition for your `if`, you have a check to see if something `matches` something else.

```rouge
if Fs::open("text_file.txt", Mode::Write) matches Ok(mut file) then
	file.write_strl("This is a text file.")
end
```

Of course, pattern matching can get a lot more complicated than that. For one, you can easily switch from matching just one pattern to matching on multiple patterns:

```rouge
if Fs::open("text_file.txt", Mode::Write) matches
	Ok(mut file) then file.write_strl("This is a text file.")
	Err(err) then Exn::throw(err)
end
```

Why would you want to do this when you can just do something like `elif x matches y do`? Well, doing it this way means _you don't have to repeat the same calculation or operation over and over again._ You just do it once.

As for why pattern matching uses `if` as a base, let's look at how Rust handles pattern matching. You have two options: `if let` and `match`:

```rust
if let Ok(mut file) = File::options().write(true).open("text_file.txt") {
	writeln!(&mut file, "This is a text file.")
}

match File::options().write(true).open("text_file.txt") {
	Ok(mut file) => writeln!(&mut file, "This is a text file."),
	Err(err) => Err(err)
}
```

This is kinda terrible though, since you need to do a lot of rewriting if you need to change from checking for one pattern to checking for multiple patterns. With how Rouge does it, you just need to replace the space after `matches` with a newline, and then each pattern you want to check for will be on its own line.

#### Loops

A loop allows you to run the same code over and over and over and - I think you get the point. There are several kinds of loops, though.

The simplest kind of loop is an `always` loop. As you might guess, this is a simple loop that will never end on its own. If you want an `always` loop to end, you will need to manually `break` it, push `Ctrl+C` on your computer, turn it off, or hope the operating system kills your program. So, uh, be careful.

```rouge
always do
	outl("It is time for crazy.")
end
```

The next simplest kinds of loop both feature some sort of conditional check, similar to `if`. These are `while` and `until` loops. The difference? `while` runs its code _while_ the condition is `true`, checking the condition before running any code. Meanwhile, `until` runs its code _until_ the condition is `true`, checking the condition after running any code. So, these `while` and `until` loops:

```rouge
mut i := 10
while i > 0 do
	outl("i = \{i}")
	i -= 1
end

until i >= 10 do
	outl("i = \{i}")
	i += 1
end
```

are equivalent to the following `always` loops:

```rouge
mut i := 10
always do
	if i > 0 then
		outl("i = \{i})
		i -= 1
	else break
end

always do
	outl("i = \{i}")
	i += 1

	if i >= 10 then break
end
```

The last kind of loop is a `for` loop. It is used for looping through each element of something that can be iterated over.

```rouge
friends := ["Chance", "Chase", "John"]

for friend in friends do
	outl("\{friend} is my friend!")
end
```

Because of how iteration is implemented, `for` loops can't really be described in terms of other loops. Instead, they are described in terms of something we haven't gone over yet: an effect handler.

```rouge
friends := ["Chance", "Chase", "John"]

when Yield::yield(friend) do
	outl("\{friend} is my friend!")
in friends.iter()
```

#### Effect Handlers

Effect handlers are hard to truly explain without first explaining effects, so this might initially be confusing. If you want, you can scroll down to where effects are described and then read this afterwards. Effect handlers are structured in one of two ways.

```rouge
when Effect::operation(params) do # you don't have to indicate the effect name if you import its operations, but it is recommended to do so.
	# ... code ...
end
```

This creates an effect handler that handles whatever scope it is inside of. If defined at the start of a function, it handles the specified operation until the function exits. If you want to specify a specific scope for the effect handler, you can do so as follows:

```rouge
when Effect::operation(params) do
	# ... code ...
in
	# ... code ...
end
```

#### Functions

A function is a clump of code that you can run whenever you want. You run it by _calling_ it, passing in any data that it needs and potentially getting data out of it.

The most basic function takes nothing and gives nothing, although that doesn't necessarily mean it _does_ nothing.

```rouge
func do_a_flip() do outl("Do a flip!")
```

A function's inputs, also known as arguments or parameters depending on who you ask, are placed inside of parentheses. You need to specify the names and types of your inputs, because you don't want to put a string into a function asking for numbers.

```rouge
func collatz(n: nat) do
	m := if n %% 2 then n / 2 else (3 * n) + 1
	outl("\{m}")
end
```

If you want a function to also return something, you specify the type of whatever is returned by placing it after an arrow.

```rouge
func collatz(n: nat) -> nat do
	if n %% 2 then n / 2 else (3 * n) + 1
end
```

Note that there is no `return` keyword. While there _is_ one in the language, it is not necessary if the returned value is the last expression.

By the way, while we still haven't really discussed effects, you can mark those on a function by putting them after a `-<`. I got the suggestion from a friend but can't remember what he called the symbol.

```rouge
func load_csv(path: Path) -> [[str]] -< Fs + Read<File> + Exn<io::Error> do
	Fs::open(path)?.lines()
		.map((line) do line.split(',').map((entry) do entry.trim()).collect())
		.collect()
end
```

Functions _can be passed around and stored as if they were data_. Wonder what that `(line) do ...` and `(entry) do ...` expressions are? Those are _closures_, also known as _lambdas_, or more usefully _anonymous functions_. Itty bitty functions that are made when needed and passed around, or even stored in a variable. The general syntax is to place your arguments inside parentheses before the start of a code block - types may be inferred in at least some cases, but you can choose to explicitly type the arguments.