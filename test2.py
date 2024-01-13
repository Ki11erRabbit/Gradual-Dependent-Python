import math
import random
from collections import Counter
from functools import reduce
name = 'John'
age = 25
print(f"'Hello, '{name}'! You are '{age}' years old.'")
if age >= 18:
    print('You are an adult.')
else:
    print('You are a minor.')

for i in range(5):
    print(f"'Loop iteration '{i}")

def square(x):
    return x ** 2

async def square(x):
    return x ** 2

result = square(4)
print(f"'The square of 4 is '{result}")
numbers_list = [1, 2, 3, 4, 5]
numbers_tuple = (6, 7, 8, 9, 10)
numbers_set = {11, 12, 13, 14, 15}
squared_numbers = [square(num) for num in numbers_list]
person = {'name': 'Alice', 'age': 30, 'city': 'Wonderland'}
class Dog:
    def __init__(self, name):
        self.name = name

    def bark(self):
        print('Woof!')


dog_instance = Dog('Buddy')
dog_instance.bark()
try:
    result = 10 / 0
except ZeroDivisionError as e:
    print(f"'Error: '{e}")


sqrt_result = math.sqrt(25)
random_number = random.randint(1, 100)
word_count = Counter('hello world')
add_one = lambda x: x + 1
numbers = [1, 2, 3, 4]
mapped_numbers = list(map(add_one, numbers))
sum_of_numbers = reduce(lambda x, y: x + y, numbers)
with open('example.txt', 'w') as file:
    file.write('Hello, Python!')

print(f"'Squared numbers: '{squared_numbers}")
print(f"'Square root of 25: '{sqrt_result}")
print(f"'Random number between 1 and 100: '{random_number}")
print(f"'Word count: '{word_count}")
print(f"'Mapped numbers: '{mapped_numbers}")
print(f"'Sum of numbers: '{sum_of_numbers}")