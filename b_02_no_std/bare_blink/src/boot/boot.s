
.section .text._start

_start:
	// park all cores that are not core 0
	mrc p15, #0, r1, c0, c0, #5
	and	r1, r1, #0b11
	cmp	r1, #0
	bne	.L_parking_loop

	// if we are here, we are on core 0

	mov sp, #0x8000

    // Clear out ram
    ldr r4, =__bss_start
    ldr r9, =__bss_end
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    b       ._init
 
_wrt:
    // store multiple at r4.
    stmia r4, {{r5, r6, r7, r8}}
 
    // loop until bss_end reached
._init:
    cmp r4, r9
    blo _wrt
	b 	_start_rust

	// park the core
.L_parking_loop:
	wfe
	b	.L_parking_loop

.type	_start, function
.global	_start
