from gdp_utils import *
import random as rand
from type_printer import type_printer

def quicksort(x):
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


def make_even(x: int) -> int:
    return x + 1

def check_quicksort(path):
    global quicksort
    quicksort = Function(quicksort, "quicksort", [DependentType.from_type(list, "x")], {}, DependentType.from_type(list, "quicksort"))
    x = [5, 3, 2, 1, 4]

    result = check("quicksort(x)", path, sharedint(0), globals=globals(), locals=locals())
    print(result)


def check_make_even(path):
    global make_even
    make_even = Function(make_even, "make_even", [DependentType.from_where_clauses("int", "x", "x % 2 != 0")], {}, DependentType.from_where_clauses("int", "x % 2 == 0"))
    x = 4

    result = check("make_even(x)", path, sharedint(0), globals=globals(), locals=locals())
    print(result)



check_quicksort("0")

check_make_even("1")