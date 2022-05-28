# Hello, Rouge!

To start, let's write a classic Hello World program. This is essentially a program that prints the words "Hello, world!" (or some variation on that) to the command-line, and is usually the first program someone writes in a programming language they're learning.

Create a file called `hello.rg`, and write the following code inside it:

```rouge
pub func main():
	outl!("Hello world!")
end
```

Then, open your command line and type the following:

```
$ rouge ./hello.rg
```

```
> rouge .\hello.rg
```

You'll see the words "Hello world!" printed in your terminal.

## The anatomy of a Rouge program

The simple program you we just wrote is a single file containing three lines. What do the lines mean though?

```rouge
pub func main():
```

 - Functions are declared using the `func` keyword - a function is just a piece of code that can be run at any time.
 - The `pub` keyword at the start means the function we're making is _public_. Anything that's public can be directly accessed by external code, like the program implementing the Rouge runtime.
 - We're calling this function `main` - this is a special name that indicates the _entrypoint_ into your program. When this file is processed by the runtime, a pointer to this entrypoint is handed to the program containing the runtime so it can be ran.
 - The parentheses can contain arguments that will be passed to the function. The entrypoint function can't take any arguments, however.
 - The colon at the end of the line says that any following lines are part of this function.

```rouge
	outl!("Hello, world!")
```

TBW: how do explain macros?

```rouge
end
```

The `end` keyword indicates the end of a block of code - such as a function.

## REPL

From this point on, we'll be using the Rouge REPL. REPL stands for Read-Eval-Print Loop - it _reads_ code from you via the command-line, it evaluates that code, it prints what the code did, and it repeats that until you tell it to stop. All you need to do to access the REPL is run the `rouge` command-line utility without any arguments. To exit the REPL, type `:q` like you would to get out of Vim. REPL prompts will be represented in this book like this:

```
main>=>
```

Anything that doesn't have a prompt like that is a general code snippet, and shouldn't be typed into the REPL.

[<-prev](0_intro.md) | [next->](2_types.md)