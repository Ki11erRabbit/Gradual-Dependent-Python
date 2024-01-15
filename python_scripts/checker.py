from gdp_utils import *
import random as rand

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
