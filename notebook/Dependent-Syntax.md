Date: 01/13/2024



# Dependent Syntax


## Ideas
* Use expressions that evaluate to a boolean value so that they can just be lifted out of the type signature and placed into the function body.


## Examples
```python
def safe_div(x: int, y: int where y != 0) -> int:
    return x / y
```
becomes
```python
def safe_div(x: int, y: int) -> int:
    if not isinstance(x, int) and not isinstance(y, int):
        raise TypeError("x, y must be of type int")
    if not (y != 0):
        raise ShapeError("y != 0 not satisfied")
    function_output = x / y
    if not isinstance(function_output, int):
        raise TypeError("return value must be of type int")
       return function_output
```

```python

```
