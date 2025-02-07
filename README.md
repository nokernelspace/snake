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
Y      = length
X      = value / memory address
T      = type (byte/half-word/word) (8/16/32)
C      = constants
______ = register offset / pointer offset to RAM
 ...   = Arbitrary Data
{...}  = PowerPC Assembly (default assembler: `gas`)
{#asd} = Includes `asd.ppc` from `powerpc/` folder
```

## Assembler Format
    - Text Sections are seperated by `:`
    - arguments follow `<src> -> <dest>` format
    - Registers are formated as `$`
    - Constants can be previewed by `L'..', 0x, 0b`

## Instructions
Instruction Format
`<verb> (:<class>)+ <size of op> <p/b> args...`

```
<Direct Memory Access>
    - fill  8     ______ <XX>         <YYYY>
    - write 16 b  ______ <XXXX>       <YYYY>
    - write 16 p  ______ <XXXX>       <YYYY>
    - write 32 b  ______ <XXXXXXXX>
    - write 32 p  ______ <XXXXXXXX>
    - write    b  ______ <YYYY> ...
    - write    p  ______ <YYYY> ...
    - fill:b   p <XXXXXXXX> <ZZZZ> <NNN>
    - fill:hw  p <XXXXXXXX> <ZZZZ> <NNN>
    - fill:w   p <XXXXXXXX> <ZZZZ> <NNN>
    - fill:b   b <XXXXXXXX> <ZZZZ> <NNN>
    - fill:hw  b <XXXXXXXX> <ZZZZ> <NNN>
    - fill:w   b <XXXXXXXX> <ZZZZ> <NNN>
<If Operations>
    - if  32 b ______ == <XXXXXXXX>
    - if  32 p ______ == <XXXXXXXX>
    - if  32 b ______ != <XXXXXXXX>
    - if  32 p ______ != <XXXXXXXX>
    - if  32 b ______ >  <XXXXXXXX>
    - if  32 p ______ >  <XXXXXXXX>
    - if  32 b ______ <  <XXXXXXXX>
    - if  32 p ______ <  <XXXXXXXX>
    - if  32 b ______ >= <XXXXXXXX>
    - if  32 p ______ >= <XXXXXXXX>
    - if  32 b ______ <= <XXXXXXXX>
    - if  32 p ______ <= <XXXXXXXX>
    - if  64 b ______ == <XXXXXXXX>
    - if  64 p ______ == <XXXXXXXX>
    - if  64 b ______ != <XXXXXXXX>
    - if  64 p ______ != <XXXXXXXX>
    - if  64 b ______ >  <XXXXXXXX>
    - if  64 p ______ >  <XXXXXXXX>
    - if  64 b ______ <  <XXXXXXXX>
    - if  64 p ______ <  <XXXXXXXX>
    - if  64 b ______ >= <XXXXXXXX>
    - if  64 p ______ >= <XXXXXXXX>
    - if  64 b ______ <= <XXXXXXXX>
    - if  64 p ______ <= <XXXXXXXX>
    - else/endif
<BA Register Operations>
    - load  1 b $ba =        [XXXXXXXX]
    - load  2 b $ba =        [XXXXXXXX]
    - load  3 b $ba =        [XXXXXXXX]
    - load  4 b $ba =        [XXXXXXXX]
    - load  1 b $ba +=       [XXXXXXXX]
    - load  2 b $ba +=       [XXXXXXXX]
    - load  3 b $ba +=       [XXXXXXXX]
    - load  4 b $ba +=       [XXXXXXXX]
    - load  1 p $ba =        [XXXXXXXX]
    - load  2 p $ba =        [XXXXXXXX]
    - load  1 p $ba +=       [XXXXXXXX]
    - load  2 p $ba +=       [XXXXXXXX]

    - store 1 b $ba =        <XXXXXXXX>
    - store 2 b $ba =        <XXXXXXXX>
    - store 3 b $ba =        <XXXXXXXX>
    - store 4 b $ba =        <XXXXXXXX>
    - store 1 b $ba +=       <XXXXXXXX>
    - store 2 b $ba +=       <XXXXXXXX>
    - store 3 b $ba +=       <XXXXXXXX>
    - store 4 b $ba +=       <XXXXXXXX>
    - store 1 p $ba =        <XXXXXXXX>
    - store 2 p $ba =        <XXXXXXXX>
    - store 1 p $ba +=       <XXXXXXXX>
    - store 2 p $ba +=       <XXXXXXXX>

    - store 1   [xxxxxxxx]  =  $ba
    - store 2   [xxxxxxxx]  =  $ba
    - store 3   [xxxxxxxx]  =  $ba
    - store 4   [xxxxxxxx]  =  $ba
    - store 5   [xxxxxxxx]  =  $ba
    - store 6   [xxxxxxxx]  =  $ba

    - rip   <XXXX> $ba
<PO Register Operations>
    - load  1 b $po=        [XXXXXXXX]
    - load  2 b $po=        [XXXXXXXX]
    - load  3 b $po=        [XXXXXXXX]
    - load  4 b $po=        [XXXXXXXX]
    - load  1 b $po+=       [XXXXXXXX]
    - load  2 b $po+=       [XXXXXXXX]
    - load  3 b $po+=       [XXXXXXXX]
    - load  4 b $po+=       [XXXXXXXX]
    - load  1 p $po=        [XXXXXXXX]
    - load  2 p $po=        [XXXXXXXX]
    - load  1 p $po+=       [XXXXXXXX]
    - load  2 p $po+=       [XXXXXXXX]


    - store 1 $po=   b     <XXXXXXXX>
    - store 2 $po=   b     <XXXXXXXX>
    - store 3 $po=   b     <XXXXXXXX>
    - store 4 $po=   b     <XXXXXXXX>
    - store 1 $po+=  b     <XXXXXXXX>
    - store 2 $po+=  b     <XXXXXXXX>
    - store 3 $po+=  b     <XXXXXXXX>
    - store 4 $po+=  b     <XXXXXXXX>
    - store 1 $po=   p     <XXXXXXXX>
    - store 2 $po=   p     <XXXXXXXX>
    - store 1 $po+=  p     <XXXXXXXX>
    - store 2 $po+=  p     <XXXXXXXX>

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
    - =              g$     <XXXXXXXX>
    - =      b       g$     <XXXXXXXX>
    - =      p       g$     <XXXXXXXX>
    - +=             g$     <XXXXXXXX>
    - +=     b       g$     <XXXXXXXX>
    - +=     p       g$     <XXXXXXXX>
    - load   b       g$     <XXXXXXXX>
    - load   b       g$     <XXXXXXXX>
    - load   b       g$     <XXXXXXXX>
    - load   b 8     g$     <XXXXXXXX>
    - load   b 16    g$     <XXXXXXXX>
    - load   b 32    g$     <XXXXXXXX>
    - load   p 8     g$     <XXXXXXXX>
    - load   p 16    g$     <XXXXXXXX>
    - load   p 32    g$     <XXXXXXXX>
    - store    8     g$     <XXXXXXXX>
    - store    16    g$     <XXXXXXXX>
    - store    32    g$     <XXXXXXXX>
    - store  b 8     g$     <XXXXXXXX>
    - store  b 16    g$     <XXXXXXXX>
    - store  b 32    g$     <XXXXXXXX>
    - store  p 8     g$     <XXXXXXXX>
    - store  p 16    g$     <XXXXXXXX>
    - store  p 32    g$     <XXXXXXXX>
    - const  C +     g$
    - const  C *     g$
    - const  C |     g$
    - const  C &     g$
    - const  C ^     g$
    - const  C <<    g$
    - const  C >>    g$
    - const  C rol   g$
    - const  C asr   g$
    - const  C fadds g$
    - const  C fmuls g$
    - +       g$  g$
    - *       g$  g$
    - |       g$  g$
    - &       g$  g$
    - ^       g$  g$
    - <<      g$  g$
    - >>      g$  g$
    - rol     g$  g$
    - asr     g$  g$
    - fadds   g$  g$
    - fmuls   g$  g$
    - memcopy  <YYYY>  g$
    - memcopyb <YYYY>  g$
    - memcopyp <YYYY>  g$
    - memcopy  <YYYY>  g$    g$         <XXXXXXXX>
    - memcopyb <YYYY>  g$    <XXXXXXXX>
    - memcopyp <YYYY>  g$    <XXXXXXXX>
    - memcopy  <YYYY>  g$    <XXXXXXXX> g$
    - memcopyb <YYYY>  <XXXXXXXX> g$
    - memcopyp <YYYY>  <XXXXXXXX> g$

<Counter Registers> (Useful for putting an upper bound on the Halting Problem :)
    - count 0 16  <XXXX> <MMMM> =  [<ZZZZ>]
    - count 1 16  <XXXX> <MMMM> =  [<ZZZZ>]
    - count 8 16  <XXXX> <MMMM> =  [<ZZZZ>]
    - count 9 16  <XXXX> <MMMM> =  [<ZZZZ>]

    - count 0 16  <XXXX> <MMMM> != [<ZZZZ>]
    - count 1 16  <XXXX> <MMMM> != [<ZZZZ>]
    - count 8 16  <XXXX> <MMMM> != [<ZZZZ>]
    - count 9 16  <XXXX> <MMMM> != [<ZZZZ>]

    - count 0 16  <XXXX> <MMMM> >  [<ZZZZ>]
    - count 1 16  <XXXX> <MMMM> >  [<ZZZZ>]
    - count 8 16  <XXXX> <MMMM> >  [<ZZZZ>]
    - count 9 16  <XXXX> <MMMM> >  [<ZZZZ>]

    - count 0 16  <XXXX> <MMMM> <  [<ZZZZ>]
    - count 1 16  <XXXX> <MMMM> <  [<ZZZZ>]
    - count 8 16  <XXXX> <MMMM> <  [<ZZZZ>]
    - count 9 16  <XXXX> <MMMM> <  [<ZZZZ>]
<ASM>
    - execute {..}
    - inject ______ {..}
    - hook   ______ <YYYYYYYY>
<Other>
    - toggle
    - ifrange <XXXX> <YYYY>
    - clear
    - else0
    - else1
    - end
<Gecko 1.8+ Only>
    - inject ______ {..} <ZZZZ>
    - search <XXXX> <YYYY> {..}
```

## Tips
1) When writting ASM use BA instructions for tmp values and PO instructions for values in RAM
