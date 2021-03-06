# SYS-GEN

--------
I noticed that JS contained no bindings for syscalls, so I decided to make one myself! This project uses sys.yaml to
generate syscall bindings using rust and [neon](https://neon-bindings.com/).
This project is still under construction so any help would be appreciated.

This project consists of two crates:`sys-gen`, the crate responsible for generating the rust code, and the
`syscall` crate, which uses a build.rs script to generate the rust code as well as an index.d.ts and index.js.

By default, the build.rs file of `syscall` will look for the syscall definitions in `../sys.yaml`, this can
be changed however with the `SYS_YAML_PATH` environment variable.

## sys.yaml
The sys.yaml file consists of two parts, `calls` and `enums`.

Each `enums` section contains a list of objects with the name of the enum as the key and then each field as a k/v pair.

**E.G.**
```yaml
enums:
  OpenFlags:
    ReadOnly: 0
    WriteOnly: 1
    # ...
```

Each `call` section contains the following fields:
- `n`: The number of the syscall
- `name`: The name of the syscall (can be arbitrary, but wouldn't recommend)
- `params`: The parameters for the generated function
  - `[0]`: The args name
  - `[1]`: The args type, prefix with `$` to access enum
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
      - [fp, usize]
      - [bytes, "[u8]"]
    args:
      - [val, fp]
      - [ptr, bytes]
      - [len, bytes]
    effects: [bytes]
    return: usize
```