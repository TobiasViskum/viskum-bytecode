# viskum-bytecode

This the is the bytecode virtual machine of the "Viskum" programming language

## Optimizations

- Store the rules in a hashmap that's initialized once (so the same rule doesn't have to be recreated countless times)
