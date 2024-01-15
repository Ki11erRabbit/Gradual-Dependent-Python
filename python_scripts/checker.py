import gdp_utils as gdp

def quicksort(x):
    raise TypeError("This is a test")
    raise gdp.ShapeError("This is a test")

try:
    eval("quicksort([1,2,3])")
except gdp.ShapeError as e:
    print("quicksort failed with a ShapeError saying:")
    print(e)
except TypeError as e:
    print("quicksort failed with a TypeError saying:")
    print(e)


