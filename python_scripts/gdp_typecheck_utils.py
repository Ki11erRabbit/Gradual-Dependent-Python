from gdp_runtime_utils import *


class UnknownTerm:
    def __init__(self):
        pass

    def __repr__(self):
        return "UnknownTerm()"

    def __str__(self):
        return "UnknownTerm"

    def __eq__(self, other):
        return isinstance(other, UnknownTerm)

    def __ne__(self, other):
        return not self.__eq__(other)

    def __lt__(self, other):
        return False

    def __le__(self, other):
        return False

    def __gt__(self, other):
        return False

    def __ge__(self, other):
        return False

    def __hash__(self):
        return hash("UnknownTerm")

    def __bool__(self):
        return False

    def __add__(self, other):
        return self

    def __radd__(self, other):
        return self

    def __sub__(self, other):
        return self

    def __rsub__(self, other):
        return self

    def __mul__(self, other):
        return self

    def __rmul__(self, other):
        return self

    def __truediv__(self, other):
        return self

    def __rtruediv__(self, other):
        return self

    def __floordiv__(self, other):
        return self

    def __rfloordiv__(self, other):
        return self

    def __mod__(self, other):
        return self

    def __rmod__(self, other):
        return self

    def __pow__(self, other):
        return self

    def __rpow__(self, other):
        return self

    def __lshift__(self, other):
        return self

    def __rlshift__(self, other):
        return self

    def __rshift__(self, other):
        return self

    def __rrshift__(self, other):
        return self

    def __and__(self, other):
        return self

    def __rand__(self, other):
        return self

    def __xor__(self, other):
        return self

    def __rxor__(self, other):
        return self

    def __or__(self, other):
        return self

    def __ror__(self, other):
        return self

    def __iadd__(self, other):
        return self

    def __neg__(self):
        return self

    def __pos__(self):
        return self

    def __abs__(self):
        return self

    def __invert__(self):
        return self

    def __round__(self, n=None):
        return self

    def __floor__(self):
        return self

    def __ceil__(self):
        return self

    def __trunc__(self):
        return self

    def __int__(self):
        return self

    def __float__(self):
        return self

    def __complex__(self):
        return self

    def __oct__(self):
        return self

    def __hex__(self):
        return self

    def __index__(self):
        return self

    def __len__(self):
        return self

    def __getitem__(self, key):
        return self

    def __setitem__(self, key, value):
        pass

    def __delitem__(self, key):
        pass

    def __iter__(self):
        return self

    def __next__(self):
        raise StopIteration

    def __reversed__(self):
        return self

    def __contains__(self, item):
        return self

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        pass

    def __call__(self, *args, **kwargs):
        return self

    def __getattr__(self, item):
        return self

    def __setattr__(self, key, value):
        pass

    def __delattr__(self, item):
        pass

    def __copy__(self):
        return self

    def __deepcopy__(self, memodict={}):
        return self

    def __getstate__(self):
        return self


    def __setstate__(self, state):
        pass

    def __reduce__(self):
        return self

    def __reduce_ex__(self, protocol):
        return self


    def __format__(self, format_spec):
        return "?"

    def __get__(self, instance, owner):
        return self

    def __set__(self, instance, value):
        pass

    def __delete__(self, instance):
        pass

    def __sizeof__(self):
        return self


class DependentType(DependentType):
    def __init__(self, type_name, var_name, value=None, where_clauses=None, has_clauses=None, when_clauses=None):
        super().__init__(type_name, var_name, value=value, where_clauses=where_clauses, has_clauses=has_clauses, when_clauses=when_clauses)

    def is_reducible(self, locals, var_name):
        if isinstance(locals[var_name], UnknownTerm):
            print(f"Value {locals[var_name]} is beta-eta reducible to {self.type_name}")
            return
        
        super().is_reducible(locals, var_name)

    @staticmethod
    def from_value(value, var_name):
        return DependentType(type(value).__name__, var_name, value)

    @staticmethod
    def from_type(type, var_name):
        return DependentType(type.__name__, var_name)

    @staticmethod
    def from_where_clauses(type_name, var_name, *where_clauses):
        return DependentType(type_name, var_name, where_clauses=where_clauses)

    @staticmethod
    def from_has_clauses(type_name, var_name, *has_clauses):
        return DependentType(type_name, var_name, has_clauses=has_clauses)

    @staticmethod
    def from_when_clauses(type_name, var_name, *when_clauses):
        return DependentType(type_name, var_name, when_clauses=when_clauses)
    
class FailedValue(Exception):
    def __init__(self, message="failed due to previous computation failing"):
        super().__init__(message)

    @staticmethod
    def fail():
        raise FailedValue("failed due to previous computation failing")
    def __repr__(self):
        FailedValue.fail()

    def __str__(self):
        FailedValue.fail()

    def __eq__(self, other):
        FailedValue.fail()

    def __ne__(self, other):
        FailedValue.fail()

    def __lt__(self, other):
        FailedValue.fail()

    def __le__(self, other):
        FailedValue.fail()

    def __gt__(self, other):
        FailedValue.fail()

    def __ge__(self, other):
        FailedValue.fail()

    def __hash__(self):
        FailedValue.fail()

    def __bool__(self):
        FailedValue.fail()

    def __add__(self, other):
        FailedValue.fail()

    def __radd__(self, other):
        FailedValue.fail()

    def __sub__(self, other):
        FailedValue.fail()

    def __rsub__(self, other):
        FailedValue.fail()

    def __mul__(self, other):
        FailedValue.fail()

    def __rmul__(self, other):
        FailedValue.fail()

    def __truediv__(self, other):
        FailedValue.fail()

    def __rtruediv__(self, other):
        FailedValue.fail()

    def __floordiv__(self, other):
        FailedValue.fail()

    def __rfloordiv__(self, other):
        FailedValue.fail()

    def __mod__(self, other):
        FailedValue.fail()

    def __rmod__(self, other):
        FailedValue.fail()

    def __pow__(self, other):
        FailedValue.fail()

    def __rpow__(self, other):
        FailedValue.fail()

    def __lshift__(self, other):
        FailedValue.fail()

    def __rlshift__(self, other):
        FailedValue.fail()

    def __rshift__(self, other):
        FailedValue.fail()

    def __rrshift__(self, other):
        FailedValue.fail()

    def __and__(self, other):
        FailedValue.fail()

    def __rand__(self, other):
        FailedValue.fail()

    def __xor__(self, other):
        FailedValue.fail()

    def __rxor__(self, other):
        FailedValue.fail()

    def __or__(self, other):
        FailedValue.fail()

    def __ror__(self, other):
        FailedValue.fail()

    def __iadd__(self, other):
        FailedValue.fail()

    def __neg__(self):
        FailedValue.fail()

    def __pos__(self):
        FailedValue.fail()

    def __abs__(self):
        FailedValue.fail()

    def __invert__(self):
        FailedValue.fail()

    def __round__(self, n=None):
        FailedValue.fail()

    def __floor__(self):
        FailedValue.fail()

    def __ceil__(self):
        FailedValue.fail()

    def __trunc__(self):
        FailedValue.fail()

    def __int__(self):
        FailedValue.fail()

    def __float__(self):
        FailedValue.fail()

    def __complex__(self):
        FailedValue.fail()

    def __oct__(self):
        FailedValue.fail()

    def __hex__(self):
        FailedValue.fail()

    def __index__(self):
        FailedValue.fail()

    def __len__(self):
        FailedValue.fail()

    def __getitem__(self, key):
        FailedValue.fail()

    def __setitem__(self, key, value):
        FailedValue.fail()

    def __delitem__(self, key):
        FailedValue.fail()

    def __iter__(self):
        FailedValue.fail()

    def __next__(self):
        FailedValue.fail()

    def __reversed__(self):
        FailedValue.fail()

    def __contains__(self, item):
        FailedValue.fail()

    def __enter__(self):
        FailedValue.fail()

    def __exit__(self, exc_type, exc_val, exc_tb):
        FailedValue.fail()

    def __call__(self, *args, **kwargs):
        FailedValue.fail()

    def __getattr__(self, item):
        FailedValue.fail()

    def __setattr__(self, key, value):
        FailedValue.fail()

    def __delattr__(self, item):
        FailedValue.fail()

    def __copy__(self):
        FailedValue.fail()

    def __deepcopy__(self, memodict={}):
        FailedValue.fail()

    def __getstate__(self):
        FailedValue.fail()

    def __setstate__(self, state):
        FailedValue.fail()

    def __reduce__(self):
        FailedValue.fail()

    def __reduce_ex__(self, protocol):
        FailedValue.fail()

    def __format__(self, format_spec):
        return "FailedValue"

    def __get__(self, instance, owner):
        FailedValue.fail()

    def __set__(self, instance, value):
        FailedValue.fail()

    def __delete__(self, instance):
        FailedValue.fail()

    def __sizeof__(self):
        FailedValue.fail()


class SharedInt:
    def __init__(self, value: int = 0):
        self.value = value

    def increment(self):
        self.value += 1

    def decrement(self):
        self.value -= 1

    def __str__(self):
        return str(self.value)




def attach_total(x, total):
    setattr(x, "__total__", total)

def is_total(x) -> bool:
    if hasattr(x, "__total__"):
        return x.__total__
    return False

def check(expr: str, path: str, current_sub_path: SharedInt, globals=globals(), locals=locals()):
    current = current_sub_path.value
    current_sub_path.increment()
    try:
        return eval(expr, globals, locals)
    except DependentTypeError as e:
        print(f"Path {path}.{str(current)} failed with a DependentTypeError saying:")
        print(e)
        return FailedValue()
    except TypeError as e:
        print(f"Path {path}.{str(current)} failed with a TypeError saying:")
        print(e)
        return FailedValue()
    except AttributeError as e:
        print(f"Path {path}.{str(current)} failed with an AttributeError saying:")
        print(e)
        return FailedValue()
    except FailedValue as e:
        print(f"Path {path}.{str(current)} failed with a FailedValue exception")
        return e
    except Exception as e:
        print(f"Path {path}.{str(current)} threw an exception saying:")
        print(e)
        raise e
