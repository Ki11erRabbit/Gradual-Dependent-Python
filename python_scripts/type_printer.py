def type_printer(x, parents=[]):
    def list_printer(x):
        parents.append(x)
        output = "["
        for i, item in enumerate(x):
            if i != 0:
                output += ", "
            if item in parents:
                output += "self"
                continue
            output += type_printer(item)
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
                output += "self"
                continue
            output += type_printer(item)
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
                output += "self"
                continue
            output += type_printer(item)
            output += ": "
            output += type_printer(x[item])
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
                output += "self"
                continue
            output += type_printer(item)
        output += "}"
        parents.pop()
        return output
    if isinstance(x, int):
        return f"int({x})"
    elif isinstance(x, float):
        return f"float({x})"
    elif isinstance(x, str):
        return f"str({x})"
    elif isinstance(x, list):
        return f"list({list_printer(x)})"
    elif isinstance(x, tuple):
        return f"tuple({tuple_printer(x)})"
    elif isinstance(x, dict):
        return f"dict({dict_printer(x)})"
    elif isinstance(x, set):
        return f"set({set_printer(list(x))})"
    elif isinstance(x, bool):
        return f"bool({x})"
    elif isinstance(x, complex):
        return f"complex({x})"
    elif isinstance(x, bytes):
        return f"bytes({x})"
    elif isinstance(x, bytearray):
        return f"bytearray({x})"
    elif isinstance(x, range):
        return f"range({x})"
    elif isinstance(x, slice):
        return f"slice({x})"
    elif isinstance(x, memoryview):
        return f"memoryview({x})"
    elif isinstance(x, type):
        return f"type({x})"
    elif isinstance(x, Exception):
        return f"{x.__class__}({x})"
    elif isinstance(x, object):
        return f"{x.__class__}({x})"
    else:
        return type(x).__class__