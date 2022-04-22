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

The final two types are unique. Unlike the others, which have a fixed size on all platforms, the `uint` and `int` types are dependent on the platform the runtime is being ran on. On 32-bit systems, the `uint` and `int` types are equivalent to the `uword` and `word` types respectively. Likewise, they are equivalent to the `ulong` and `long` types on 64-bit systems. Type inferrence will prefer the `int` type. Moving along though, each size of integer can only hold certain values. This is as follows, with the knowledge that the unsigned minimum value is _always_ 0:

| # Bits | Unsigned Maximum     | Signed Minimum       | Signed Maximum       |
| ------ | -------------------- | -------------------- | -------------------- |
| 8 	 | 255					| -128				   | -127				  |
| 16	 | 65535				| -32768			   | -32765				  |
| 32	 | 4294967295			| -2147483648		   | -2147483647		  |
| 64	 | 18446744073709551615 | -9223372036854775808 | -9223372036854775807 |

For the `uint` or `int` types, the maximums and minimum is the same as either the 32-bit row or the 64-bit row depending on your platform.

#### Floating-Point Types

Rouge has two types for handling floating-point numbers. Floating point numbers are capable of handling fractional or decimal numbers - like 0.25 (or your circle constant of choice) - and extremely large numbers. There are two floating point types - the 32-bit single-precision type (written as `flt` in code) and the 64-bit double-precision type (written as `dbl` in code).

### Booleans

Booleans (written `bool`) are extremely simple: they are either true or false.

### Characters

Characters (written `char`) are single Unicode scalar values encoded in UTF-8. Now, the term 'Unicode scalar value' here is important - what your code considers a single character and what you consider a single character may be different.

One example that may confuse people is "é" versus "é". They look exactly the same, right? But, if you try to make the latter into a `char` it will throw an error. The first "é" is a single character - specifically the Unicode code point U+00E9 'latin small letter e with acute'. However, the second "é" is in fact two characters - U+0065 'latin small letter e' and U+0301 'combining acute accent'.

## Compound Types

### Tuples

TBW

### Lists

Lists, written in code as `[T]`, are collections that contain things of some type T. By default they have no set size, but you can declare a sized list (also known as an array) by adding a semicolon after the type, followed by the number of elements, as such: `[T; n]`.

#### Strings

A string is just a list of characters (`[char]`) under the hood. However, it has extra functions and methods for dealing with usual text manipulation processes like making everything upper or lower case, or stripping prefixes and suffixes, or performing a Unicode normalization.

### Maps

Maps are generalizations of lists. With a list, the _index_ of a given element is assumed to be of type `uint`. However, with a map, you can have any type act as your index. Maps are declared using the syntax `[K: V]` where K is the type of the indices (or keys) and V is the type of the contents (or values). You can declare a sized map (or array-map) using similar syntax to arrays: `[K: V; n]`.