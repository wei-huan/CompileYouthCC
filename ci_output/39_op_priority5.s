        .data
c:
        .word   1
d:
        .word   2
a:
        .word   1
e:
        .word   4

        .bss
b:
        .zero   4

        .text
.global main

__sysy_homemade_mem_zero_init:
addi    sp,sp,-48
sd      ra,40(sp)
sd      s0,32(sp)
addi    s0,sp,48
sd      a0,-40(s0)
mv      a5,a1
mv      a4,a2
sw      a5,-44(s0)
mv      a5,a4
sw      a5,-48(s0)
lw      a5,-44(s0)
sw      a5,-20(s0)
j       .L1
.L2:
lw      a5,-20(s0)
slli    a5,a5,2
ld      a4,-40(s0)
add     a5,a4,a5
sw      zero,0(a5)
lw      a5,-20(s0)
addiw   a5,a5,1
sw      a5,-20(s0)
.L1:
lw      a5,-20(s0)
mv      a4,a5
lw      a5,-48(s0)
sext.w  a4,a4
sext.w  a5,a5
blt     a4,a5,.L2
nop
nop
ld      ra,40(sp)
ld      s0,32(sp)
addi    sp,sp,48
jr      ra


main:
.entry_main:
addi sp, sp, -32
sd ra, 24(sp)
sd s0, 8(sp)
sd s1, 0(sp)
.L4:
li a0, 0
lw t1, a
lw t0, b
mulw t1, t1, t0
lw t0, c
divw t2, t1, t0
lw t1, e
lw t0, d
addw t0, t1, t0
sub t0, t2, t0
seqz t0, t0
li s1, 1
bne t0, zero, .L8
j .L7
.L6:
mv a0, s1
call putint
mv a0, s1
ld ra, 24(sp)
ld s0, 8(sp)
ld s1, 0(sp)
addi sp, sp, 32
ret
.L7:
lw t2, a
lw t1, b
lw t0, c
mulw t0, t1, t0
subw s0, t2, t0
lw t2, d
lw t1, a
lw t0, c
divw t0, t1, t0
subw t0, t2, t0
sub t0, s0, t0
seqz t0, t0
bne t0, zero, .L6
j .L9
.L8:
lw t2, a
lw t1, a
lw t0, b
addw t0, t1, t0
mulw t1, t2, t0
lw t0, c
addw t2, t1, t0
lw t1, d
lw t0, e
addw t0, t1, t0
sub t0, t2, t0
seqz t1, t0
sltz t0, t0
or t0, t1, t0
bne t0, zero, .L6
j .L7
.L9:
mv s1, a0
j .L6
