

import random as rand
import gdp_utils as gdp








def quicksort(x):
    return gdp.UnknownTerm()
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







def main():
    x = [5,3,2,1,4]
    y = quicksort(x)
    y.append(6)
    y = y + [7]
    z = y[0:3]
    q = [7] + y
    print(y)
    print(z)
    print(q)







if __name__ == '__main__':
    main()