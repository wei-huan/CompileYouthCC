        .text
.global main

move:
.entry_move:
addi sp, sp, -48
sd ra, 40(sp)
.L1:
addi t1, sp, 0
sw a0, 0(t1)
addi t0, sp, 4
sw a1, 0(t0)
lw a0, 0(t1)
sd t0, 8(sp)
call putint
ld t0, 8(sp)
li a0, 32
sd t0, 16(sp)
call putch
ld t0, 16(sp)
lw a0, 0(t0)
call putint
li a0, 44
call putch
li a0, 32
call putch
ld ra, 40(sp)
addi sp, sp, 48
ret

hanoi:
.entry_hanoi:
addi sp, sp, -104
sd ra, 96(sp)
sd s1, 88(sp)
sd s0, 72(sp)
.L3:
addi a4, sp, 0
sw a0, 0(a4)
addi s1, sp, 4
sw a1, 0(s1)
addi s0, sp, 8
sw a2, 0(s0)
addi t2, sp, 12
sw a3, 0(t2)
lw t1, 0(a4)
li t0, 1
addi t0, t1, -1
seqz t0, t0
bne t0, zero, .L4
j .L5
.L4:
lw a0, 0(s1)
lw a1, 0(t2)
call move
j .L6
.L5:
lw t1, 0(a4)
li t0, 1
addiw a0, t1, -1
lw a1, 0(s1)
lw a2, 0(t2)
lw a3, 0(s0)
sd t2, 24(sp)
sd a4, 16(sp)
call hanoi
ld t2, 24(sp)
ld a4, 16(sp)
lw a0, 0(s1)
lw a1, 0(t2)
sd a4, 40(sp)
sd t2, 32(sp)
call move
ld a4, 40(sp)
ld t2, 32(sp)
lw t1, 0(a4)
li t0, 1
addiw a0, t1, -1
lw a1, 0(s0)
lw a2, 0(s1)
lw a3, 0(t2)
call hanoi
.L6:
ld ra, 96(sp)
ld s1, 88(sp)
ld s0, 72(sp)
addi sp, sp, 104
ret

main:
.entry_main:
addi sp, sp, -48
sd ra, 40(sp)
.L8:
call getint
mv t0, a0
j .L9
.L9:
li t1, 0
addi t1, t0, 0
sgtz t1, t1
bne t1, zero, .L10
j .L11
.L10:
sd t0, 0(sp)
call getint
ld t0, 0(sp)
li a1, 1
li a2, 2
li a3, 3
sd t0, 8(sp)
call hanoi
ld t0, 8(sp)
li a0, 10
sd t0, 16(sp)
call putch
ld t0, 16(sp)
li t1, 1
addiw t0, t0, -1
j .L9
.L11:
li a0, 0
ld ra, 40(sp)
addi sp, sp, 48
ret
j .L9
