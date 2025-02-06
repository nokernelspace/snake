# Snake
## A Gecko Code Assembler  written in Rust

Gecko is a cheat-code specification popularized by modchips and Action Replay for the Nintendo Wii
It has since been made freely aviable for use in the Dolphin Emulator allow users to easily achive ACE in their favorite games
Why use an assembler? Because I'm not writting my code with just numbers bro

Gecko is fundementally a bootloader that implements 4 registers
**Base Addresses**, **Pointer Offsets**,  **Gecko Registers**, & **Counter Registers**

**Base Addresses (`BA`)** can be thought of as analagous to `rbp` or `ebp` in _ARM_ or _x86_ respectively
By controlling the Base Address we can controll the execution flow and achive ACE. How we achieve ACE can vary from game-to-game. When using a gecko "return" `rip = $ba`

**Pointers Offsets (`PO`)** are required to index beyond 0x01000000

**Gecko Registers (`g[1-16]`)**  General Purpose Registers (16)

**Counter Registers** unaccessible and represects the program counter

**Comparision Register** - unaccessible and typically called the _execution register_. Store's the result of comparisons

Keep in mind that `$BA` instructions  can only be indexed until 0x01000000

This Gecko code specification can be expanded upon to create consumer focused applications for all platforms

# Snake -> Gecko Mapping

## Symbols Sheet
```
Y      = length
X      = value / memory address
T      = type (byte/half-word/word) (8/16/32)
C      = constants
______ = register offset / pointer offset to RAM
...    = byte array
```

## Assembler Format
    - Text Sections are seperated by `:`
    - arguments follow `<src> -> <dest>` format
    - Registers are formated as `$`
    - Constants can be previewed by `L'..', 0x, 0b`

## Instructions
```
<Direct Memory Access>
    - fill  8    ______ <XX>         <YYYY>
    - write 16b  ______ <XXXX>       <YYYY>
    - write 16p  ______ <XXXX>       <YYYY>
    - write 32b  ______ <XXXXXXXX>
    - write 32p  ______ <XXXXXXXX>
    - write b    ______ <YYYY> ...
    - write p    ______ <YYYY> ...
    - fill  p <XXXXXXXX> <T> <ZZZZ> <NNN>
    - fill  b <XXXXXXXX> <T> <ZZZZ> <NNN>
<If Operations>
    - if  e b32  ______ <XXXXXXXX>
    - if  e p32  ______ <XXXXXXXX>
    - if ne b32  ______ <XXXXXXXX>
    - if ne p32  ______ <XXXXXXXX>
    - if  g b32  ______ <XXXXXXXX>
    - if  g p32  ______ <XXXXXXXX>
    - if  l b32  ______ <XXXXXXXX>
    - if  l p32  ______ <XXXXXXXX>
    - if  geb32  ______ <XXXXXXXX>
    - if  gep32  ______ <XXXXXXXX>
    - if  leb32  ______ <XXXXXXXX>
    - if  lep32  ______ <XXXXXXXX>
    - if  e b64  ______ <XXXXXXXX>
    - if  e p64  ______ <XXXXXXXX>
    - if ne b64  ______ <XXXXXXXX>
    - if ne p64  ______ <XXXXXXXX>
    - if  g b64  ______ <XXXXXXXX>
    - if  g p64  ______ <XXXXXXXX>
    - if  l b64  ______ <XXXXXXXX>
    - if  l p64  ______ <XXXXXXXX>
    - if  geb64  ______ <XXXXXXXX>
    - if  gep64  ______ <XXXXXXXX>
    - if  leb64  ______ <XXXXXXXX>
    - if  lep64  ______ <XXXXXXXX>
    - else/endif
<BA Register Operations>
    - load  $ba=   1b     [XXXXXXXX]
    - load  $ba=   2b     [XXXXXXXX]
    - load  $ba=   3b     [XXXXXXXX]
    - load  $ba=   4b     [XXXXXXXX]
    - load  $ba+=  1b     [XXXXXXXX]
    - load  $ba+=  2b     [XXXXXXXX]
    - load  $ba+=  3b     [XXXXXXXX]
    - load  $ba+=  4b     [XXXXXXXX]
    - load  $ba=   1p     [XXXXXXXX]
    - load  $ba=   2p     [XXXXXXXX]
    - load  $ba+=  1p     [XXXXXXXX]
    - load  $ba+=  2p     [XXXXXXXX]

    - store $ba=   1b     <XXXXXXXX>
    - store $ba=   2b     <XXXXXXXX>
    - store $ba=   3b     <XXXXXXXX>
    - store $ba=   4b     <XXXXXXXX>
    - store $ba+=  1b     <XXXXXXXX>
    - store $ba+=  2b     <XXXXXXXX>
    - store $ba+=  3b     <XXXXXXXX>
    - store $ba+=  4b     <XXXXXXXX>
    - store $ba=   1p     <XXXXXXXX>
    - store $ba=   2p     <XXXXXXXX>
    - store $ba+=  1p     <XXXXXXXX>
    - store $ba+=  2p     <XXXXXXXX>

    - store 1 [xxxxxxxx]=  $ba
    - store 2 [xxxxxxxx]=  $ba
    - store 3 [xxxxxxxx]=  $ba
    - store 4 [xxxxxxxx]=  $ba
    - store 5 [xxxxxxxx]=  $ba
    - store 6 [xxxxxxxx]=  $ba

    - rip   <XXXX> $ba
<PO Register Operations>
    - load   $po=   1b     [XXXXXXXX]
    - load   $po=   2b     [XXXXXXXX]
    - load   $po=   3b     [XXXXXXXX]
    - load   $po=   4b     [XXXXXXXX]
    - load   $po+=  1b     [XXXXXXXX]
    - load   $po+=  2b     [XXXXXXXX]
    - load   $po+=  3b     [XXXXXXXX]
    - load   $po+=  4b     [XXXXXXXX]
    - load   $po=   1p     [XXXXXXXX]
    - load   $po=   2p     [XXXXXXXX]
    - load   $po+=  1p     [XXXXXXXX]
    - load   $po+=  2p     [XXXXXXXX]


    - store  $po=   1b     <XXXXXXXX>
    - store  $po=   2b     <XXXXXXXX>
    - store  $po=   3b     <XXXXXXXX>
    - store  $po=   4b     <XXXXXXXX>
    - store  $po+=  1b     <XXXXXXXX>
    - store  $po+=  2b     <XXXXXXXX>
    - store  $po+=  3b     <XXXXXXXX>
    - store  $po+=  4b     <XXXXXXXX>
    - store  $po=   1p     <XXXXXXXX>
    - store  $po=   2p     <XXXXXXXX>
    - store  $po+=  1p     <XXXXXXXX>
    - store  $po+=  2p     <XXXXXXXX>

    - store 1 [xxxxxxxx]=  g$
    - store 2 [xxxxxxxx]=  g$
    - store 3 [xxxxxxxx]=  g$
    - store 4 [xxxxxxxx]=  g$
    - store 5 [xxxxxxxx]=  g$
    - store 6 [xxxxxxxx]=  g$

    - rip   <XXXX> $po
<Flow Control>
    - setbound  <XXXX>
    - repeat
    - return
    - goto 1 <XXXX>
    - goto 2 <XXXX>
    - goto 3 <XXXX>
    - goto 4 <XXXX>
    - call   <XXXX>
<Gecko Registers>
    - =            g$     <XXXXXXXX>
    - =      b     g$     <XXXXXXXX>
    - =      p     g$     <XXXXXXXX>
    - +=           g$     <XXXXXXXX>
    - +=     b     g$     <XXXXXXXX>
    - +=     p     g$     <XXXXXXXX>
    - load   b     g$     <XXXXXXXX>
    - load   b     g$     <XXXXXXXX>
    - load   b     g$     <XXXXXXXX>
    - load   b8    g$     <XXXXXXXX>
    - load   b16   g$     <XXXXXXXX>
    - load   b32   g$     <XXXXXXXX>
    - load   p8    g$     <XXXXXXXX>
    - load   p16   g$     <XXXXXXXX>
    - load   p32   g$     <XXXXXXXX>
    - store  8     g$     <XXXXXXXX>
    - store  16    g$     <XXXXXXXX>
    - store  32    g$     <XXXXXXXX>
    - store  8     g$ b   <XXXXXXXX>
    - store  16    g$ b   <XXXXXXXX>
    - store  32    g$ b   <XXXXXXXX>
    - store  8     g$ p   <XXXXXXXX>
    - store  16    g$ p   <XXXXXXXX>
    - store  32    g$ p   <XXXXXXXX>
    - const  +     g$ C
    - const  *     g$ C
    - const  |     g$ C
    - const  &     g$ C
    - const  ^     g$ C
    - const  <<    g$ C
    - const  >>    g$ C
    - const  rol   g$ C
    - const  asr   g$ C
    - const  fadds g$ C
    - const  fmuls g$ C
    - +      g$  g$
    - *      g$  g$
    - |      g$  g$
    - &      g$  g$
    - ^      g$  g$
    - <<     g$  g$
    - >>     g$  g$
    - rol    g$  g$
    - asr    g$  g$
    - fadds  g$  g$
    - fmuls  g$  g$
    - memcopy  <YYYY>  g$
    - memcopyb <YYYY>  g$
    - memcopyp <YYYY>  g$
    - memcopy  <YYYY>  g$    g$         <XXXXXXXX>
    - memcopyb <YYYY>  g$    <XXXXXXXX>
    - memcopyp <YYYY>  g$    <XXXXXXXX>
    - memcopy  <YYYY>  g$    <XXXXXXXX> g$
    - memcopyb <YYYY>  <XXXXXXXX> g$
    - memcopyp <YYYY>  <XXXXXXXX> g$

<Counter Registers>
<ASM>
    - execute {..}
    - inject ______ {..}
    - hook   ______ <YYYYYYYY>
<Other>
```

## Tips
1) When writting ASM use BA instructions for tmp values and PO instructions for values in RAM
