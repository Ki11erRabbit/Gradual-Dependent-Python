Date: 01/14/2024

# Scope
It might be possible to extend the scope of this project to include more features of Python by taking advantage of the bytecode interpreter of Rust Python.
This means that all we would have to do is figure out how to add the unknown type to the possible values of the Python VM.
Then we can hijack the VM to do our [Approximate Normalization](Approximate-Normalization.md) for us. This would then make the process
much faster to Normalize a script.