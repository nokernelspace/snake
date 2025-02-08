# Text
```
<Counter Registers> (Useful for putting an upper bound on the Halting Problem :)
    a) count:0 16  <XXXX> <MMMM> =  [<ZZZZ>]
    b) count:1 16  <XXXX> <MMMM> =  [<ZZZZ>]
    c) count:8 16  <XXXX> <MMMM> =  [<ZZZZ>]
    d) count:9 16  <XXXX> <MMMM> =  [<ZZZZ>]

    e) count:0 16  <XXXX> <MMMM> != [<ZZZZ>]
    f) count:1 16  <XXXX> <MMMM> != [<ZZZZ>]
    g) count:8 16  <XXXX> <MMMM> != [<ZZZZ>]
    h) count:9 16  <XXXX> <MMMM> != [<ZZZZ>]

    i) count:0 16  <XXXX> <MMMM> >  [<ZZZZ>]
    j) count:1 16  <XXXX> <MMMM> >  [<ZZZZ>]
    k) count:8 16  <XXXX> <MMMM> >  [<ZZZZ>]
    l) count:9 16  <XXXX> <MMMM> >  [<ZZZZ>]

    m) count:0 16  <XXXX> <MMMM> <  [<ZZZZ>]
    n) count:1 16  <XXXX> <MMMM> <  [<ZZZZ>]
    o) count:8 16  <XXXX> <MMMM> <  [<ZZZZ>]
    p) count:9 16  <XXXX> <MMMM> <  [<ZZZZ>]
```

# OpCodes
```
a) A80ZZZZ0 MMMMXXXX
b) A80ZZZZ1 MMMMXXXX
c) A80ZZZZ2 MMMMXXXX
d) A80ZZZZ3 MMMMXXXX

e) AA0ZZZZ0 MMMMXXXX
f) AA0ZZZZ1 MMMMXXXX
g) AA0ZZZZ8 MMMMXXXX
h) AA0ZZZZ9 MMMMXXXX


f) AC0ZZZZ0 MMMMXXXX
g) AC0ZZZZ1 MMMMXXXX
h) AC0ZZZZ8 MMMMXXXX
i) AC0ZZZZ9 MMMMXXXX

m) AE0ZZZZ0 MMMMXXXX
n) AE0ZZZZ1 MMMMXXXX
o) AE0ZZZZ8 MMMMXXXX
p) AE0ZZZZ9 MMMMXXXX
```
