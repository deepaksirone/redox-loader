global start
extern long_mode_start

section .bss
align 4096			       ; page tables are 4KB aligned as the first 12 bits of a pte are options

p4_table:
	resb 4096
p3_table:
	resb 4096
p2_table:
	resb 4096

stack_bottom:
	resb 4096				; reserve space for stack
stack_top: 

section .text
bits 32
start:
	mov esp, stack_top 		; setup stack pointer, stack is full descending

	mov eax, p3_table
	or eax, 0b11	  		; present and writable 
	mov dword [p4_table + 0], eax

	mov eax, p2_table
	or eax, 0b11
	mov dword [p3_table + 0], eax

	mov ecx, 0
.map_p2_table:				; loop to identity map pages
	mov eax, 0x200000
	mul ecx
	or eax, 0b10000011		; present + writable + huge page (in p2) page size 2MB
	mov [p2_table + ecx * 8], eax	; each page table entry is 8 bytes

	inc ecx
	cmp ecx, 512
	jne .map_p2_table

	mov eax, p4_table		; load P4 to cr3 register (cpu uses this to access the P4 table)
	mov cr3, eax

	mov eax, cr4			; enable PAE-flag in cr4 (Physical Address Extension)
	mov eax, 1 << 5
	mov cr4, eax

	mov ecx, 0xc0000080		; set the long mode bit in the EFER MSR (model specific register)
	rdmsr
	or eax, 1 << 8
	wrmsr
	
					; enable paging
	mov eax, cr0
	or eax, 1 << 31
	or eax, 1 << 16
	mov cr0, eax

	lgdt [gdt64.pointer]		; load the gdtr with the base address of the gdt
	
	mov ax, gdt64.data		; make all the segments point to data
	mov ss, ax
	mov ds, ax
	mov es, ax

	call set_up_SSE
	jmp gdt64.code:long_mode_start	; change the cs segment by a far jump

	mov word [0xb8000], 0x0248
	mov word [0xb8002], 0x0269
	mov word [0xb8004], 0x0221
	hlt

set_up_SSE:
    				; check for SSE
    	mov eax, 0x1
    	cpuid
    	test edx, 1<<25
    	jz .no_SSE

    				; enable SSE
    	mov eax, cr0
    	and ax, 0xFFFB      	; clear coprocessor emulation CR0.EM
    	or ax, 0x2          	; set coprocessor monitoring  CR0.MP
    	mov cr0, eax
    	mov eax, cr4
    	or ax, 3 << 9       	; set CR4.OSFXSR and CR4.OSXMMEXCPT at the same time
    	mov cr4, eax

    	ret
.no_SSE:
	mov al, "a"

error:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
	hlt
	

section .rodata
gdt64:				; global descriptor table
	dq 0
.code: equ $ - gdt64		
	dq (1 << 44) | (1 << 47) | (1 << 41) | (1 << 43) | (1 << 53)
.data: equ $ - gdt64
	dq (1<<44) | (1<<47) | (1<<41)
	
.pointer:
	dw .pointer - gdt64 - 1
	dq gdt64
