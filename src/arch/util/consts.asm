%define VIRT_BASE 0xFFFFFFFF80000000

; 4k page size
%define PAGE_SHIFT      12
%define PAGE_SIZE       (1 << PAGE_SHIFT)
%define PAGE_MASK	(PAGE_SIZE - 1)

%define PTE_SHIFT	(PAGE_SHIFT + 9*0)
%define PDE_SHIFT	(PAGE_SHIFT + 9*1)
%define PDPE_SHIFT	(PAGE_SHIFT + 9*2)
%define PML4E_SHIFT	(PAGE_SHIFT + 9*3)

; Find index based on virtual address
%define PTE(x)		(((x) >> PTE_SHIFT) & 0x1FF)
%define PDE(x)		(((x) >> PDE_SHIFT) & 0x1FF)
%define PDPE(x)		(((x) >> PDPE_SHIFT) & 0x1FF)
%define PML4E(x)	(((x) >> PML4E_SHIFT) & 0x1FF)

;/* Find page structure based on virtual address */

;/* Because we mapped the PML4 page into itself as the second to last entry, and
; * the structure of the PML4/PDP/PD/PTs are all compatible, we can then use the
; * bits in the virtual address to locate its relevant page structures.
; *
; * With the standard 4-level page tables, the "leaves" are data pages.
; *
; * With the PML4 mapped into itself at 0x1fe, if you have an address with PML4E
; * = 0x1fe, then the PML4 becomes the PDP, the PDP becomes the PD, the PD
; * becomes the PT and PTs are now the leaves and accessible.
; *
; * If you create an address with PML4E = 0x1fe and PDP = 0x1fe, then the PDs
; * are the leaves and accessible.
; *
; * So if we want to access, for example, the page directory for a certain
; * address, we can set PML4E = 0x1fe and PDP = 0x1fe... but what are the rest
; * of the bits? The look ups that the MMU already does! The PML4E in the
; * address becomes the PDE index and the PDPE in the address becomes the PTE
; * index.
; */
;
;/* The BASE definitions are merely handling setting the entries we know need to
; * be 0x1FE for each form of lookup. P_VIRT_BASE is just the sign extension we
; * know must be present for a valid address.
; *
; * As we layed out above, to get the PTs to be the leaf nodes (accessible
; * data), just the PML4E has to be 0x1fe, and so on
; */

%define P_VIRT_BASE	(0xFFFF000000000000)
%define PT_VIRT_BASE	(P_VIRT_BASE | ((long) 0x1FE << PML4E_SHIFT))
%define PD_VIRT_BASE	(PT_VIRT_BASE | ((long) 0x1FE << PDPE_SHIFT))
%define PDP_VIRT_BASE	(PD_VIRT_BASE | ((long) 0x1FE << PDE_SHIFT))
%define PML4_VIRT_BASE	(PDP_VIRT_BASE | ((long) 0x1FE << PTE_SHIFT))

;/*
; * Derive addresses of these entries. This macro is complicated so let's break
; * it down.
; *
; * Let's say I'm trying to find PTE address for 0x12345678, which in binary is:
; *
; * 00 | 01 0010 001 | 1 0100 0101 | 0110 0111 1000
; *    | PDE         | PTE         | PAGE OFFSET
; *
; * There's no sign extension, but nonetheless (x & ~P_VIRT_BASE) doesn't hurt.
; * Then we shift it down by 9, we get
; *
; * 0000 0 | 000 0000 00 | 01 0010 001 | 1 0100 0101 011
; *        | PDE         | PTE         | PAGE OFFSET
; *
; * As you can see, the new PTE is the old PDE, the new PDE is the old PDP (all
; * zeroes) and so forth, but the top three bits of the original page offset are
; * now the bottom three bits of it, and are irrelevant, so we zero them with an
; * (& ~7).
; *
; * 0000 0 | 000 0000 00 | 01 0010 001 | 1 0100 0101 000
; *        | PDE         | PTE         | PAGE OFFSET
; *
; * Now that we've properly shifted all of our lookups, and discarded the
; * unnecessary page offset and sign extension bits, we can and in the BASE,
; * which as explained above is a constructed value where all of the unset
; * lookups are set to 0x1fe. In this case, our PML4E which is now 0 thanks to
; * shifting, needs to be set to 0x1fe, which is exactly what PT_VIRT_BASE
; * includes, along with the sign extension required due to the top bit of the
;* PML4E being 1
; */

%define PTE_ADDR(x)      (PT_VIRT_BASE	 | (((x & ~P_VIRT_BASE) >> 9)  & ~7))
%define PDE_ADDR(x)      (PD_VIRT_BASE	 | (((x & ~P_VIRT_BASE) >> 18) & ~7))
%define PDPE_ADDR(x)     (PDP_VIRT_BASE  | (((x & ~P_VIRT_BASE) >> 27) & ~7))
%define PML4E_ADDR(x)    (PML4_VIRT_BASE | (((x & ~P_VIRT_BASE) >> 36) & ~7))

; Present
%define PF_P                (1 << 0)
; R/W allowed 
%define PF_RW               (1 << 1)
%define PF_USER             (1 << 2)
%define PF_WRITETHRU        (1 << 3)
%define PF_DISABLE_CACHE    (1 << 4)

;/* Early physical addresses */
%define KERNEL_START 0x100000

%define S_PAGES			2	// 8k stack, for now
%define STACK_PAGES_PHYS	0	// Starting at 0
%define STACK_PAGES_START (KERNEL_START - (S_PAGES * PAGE_SIZE))
