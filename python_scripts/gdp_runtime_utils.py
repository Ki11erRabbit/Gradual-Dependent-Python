
class DependentTypeError(TypeError):
    def __init__(self, message):
        super().__init__(message)

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
            out += " ".join(map(lambda x: "where " + x, self.has_clauses))
        if self.has_clauses is not None:
            out += "has " + self.has_clauses.__repr__()
        if self.when_clauses is not None:
            out += " ".join(map(lambda x: "when " + x, self.has_clauses))


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

