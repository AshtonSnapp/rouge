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
	outln("Bonjour le monde!")
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
| `num`        | Arbitrary number that changes under the hood.                                                                         |
| `nat`        | Arbitrary unsigned whole number that changes size under the hood.                                                     |
| `natX`       | Unsigned whole number that is specifically `X` bits long (`X` can be 8, 16, 32, or 64)                                |
| `int`        | Arbitrary signed whole number that changes size under the hood.                                                       |
| `intX`       | Signed whole number that is specifically `X` bits long (`X` can be 8, 16, 32, or 64)                                  |
| `flo`        | Arbitrary floating-point number that changes size under the hood.                                                     |
| `floX`       | Floating-point number that is specifically `X` bits long (`X` can be 32 or 64)                                        |
| `[T]`        | Dynamically-sized list containing items of type `T`                                                                   |
| `[T; N]`     | Statically-sized list containing `N` items of type `T`                                                                |
| `[K: V]`     | Map between values of type `K` to values of type `V`                                                                  |
| `str`        | A 'string' of text, equivalent to `[char]`                                                                            |
| `any`        | Opt-out of static typing, can represent any value and can take trait bounds                                           |

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
	outln("You can drink alcohol.")
elif person.age >= 18 then
	outln("You can vote.")
else
	outln("You can't do anything. :<")
end
```

If you really want to, you can generalize this as follows:

```
if condition_0 then code_0 (elif condition_n then code_n)* (else code_last)?
```

And for a multi-line version, add line breaks and an `end` keyword at the end.

#### Pattern Matching

There's another kind of branching - pattern matching. Here, instead of having a condition for your `if`, you have a check to see if something `matches` something else.

```rouge
if Fs::open("text_file.txt", Mode::Write) matches Ok(mut file) then
	perform file.write_str("This is a text file.\n")
end
```

Of course, pattern matching can get a lot more complicated than that. For one, you can easily switch from matching just one pattern to matching on multiple patterns:

```rouge
if Fs::open("text_file.txt", Mode::Write) matches
	Ok(mut file) then perform file.write_strln("This is a text file.")
	Err(err) then perform throw(err)
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
	outln("It is time for crazy.")
end
```

The next simplest kinds of loop both feature some sort of conditional check, similar to `if`. These are `while` and `do while` loops. The difference? `while` runs its code _while_ the condition is `true`, checking the condition before running any code. Meanwhile, `do while` checks the condition after running any code. So, these `while` and `do while` loops:

```rouge
mut i := 10
while i > 0 do
	outln("i = \{i}")
	i -= 1
end

do
	outln("i = \{i}")
	i += 1
while i < 10 end
```

are equivalent to the following `always` loops:

```rouge
mut i := 10
always do
	if i > 0 then
		outln("i = \{i})
		i -= 1
	else break
end

always do
	outln("i = \{i}")
	i += 1

	if i >= 10 then break
end
```

The last kind of loop is a `for` loop. It is used for looping through each element of something that can be iterated over.

```rouge
friends := ["Chance", "Chase", "John"]

for friend in friends do
	outln("\{friend} is my friend!")
end
```

 > **Note:** At the moment, I am thinking about how how iterators (and therefore `for` loops) work.

#### Effect Handlers

Effect handlers are hard to truly explain without first explaining effects, so this might initially be confusing. If you want, you can scroll down to where effects are described and then read this afterwards. Effect handlers are structured in one of two ways.

```rouge
when Effect::operation(params) do # you don't have to indicate the effect name if you also make use of the effect in your function, but it is recommended to specify it anyways.
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
...or:
```rouge
with
    # ... code ...
when Effect::operation(params) do
    # ... code ...
end
```

#### Functions

A function is a clump of code that you can run whenever you want. You run it by _calling_ it, passing in any data that it needs and potentially getting data out of it.

The most basic function takes nothing and gives nothing, although that doesn't necessarily mean it _does_ nothing.

```rouge
func do_a_flip() do outln("Do a flip!")
```

A function's inputs, also known as arguments or parameters depending on who you ask, are placed inside of parentheses. You need to specify the names and types of your inputs, as this allows Rouge to help you out if you make a mistake later on.

```rouge
func collatz(n: nat) do
	m := if n %% 2 then n / 2 else (3 * n) + 1
	outln("\{m}")
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
func load_csv(path: Path) -> [[str]] -< Fs, Read(File), Exn(io::Error) do
	perform open(path)?.lines()
		.map((line) do line.split(',').map((entry) do entry.trim()).collect())
		.collect()
end
```

Functions _can be passed around and stored as if they were data_. Wonder what that `(line) do ...` and `(entry) do ...` expressions are? Those are _closures_, also known as _lambdas_, or more usefully _anonymous functions_. Itty bitty functions that are made when needed and passed around, or even stored in a variable. The general syntax is to place your arguments inside parentheses before the start of a code block - types may be inferred in at least some cases, but you can choose to explicitly type the arguments.

#### `with` Expressions

A `with` expression, outside of its use with effect handlers, is an immediately executed closure. Now, you might ask, why do that instead of using a plain code block? The answer is simple: unlike regular closures, a `with` expression will **only allow you to use values that are expressly passed in.** This was inspired by a small comment from Brian Will's [Object-Oriented Programming is Bad](https://youtu.be/QM1iUe6IofM?si=J5BJmp1fu8ZWy3Hu&t=2460) at around 41 minutes in:

> Unfortunately, what I often really want when creating subsections of longer functions is a feature that doesn't exist in any language I know of. It's an idea I've only seen in one other place, it was Jonathan Blow and his talks about his programming language that he's making. And the idea is that we want something like an anonymous function which doesn't see anything of its enclosing scope. The virtue of extracting a section of code out to a truly separate function is that everything that comes into the function must be explicitly passed through a parameter. It would be great if we could write inline anonymous functions with this same virtue.
>
> Specifically what I propose is: imagine we had a reserved word `use` that introduces a block and, in the header of the `use` we list variables from the enclosing scope which we want accessible in this block, but otherwise anything from the enclosing scope would not be visible. These listed variables, however, would actually be copies of those variables, so if you assign to `x` and `y`, you would be assigning to the `x` and `y` of the `use` block, not to `x` and `y` of the enclosing scope. ...

Now, since Rouge uses mutable value semantics, parameters are copies anyways, so this can easily be implemented. So, you could easily do something like this:

```rouge
thing := with (other_thing) do
    # ... code ...
end
```

But, the parameters passed in don't have to be existing variables. You could also call a function or perform an effect and pass in its output with a name:

```rouge
contents := with (f: perform open("data.txt")?) do
    f.read_all_chars()?
end
```

Realistically though, we don't need the multiple lines here and can just do it as:

```rouge
contents := with (f: perform open("data.txt")?) do f.read_all_chars()?
```

> **Note:** If you're coming from Python, yes the use of the `with` keyword was inspired by Python. A lot of things in Rouge are inspired by other languages. 'Nothing is original, everything is derived from what came before' :D

#### Complex Types

Like most other languages, Rouge allows you to define your own complex types.

Specifically, complex types in Rouge are _algebraic_. This basically means we have two kinds of types:

 - Product types, also known as records, structs, or classes, have fields.
 - Sum types, also known as tagged unions, or enums, have variants.

These are so named because the number of possible values for a product type is the product of the number of possible values for each of its fields, and the number of possible values for a sum type is the sum of the number of possible values for each of its variants.

Both kinds of types are defined using the `type` keyword. For example, a product type `Person` would be created like so:

```rouge
type Person is
    name: str
    age: nat
end
```
Meanwhile, a sum type `Status` would be created like this:
```rouge
type Status is
    | idle
    | active
    | down
end
```
Variants of sum types can have fields, similar to product types.
```rouge
type Status is
    | idle
    | active(current_task: Task)
    | down(last_error: any Error)
end
```

Rouge's standard library has a few useful built-in complex types - such as `Option(T)`, a sum type that represents a value that might not exist. For example, `list.get(i)` returns an `Option(T)` because there might not be any data at index `i`.

To make an instance of a product type, you use a pair of parentheses which contain your fields and place it after the name of the type.
```rouge
me := Person(
    name = "Ashton"
    age = 23
)
```
Meanwhile, to make an instance of a sum type, you write the name of the type and the name of the variant separated by a dot.
```rouge
worker_status := Status.idle
```

#### Associating items to types

Complex types can have things other than fields or variants - things like constants, functions, or even other types, can be associated with a type. This can be done in one of two ways: either you can write the thing your associating the type with directly in the type definition, or you can write it within an `impl` (implementation) block. Which one you go with is really just a code style question.

The first thing you'll likely want to do, at least for product types, is create a factory function. This is an associate function that creates instances of your type. A common name for it is `new`:
```rouge
type Person is
    name: str
    age: nat

    pub func new(name: str, age: nat) -> Person do
        Person(
            name
            age
        )
    end
end
```
Notice how I only wrote the name of the field? One neat thing is that if you have a variable that has the same name and type as a field in a product type, you can just write the name and Rouge will automatically put the corresponding variable in that field. Pretty easy!

Also, I should note here that you don't _have_ to call your type's factory function `new`. If another name makes more sense, you can use that instead. For example, if you have a factory function for a type that represents time, its factory function could reasonably be called `now` and return the current time. Plus, you can have multiple factory functions that have different meanings.

You may also want to keep some constants for your type. For example, each of the number types in Rouge has two associated constants called `MIN` and `MAX`, representing the minimum and maximum values respectively. This could be done as so:
```rouge
impl int as
    const MIN: int = -9_223_372_036_854_775_808
    const MAX: int = 9_223_372_036_854_775_807
end
```
Types can also be associated with other types. This is useful when, for example, you have an error where you want to hold some data alongside an indicator of what kind of error occurred. As you can associate a type with a type, you could easily define a `Kind` type within your `Error` type:
```rouge
type Error is
    kind: type Kind is
        | notEnoughData
        # ...
    end

    # ...
end
```

#### Generics

A generic is essentially a placeholder for a type that you don't know yet, or a type parameter - whichever is easier for you to reason about. Generics can be used with types and functions, as well as traits which will be discussed in a following section. To define a generic for a type or function, you add it as a special kind of parameter like so:
```rouge
type Option(`T) is
    | none
    | some(T)
end

func map(`T, `U, in: T, f: func(T) -> U) -> U do f(in)
```
You can then add requirements to generics, either using a colon `:` or by adding a `where` clause:
```rouge
type Result(`T, `E: Error) is
    | ok(T)
    | err(E)
end

func filter(`T, `F, in: T, f: F) -> bool where
    F: Func(T) -> bool
do f(in)
```

#### Aliases

You can create aliases of types and functions using the `alias` keyword:
```rouge
type Result(`T) is alias Result(T, Error)
```

#### Traits

A trait defines a set of behavior that a type could implement. In short, it is a set of associated items and item prototypes.

You can define a trait as follows:
```rouge
trait Speak is
    func speak(self) -> str
end
```
This defines a trait named `Speak`, and defines a function _prototype_ called `speak`. As this is a prototype, it must be implemented by any type that implements this trait - which must be done with an `impl` block. However, in this case, you can have an `impl` inside of the type definition - like so:
```rouge
type Dog is
    name: str
    breed: str

    impl Speak as
        func speak(self) -> str do "Woof!"
    end
end

# External implementation blocks are also okay
type Cat is
    name: str
    breed: str
end

impl Speak for Cat as
    func speak(self) -> str do "Meow!"
end
```
(For some reason, VSCodium is inconsistently trying to highlight the syntax. Random parentheses are purple, random PascalCase identifiers are green, and the type keyword before `Cat` is blue. ???)

One useful trait is the `Default` trait, which defines a factory function called `default` that you can define to return a default version of your type. Using traits for this makes it easy for people who may need to use such a default factory, as they have a known interface for it. Same goes for traits that cover converting between types, like `From(T)`, `Into(T)`, and their fallible counterparts.

#### Effects

TBW