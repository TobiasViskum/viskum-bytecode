# viskum-bytecode

This the is the bytecode virtual machine of the "Viskum" programming language

## Optimizations

### 1

- 1+1+1+1+1 takes 0.5ms while 1+1+....1+1 260 times takes 9ms. BAD PERFORMANCE
  - Could be due to O(n) lookup times. Maybe use pointers instead of indexing, however this involve unsafe rust.
