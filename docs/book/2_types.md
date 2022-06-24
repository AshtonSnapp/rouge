# Data Types

Rouge supports a set of so-called _primitive_ data types. They're called primitive types because they are natively supported by the language - anything else is a _custom_ data type. We'll talk more about those later.

## Scalar Types

### Numeric Types

#### Integer Types

Rouge has several types for storing whole numbers - or integers. Specifically, there are 10. The distinction between the types is based on the amount of memory used to store the number and whether the number is signed (capable of holding negative numbers) or unsigned (incapable of holding negative numbers). They are indicated via keywords - those prefixed with u- are unsigned types, the rest are signed.

| Length | Unsigned | Signed  |
| ------ | -------- | ------- |
| 8-bit  | `ubyte`  | `byte`  |
| 16-bit | `ushort` | `short` |
| 32-bit | `uword`  | `word`  |
| 64-bit | `ulong`  | `long`  |
| arch.  | `uint`   | `int`   |

The final two types are unique. Unlike the others, which have a fixed size on all platforms, the `uint` and `int` types are dependent on the platform the runtime is being ran on. On 32-bit systems, the `uint` and `int` types are equivalent to the `uword` and `word` types respectively. Likewise, they are equivalent to the `ulong` and `long` types on 64-bit systems. Type inferrence (which we will discuss in the next chapter) will prefer the `int` type. Moving along though, each size of integer can only hold certain values. This is as follows, with the knowledge that the unsigned minimum value is _always_ 0:

| # Bits | Unsigned Maximum     | Signed Minimum       | Signed Maximum       |
| ------ | -------------------- | -------------------- | -------------------- |
| 8 	 | 255					| -128				   | -127				  |
| 16	 | 65535				| -32768			   | -32765				  |
| 32	 | 4294967295			| -2147483648		   | -2147483647		  |
| 64	 | 18446744073709551615 | -9223372036854775808 | -9223372036854775807 |

For the `uint` or `int` types, the maximums and minimum is the same as either the 32-bit row or the 64-bit row depending on your platform.

> **Caution:** With smaller integer types, it becomes more likely that a value will wrap around. For example, if you have a `ubyte` with a value of 255 and you try to add 1, it will wrap around back to 0. This is called **integer overflow**. On the other side, if you have a `ubyte` with a value of 0 and you try to subtract 1, it will wrap around to 255. This is called **integer underflow**. The platform-dependent types `uint` and `int` should be sufficient for most use cases. If you need to use explicitly-sized types, for any reason, make sure the value you're trying to store is within the given bounds for the type you're using.

#### Floating-Point Types

Rouge has two types for handling floating-point numbers. Floating point numbers are capable of handling fractional or decimal numbers - like 0.25 (or your circle constant of choice) - and extremely large numbers like the speed of light. There are two floating point types - the 32-bit single-precision type (written as `flt` in code) and the 64-bit double-precision type (written as `dbl` in code). Type inferrence will prefer the `dbl` type as it is more precise.

### Booleans

Booleans (written `bool`) are extremely simple: they are either `true` or `false`. Not much else to it, really.

### Characters

Characters (written `char`) are single Unicode scalar values encoded in UTF-8. Now, the term 'Unicode scalar value' here is important - what your code considers a single character and what you consider a single character may be different.

One example that may confuse people is "é" versus "é". They look exactly the same, right? But, if you try to make the latter into a `char` it will throw an error. The first "é" is a single character - specifically the Unicode code point U+00E9 'latin small letter e with acute'. However, the second "é" is in fact two characters - U+0065 'latin small letter e' and U+0301 'combining acute accent'.

Characters can be converted into `uword`s (32-bit integers) freely - every character coresponds to a particular number. However, any given `uword` is not guaranteed to correspond to a character. There is a gap in Unicode values - there are no characters corresponding to the values between 0xD800 and 0xDFFF and there are no characters corresponding to the values beyond 0x10FFFF.

## Compound Types

### Tuples

A tuple is a fixed-size collection of multiple types of things. For example, a 2-element tuple containing an unsigned integer and a boolean would be written in code as `(uint, bool)`. You just write down a list of all the different elements and types, separated by commas. You access the individual members of the tuple with `.n` (where n is the number of the member you want to access).

An empty tuple, written `()`, is equivalent to `void` in other languages. In the REPL, you'll see that anything that doesn't return a value will return `()`. We'll generally refer to this as the _unit type_.

### Lists

Lists, written in code as `[T]`, are collections that contain things of some type T. By default they have no set size, but you can declare a sized list (also known as an array) by adding a semicolon after the type, followed by the number of elements, as such: `[T; n]`. You access the individual elements of the list with `array[n]` where n is the element index. Note that accessing elements in this manner will result in the program crashing if `n` is larger than the length of the list. There is a way to do this that allows you to gracefully handle the possibility of a missing element, but that will be discussed in a later chapter.

#### Strings

A string is just a list of characters (`[char]`) under the hood. However, it has extra functions and methods for dealing with usual text manipulation processes like making everything upper or lower case, or stripping prefixes and suffixes, or performing a Unicode normalization. Accessing the individual characters in the string is done like accessing the elements of a list.

If, for some reason, you want to access the individual _bytes_ that make up a string (rather than the characters), you can call the `encode()` function which returns a `[ubyte]` containing the UTF-8 encoded bytes of the string.

### Maps

Maps are generalizations of lists. With a list, the _index_ of a given element is assumed to be of type `uint`. However, with a map, you can have any type act as your index. In addition, your indecies don't have to be _consecutive_: you don't have to have elements 0-19 in order to have element 20, as an example. Maps are declared using the syntax `[K: V]` where K is the type of the indices (or keys) and V is the type of the contents (or values). You access the different values in the map using the same syntax as accessing the elements of a list, `map[k]` where k is the key of the value you want to access. Note that trying to access the value of a key that doesn't exist in the map will result in the program crashing. There is a way to do this that allows you to gracefully handle the possibility of a missing value, but that will be discussed in a later chapter.

[<-prev](1_start.md) | [next->](3_variables.md)