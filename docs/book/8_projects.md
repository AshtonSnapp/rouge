# Project Structure

Let's back out of the REPL for a bit and talk about projects. When you're first starting out in a programming language, you'll often be writing projects where all of your code is in one file, sometimes called a _script_. For simple programs and libaries, this isn't a big deal. In fact, sometimes it's actually preferrable. However, as your projects get bigger and more complicated, it will be impossible to keep track of the project's structure in your head. You can organize your code by splitting it up into multiple files - here each file is called a _module_. Each module will generally contain related code. Following this, all modules that are part of the same project form a single _package_.

Let's make a practical example of how a project can be structured. Let's say you're writing a client for some online service - for this example we'll use a microblogging platform like Twitter or Mastodon. At a high level, you can split your code into two groups: the front-end (the code responsible for interacting with the user) and the back-end (the code responsible for interacting with the service). So, you can structure your project like this:

```
\src
 └-- main.rg
 └-- frontend.rg
 └-- backend.rg
```

This works, but you can split things up even further if desired. For example, it may make sense to split the backend authentication (login) code into a separate file - maybe `auth.rg` or `login.rg`. Doing this, you move `backend.rg` and this other file into a folder called `backend` - this makes `login` a _sub-module_ of `backend`. You may have noticed that `backend.rg` is also moved into the backend folder. If it stayed in the same place, this would cause a conflict - you're trying to make two modules called `backend`. Instead, it is moved into the `backend` folder. Since it keeps its name, this causes it to act as the module's _root_. It would also act as the module's root if it was named `mod.rg`, but keeping the name the same can make things a bit more clear (though this depends on personal opinion).

## Visibility

When you move code into a module, it is by default _private_. This means that it cannot be accessed by anything outside of the module. However, a module containing only private code isn't very useful. So, different items like functions, constants, structs, enums, and things we haven't covered yet can be marked as either _public_, using the `pub` keyword, or _protected_, using the `prt` keyword.

Public is easy to explain - something made public is accessible from any module or package that can reach it. The last part is crucial - just because a function is public doesn't mean something can reach it. The module containing the function must also be public, same with any parent, grandparent, great-grandparent, and so on modules. Another thing is that _any_ public code can be directly accessed by code outside of the runtime - regardless of whether other Rouge code would or wouldn't be able to.

Protected is a bit more complicated. Essentially marking something as protected means it is public so long as the accessing code is within a certain _scope_. By default that scope is the package, however a smaller scope can be speicifed by putting it in parentheses attached to the `prt` keyword (like if it was a function - `prt(scope)`). What goes into those parentheses is a module path, which will be discussed next along with how one makes code from a module or another package accessible - `use` statements.

## `use` statements

A `use` statement is essentially the same as an `import` statement in other programming languages. If you have code in another module, or even another package, you need to let the compiler or interpreter know that you're using code from a different file. This is generally done at the top of a file, like so:

```rouge
use std::fs # import the filesystem module from the standard library
```

Using something makes it known within the scope of the current module. Any descendant or child items can be accessed using `::`, the same operator used to access variants of an enum or associated items (not methods, those use `.`) of a struct or enum. You can think of it like the `/` used to separate parent and child directories in UNIX-style file paths and web addresses, or the `\` used in Windows/DOS-style file paths.

You can import multiple descendants at the same time using square brackets.

```rouge
use std::[fs, io] # import the filesystem and input/output modules from the standard library
```

Sometimes it will make sense to just import the module and access its items using the `::` operator in your code. This can make it more clear where something comes from, plus it might not make sense to import something that's only used once or twice.

### Renaming using `as`

When importing items, you'll sometimes want to rename them. This is useful, for example, when you have two types that have the same name but come from different packages or modules. You can do this using the `as` keyword.

```rouge
use std::io::Error as IOError
use std::fmt::Error as FormatError
```

### Re-exporting using `pub use`

There are some situations where you want to import something from a module, and provide a public way to access it without making the containing module (or its parent modules) public as well. You can do this by using the `pub` keyword in front of `use`. This essentially creates a shortcut. It can also be used to re-export a package you're using.

```rouge
pub use self::private::ThingThatNeedsToBePublic
```

[<-prev](7_enums.md) | [next->](9_errors.md)