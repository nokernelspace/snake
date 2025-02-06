# Comparison Instructions
comparison instructions compare stuff. When paired with an `else/endif` they can be optimized manually to execute the else branch first

### ifeqb - If Equal via BA
Execute If Equal

```
20______ XXXXXXXX

Adding 1 to ______ will make this code first apply an Endif.

If [BA+______]==XXXXXXXX,
then codes are executed
otherwise we halt

```


### ifeqp - If Equal via PO
Similar to `ifeqb`
```
30______ XXXXXXXX

Adding 1 to ______ will make this code first apply an Endif.

If [PO+______]==XXXXXXXX,
then codes are executed
otherwise we halt

```


#### Syntax: `ifeqp ______ XXXXXXXX`
