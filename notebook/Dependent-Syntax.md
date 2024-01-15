Date: 01/13/2024



# Dependent Syntax


## Ideas
* Use expressions that evaluate to a boolean value so that they can just be lifted out of the type signature and placed into the function body.


## Syntax
* `where` is used to specify constraints on the type of the variable. An expression should follow that returns a boolean value.
  * for return types, use the function name to refer to the output of the function.
  * The syntax is `where <expr>`.
* `has` is used to specify that the type must have a certain attribute or list of attributes.
  * The syntax is `has '<attr>'`.
  * The syntax is `has ['<attr>', '<attr>', ...]`
* `when` can be used to specify constraints on when a function can return a type.
  * This can be useful for specifying multiple return types.
  * The syntax is `when <var> is <type>`.
* A `|` can be used to specify multiple possible constraints on a type.
  * The syntax is `where <expr> | <expr> | ...`.

## Examples
#### Safe Division
##### Gradual Dependent Version
```python
def safe_div(x: int, y: int where y != 0) -> float:
    return x / y
```
##### Output When Gradual Normalization returns the Unknown Type
```python
def safe_div(x: int, y: int) -> float:
    if not isinstance(x, int) and not isinstance(y, int):
        raise TypeError("x, y must be of type int")
    if not (y != 0):
        raise ShapeError("y != 0 not satisfied")
    __function_output__ = x / y
    if not isinstance(__function_output__, float):
        raise TypeError("return value must be of type float")
    return __function_output__
```
##### Gradual Normalization Form
```python
def safe_div(x: int, y: int) -> float:
    if isinstance(x, UnknownType) or isinstance(y, UnknownType):
        return UnknownType()
    if not isinstance(x, int) and not isinstance(y, int):
        raise TypeError("x, y must be of type int")
    if not (y != 0):
        raise ShapeError("y != 0 not satisfied")
    __function_output__ = x / y
    if not isinstance(__function_output__, float):
        raise TypeError("return value must be of type float")
    return __function_output__
```
#### Matrix Multiplication
##### Gradual Dependent Version
```python
def mat_mul(x: list where len(x[0]) == len(y) , y: list) -> list where len(mat_mul) == len(x) and len(mat_mul[0]) == len(y[0]):
    out = []
    for i in range(len(x)):
        out.append([])
        for j in range(len(y[0])):
            out[i].append(0)
            for k in range(len(y)):
                out[i][j] += x[i][k] * y[k][j]
    return out
```
##### Output When Gradual Normalization returns the Unknown Type
```python
def mat_mul(x: list, y: list) -> list where len(x) == len(y[0]):
    if not isinstance(x, list) and not isinstance(y, list):
        raise TypeError("x, y must be of type list")
    if not len(x[0]) == len(y):
        raise ShapeError("len(x[0]) == len(y) not satisfied")
    out = []
    for i in range(len(x)):
        out.append([])
        for j in range(len(y[0])):
            out[i].append(0)
            for k in range(len(y)):
                out[i][j] += x[i][k] * y[k][j]
    
    if not isinstance(out, list):
        raise TypeError("return value must be of type list")
    if not len(out) == len(y[0]):
        raise ShapeError("len(out) == len(y[0]) not satisfied")
    return out
```
##### Gradual Normalization Form
```python
def mat_mul(x: list, y: list) -> list where len(x) == len(y[0]):
    if isinstance(x, UnknownType) or isinstance(y, UnknownType):
        return UnknownType()
    if not isinstance(x, list) and not isinstance(y, list):
        raise TypeError("x, y must be of type list")
    if not len(x[0]) == len(y):
        raise ShapeError("len(x[0]) == len(y) not satisfied")
    out = []
    for i in range(len(x)):
        out.append([])
        for j in range(len(y[0])):
            out[i].append(0)
            for k in range(len(y)):
                out[i][j] += x[i][k] * y[k][j]
    
    if not isinstance(out, list):
        raise TypeError("return value must be of type list")
    if not len(out) == len(y[0]):
        raise ShapeError("len(out) == len(y[0]) not satisfied")
    return out
```
#### Head
##### Gradual Dependent Version
```python
def head(x: list where len(x) > 0) -> Any:
    return x[0]
```
##### Output When Gradual Normalization returns the Unknown Type
```python
def head(x: list) -> Any:
    if not isinstance(x, list):
        raise TypeError("x must be of type list")
    if not len(x) > 0:
        raise ShapeError("len(x) > 0 not satisfied")
    __function_output__ = x[0]
    return __function_output__
```
##### Gradual Normalization Form
```python
def head(x: list) -> Any:
    return UnknownType()
```
#### Tail
##### Gradual Dependent Version
```python
def tail(x: list where len(x) > 0) -> list where len(tail) == len(x[:1]) + 1:
    return x[1:]
```
##### Output When Gradual Normalization returns the Unknown Type
```python
def tail(x: list) -> list:
    if not isinstance(x, list):
        raise TypeError("x must be of type list")
    if not len(x) > 0:
        raise ShapeError("len(x) > 0 not satisfied")
    __function_output__ = x[1:]
    
    if not isinstance(__function_output__, list):
        raise TypeError("return value must be of type list")
    if not len(__function_output__) == len(x[:1]) + 1:
        raise ShapeError("len(tail) == len(x[:1]) + 1 not satisfied")
    return __function_output__
```
##### Gradual Normalization Form
```python
def tail(x: list) -> list:
    if isinstance(x, UnknownType):
        return UnknownType()
    if not isinstance(x, list):
        raise TypeError("x must be of type list")
    if not len(x) > 0:
        raise ShapeError("len(x) > 0 not satisfied")
    __function_output__ = x[1:]
    
    if not isinstance(__function_output__, list):
        raise TypeError("return value must be of type list")
    if not len(__function_output__) == len(x[:1]) + 1:
        raise ShapeError("len(tail) == len(x[:1]) + 1 not satisfied")
    return __function_output__
```
#### Different Return Types
##### Gradual Dependent Version
```python
def f(x: int) -> int when x > 0 | str when x < 0 | float when x == 0:
    if x > 0:
        return x
    elif x < 0:
        return str(x)
    else:
        return float(x)
```
##### Output When Gradual Normalization returns the Unknown Type
```python
def f(x: int) -> Any:
    # (int > 0) -> int
    # (int < 0) -> str
    # (int == 0) -> float
    if not isinstance(x, int):
        raise TypeError("x must be of type int")
    
    if x > 0:
        __function_output__ = x
        if not isinstance(__function_output__, int):
            raise TypeError("return value must be of type int")
        return __function_output__
    elif x < 0:
        __function_output__ = str(x)
        if not isinstance(__function_output__, str):
            raise TypeError("return value must be of type str")
        return __function_output__
    else:
        __function_output__ = float(x)
        if not isinstance(__function_output__, float):
            raise TypeError("return value must be of type float")
        return __function_output__
```
##### Gradual Normalization Form
```python
def f(x: int) -> Any:
    if isinstance(x, UnknownType):
        return UnknownType()
    if not isinstance(x, int):
        raise TypeError("x must be of type int")
    
    if x > 0:
        __function_output__ = x
        if not isinstance(__function_output__, int):
            raise TypeError("return value must be of type int")
        return __function_output__
    elif x < 0:
        __function_output__ = str(x)
        if not isinstance(__function_output__, str):
            raise TypeError("return value must be of type str")
        return __function_output__
    else:
        __function_output__ = float(x)
        if not isinstance(__function_output__, float):
            raise TypeError("return value must be of type float")
        return __function_output__
```
