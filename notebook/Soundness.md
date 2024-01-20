# Soundness


## Things that are not Sound
* Any code in a loop that is (potentially) non-total
* Any code that is recursive
 

## Things that are Sound
* Any code that is not in a loop or is recursive
* Most for loops
  * Most for loops are going to be total which means that we can safely execute their code.
  * 