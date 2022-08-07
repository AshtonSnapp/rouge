# Data Types

Rouge supports a set of so-called _primitive_ data types. They're called primitive types because they are natively supported by the language - anything else is a _custom_ data type. We'll talk more about those later.

## Scalar Types

### Numbers

Rouge has three main types for handling numbers: `nat`, `int`, and `float`. The `nat` type holds a positive whole number. If you need to hold negative whole numbers, you use the `int` type. Finally, if you need to hold non-whole numbers, or really big numbers, you use the `float` type. The names for the types come from the kinds of numbers they hold. `nat` holds _natural (counting) numbers_, `int` holds _integers_, and `float` holds _floating-point numbers_.

All three types of numbers are represented using 64 bits (8 bytes, using Rust's `u64`, `i64`, and `f64` types respectively). Because of this, a `nat` can hold any number from 0 to 18,446,744,073,709,551,615, an `int` can hold any number from -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807, and a `float` can hold any number from -1.7976931348623157&times;10<sup>38</sup> to 1.7976931348623157&times;10<sup>38</sup> (with the smallest positive number it can hold being 2.2250738585072014&times;10<sup>-308</sup>).

You might be asking: why does Rouge have three different types for holding numbers? A `float` can hold larger numbers than anyone would realistically need, and can also handle fractions - why not just use those? An `int` can handle positive and negative numbers - why use `nat`s? The answer is simple: providing the right tool for the job. Floating point numbers have a huge range of values they can represent, but that comes at the cost of precision - rounding errors, basically. Plus, dealing with floating point is slower than dealing with plain integers. Similarly, there are some situations where it doesn't make sense to allow negative numbers - like if you're counting something. As said previously, use the right tool for the right job.

There is one other type, and I saved it for last because it's intended use case is different: the `byte` type holds a single 8-bit number, generally interpreted as an unsigned value - from 0 to 255. The `byte` type primarily exists because many file types store data as a series of bytes, rather than text. And some tasks, like communicating over the network, may involve turning your data into bytes. So it's useful to have a byte type.

### Booleans

Booleans (written `bool`) are extremely simple: they are either `true` or `false`. Not much else to it, really.

### Characters

Characters (written `char`) are single Unicode scalar values encoded in UTF-8. Now, the term 'Unicode scalar value' here is important - what your code considers a single character and what you consider a single character may be different.

One example that may confuse people is "é" versus "é". They look exactly the same, right? But, if you try to make the latter into a `char` it will throw an error. The first "é" is a single character - specifically the Unicode code point U+00E9 'latin small letter e with acute'. However, the second "é" is in fact two characters - U+0065 'latin small letter e' and U+0301 'combining acute accent'.

## Compound Types

### Tuples

A tuple is a fixed-size collection of multiple types of things. For example, a 2-element tuple containing a natural number and a boolean would be written in code as `(nat, bool)`. You just write down a list of all the different elements and types, separated by commas. You access the individual members of the tuple with `.n` (where n is the number of the member you want to access).

An empty tuple, written `()`, is equivalent to `void` in other languages. In the REPL, you'll see that anything that doesn't return a value will return `()`. We'll generally refer to this as the _unit type_.

### Lists

Lists, written in code as `[T]`, are collections that contain things of some type T. By default they have no set size, but you can declare a sized list (also known as an array) by adding a semicolon after the type, followed by the number of elements, as such: `[T; n]`. You access the individual elements of the list with `array[n]` where n is the element index. Note that accessing elements in this manner will result in the program crashing if `n` is larger than the length of the list. There is a way to do this that allows you to gracefully handle the possibility of a missing element, but that will be discussed in a later chapter.

#### Strings

A string is just a list of characters (`[char]`) under the hood. However, it has extra functions and methods for dealing with usual text manipulation processes like making everything upper or lower case, or stripping prefixes and suffixes, or performing a Unicode normalization. Accessing the individual characters in the string is done like accessing the elements of a list.

If, for some reason, you want to access the individual _bytes_ that make up a string (rather than the characters), you can call the `encode()` function which returns a `[byte]` containing the UTF-8 encoded bytes of the string.

### Maps

Maps are generalizations of lists. With a list, the _index_ of a given element is assumed to be of type `uint`. However, with a map, you can have any type act as your index. In addition, your indecies don't have to be _consecutive_: you don't have to have elements 0-19 in order to have element 20, as an example. Maps are declared using the syntax `[K: V]` where K is the type of the indices (or keys) and V is the type of the contents (or values). You access the different values in the map using the same syntax as accessing the elements of a list, `map[k]` where k is the key of the value you want to access. Note that trying to access the value of a key that doesn't exist in the map will result in the program crashing. There is a way to do this that allows you to gracefully handle the possibility of a missing value, but that will be discussed in a later chapter.

[<-prev](1_start.md) | [next->](3_variables.md)