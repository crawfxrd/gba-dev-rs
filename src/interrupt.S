@ SPDX-FileCopyrightText: NONE
@ SPDX-License-Identifier: CC0-1.0
@
@ Master interrupt handler
@
@ Ref: https://problemkaputt.de/gbatek.htm#gbainterruptcontrol

#define REG_BIOS_IF     0x03007FF8
#define REG_IF          0x04000202

    .section .text
    .arm
    .cpu arm7tdmi

    .global master_isr
    .type master_isr, STT_FUNC
master_isr:
    @ Acknowledge VBLANK interrupt
    @ IF = 1
    mov r1, #1
    ldr r2, =REG_IF
    strh r1, [r2]

    @ BIOS_IF |= 1
    ldr r3, =REG_BIOS_IF
    ldrh r1, [r3]
    orr r1, r1, #1
    strh r1, [r3]

    bx lr
    .size master_isr, . - master_isr
