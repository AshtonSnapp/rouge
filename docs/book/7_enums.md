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

When you have a function that returns an Option or similar enum, you can't just access the value contained in the variants directly. You first need to check what variant it is - and you do that via _pattern matching_. This is done using the `is` keyword, which acts as an operator in an `if` expression. For example, let's make a list of names.

```
main>=> var names = ["Ashton", "Bob", "Chance", "Drake"]
<=< ()
```

Now let's make a simple function that takes in this list and an index and prints a greeting if there's a name there, but prints something else if there isn't.

```
main>=> func greet([string] names, nat index):
greet>=> if names.get(index) is Some(name) then:
		... outl!("Hello, {}!", name)
		... else:
		... outl!("No person!")
		... end
greet>=> end
<=< func([string], uint)
```

Then we can do some testing with this.

```
main>=> greet(names, 2)
Hello, Chance!
<=< ()
main>=> greet(names, 4)
No person!
<=< ()
```

You can also invert this, although you can't get any sort of argument out of destructuring the pattern you're checking for. For example, let's say you're writing code to go in a vending machine and you want to reject any coins that aren't quarters. Assuming we have some struct that represents the vending machine, and you have that coin enum from an earlier example, you could write the following code:

```rouge
func on_coin_inserted(mut self, Coin coin):
	if coin is not Coin::Quarter then:
		self.dispense_coin(coin)
	else:
		self.cents_inserted += 25
	end
end
```

Finally, you can create a pattern matching _block_ by changing how you write the statement slightly. This is actually inspired by how Jai handles switch statments:

```rouge
if coin is:
	Coin::Penny then self.cents_inserted += 1
	Coin::Nickel then self.cents_inserted += 5
	Coin::Dime then self.cents_inserted += 10
	Coin::Quarter then self.cents_inserted += 25
end
```

And, for comparison, here's roughly the same code in Jai (including an enum definition):

```jai
Coin :: enum {
	Penny;
	Nickel;
	Dime;
	Quarter;
}

if coin == {
	case Coin.Penny; machine.cents_inserted += 1;
	case Coin.Nickel; machine.cents_inserted += 5;
	case Coin.Dime; machine.cents_inserted += 10;
	case Coin.Quarter; machine.cents_inserted += 25;
}
```

The only difference between doing things like this and using `is` as an operator is that, when you have a pattern matching block, you must be _exhaustive_. This means that you have to cover all possible patterns. Luckily, if you only care about some of the possible patterns, you can handle the rest using `else` as follows:

```rouge
if coin is:
	Coin::Quarter then self.cents_inserted += 25
	Coin::Dime then self.cents_inserted += 10
	else self.dispense_coin(coin)
end
```

And remember, you can put `:` after `then` or `else` to start a multi-line code block, which must be ended with the `end` keyword.

### Pattern Matching and Conditional Loops

Pattern matching can also be used when writing conditional loops. This allows you to write a loop which only runs if, for example, you get a `Some` from a function that returns an `Option<T>`, or a loop that automatically runs different code depending on what it gets back from the function.

```rouge
var range = 1..=100

while range.next() is Some(i) do outl!("{}", i)
```

Would you like to know a fun fact? The above is functionally identical to the following code:

```rouge
for i in 1..=100 do outl!("{}", i)
```

And, at a lower level, is equivalent to the following code using simple loops:

```rouge
var range = 1..=100

loop:
	if range.next() is Some(i) then outl!("{}", i) else break
end
```

[<-prev](6_structs.md) | [next->](8_projects.md)