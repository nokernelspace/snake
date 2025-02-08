```
<Direct Memory Access>
    - fill  8   ______ <XX>         <YYYY>
    - write 16 b______ <XXXX>       <YYYY>
    - write 16 p______ <XXXX>       <YYYY>
    - write 32 b______ <XXXXXXXX>
    - write 32 p______ <XXXXXXXX>
    - write    b______ <YYYY> ...
    - write    p______ <YYYY> ...
    - fill:b   p<XXXXXXXX> <ZZZZ> <NNN>
    - fill:hw  p<XXXXXXXX> <ZZZZ> <NNN>
    - fill:w   p<XXXXXXXX> <ZZZZ> <NNN>
    - fill:b   b<XXXXXXXX> <ZZZZ> <NNN>
    - fill:hw  b<XXXXXXXX> <ZZZZ> <NNN>
    - fill:w   b<XXXXXXXX> <ZZZZ> <NNN>
