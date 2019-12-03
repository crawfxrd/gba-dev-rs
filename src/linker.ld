/* SPDX-License-Identifier: CC0-1.0 */
/*
 * https://sourceware.org/binutils/docs/ld/Scripts.html
 */

OUTPUT_FORMAT("elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

/*
 * Memory map for ARM7TDMI
 */
MEMORY
{
    EWRAM (w!x) : ORIGIN = 0x02000000, LENGTH = 256K
    IWRAM (w!x) : ORIGIN = 0x03000000, LENGTH = 32K
    ROM (rx)    : ORIGIN = 0x08000000, LENGTH = 32M
}

/*
 * section [address] [(type)] :
 *   [AT(lma)]
 *   [ALIGN(section_align) | ALIGN_WITH_INPUT]
 *   [SUBALIGN(subsection_align)]
 *   [constraint]
 *   {
 *     output-section-command
 *     output-section-command
 *     ...
 *   } [>region] [AT>lma_region] [:phdr :phdr …] [=fillexp] [,]
 */

SECTIONS
{
    .text : ALIGN(4)
    {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > ROM

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
    } > ROM

    .data : ALIGN(4)
    {
        *(.data .data.*)
    } > IWRAM AT > ROM

    .bss : ALIGN(4)
    {
        *(.bss)
    } > IWRAM

    /* TODO: Keep/Discard sections */
    /DISCARD/ :
    {
      *(.ARM.exidx*)
    }
}
