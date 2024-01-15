

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
        return self

    def __get__(self, instance, owner):
        return self

    def __set__(self, instance, value):
        pass

    def __delete__(self, instance):
        pass

    def __sizeof__(self):
        return self



class TypeShapeError(Exception):
    def __init__(self, message):
        super().__init__(message)

def abs(x):
    # (int) -> int
    # (float) -> float
    # (has '__abs__') -> has '__abs__'
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__abs__' not in dir(x):
        raise TypeShapeError("object has no attribute '__abs__'")
    else:
        return x.__abs__()

old_all = all
def all(x):
    # (list) -> bool
    # (has '__bool__') -> bool
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    return old_all(x)

old_any = any
def any(x):
    # (list) -> bool
    # (has '__bool__') -> bool
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    return old_any(x)

old_ascii = ascii
def ascii(x):
    # (has '__repr__') -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__repr__' not in dir(x):
        raise AttributeError("object has no attribute '__repr__'")
    else:
        return old_ascii(x)

old_bin = bin
def bin(x):
    # (int) -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__index__' not in dir(x):
        raise AttributeError("object has no attribute '__index__'")
    else:
        return old_bin(x)

old_bool = bool

def bool(x = False):
    # (has '__bool__') -> bool
    # (has '__len__') -> bool
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    else:
        return old_bool(x)



def create_bytearray(source, encoding, errors):
    # (str) -> bytearray
    if isinstance(source, UnknownTerm) or isinstance(encoding, UnknownTerm) or isinstance(errors, UnknownTerm):
        return UnknownTerm()
    else:
        if encoding is None and errors is None:
            bytearray(source)
        elif encoding is None:
            bytearray(source, encoding)
        else:
            bytearray(source, encoding, errors)


def create_bytes(source, encoding, errors):
    # (str) -> bytes
    if isinstance(source, UnknownTerm) or isinstance(encoding, UnknownTerm) or isinstance(errors, UnknownTerm):
        return UnknownTerm()
    else:
        if encoding is None and errors is None:
            bytes(source)
        elif encoding is None:
            bytes(source, encoding)
        else:
            bytes(source, encoding, errors)



def callable(x):
    # (has '__call__') -> bool
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    else:
        return "__call__" in dir(x)


old_chr = chr
def chr(x):
    # (int) -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    if not isinstance(x, int):
        raise TypeError("an integer is required for chr()")
    else:
        return old_chr(x)


old_compile = compile
def compile(source, filename, mode, flags=0, dont_inherit=False, optimize=-1):
    # (str) -> AST
    if isinstance(source, UnknownTerm) or isinstance(filename, UnknownTerm) or isinstance(mode, UnknownTerm) or isinstance(flags, UnknownTerm) or isinstance(dont_inherit, UnknownTerm) or isinstance(optimize, UnknownTerm):
        return UnknownTerm()
    else:
        return old_compile(source, filename, mode, flags, dont_inherit, optimize)



old_divmod = divmod
def divmod(x, y):
    # (int | float, int | float) -> tuple
    if isinstance(x, UnknownTerm) or isinstance(y, UnknownTerm):
        return UnknownTerm()
    else:
        return old_divmod(x, y)

old_enumerate = enumerate
def enumerate(x, start=0):
    # (list) -> list
    if isinstance(x, UnknownTerm) or isinstance(start, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_enumerate(x, start)


def normal_eval(expression, globals=None, locals=None):
    # (str) -> object
    if isinstance(expression, UnknownTerm) or isinstance(globals, UnknownTerm) or isinstance(locals, UnknownTerm):
        return UnknownTerm()
    else:
        return eval(expression, globals, locals)

def normal_exec(expression, globals=None, locals=None, /, *, closure=None):
    # (str) -> None
    if isinstance(expression, UnknownTerm) or isinstance(globals, UnknownTerm) or isinstance(locals, UnknownTerm) or isinstance(closure, UnknownTerm):
        return UnknownTerm()
    else:
        return exec(expression, globals, locals, closure=closure)

old_filter = filter
def filter(function, iterable):
    # (function, list) -> list
    if isinstance(function, UnknownTerm) or isinstance(iterable, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(iterable) and '__next__' not in dir(iterable):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    elif '__call__' not in dir(function):
        raise AttributeError("object has no attribute '__call__'")
    else:
        return old_filter(function, iterable)

old_float = float
def float(x=0.0):
    # (int | float) -> float
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif isinstance(x, str):
        return old_float(x)
    elif '__float__' not in dir(x) or '__index__' not in dir(x):
        raise AttributeError("object has no attribute '__float__' or '__index__'")
    else:
        return old_float(x)

old_format = format
def format(value, format_spec=None):
    # (int | float) -> str
    if isinstance(value, UnknownTerm) or isinstance(format_spec, UnknownTerm):
        return UnknownTerm()
    elif '__format__' not in dir(value):
        raise AttributeError("object has no attribute '__format__'")
    else:
        return old_format(value, format_spec)

old_hash = hash

def hash(x):
    # (int | float) -> int
    # (has '__hash__') -> int
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__hash__' not in dir(x):
        raise AttributeError("object has no attribute '__hash__'")
    else:
        return old_hash(x)

old_hex = hex
def hex(x):
    # (int) -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__index__' not in dir(x):
        raise AttributeError("object has no attribute '__index__'")
    else:
        return old_hex(x)

old_id = id
def id(x):
    # (object) -> int
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    else:
        return old_id(x)


def input(prompt=None):
    # (str) -> str
    return UnknownTerm()

old_int = int
def int(x=0, base=10):
    # (int | float) -> int
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif isinstance(x, str):
        return old_int(x, base)
    elif '__index__' not in dir(x):
        raise AttributeError("object has no attribute '__index__'")
    else:
        return old_int(x, base)

old_iter = iter
def iter(x, sentinel=None):
    # (list) -> iterator
    if isinstance(x, UnknownTerm) or isinstance(sentinel, UnknownTerm):
        return UnknownTerm()
    elif sentinel is None and ('__iter__' not in dir(x) and '__next__' not in dir(x) or '__getitem__' not in dir(x)):
        raise AttributeError("object has no attribute '__iter__', '__next__' or '__getitem__'")
    elif sentinel is None:
        return old_iter(x)
    elif sentinel is not None and '__call__' not in dir(x):
        raise AttributeError("object has no attribute '__call__'")
    else:
        return old_iter(x, sentinel)

old_len = len
def len(x):
    # (list) -> int
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__len__' not in dir(x):
        raise AttributeError("object has no attribute '__len__'")
    else:
        return old_len(x)

old_list = list
def list(x):
    # (list) -> list
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_list(x)

old_map = map
def map(function, iterable, *iterables):
    # (function, list) -> list
    if isinstance(function, UnknownTerm) or isinstance(iterable, UnknownTerm) or isinstance(iterables, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(iterable) and '__next__' not in dir(iterable):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    elif '__call__' not in dir(function):
        raise AttributeError("object has no attribute '__call__'")
    else:
        return old_map(function, iterable, *iterables)

#todo: max and min

old_next = next
def next(iterator, default=None):
    # (iterator) -> object
    if isinstance(iterator, UnknownTerm) or isinstance(default, UnknownTerm):
        return UnknownTerm()
    elif '__next__' not in dir(iterator):
        raise AttributeError("object has no attribute '__next__'")
    elif default is None:
        return old_next(iterator)
    else:
        return old_next(iterator, default)

old_oct = oct
def oct(x):
    # (int) -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__index__' not in dir(x):
        raise AttributeError("object has no attribute '__index__'")
    else:
        return old_oct(x)

old_open = open
def open(file, mode='r', buffering=-1, encoding=None, errors=None, newline=None, closefd=True, opener=None):
    # (str) -> file
    return UnknownTerm()

old_ord = ord
def ord(x):
    # (str) -> int
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif not isinstance(x, str):
        raise TypeError("ord() expected string of length 1, but {} found".format(type(x)))
    elif len(x) != 1:
        raise TypeShapeError("ord() expected string of length 1, but string of length {} found".format(len(x)))
    else:
        return old_ord(x)

old_pow = pow
def pow(x, y, z=None):
    # (int | float, int | float) -> int | float
    if isinstance(x, UnknownTerm) or isinstance(y, UnknownTerm) or isinstance(z, UnknownTerm):
        return UnknownTerm()
    else:
        return old_pow(x, y, z)

old_range = range
def range(start, stop=None, step=1):
    # (int) -> list
    if isinstance(start, UnknownTerm) or isinstance(stop, UnknownTerm) or isinstance(step, UnknownTerm):
        return UnknownTerm()
    elif stop is None:
        return old_range(start)
    else:
        return old_range(start, stop, step)

old_repr = repr
def repr(x):
    # (object) -> str
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    else:
        return old_repr(x)

old_reversed = reversed
def reversed(x):
    # (list) -> list
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__reversed__' not in dir(x):
        raise AttributeError("object has no attribute '__reversed__'")
    else:
        return old_reversed(x)

old_round = round
def round(x, n=None):
    # (int | float) -> int | float
    if isinstance(x, UnknownTerm) or isinstance(n, UnknownTerm):
        return UnknownTerm()
    else:
        return old_round(x, n)

old_set = set
def set(x):
    # (list) -> set
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_set(x)

old_sorted = sorted
def sorted(x, /, *, key=None, reverse=False):
    # (list) -> list
    if isinstance(x, UnknownTerm) or isinstance(key, UnknownTerm) or isinstance(reverse, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    elif key is None:
        return old_sorted(x, reverse=reverse)
    else:
        return old_sorted(x, key=key, reverse=reverse)

old_str = str
def str(x, encoding=None, errors=None):
    # (object) -> str
    if isinstance(x, UnknownTerm) or isinstance(encoding, UnknownTerm) or isinstance(errors, UnknownTerm):
        return UnknownTerm()
    elif encoding is None and errors is None:
        return old_str(x)
    else:
        return old_str(x, encoding, errors)

old_sum = sum
def sum(x, /, start=0):
    # (list) -> int | float
    if isinstance(x, UnknownTerm) or isinstance(start, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_sum(x, start)

old_tuple = tuple
def tuple(x):
    # (list) -> tuple
    if isinstance(x, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(x) and '__next__' not in dir(x):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_tuple(x)

old_zip = zip
def zip(*iterables, strict=False):
    # (list) -> list
    if isinstance(iterables, UnknownTerm) or isinstance(strict, UnknownTerm):
        return UnknownTerm()
    elif '__iter__' not in dir(iterables) and '__next__' not in dir(iterables):
        raise AttributeError("object has no attribute '__iter__' or '__next__'")
    else:
        return old_zip(*iterables)

class sharedint:
    def __init__(self, value:int = 0):
        self.value = value

    def increment(self):
        self.value += 1

class failedvalue(Exception):
    def __init__(self, message="failed due to previous computation failing"):
        super().__init__(message)

    @staticmethod
    def fail():
        raise failedvalue("failed due to previous computation failing")
    def __repr__(self):
        failedvalue.fail()

    def __str__(self):
        failedvalue.fail()

    def __eq__(self, other):
        failedvalue.fail()

    def __ne__(self, other):
        failedvalue.fail()

    def __lt__(self, other):
        failedvalue.fail()

    def __le__(self, other):
        failedvalue.fail()

    def __gt__(self, other):
        failedvalue.fail()

    def __ge__(self, other):
        failedvalue.fail()

    def __hash__(self):
        failedvalue.fail()

    def __bool__(self):
        failedvalue.fail()

    def __add__(self, other):
        failedvalue.fail()

    def __radd__(self, other):
        failedvalue.fail()

    def __sub__(self, other):
        failedvalue.fail()

    def __rsub__(self, other):
        failedvalue.fail()

    def __mul__(self, other):
        failedvalue.fail()

    def __rmul__(self, other):
        failedvalue.fail()

    def __truediv__(self, other):
        failedvalue.fail()

    def __rtruediv__(self, other):
        failedvalue.fail()

    def __floordiv__(self, other):
        failedvalue.fail()

    def __rfloordiv__(self, other):
        failedvalue.fail()

    def __mod__(self, other):
        failedvalue.fail()

    def __rmod__(self, other):
        failedvalue.fail()

    def __pow__(self, other):
        failedvalue.fail()

    def __rpow__(self, other):
        failedvalue.fail()

    def __lshift__(self, other):
        failedvalue.fail()

    def __rlshift__(self, other):
        failedvalue.fail()

    def __rshift__(self, other):
        failedvalue.fail()

    def __rrshift__(self, other):
        failedvalue.fail()

    def __and__(self, other):
        failedvalue.fail()

    def __rand__(self, other):
        failedvalue.fail()

    def __xor__(self, other):
        failedvalue.fail()

    def __rxor__(self, other):
        failedvalue.fail()

    def __or__(self, other):
        failedvalue.fail()

    def __ror__(self, other):
        failedvalue.fail()

    def __iadd__(self, other):
        failedvalue.fail()

    def __neg__(self):
        failedvalue.fail()

    def __pos__(self):
        failedvalue.fail()

    def __abs__(self):
        failedvalue.fail()

    def __invert__(self):
        failedvalue.fail()

    def __round__(self, n=None):
        failedvalue.fail()

    def __floor__(self):
        failedvalue.fail()

    def __ceil__(self):
        failedvalue.fail()

    def __trunc__(self):
        failedvalue.fail()

    def __int__(self):
        failedvalue.fail()

    def __float__(self):
        failedvalue.fail()

    def __complex__(self):
        failedvalue.fail()

    def __oct__(self):
        failedvalue.fail()

    def __hex__(self):
        failedvalue.fail()

    def __index__(self):
        failedvalue.fail()

    def __len__(self):
        failedvalue.fail()

    def __getitem__(self, key):
        failedvalue.fail()

    def __setitem__(self, key, value):
        failedvalue.fail()

    def __delitem__(self, key):
        failedvalue.fail()

    def __iter__(self):
        failedvalue.fail()

    def __next__(self):
        failedvalue.fail()

    def __reversed__(self):
        failedvalue.fail()

    def __contains__(self, item):
        failedvalue.fail()

    def __enter__(self):
        failedvalue.fail()

    def __exit__(self, exc_type, exc_val, exc_tb):
        failedvalue.fail()

    def __call__(self, *args, **kwargs):
        failedvalue.fail()

    def __getattr__(self, item):
        failedvalue.fail()

    def __setattr__(self, key, value):
        failedvalue.fail()

    def __delattr__(self, item):
        failedvalue.fail()

    def __copy__(self):
        failedvalue.fail()

    def __deepcopy__(self, memodict={}):
        failedvalue.fail()

    def __getstate__(self):
        failedvalue.fail()

    def __setstate__(self, state):
        failedvalue.fail()

    def __reduce__(self):
        failedvalue.fail()

    def __reduce_ex__(self, protocol):
        failedvalue.fail()

    def __format__(self, format_spec):
        failedvalue.fail()

    def __get__(self, instance, owner):
        failedvalue.fail()

    def __set__(self, instance, value):
        failedvalue.fail()

    def __delete__(self, instance):
        failedvalue.fail()

    def __sizeof__(self):
        failedvalue.fail()



def check(expr: str, universe: str, current_sub_path: sharedint, globals=globals(), locals=locals()):
    current = current_sub_path.value
    current_sub_path.increment()
    try:
        return eval(expr, globals, locals)
    except TypeShapeError as e:
        print(f"Path {universe}.{str(current)} failed with a ShapeError saying:")
        print(e)
        return failedvalue()
    except TypeError as e:
        print(f"Path {universe}.{str(current)} failed with a TypeError saying:")
        print(e)
        return failedvalue()
    except AttributeError as e:
        print(f"Path {universe}.{str(current)} failed with an AttributeError saying:")
        print(e)
        return failedvalue()
    except failedvalue:
        print(f"Path {universe}.{str(current)} failed with a failedvalue exception")
        return failedvalue()

