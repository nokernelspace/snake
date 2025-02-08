# Snake
## A Gecko Code Assembler  written in Rust

Gecko is a cheat-code specification popularized by modchips and Action Replay for the Nintendo Wii and Gamecube
It has since been made freely aviable for use in the Dolphin Emulator allow users to easily achive ACE in their favorite games
Why use an assembler? Because I'm not writting my code with just numbers bro

Gecko is fundementally a bootloader that implements 4 registers
**Base Addresses**, **Pointer Offsets**,  **Gecko Registers**, & **Counter Registers**

**Base Addresses (`BA`)** can be thought of as analagous to `rbp` or `ebp` in _ARM_ or _x86_ respectively
By controlling the Base Address we can controll the execution flow and achive ACE. How we achieve ACE can vary from game-to-game. When using a gecko "return" `rip = $ba`

**Pointers Offsets (`PO`)** are required to index beyond 0x01000000

**Gecko Registers (`g[0-15]`)**  General Purpose Registers (16)

**Counter Registers ('c[0-255])** unaccessible and counts stuff depending on the _Comparison Register_

**Comparision Register** - unaccessible and typically called the _execution register_. Store's the result of comparisons

Keep in mind that `$BA` instructions  can only be indexed until 0x01000000

This Gecko code specification can be expanded upon to create consumer focused applications for all platforms

Gecko is best thought of as a Runtime similar to that of the C standard library for PowerPC architectures. Its primary purpose is cheating however a memory allocator can be added via `snake install reed`

# Usage
`snake build   <project root>`
`snake clean   <project root>`
`snake release <project root>`
`snake publish <project root>`
`snake install <project root>`

# Snake -> Gecko Mapping

This is a reference to all opcode instructions for Gecko

## Symbols Sheet
```
Y       = length
X       = value / memory address
T       = type (byte/half-word/word) (8/16/32)
C       = constants
______  = register offset / pointer offset to RAM
 ...    = Arbitrary Data
{...}   = PowerPC Assembly (default assembler: `gas`)
{#asd}  = Includes `asd.ppc` from `powerpc/` folder
p <XXXX> =   PO+XXXX  | Scalar
b <XXXX> =   BA+XXXX  | Scalar
b [XXXX] =  [BA+XXXX] | Map
p [XXXX] =  [PO+XXXX] | Map
g$[XXXX] =  [g$+XXXX]
```

## Assembler Format
    - Text Sections are seperated by `:`
    - arguments follow `<src> -> <dest>` format
    - Registers are formated as `$`
    - Constants can be previewed by `L'..', 0x, 0b`

## Instructions
Instruction Format
`<verb> (:<class>)+ <size of op> <p/b> args...`

Each Instruction has a different OpCode
``

## Tips
1) When writting ASM use BA instructions for tmp values and PO instructions for values in RAM
