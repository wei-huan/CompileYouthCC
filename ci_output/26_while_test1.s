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


doubleWhile:
.entry_doubleWhile:
.L4:
li t2, 5
li t1, 7
j .L5
.L5:
addi t0, t2, -100
sltz t0, t0
beq t0, zero, .L7
j .L8
.L7:
mv a0, t1
ret
.L8:
addi t0, t1, -100
sltz t0, t0
beq t0, zero, .L10
.L9:
addiw t1, t1, 6
j .L8
.L10:
addiw t2, t2, 30
addiw t1, t1, -100
j .L5

main:
.entry_main:
addi sp, sp, -16
sd ra, 8(sp)
.L16:
call doubleWhile
ld ra, 8(sp)
addi sp, sp, 16
ret
