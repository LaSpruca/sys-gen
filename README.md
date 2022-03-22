# SYS-GEN
---
I noticed that JS contained no bindings for syscalls, so I decided to make one myself! This project uses sys.yaml to
generate syscall bindings using rust and [neon](https://neon-bindings.com/).
This project is still under construction so any help would be appreciated

## sys.yaml
The sys.yaml file consists of one part atm, `calls`.
Each call contains the following fields:
- `n`: The number of the syscall
- `name`: The name of the syscall (can be arbitrary, but wouldn't recommend)
- `params`: The parameters for the generated function
- `args`: The arguments to be passed to the syscall
  - `[0]`: The way you're accessing the value, `ptr` (As a pointer), `val` (As a value), `len` (Call .len (from rust) on the thing)
  - `[1]`: The value you are assigning
- `return`: The return type of the function
- `effects`: *Optional* Any values that might be mutated during the process

**E.G.**
```yaml
calls:
  - n: 0
    name: read
    params:
      fp: usize
      bytes: [u8]
    args:
      - [val, fp]
      - [ptr, bytes]
      - [len, bytes]
    effects: [bytes]
    return: usize
```