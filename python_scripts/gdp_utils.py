

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



class DependentTypeError(TypeError):
    def __init__(self, message):
        super().__init__(message)


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
        return "failedvalue"

    def __get__(self, instance, owner):
        failedvalue.fail()

    def __set__(self, instance, value):
        failedvalue.fail()

    def __delete__(self, instance):
        failedvalue.fail()

    def __sizeof__(self):
        failedvalue.fail()


class DependentType:
    def __init__(self, type_name, var_name, value=None, where_clauses=None, has_clauses=None, when_clauses=None):
        self.type_name = type_name
        self.value = value
        self.var_name = var_name
        self.where_clauses = where_clauses
        self.has_clauses = has_clauses
        self.when_clauses = when_clauses

    def __repr__(self):
        out = f"{self.type_name}"
        if self.value is not None:
            out += f"({self.format_value()}"
        if self.where_clauses is not None:
            out += " | ".join(map(lambda x: "where " + x, self.has_clauses))
        if self.has_clauses is not None:
            out += "has " + self.has_clauses.__repr__()
        if self.when_clauses is not None:
            out += " | ".join(map(lambda x: "when " + x, self.has_clauses))


        out += ")"
        return out

    def format_value(self, parents=[]):

        def list_printer(x):
            parents.append(x)
            output = "["
            for i, item in enumerate(x):
                if i != 0:
                    output += ", "
                if item in parents:
                    output += "[...]"
                    continue
                output += self.format_value(item)
            output += "]"
            parents.pop()
            return output

        def tuple_printer(x):
            parents.append(x)
            output = "("
            for i, item in enumerate(x):
                if i != 0:
                    output += ", "
                if item in parents:
                    output += "(...)"
                    continue
                output += self.format_value(item)(item)
            output += ")"
            parents.pop()
            return output

        def dict_printer(x):
            parents.append(x)
            output = "{"
            for i, item in enumerate(x):
                if i != 0:
                    output += ", "
                if item in parents:
                    output += "{...}"
                    continue
                output += self.format_value(item)(item)
                output += ": "
                output += self.format_value(item)(x[item])
            output += "}"
            parents.pop()
            return output

        def set_printer(x):
            parents.append(x)
            output = "{"
            for i, item in enumerate(x):
                if i != 0:
                    output += ", "
                if item in parents:
                    output += "{...}"
                    continue
                output += self.format_value(item)(item)
            output += "}"
            parents.pop()
            return output

        if isinstance(self.value, int):
            return f"({self.value})"
        elif isinstance(self.value, float):
            return f"({self.value})"
        elif isinstance(self.value, str):
            return f"({self.value})"
        elif isinstance(self.value, list):
            return f"({list_printer(self.value)})"
        elif isinstance(self.value, tuple):
            return f"({tuple_printer(self.value)})"
        elif isinstance(self.value, dict):
            return f"({dict_printer(self.value)})"
        elif isinstance(self.value, set):
            return f"({set_printer(list(self.value))})"
        elif isinstance(self.value, bool):
            return f"({self.value})"
        elif isinstance(self.value, complex):
            return f"({self.value})"
        elif isinstance(self.value, bytes):
            return f"({self.value})"
        elif isinstance(self.value, bytearray):
            return f"({self.value})"
        elif isinstance(self.value, object):
            return f"({self.value})"
        else:
            return type(self.value).__name__

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

    def is_reducible(self, locals, var_name):
        if isinstance(locals[var_name], UnknownTerm):
            return

        if self.type_name == 'Any' and self.where_clauses is None and self.has_clauses is None and self.when_clauses is None:
            return

        if type(locals[var_name]).__name__ != self.type_name:
            print(f"Value {type(locals[var_name]).__name__} is not of type {self.type_name}")
            raise DependentTypeError(f"Value {locals[var_name]} is not beta-eta reducible to {self.type_name}")
        if self.where_clauses is not None:
            for clause in self.where_clauses:
                if not eval(clause, None, locals):
                    raise DependentTypeError(f"Value {locals[var_name]} is not beta-eta reducible to {self.type_name} {clause}")

        if self.has_clauses is not None:
            for clause in self.has_clauses:
                if not hasattr(locals[var_name], clause):
                    raise DependentTypeError(f"Value {locals[var_name]} is not beta-eta reducible to {self.type_name} {clause}")

        #TODO: implement when clauses




class Function:
    def __init__(self, function, name: str, argument_types: list[DependentType], keyword_argument_types: dict[str: DependentType], return_type: DependentType):
        self.function = function
        self.name = name
        self.argument_types = argument_types
        self.keyword_argument_types = keyword_argument_types
        self.return_type = return_type

    def __call__(self, *args, **kwargs):

        arguments = {}
        for i, arg in enumerate(args):
            arguments[self.argument_types[i].var_name] = arg

        for kwarg in kwargs:
            arguments[kwarg] = kwargs[kwarg]

        for i, arg in enumerate(args):
            self.argument_types[i].is_reducible(arguments, self.argument_types[i].var_name)

        for kwarg in kwargs:
            self.keyword_argument_types[kwarg].check_value(arguments, kwarg)

        return_value = self.function(*args, **kwargs)
        arguments[self.name] = return_value
        self.return_type.is_reducible(locals=arguments, var_name=self.name)
        return return_value




def check(expr: str, path: str, current_sub_path: sharedint, globals=globals(), locals=locals()):
    current = current_sub_path.value
    current_sub_path.increment()
    try:
        return eval(expr, globals, locals)
    except DependentTypeError as e:
        print(f"Path {path}.{str(current)} failed with a DependentTypeError saying:")
        print(e)
        return failedvalue()
    except TypeError as e:
        print(f"Path {path}.{str(current)} failed with a TypeError saying:")
        print(e)
        return failedvalue()
    except AttributeError as e:
        print(f"Path {path}.{str(current)} failed with an AttributeError saying:")
        print(e)
        return failedvalue()
    except failedvalue as e:
        print(f"Path {path}.{str(current)} failed with a failedvalue exception")
        return e






