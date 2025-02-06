


### f8 - Fill Memory @ Internvals of 8-bits with BA
```
00 ______ YYYY00XX

Writes the value XX to YYYY+1 consecutive byte-sized addresses, starting with the address BA+______
```

#### Syntax: `f8 ______ <XX> <YYYY> `


### w16b - Fill Memory @ Intervals of 16-bits with BA
```
02______ YYYYXXXX

Writes the value XXXX to YYYY+1 consecutive halfword-sized addresses, starting with the address BA+______

```
#### Syntax: `w16b ______ <XXXX> <YYYY> `


### w16p - Fill Memory @ Intervals of 16-bits with PO
Behaves Similar to `w16b` except with PO
```
12______ YYYYXXXX

Writes the value XXXX to YYYY+1 consecutive halfword-sized addresses, starting with the address PO+______
```

#### Syntax: `w16p ______ <XXXX> <YYYY> `



### w32b - Fill Memory @ Intervals of 16-bits with BA
```
04______ XXXXXXXX

Writes the value XXXXXXXX to BA+______

```
#### Syntax: `w32b <XXXXXXXX>`

### w32p - Fill Memory @ Intervals of 16-bits with BA
```
14______ XXXXXXXX

Writes the value XXXXXXXX to PO+______

```
#### Syntax: `w32p <XXXXXXXX>`

### wb - Write with BA
```
06______ YYYYYYYY
d1d2d3d4 d5d6....

Writes each byte (d1, d2, d3, ...) consecutively, starting at address BA+______
YYYYYYYY is the number of bytes to write
```

#### Syntax: `wb <YYYYYYYY> ...`


### wp - write with PO
```
16______ YYYYYYYY
d1d2d3d4 d5d6....

Writes each byte (d1, d2, d3, ...) consecutively, starting at address PO+______
YYYYYYYY is the number of bytes to write
```

#### Syntax: `wp <YYYYYYYY> ...`


### fillb
Fills memory with gaps
```
08______ XXXXXXXX
TNNNZZZZ VVVVVVVV

______ + BA = Initial Address
X = Initial value for the RAM write
T = Value Size (0 = 8-bits, 1 = 16-bits, 2 = 32-bits)

N = Length
Z = Address Increment; in bytes (How many To skip By)

Unsupported
V = Value Increment (How much to add to the value after each additional RAM write)
```
#### Syntax: `fillb <XXXXXXXX> <T> <ZZZZ> <NNN>`

### fillp
Behaves similar to `fillb` except with PO
```
18______ XXXXXXXX
TNNNZZZZ VVVVVVVV

______ + PO = Initial Address
```
#### Syntax: `fillp <XXXXXXXX> <T> <ZZZZ> <NNN>`
