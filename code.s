	.file	"code.c"
	.option nopic
	.attribute arch, "rv64i2p1_m2p0_a2p1"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-32
	sd	ra,24(sp)
	sd	s0,16(sp)
	addi	s0,sp,32
	li	a0,234
	call	collatz_n
	sd	a0,-24(s0)
	ld	a0,-24(s0)
	call	debug
	nop
	mv	a0,a5
	ld	ra,24(sp)
	ld	s0,16(sp)
	addi	sp,sp,32
	jr	ra
	.size	main, .-main
	.align	2
	.globl	debug
	.type	debug, @function
debug:
	addi	sp,sp,-32
	sd	s0,24(sp)
	addi	s0,sp,32
	sd	a0,-24(s0)
	li	a5,1
	slli	a5,a5,32
	ld	a4,-24(s0)
	sd	a4,0(a5)
	nop
	ld	s0,24(sp)
	addi	sp,sp,32
	jr	ra
	.size	debug, .-debug
	.align	2
	.globl	collatz_n
	.type	collatz_n, @function
collatz_n:
	addi	sp,sp,-48
	sd	ra,40(sp)
	sd	s0,32(sp)
	addi	s0,sp,48
	sd	a0,-40(s0)
	ld	a0,-40(s0)
	call	debug
	sd	zero,-24(s0)
	j	.L4
.L7:
	ld	a5,-40(s0)
	andi	a5,a5,1
	bne	a5,zero,.L5
	ld	a5,-40(s0)
	srli	a4,a5,63
	add	a5,a4,a5
	srai	a5,a5,1
	sd	a5,-40(s0)
	j	.L6
.L5:
	ld	a4,-40(s0)
	mv	a5,a4
	slli	a5,a5,1
	add	a5,a5,a4
	addi	a5,a5,1
	sd	a5,-40(s0)
.L6:
	ld	a0,-40(s0)
	call	debug
	ld	a5,-24(s0)
	addi	a5,a5,1
	sd	a5,-24(s0)
.L4:
	ld	a4,-40(s0)
	li	a5,1
	bne	a4,a5,.L7
	ld	a5,-24(s0)
	mv	a0,a5
	ld	ra,40(sp)
	ld	s0,32(sp)
	addi	sp,sp,48
	jr	ra
	.size	collatz_n, .-collatz_n
	.ident	"GCC: (gc891d8dc2) 13.2.0"
