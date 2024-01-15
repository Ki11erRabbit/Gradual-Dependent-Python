# Approximate Normalization

The process is simple. We maintain an environment that knows things that are bound. We then execute code, keeping track of the types of expressions and bound variables.
If a type contains an unknown term or is an unknown term we convert the term into the unknown term. So a function that
returns an unknown type then the whole expression becomes the unknown type.





# Epiphany
Since Gradual Normalization is extremely similar to Execution, We might be able to type check as a python script.
We would just need an object that represents the unknown term and objects that wrap the base python object. 
This way, whenever we perform an operation with an unknown type, we just return the unknown type. Then if we get the unknown
type at the end of some evaluation then we know to insert runtime checks in to the program.

I have learned that we can define ops on the unknown type that allow us to have the unknown type eat other types.
This is extremely useful since once we introduce the unknown type it will convert other types into itself.
The problem now is figuring out when to introduce the unknown type into the code for type checking.

## Introducing the Unknown Type
I think we can introduce it into functions that take types that override the `__len__` method.
This is because it makes it so that we can say something like this `has '__len__'` instead of a concrete type (i.e. `list`). This tells the compiler to check that 
value has the `__len__` method defined. This is because most collections will have this overridden. This means that we can 
add it to the type signature of functions that take collections. Anything that does this will by default return the unknown type.
This is because the length is the unknown type. 

Anything that deals with runtime data (such as `input()` or file IO) will always return the empty type when typechecking.

Anything that returns the `Any` type will return the unknown type when type checking.

## How to normalize
* Constants:
  * Integers: `33` -> `int(33)`
  * Floats: `33.3` -> `float(33.3)`
  * Strings: `"hello"` -> `str("hello")`
  * Lists: `[1, 2, 3]` -> `list([int(1), int(2), int(3)])`
  * Tuples: `(1, 2, 3)` -> `tuple((int(1), int(2), int(3)))`
  * Sets: `{1, 2, 3}` -> `set({int(1), int(2), int(3)})`
  * Dicts: `{1: 2, 3: 4}` -> `dict({int(1): int(2), int(3): int(4)})`
  * Functions: `lambda x: x` -> `function(lambda x: x)`
  * Classes: `class A: pass` -> `class A: pass`
* Functions:
  *  `def f(x): return x` -> `def f(x): return x`
 



## How to convert into Normalization Form
Normalization Form is the form that we convert the code into so that we can type check it.
To do this we can use the Python eval function to evaluate expressions get their results.