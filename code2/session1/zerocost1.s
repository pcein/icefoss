	.text
	.file	"zerocost1.cgu-0.rs"
	.section	.text._ZN9zerocost14main17hb04dd12581d88df9E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN9zerocost14main17hb04dd12581d88df9E,@function
_ZN9zerocost14main17hb04dd12581d88df9E:
	.cfi_startproc
	subq	$72, %rsp
.Lcfi0:
	.cfi_def_cfa_offset 80
	movabsq	$500000000500000000, %rax
	movq	%rax, (%rsp)
	movq	%rsp, %rax
	movq	%rax, 8(%rsp)
	movq	_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17he96b040776df5a88E@GOTPCREL(%rip), %rax
	movq	%rax, 16(%rsp)
	leaq	ref.2(%rip), %rax
	movq	%rax, 24(%rsp)
	movq	$2, 32(%rsp)
	movq	$0, 40(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 56(%rsp)
	movq	$1, 64(%rsp)
	leaq	24(%rsp), %rdi
	callq	_ZN3std2io5stdio6_print17h7ce09810ca5a75e5E@PLT
	addq	$72, %rsp
	retq
.Lfunc_end0:
	.size	_ZN9zerocost14main17hb04dd12581d88df9E, .Lfunc_end0-_ZN9zerocost14main17hb04dd12581d88df9E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdi, %rcx
	leaq	_ZN9zerocost14main17hb04dd12581d88df9E(%rip), %rdi
	movq	%rcx, %rsi
	movq	%rax, %rdx
	jmp	_ZN3std2rt10lang_start17h97aba2334c85f570E@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc

	.type	str.0,@object
	.section	.rodata.str.0,"a",@progbits
str.0:
	.size	str.0, 0

	.type	str.1,@object
	.section	.rodata.str.1,"a",@progbits
str.1:
	.byte	10
	.size	str.1, 1

	.type	ref.2,@object
	.section	.data.rel.ro.ref.2,"aw",@progbits
	.p2align	3
ref.2:
	.quad	str.0
	.quad	0
	.quad	str.1
	.quad	1
	.size	ref.2, 32


	.section	".note.GNU-stack","",@progbits
