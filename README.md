# Snake
## A Gecko Code Assembler  written in Rust

Gecko is a cheat-code specification popularized by modchips and Action Replay for the Nintendo Wii
It has since been made freely aviable for use in the Dolphin Emulator allow users to easily achive ACE in their favorite games
Why use an assembler? Because I'm not writting my code with just numbers bro

Gecko is fundementally a bootloader that implements 4 registers
**Base Addresses**, **Pointer Offsets**,  **Gecko Registers**, & **Counter Registers**

**Base Addresses (`BA`)** can be thought of as analagous to `rbp` or `ebp` in _ARM_ or _x86_ respectively
By controlling the Base Address we can controll the execution flow and achive ACE. How we achieve ACE can varry from game-to-game.

**Pointers Offsets (`PO`)** are required to index beyond 0x01000000 (16)

**Gecko Registers (`GR`)** are used for their own comparison operations (16)

**Counter Registers (`CR[1-16]`)**  and are like GR but there are 16 of them

Keep in mind that BA instructions  can only be indexed until 0x01000000

This Gecko code specification can be expanded upon to create consumer focused applications for all platforms

# Snake -> Gecko Mapping

## Symbols Sheet
```
Y      = length
X      = value
______ = register offset
...    = byte array
```


## Instructions
```
<Direct Memory Access>
    - f8    ______ <XX>         <YYYY>
    - w16b  ______ <XXXX>       <YYYY>
    - w16p  ______ <XXXX>       <YYYY>
    - w32p         <XXXXXXXX>
    - wb    <YYYY> ...
    - wp    <YYYY> ...
    - fillp <XXXXXXXX> <T> <ZZZZ> <NNN>
    - fillb <XXXXXXXX> <T> <ZZZZ> <NNN>
<If Operations>
<Gecko Register Operations
<BA Register Operations>
<PO Register Operations>
<Flow Control>
<Gecko Registers>
<Counter Registers>
<ASM>
<Other>
```
