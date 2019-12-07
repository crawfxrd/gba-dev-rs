@ SPDX-License-Identifier: CC0-1.0
@
@ Master interrupt handler
@
@ Ref: https://problemkaputt.de/gbatek.htm#gbainterruptcontrol

#define REG_IF          0x03007FF8
#define REG_BIOS_IF     0x04000202

    .section .text
    .arm
    .cpu arm7tdmi

    .global master_isr
    .func
master_isr:
    @ Acknowledge VBLANK interrupt
    @ IF = 1
    mov r0, #1
    ldr r1, =REG_IF
    str r0, [r1]

    @ BIOS_IF |= 1
    ldr r1, =REG_BIOS_IF
    ldr r0, [r1]
    orr r0, #1
    str r0, [r1]

    bx lr
    .endfunc
