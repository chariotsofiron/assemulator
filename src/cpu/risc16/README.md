# RiSC-16

A simple load-store 16-bit instruction set.

- https://user.eng.umd.edu/~blj/risc/
- https://user.eng.umd.edu/~blj/risc/RiSC-isa.pdf


```
add
addi
nand
lui
lw
sw
beq
jalr
```



## Calling convention

```c
int callee(int, int, int);
int caller(void) {
	return callee(1, 2, 3) + 5;
}

st arg, r7, -1
st arg, r7, -2
st arg, r7, -3
jalr callee

callee:
    adi r7, r7, param_sz + local_scope_sz
    ...
    code
    ...
    
    ret param_sz + local_scope_sz


push arg
push arg
call function

stack:
    arg1
    arg2
    return address
```