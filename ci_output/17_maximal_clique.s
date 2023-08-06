        .data
maxN:
        .word   30
maxM:
        .word   600

        .bss
store:
        .zero   120
n:
        .zero   4
graph:
        .zero   3600
m:
        .zero   4

        .text
.global main

maxCliques:
.entry_maxCliques:
addi sp, sp, -96
sd ra, 88(sp)
sd s1, 72(sp)
sd s0, 64(sp)
.L1:
addi s0, sp, 0
sw a0, 0(s0)
addi s1, sp, 4
sw a1, 0(s1)
li t2, 0
li t1, 1
li t0, 1
sw t0, 0(s0)
j .L2
.L2:
lw t0, n
sub s0, t1, t0
seqz t0, s0
sltz s0, s0
or s0, t0, s0
bne s0, zero, .L3
j .L4
.L3:
lw a0, 0(s1)
la s0, store
li t0, 4
mul t0, a0, t0
add t0, s0, t0
sw t1, 0(t0)
lw s0, 0(s1)
li t0, 1
addiw a0, s0, 1
sd t1, 16(sp)
sd t2, 8(sp)
call is_clique
ld t1, 16(sp)
ld t2, 8(sp)
li t0, 0
addi t0, a0, 0
snez t0, t0
bne t0, zero, .L5
j .L6
.L4:
mv a0, t2
ld ra, 88(sp)
ld s1, 72(sp)
ld s0, 64(sp)
addi sp, sp, 96
ret
.L5:
lw t0, 0(s1)
sub t0, t0, t2
sgtz t0, t0
bne t0, zero, .L7
j .L8
.L6:
li t0, 1
addiw t1, t1, 1
j .L2
.L7:
lw t2, 0(s1)
j .L8
.L8:
mv a0, t1
lw s0, 0(s1)
li t0, 1
addiw a1, s0, 1
sd t1, 32(sp)
sd t2, 24(sp)
call maxCliques
ld t1, 32(sp)
ld t2, 24(sp)
mv t0, a0
sub s0, t0, t2
sgtz s0, s0
bne s0, zero, .L9
j .L6
.L9:
mv t2, t0
j .L6

is_clique:
.entry_is_clique:
addi sp, sp, -40
sd s0, 28(sp)
sd s1, 20(sp)
.L20:
addi s1, sp, 0
sw a0, 0(s1)
li t1, 1
j .L21
.L21:
lw t0, 0(s1)
sub t0, t1, t0
sltz t0, t0
bne t0, zero, .L22
j .L23
.L22:
li t0, 1
addiw t0, t1, 1
j .L24
.L23:
li t0, 1
j .L29
.L24:
lw t2, 0(s1)
sub t2, t0, t2
sltz t2, t2
bne t2, zero, .L25
j .L26
.L25:
la s0, store
li t2, 4
mul t2, t1, t2
add t2, s0, t2
lw a1, 0(t2)
la s0, store
li t2, 4
mul t2, t0, t2
add t2, s0, t2
lw a0, 0(t2)
la s0, graph
li t2, 120
mul t2, a1, t2
add s0, s0, t2
li t2, 4
mul t2, a0, t2
add t2, s0, t2
lw s0, 0(t2)
li t2, 0
addi t2, s0, 0
seqz t2, t2
bne t2, zero, .L27
j .L28
.L26:
li t0, 1
addiw t1, t1, 1
j .L21
.L27:
li t0, 0
j .L29
.L28:
li t2, 1
addiw t0, t0, 1
j .L24
.L29:
mv a0, t0
ld s0, 28(sp)
ld s1, 20(sp)
addi sp, sp, 40
ret
j .L29

main:
.entry_main:
addi sp, sp, -2036
addi sp, sp, -2036
addi sp, sp, -816
addi sp, sp, 2032
addi sp, sp, 2032
sd ra, 816(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
sd s1, 808(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
sd s0, 800(sp)
addi sp, sp, -2032
addi sp, sp, -2032
.L37:
call getint
lui t0, %hi(n)
sw a0, %lo(n)(t0)
call getint
lui t0, %hi(m)
sw a0, %lo(m)(t0)
addi s0, sp, 0
li t0, 0
j .L38
.L38:
lw t1, m
sub t1, t0, t1
sltz t1, t1
bne t1, zero, .L39
j .L40
.L39:
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 0
li t1, 4
li t1, 0
addi t1, s1, 0
addi sp, sp, 2032
addi sp, sp, 2032
sd t0, 744(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
sd t1, 736(sp)
addi sp, sp, -2032
addi sp, sp, -2032
call getint
addi sp, sp, 2032
addi sp, sp, 2032
ld t0, 744(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
ld t1, 736(sp)
addi sp, sp, -2032
addi sp, sp, -2032
sw a0, 0(t1)
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 1
li t1, 4
li t1, 4
addi t1, s1, 4
addi sp, sp, 2032
addi sp, sp, 2032
sd t0, 760(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
sd t1, 752(sp)
addi sp, sp, -2032
addi sp, sp, -2032
call getint
addi sp, sp, 2032
addi sp, sp, 2032
ld t0, 760(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
ld t1, 752(sp)
addi sp, sp, -2032
addi sp, sp, -2032
sw a0, 0(t1)
li t1, 1
addiw t0, t0, 1
j .L38
.L40:
li t0, 0
j .L41
.L41:
lw t1, m
sub t1, t0, t1
sltz t1, t1
bne t1, zero, .L42
j .L43
.L42:
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 0
li t1, 4
li t1, 0
addi t1, s1, 0
lw a0, 0(t1)
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 1
li t1, 4
li t1, 4
addi t1, s1, 4
lw s1, 0(t1)
la t2, graph
li t1, 120
mul t1, a0, t1
add t2, t2, t1
li t1, 4
mul t1, s1, t1
add t2, t2, t1
li t1, 1
sw t1, 0(t2)
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 1
li t1, 4
li t1, 4
addi t1, s1, 4
lw a0, 0(t1)
li t1, 8
mul t1, t0, t1
add s1, s0, t1
li t2, 0
li t1, 4
li t1, 0
addi t1, s1, 0
lw s1, 0(t1)
la t2, graph
li t1, 120
mul t1, a0, t1
add t2, t2, t1
li t1, 4
mul t1, s1, t1
add t2, t2, t1
li t1, 1
sw t1, 0(t2)
li t1, 1
addiw t0, t0, 1
j .L41
.L43:
li a0, 0
li a1, 1
call maxCliques
call putint
li a0, 0
addi sp, sp, 2032
addi sp, sp, 2032
ld ra, 816(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
ld s1, 808(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2032
addi sp, sp, 2032
ld s0, 800(sp)
addi sp, sp, -2032
addi sp, sp, -2032
addi sp, sp, 2036
addi sp, sp, 2036
addi sp, sp, 816
ret
j .L41
