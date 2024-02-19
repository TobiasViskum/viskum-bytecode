# viskum-bytecode

This the is the bytecode virtual machine of the "Viskum" programming language

## Todo

### 1 (OpConstantLong)

- Implement OpConstantLong

## Optimizations

### 1 (false - it's only present in development)

Why it's not happpening:

- The rust compiler optimizes the code, so no matter how many times a constant is added, it always takes between 0.1 and 0.3 miliseconds. So very quick

- 1+1+1+1+1 takes 0.5ms while 1+1+....1+1 260 times takes 9ms. BAD PERFORMANCE
  - Could be due to O(n) lookup times. Maybe use pointers instead of indexing, however this involves unsafe rust.
