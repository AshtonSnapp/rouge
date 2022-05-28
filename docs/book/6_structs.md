# Structs

You'll often find yourself using multiple different values to represent one chunk of information. Rather than deal with them individually, which can be error-prone and generally tedious or annoying to handle, you can bundle them up into a _struct_ (short for structure). A struct is what's called a _custom data type_. THis is because it's defined by code, and not built into the language itself like primitives. A given struct contains a number of _fields_, or member variables if you prefer.

Let's define a struct that represents a student at a school. We'll need to know the student's name, what school year they're in, their GPA, and what classes they have.

```
main>=> struct Student:
		... string name
		... ubyte year
		... flt gpa
		... [string] classes
		... end
<=< ()
```

Notice how that didn't return anything? Well, when you define a struct, all you're really doing is writing a blueprint. In order to use your new struct, you need to _instantiate_ it, or create an instance of it. This can be done by stating the name of the struct (blueprint) followed by curly braces containing the contents, in the format of `field: value`. For example, let's make a struct for Alice from our for loop example in the control flow chapter.

```
main>=> var alice = Student {
		... name: "Alice Anderson"
		... year: 10
		... gpa: 2.0
		... classes: [
		... ... "English II",
		... ... "Algebra II",
		... ... "Physical Science",
		... ... "Physical Education",
		... ... "World Geography",
		... ... "Spanish I",
		... ... "Art II"
		... ... ]
		... }
<=< ()
main>=> alice
<=< <Student { name: "Alice Anderson", year: 10, gpa: 2.0, classes: [...] }>
```

So we now have an instance of a student. Great. We can access the different fields of our new struct using dot syntax, like how you access the elements in a tuple but using the name of the field instead of a number.

```
main>=> alice.classes
<=< ["English II", "Algebra II", "Physical Science", "Physical Education", "World Geography", "Spanish I", "Art II"]
```

If the instance is declared as mutable, you can change a particular field's value using the same syntax.

```
main>=> mut var bob = Student {
		... name: "Bob Baker"
		... year: 9
		... gpa: 1.2
		... classes: [
		... ... "English I",
		... ... "Algebra I",
		... ... "Biology I",
		... ... "Physical Education",
		... ... "Civics",
		... ... "Art I",
		... ... "Computer Science I"
		... }
<=< ()
main>=> bob.gpa = 1.5
<=< ()
```

Note that the _entire instance_ must be mutable. There's no way to make only certain fields mutable or immutable - the entire thing has to be mutable or none of it is.

## Tuple Structs

Structs can also have unnamed fields, like tuples. These are useful for when you want to have a name for a commonly-used tuple. As an example, let's make a tuple that represents a 24-bit RGB color.

```
main>=> struct Color (ubyte, ubyte, ubyte)
<=< ()
main>=> var lime_green = Color(50, 205, 50)
<=< ()
main>=> lime_green
<=< <Color (50, 205, 50)>
```

## Unit Structs

Structs can also have... well, no fields at all. This may seem useless, but there are situations where it is useful.

```
main>=> struct Empty
<=< ()
main>=> var boi = Empty
<=< ()
main>=> boi
<=< <Empty>
```

## Associated Functions

You'll often create functions that deal with a specific struct or other custom type. You can create these functions the normal way, but you can also make them into _associated functions_. This is done by either prepending the function name with the name of the type, separated by two colons, or by declaring them within an implementation block with the `impl` keyword. The latter is most often used. Let's create an associated function that will create a Student struct for us. Constructors are a common use for associated functions, after all.

```
main>=> impl Student:
Student::>=> pub func new(string name, ubyte year, flt gpa, ...string) Student:
Student::new>=> return Student {
		... name
		... year
		... gpa
		... classes: vargs
		... }
Student::new>=> end
<=< func(string, ubyte, flt, ...string) Student
Student::>=> end
<=< <impl on Student>
main>=> mut var mary = Student::new("Mary Mallory", 12, 4.0, ["English IV", "Precalculus", "AP Physics", "AP Computer Science Principles"])
<=< ()
main>=> mary.classes[0]
<=< "English IV"
```

### Methods

A method is a special type of associated function that takes a specific instance of the struct, referred to using the `self` keyword, as its first argument. Methods can either have mutable or immutable access to the instance, depending on whether the first argument is just `self` or `mut self`.

This time, let's make a method that sets the student's GPA based on their grades in their classes.

```
main>=> pub func Student::update_gpa(mut self, [string: flt] grades):
Student::update_gpa>=> mut flt average = 0.0
Student::update_gpa>=> for class in self.classes:
		... if grades.get(class) is Some(grade):
		... ... average += grade
		... ... end
		... end
Student::update_gpa>=> average /= (grades.len() as flt)
Student::update_gpa>=> self.gpa = average
Studnet::update_gpa>=> end
<=< <impl on Student>
main>=> mary.update_gpa(["English IV": 4.0, "Precalculus": 3.8, "AP Physics": 3.6, "AP Computer Science Principles": 4.0])
<=< ()
main>=> mary.gpa
<=< 3.85
```

### Associated Constants

Functions aren't the only thing you can associate with a struct. You can also associate constants.

```rouge
# You can do it like this...
impl Type:
	const CONSTANT: ConstType = value
end

# Or like this...
const Type::CONSTANT: ConstType = value
```

[<-prev](5_functions.md) | [next->](7_enums.md)