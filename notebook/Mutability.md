# Mutability


## Variable Mutation
I think that variable mutation is structural mutation of the environment.
I say this because Python lets you gain access to said environment as a dictionary.
Another reason why is because when I wrote a stack-based VM I had an environment that used strings as keys and was
used to store variables for non-stack based storage.

Another reason is that in Python at least we can change the type of a variable. 

To describe this
let `x` be a variable that that has type `t` and `T` be the set of all types.

The type `t` in Python is `t` ∈ ⊆ `T`.

Therefore, `x` can be only one type at any given time from a subset of types in `T` during the lifetime of the variable `x`.

Whereas in a language like Rust, `x` can only be the same type `t` for the entire scope unless it is rebound.

In this case `t` ∈ `T`


## Structural Mutation
