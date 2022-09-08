# Tokens

A token is the smallest unit of meaningful information defined by a programming language. Rouge source code can be generally broken down into the following types of tokens:

 - Literals
 - Punctuation
 - Keywords
 - Identifiers

## Literals

Literals are tokens that represent an immediately known value.

### Character Literal

> **Literal(UTFCharacter):**
> `'(CharacterEscapes|[\u{0}-\u{10FFFF}])'`
> 
> **CharacterEscapes:**
> `\x[0-7][0-9a-fA-F]` | `\n` | `\r` | `\t` | `\\` | `\0` | `\'` | `\u\{[0-9a-fA-F]{1,6}\}`

### String Literal

> **Literal(UTFString):**
> `"(StringEscapes|[\u{0}-\u{10FFFF}])*"`
>
> **StringEscapes:**
> `\x[0-7][0-9a-fA-F]` | `\n` | `\r` | `\t` | `\\` | `\0` | `\"` | `\u\{[0-9a-fA-F]{1,6}\}`

### Raw String Literal

> **Literal(UTFString): (Raw)**
> `r#"[\u{0}-\u{10FFFF}]*"#r`

### Byte Character Literal

> **Literal(ByteCharacter):**
> `b'(ByteEscapes|[\x00-\x7F])'b`
> 
> **ByteEscapes:**
> `\x[0-9a-fA-F][0-9a-fA-F]` | `\n` | `\r` | `\t` | `\\` | `\0` | `\'`

### Byte String Literal

> **Literal(ByteString):**
> `b"(ByteStringEscapes|[\x00-\x7F])*"b`
>
> **ByteStringEscapes:**
> `\x[0-9a-fA-F][0-9a-fA-F]` | `\n` | `\r` | `\t` | `\\` | `\0` | `\"`

### Raw Byte String Literal

> **Literal(ByteString): (Raw)**
> `br#"[\x00-\x7F]*"#rb`

### Unsigned Integer Literal

> **Literal(UnsignedInteger):**
> `(BinaryInteger|OctalInteger|DecimalInteger|HexadecimalInteger)`
>
> **BinaryInteger:**
> `0[bB][01][_01]`
>
> **OctalInteger:**
> `0[oO][0-7][_0-7]`
>
> **DecimalInteger:**
> `[0-9][_0-9]`
>
> **HexadecimalInteger:**
> `0[xX][0-9a-fA-F][_0-9a-fA-F]`

### Signed Integer Literal

> **Literal(SignedInteger):**
> `[+-](UnsignedInteger)`

### Float Literal

> **Literal(Float):**
> `[+-]?[0-9][_0-9](.[0-9][_0-9])?([eE][0-9]+)?`

## Punctuation

