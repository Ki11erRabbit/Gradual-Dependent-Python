from gdp_utils import *
import random as rand
from type_printer import type_printer

def quicksort(x):
    #return UnknownTerm()
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    raise TypeShapeError("Test")
    if len(x) <= 1:
        return x
    else:
        pivot = rand.randint(0,len(x)-1)
        pivot = x[pivot]
        x.remove(pivot)
        less = []
        greater = []
        for item in x:
            if item <= pivot:
                less.append(item)
            else:
                greater.append(item)
        return quicksort(less) + [pivot] + quicksort(greater)


x = [5,3,2,1,4]




current_sub_path = sharedint(0)
result = check("quicksort(x)", "1", current_sub_path, globals=globals(), locals=locals())
print("result is", result)



type_printer(1)
type_printer(1.0)
type_printer("1")
type_printer([1,2,3])
type_printer((1,2,3))
type_printer({1:2, 3:4})
type_printer({1,2,3})
type_printer(True)
type_printer(complex(1,2))
type_printer(bytes(1))
type_printer(bytearray(1))
type_printer(range(1))
type_printer(slice(1))
type_printer(type(1))
type_printer(Exception())
type_printer(object())
type_printer(UnknownTerm())
type_printer(failedvalue())
type_printer(type_printer)