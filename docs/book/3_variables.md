# Variables

A lot of the time, you'll want places to store values instead of just working with immediate (or _literal_) values. To do this you declare a variable, which is essentially a bucket that you can put data into. Rouge is _statically-typed_, meaning each bucket can only hold a certain data type.

Let's create a simple variable to hold your name. In the REPL, type:

```
main>=> var name = "Ashton"
<=< ()
```

Obviously, you replace the last part with your actual name instead of putting in mine. But, regardless, we just create a variable called `name` that holds a string. Let's verify that by accessing the variable:

```
main>=> name
<=< "Ashton"
```

> **Note:** If you're used to programming languages like C, you may be wondering why we didn't say what type the variable we just made was. That's because the keyword `var` tells the runtime or compiler that it should be able to figure out the type of the variable without our help. This is called **type inferrence**. If you really wanted to though, you could've typed `string name = "Ashton"` (or whatever your name is) instead and that would have worked.
>
> There are some situations where the runtime or compiler won't be able to figure it out without help. You'll know when you encounter them because you'll get an error.

Now, what can we do with our new variable? Just about anything - with one major exception. You see, variables are by default _immutable_ in Rouge. This means that, once they contain a value, that value cannot be changed. If you want to be able to change it (which you might want to do with a name variable since people change their names sometimes), you need to make it _mutable_ by adding the `mut` keyword before the type or `var` keyword.

Let's make a new variable called `age` that will contain how old you are. This definitely needs to be mutable, since people grow older every year.

```
main>=> mut nat age = 21
<=< ()
main>=> age
<=< 21
```

Here I explicitly said that the type of the variable was `nat`. Why did I do that? Well, whenever Rouge tries to infer the type of a variable that's being set to a number, it will default to the `int` type unless it can't. However, having a negative age doesn't make any sense. So, it makes sense to set the age to a natural number rather than an integer.

Now we can change the value in this variable however we wish. For example, say your birthday comes around and it's time to add another year to your age.

```
main>=> age += 1
<=< ()
main>=> age
<=< 22
```

## Constants

Sometimes you'll want to declare values as _constants_ - values that cannot be changed. However, these differ from immutable variables in that the value must be known _before any of your code actually runs._ Constants are declared using the `const` keyword, and they must have their type set explicitly - it _cannot_ be inferred.

```rouge
const byte MEANING_OF_LIFE = 42
```

[<-prev](2_types.md) | [next->](4_flow.md)