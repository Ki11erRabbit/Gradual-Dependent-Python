# Type Checking Limitations


* Any code that is called during while loops and non-total for loops
  * This is because we cannot guarantee that the code will terminate
  * The best we can do is give the variables the unknown type if mutated within a loop
* Any code that is recursive
  * This requires some evidence passing to ensure that the code terminates