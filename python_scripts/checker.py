from gdp_typecheck_utils import *
import random as rand

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

def mat_mult(x: list, y: list) -> list:
    output = []
    for i in range(len(x)):
        output.append([])
        for j in range(len(y[0])):
            output[i].append(0)
            for k in range(len(y)):
                output[i][j] += x[i][k] * y[k][j]
    return output

def check_quicksort(path):
    global quicksort
    quicksort = Function(quicksort, "quicksort", [DependentType.from_type(list, "x")], {}, DependentType.from_type(list, "quicksort"))
    x = [5, 3, 2, 1, 4]

    result = check("quicksort(x)", path, SharedInt(0), globals=globals(), locals=locals())
    print(result)


def check_make_even(path):
    global make_even
    make_even = Function(make_even, "make_even", [DependentType.from_where_clauses("int", "x", "x % 2 != 0")], {}, DependentType.from_where_clauses("int", "x % 2 == 0"))
    x = 5

    result = check("make_even(x)", path, SharedInt(0), globals=globals(), locals=locals())
    print(result)


def check_mat_mul(path):
    global mat_mult
    mat_mult = Function(mat_mult, "mat_mult", [DependentType.from_where_clauses("list","x", "len(x[0]) == len(y)"), DependentType.from_where_clauses('list', "y", "len(y[0]) == len(x)")], {}, DependentType.from_where_clauses('list', "mat_mult", "len(mat_mult) == len(x) and len(mat_mult[0]) == len(y[0])"))
    x = [[1, 2, 3], [4, 5, 6]]
    y = [[7, 8], [9, 10], [11, 12]]

    result = check("mat_mult(x, y)", path, SharedInt(0), globals=globals(), locals=locals())
    print(result)



check_quicksort("0")

check_make_even("1")

check_mat_mul("2")
