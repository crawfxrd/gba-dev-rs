/* SPDX-FileCopyrightText: NONE */
/* SPDX-License-Identifier: CC0-1.0 */

/*
 * https://sourceware.org/binutils/docs/ld/Scripts.html
 */

OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

MEMORY
{
    ewram : ORIGIN = 0x02000000, LENGTH = 256K
    iwram : ORIGIN = 0x03000000, LENGTH = 32K
    rom   : ORIGIN = 0x08000000, LENGTH = 32M
}

SECTIONS
{
    .text : ALIGN(4)
    {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > rom

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
    } > rom

    .data : ALIGN(4)
    {
        *(.data .data.*)
    } > iwram AT > rom

    .bss (NOLOAD) : ALIGN(4)
    {
        *(.bss)
    } > iwram

    /DISCARD/ :
    {
      *(.ARM.exidx .ARM.exidx.*)
    }
}
