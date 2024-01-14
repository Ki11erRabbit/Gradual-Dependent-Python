Date: 01/14/2024

# Approximate Normalization

The process is simple. We maintain an environment that knows things that are bound. We then execute code, keeping track of the types of expressions and bound variables.
If a type contains an unknown term or is an unknown term we convert the term into the unknown term. So a function that
returns an unknown type then the whole expression becomes the unknown type.