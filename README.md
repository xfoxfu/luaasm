# luaasm

Assembler for Lua Bytecode

## Example

```
.fn(R0)

.instruction
GETTABUP  R1 U0 K1
GETTABLE  R1 R1 K2
GETTABLE  R2 R0 K3
CALL      R1 2 2
SETTABLE  R0 K0 R1
RETURN    R0 1

.const
K0 = "accuracy_flash"
K1 = "Accuracy"
K2 = "create"
K3 = "task_form"

.upvalue
U0 = L1 R23

.endfn
```

- function with variable arguments should be declared like `.fn(R0, __va_args__)`
- nested function should be placed between end of `.upvalue` and `.endfn`
- you may produce luaasm samples with [PyLuaDec-fork](https://github.com/coderfox/PyLuaDec) `python3 dis.py <LUA_BYTECODE_FILE> luaasm`
