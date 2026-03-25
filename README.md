# rsvm
A cool 32 bit VM for a custom ISA, written in Rust.

## ISA
| Opcode | Source1 | Source2 | Destination |
| --------------- | --------------- | --------------- | --------------- |
| put | - | - | Reg. to contain the next itm. in RAM |
| mov | Reg. to take from | - | Reg. to copy the value in from the source |
| printreg | Reg. to display | - | - |
| add | First reg. to add | Second reg. to add | Sum of the registers |

## Assembly language

> [!warn]
> The assembler is currently unfinished.

Concept:
```asm
[__start]
put @g0, =1        ; put the intermediate 1 into g0
put @g1, =2        ; put the intermediate 2 into g1
add @g0, @g1, @g3  ; sum them 
printreg @g3       ; display them (has a debug-like format)

exit
```

# To-Do
- All basic arithmetic operators (add, sub, mul, div) [ ]
- Instructions to convert betweeen float to int registers and back [ ]
- Basic assembler [ ]
- CLI tool [ ]
