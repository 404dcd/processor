    imm 2 $a
    imm 1000 $k
    out $a
    subi 1 $a
newprm:
    addi 2 $a
    imm 3 $i
    mov $a $f
    shr 1 $f
check:
    mod $a $i $b
    beq $b $0 newprm
    addi 2 $i
    blt $i $f check
    
    out $a
    blt $a $k newprm
    hlt