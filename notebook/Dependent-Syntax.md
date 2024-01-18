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
import gdp_runtime_utils as gdp
def safe_div(x: int, y: int) -> float:
    return x / y

# Call Site
global safe_div
safe_div = gdp.Function(safe_div, "safe_div", [gdp.DependentType.from_type(int, "x"), gdp.DependentType.from_where_clauses('int', "y","y != 0")], {}, gdp.DependentType.from_type('float',"safe_div"))
safe_div(1, 0)
```
##### Gradual Normalization Form
```python
import gdp_typecheck_utils as gdp
def safe_div(x: int, y: int) -> float:
    return x / y

# Call Site
global safe_div
safe_div = gdp.Function(safe_div, "safe_div", [gdp.DependentType.from_type(int, "x"), gdp.DependentType.from_where_clauses('int', "y","y != 0")], {}, gdp.DependentType.from_type('float',"safe_div"))
x, y = 2, 0
result = gdp.check("safe_div(x,y)", __parent_path__, __path__, globals(), locals())
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
import gdp_runtime_utils as gdp
def mat_mul(x: list, y: list) -> list:
    out = []
    for i in range(len(x)):
        out.append([])
        for j in range(len(y[0])):
            out[i].append(0)
            for k in range(len(y)):
                out[i][j] += x[i][k] * y[k][j]
    return out

# Call Site
global mat_mul
mat_mul = gdp.Function(mat_mul, "mat_mul", [gdp.DependentType.from_type('list', "x"), gdp.DependentType.from_type('list', "y")], {}, gdp.DependentType.from_where_clauses('list', "mat_mul","len(x) == len(y[0])"))
mat_mul([[1, 2, 3], [4, 5, 6]], [[7, 8], [9, 10], [11, 12]])
```
##### Gradual Normalization Form
```python
import gdp_typecheck_utils as gdp
from gdp_stdlib import *

def mat_mul(x: list, y: list) -> list:
    out = []
    __r0__ = range(len(x))
    for i in __r0__:
        out.append([])
        __r1__ = range(len(y[0]))
        for j in __r1__:
            out[i].append(0)
            __r2__ = range(len(y))
            for k in __r2__:
                out[i][j] += x[i][k] * y[k][j]
                if not gdp.is_total(__r2__):
                    out = gdp.UnknownTerm()
                    break
  
            if not gdp.is_total(__r1__):
                out = gdp.UnknownTerm()
                break
        # If we find that a variable gets mutated in a non-total loop, then we modify the variable to have the unknown term and break the loop.
        # This ensures that we are able to terminate the loop and propagate the unknown term.
        if not gdp.is_total(__r0__):
            out = gdp.UnknownTerm()
            break
    
    return out


# Call Site
global mat_mul
mat_mul = gdp.Function(mat_mul, "mat_mul",
                       [gdp.DependentType.from_type('list', "x"), gdp.DependentType.from_type('list', "y")], {},
                       gdp.DependentType.from_where_clauses('list', "mat_mul", "len(x) == len(y[0])"))
x, y = [[1, 2, 3], [4, 5, 6]], [[7, 8], [9, 10], [11, 12]]
result = gdp.check("mat_mul(x,y)", __parent_path__, __path__, globals(), locals())
```
#### Head
##### Gradual Dependent Version
```python
def head(x: list where len(x) > 0) -> Any:
    return x[0]
```
##### Output When Gradual Normalization returns the Unknown Type
```python
import gdp_runtime_utils as gdp
def head(x: list) -> Any:
    return x[0]

# Call Site
global head
head = gdp.Function(head, "head", [gdp.DependentType.from_type('list', "x")], {}, gdp.DependentType.from_where_clauses('Any', "head","len(x) > 0"))
head([1,2,3])
```
##### Gradual Normalization Form
```python
import gdp_typecheck_utils as gdp
def head(x: list) -> Any:
    return x[0]

# Call Site
global head
head = gdp.Function(head, "head", [gdp.DependentType.from_type('list', "x")], {}, gdp.DependentType.from_type('Any', "head"))
result = gdp.check("head([1,2,3])", __parent_path__, __path__, globals(), locals())
```
#### Tail
##### Gradual Dependent Version
```python
def tail(x: list where len(x) > 0) -> list where len(tail) == len(x[:1]) + 1:
    return x[1:]
```
##### Output When Gradual Normalization returns the Unknown Type
```python
import gdp_runtime_utils as gdp
def tail(x: list) -> list:
    return x[1:]

# Call Site
global tail
tail = gdp.Function(tail, "tail", [gdp.DependentType.from_type('list', "x")], {}, gdp.DependentType.from_where_clauses('list', "tail","len(tail) == len(x[:1]) + 1"))
tail([1,2,3])
```
##### Gradual Normalization Form
```python
import gdp_runtime_utils as gdp
def tail(x: list) -> list:
    return x[1:]

# Call Site
global tail
tail = gdp.Function(tail, "tail", [gdp.DependentType.from_type('list', "x")], {}, gdp.DependentType.from_where_clauses('list', "tail","len(tail) == len(x[:1]) + 1"))
x = [1,2,3]
result = gdp.check("tail(x)", __parent_path__, __path__, globals(), locals())
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
