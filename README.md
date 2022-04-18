# Rouge

Rouge (pronounced 'rooj') is a statically-typed programming language designed for two primary uses: applications (graphical and command-line), and embedding into native programs (plugins, config files). To be suitable for both use cases, Rouge aims to have the following feature set:

 - A memory management model that aims to be intuitive but with at least some guarantees towards memory and thread safety.
	- Current thoughts: reference counting with copy-on-write and maybe some idea of ownership?
 - A simple, easy-to-learn syntax inspired primarily by Ruby and Lua.
 - Interpreted for development and use in config files, bytecode-compiled for distribution.

While not entirely important to it's stated use cases, Rouge also aims to have tagged unions for enums, traits for shared functionality (rather than inheritance, which means Rouge may or may not be object-oriented depending on your definition), and probably some other stuff that I can't think of right now.

The custom runtime environment (RTE) for the Rouge programming language will be provided as a Rust library (and someone can probably work on making a wrapping cdylib for interfacing with other languages) for embedding, and as a standalone utility for applications. Both will simply be called `rouge` and contain everything necessary to run and compile Rouge code.

Rouge is currently licensed under MIT.

For the project's Code of Conflict, please click [here](./CONFLICT.md).

## Current TODO List

### High Priority

 - [ ] Create a functioning RTE and compiler.
	- [ ] Design the RTE's instruction set and the bytecode file format.
	- [ ] Design an intuitive interface for communication between a native program and an embedded Rouge runtime.
	- [ ] Figure out how to implement JIT compilation.
 - [ ] Create the standard library.
 - [ ] Create documentation.
	- [ ] (Optional) Specifications (at least semi-formal) for the language and related things.
 - [ ] Create a functioning command-line utility.

### Medium Priority

 - [ ] Create a toolchain (e.g. dependency/project manager, doctool, language server)

### Low Priority

 - [ ] Branding.

## Thoughts on Memory Management

Originally, I planned to use an automatic memory management model similar to that of Rust's ownership and borrowing system. However, Rust's system isn't really intuitive and it's designed to give you a level of control over memory that you may need in a systems programming context. However, Rouge doesn't aim to be a systems programming language. It aims to be an embedded or general applications programming language. So, to simplify the memory management model, I'm thinking about going with a form of automatic reference counting with copy-on-write and possibly some idea of ownership (but not to the extent of Rust).

But what do I mean with all of this? Well, let's go through an example. In your source code, let's create a string variable like so:

```rouge
str text = "Hello, world!"
```

When interpreted or compiled into bytecode, this would become some set of instructions that creates a heap object and puts the characters of the string into that heap object. Somehow. At time of writing I actually haven't decided on the instruction set. Now this is am immutable variable, and there will be compile-time/interpret-time checks to prevent creating a mutable reference to an immutable thing. But let's add a `mut` to the beginning of that statement to make it a mutable variable. Now, let's create a reference to that text.

```rouge
str text2 = text
```

Yep, pass by reference. Now, this is an immutable reference. But, again, add a `mut` and it's a mutable reference. Now what happens if you try to modify text2? Say, by changing that exclamation point to a period.

```rouge
text2[-1] = '.'
```

When that happens, the runtime will see that you're trying to modify something that has multiple references to it. You could just disallow that, but instead what happens is the runtime creates a copy of the object on the heap, changes the reference to the copy, and modifies the copy. And this would just be done automatically.

The only question would be whether to implement ownership, and how. It would have to be a simple system, and I have no idea whether it would be implementable on the runtime architecture level. And what would mutating something owned do to any references to it? If we follow Rust, all of those references would be dumped/invalidated, but how would that be handled?

Safe to say, ownership is a question for a later date.