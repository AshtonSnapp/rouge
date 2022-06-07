# Enums

Sometimes you want a piece of data that can only be one of a given set of values. The most common way to do this is using _enums_, or enumerations. An enum is a custom data type which has multiple _variants_, and any given instance of it is exactly one of those variants (except in the case of bitflags). In most programming languages, that's where the explanation stops. However, in Rouge, the variants of an enum are actually structs. This means that each variant of an enum can contain data, and the data contained can differ from variant to variant. And, because unit structs exist, a variant of an enum can hold no extra data at all!

Let's start by defining a simple enum that represents the different denominations of US coins (feel free to replace this with the different denominations of coins wherever you live).

```
main>=> enum Coin:
		... Penny
		... Nickel
		... Dime
		... Quarter
		... end
<=< ()
```

Then, of course, we need to instance our enum. Instead of just making a single instance though, let's make a list of coins.

```
main>=> [Coin] coin_wallet = [Coin::Penny, Coin::Nickel, Coin::Nickel, Coin::Dime, Coin::Quarter, Coin::Quarter]
<=< ()
main>=> coin_vallet[4]
<=< <Coin::Quarter>
```

Of course, enums are much more powerful than this due to the fact that the variants are structs. However, rather than running through creating such an enum ourselves, let's take a look at a commonly used enum included in the standard library: `Option<T>`.

## `Option<T>` and Possibly Missing Data

At its core, Option is an enum defined with two variants: Some and None.

```rouge
pub enum Option<T>:
	Some(T)
	None
end
```

A quick explanation on that `<T>` at the end of the name: while we'll go into greater depth on this topic later, that is a _generic type parameter_. That's basically fancy talk for "hey, an instance of this type will have a field with a currently unknown type that we'll call T". And the Option type is essentially Rouge's method for dealing with the fact that, sometimes, you won't have any data. It is almost always used as a return value for functions.

One example is a function present on lists (including arrays and strings) and maps: `get()`. On lists and the like, it take a single argument called `index` containing the index of the element you're trying to get, while maps have it take an argument called `key` that contains the key of the element you're trying to get. It then returns an `Option` of the type the collection contains, with the Some variant containing whatever was at that index or key. Lists also have `get_slice()` which takes a range of some sorts as an argument and returns an `Option` of the collection type, with the Some variant containing a sub-slice of the collection. In any case, the None variant indicates that the index or key doesn't exist, or the range is out of the list's bounds.

## Pattern Matching

When you have a function that returns an Option or similar enum, you can't just access the value contained in the variants directly. You first need to check what variant it is - and you do that via _pattern matching_. This is most commonly done using a `match` block. As an example, let's make a list of names and match whatever is returned from calling the get function on it.

```
main>=> var names = ["Ashton", "Chance", "John", "Trista"]
<=< ()
main>=> match names.get(2):
		... Some(n) then outl!("Hello, {}!", n)
		... None then errl!("No person!")
		... end
Hello, John!
<=< ()
main>=> match names.get(4):
		... Some(n) then outl!("Hello, {}!", n)
		... None then errl!("No person!")
		... end
No person!
<=< ()
```

You can see this in full effect if you have an enum with more than 2 variants, or if you match against something that isn't an enum. Yes, you can do that. One example is if you want to check if some user input string matches one of several pre-determined strings, or if a number is in one of many given ranges. If you're just checking if something is one variant of an enum, you can use `if is`. This is just an if expression, but the condition is `x is y` where x is some data and y is the pattern you want to match against.

[<-prev](6_structs.md) | [next->](8_projects.md)