```
<BA Register Operations>
    - $ba =        b[XXXXXXXX]
    - $ba =        b[XXXXXXXX]
    - $ba =        b[XXXXXXXX]
    - $ba =        b[XXXXXXXX]
    - $ba +=       b[XXXXXXXX]
    - $ba +=       b[XXXXXXXX]
    - $ba +=       b[XXXXXXXX]
    - $ba +=       b[XXXXXXXX]
    - $ba =        p[XXXXXXXX]
    - $ba =        p[XXXXXXXX]
    - $ba +=       p[XXXXXXXX]
    - $ba +=       p[XXXXXXXX]

    - $ba =        b<XXXXXXXX>
    - $ba =        b<XXXXXXXX>
    - $ba =        b<XXXXXXXX>
    - $ba =        b<XXXXXXXX>
    - $ba +=       b<XXXXXXXX>
    - $ba +=       b<XXXXXXXX>
    - $ba +=       b<XXXXXXXX>
    - $ba +=       b<XXXXXXXX>
    - $ba =        p<XXXXXXXX>
    - $ba =        p<XXXXXXXX>
    - $ba +=       p<XXXXXXXX>
    - $ba +=       p<XXXXXXXX>

    -    [xxxxxxxx]  =  $ba
    - g$ [xxxxxxxx]  =  $ba
    - b  [xxxxxxxx]  =  $ba
    - bg$[xxxxxxxx]  =  $ba
    - p  [xxxxxxxx]  =  $ba
    - pg$[xxxxxxxx]  =  $ba

    - rip   <XXXX> $ba

```

```
<PO Register Operations>
    - $po=           [XXXXXXXX]
    - $po=        g$ [XXXXXXXX]
    - $po=        b  [XXXXXXXX]
    - $po=        bg$[XXXXXXXX]
    - $po+=          [XXXXXXXX]
    - $po+=       g$ [XXXXXXXX]
    - $po+=       b  [XXXXXXXX]
    - $po+=       bg$[XXXXXXXX]
    - $po=        p  [XXXXXXXX]
    - $po=        pg$[XXXXXXXX]
    - $po+=       p  [XXXXXXXX]
    - $po+=       pg$[XXXXXXXX]


    - $po=       <XXXXXXXX>
    - $po=   g$  <XXXXXXXX>
    - $po=   b   <XXXXXXXX>
    - $po=   bg$ <XXXXXXXX>
    - $po+=      <XXXXXXXX>
    - $po+=  g$  <XXXXXXXX>
    - $po+=  b   <XXXXXXXX>
    - $po+=  bg$ <XXXXXXXX>
    - $po=   p   <XXXXXXXX>
    - $po=   pg$ <XXXXXXXX>
    - $po+=  p   <XXXXXXXX>
    - $po+=  pg$ <XXXXXXXX>

    - [xxxxxxxx]=  $po
    - g$[xxxxxxxx]=  $po
    - b[xxxxxxxx]= $po
    - bg$[xxxxxxxx]= $po
    - p[xxxxxxxx]= $po
    - pg$[xxxxxxxx]= $po

    - rip   <XXXX> $po
```

```
<Gecko Registers>
    - g$  =  <XXXXXXXX>
    - g$  = b<XXXXXXXX>
    - g$  =  <XXXXXXXX>
    - g$ += b<XXXXXXXX>
    - g$ += p<XXXXXXXX>
    - g$ += p<XXXXXXXX>
    - 8     g$  =  [XXXXXXXX]
    - 16    g$  =  [XXXXXXXX]
    - 32    g$  =  [XXXXXXXX]
    - 8     g$  = b[XXXXXXXX]
    - 16    g$  = b[XXXXXXXX]
    - 32    g$  = b[XXXXXXXX]
    - 8     g$  = p[XXXXXXXX]
    - 16    g$  = p[XXXXXXXX]
    - 32    g$  = p[XXXXXXXX]
    - store:b        g$     <XXXXXXXX>
    - store:hw       g$    b<XXXXXXXX>
    - store:po       g$    p<XXXXXXXX>
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
```
