; Taken from https://forum.osdev.org/viewtopic.php?f=1&t=23125
; Switches the CPU from long mode to unreal mode
; Copies the kernel from start_address to 1MB

ORG 0x9000
SECTION .bios
%include "descriptor_flags.inc"
%include "gdt_entry.inc"

USE64
%define retfq o64 retf
args:
     .kernel_base dq 0x100000
     .kernel_size dq 0
     .stack_base dq 0
     .stack_size dq 0
     .env_base dq 0
     .env_size dq 0
     .disk dq 0

_start:
	cli
	push rbp
	mov rbp, rsp 
	; 64 bit C calling convention
	mov [.start_address], edi
	mov [args.kernel_size], esi
	mov [args.env_size], edx

	mov          qword [.stckptr], rsp 	;save stack
	sgdt         [.gdtv64]			;save your gdt pointer
	lgdt         [.gdtv16]			;load a new one
	sidt         [.idt64]			;save your idt pointer
	lidt         [.idt16]			;load real mode idt
	;far jump in long mode is not possible, do a trick
	mov ax, DESC_REAL
	push ax
	push qword comp_mode
	retfq

.start_address:
	dd 	0
	align 	16, db 0
.stckptr:
	dq 	0
	align 	16, db 0
.gdtv64:
	dw 	0
	dq 	0
	align 	16, db 0
.gdtv16:
	dw           .gdtend + 1	
	dq           .gdt
	align 	   16, db 0 
.gdt:
	dq           0                     ;null descriptor
DESC_DATA equ 0x10                                 ;descriptor in YOUR GDT (modify)
	;dd 	   00000000h,00209200h    
DESC_LONG equ $-.gdt
	;dd           00000000h,00209800h      ;64 bit long mode cs
	dw 0
	dw 0
	db 0
	db 0x9a
	db 0x20
	db 0
DESC_REAL equ $-.gdt
	dd           0000FFFFh,00009a00h      ;16 bit real mode cs (modify base if needed!)
DESC_REAL_DATA equ $ - .gdt
    istruc GDTEntry
        at GDTEntry.limitl,        dw 0xFFFF
        at GDTEntry.basel,         dw 0x0
        at GDTEntry.basem,         db 0x0
        at GDTEntry.attribute,        db attrib.present | attrib.user | attrib.writable
        at GDTEntry.flags__limith, db 0xFF | flags.granularity | flags.default_operand_size
        at GDTEntry.baseh,         db 0x0
    ;iend

.gdtend equ $-.gdt 
	align        16, db 0
.idt64:
	dw           0
	dq           0
	align        16, db 0
.idt16:
	dw           3FFh
	dq           0

USE16
comp_mode:
	mov ax, DESC_REAL_DATA ;Use the segment descriptors for real mode
	mov es, ax
	mov ds, ax
	mov ss, ax

	mov eax, cr0
	and eax, 7FFFFFFEh
	mov cr0, eax
	

	xor ax, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	mov sp, 0xafff

	;jmp 0h:real
	push ax
	push word real
	retf

%ifdef COMMENT
print_line:
	mov al, 13
	call print_char
	mov al, 10
	jmp print_char

; print a string
; IN
;   si: points at zero-terminated String
; CLOBBER
;   si, ax
print:
	pushf
	cld
.loop:
	lodsb
	test al, al
	jz .done
	call print_char
	jmp .loop
.done:
	popf
	ret

; print a character
; IN
;   al: character to print
print_char:
	pusha
	mov bx, 7
	mov ah, 0x0e
	int 0x10
	popa
	ret

hello: db "Hello from real mode", 0
%endif

real:
	; copy kernel to start_address from load_address
	mov ecx, [args.kernel_size]
	mov edi, [args.kernel_base]
	mov esi, [_start.start_address]
	cld
	a32 rep movsb

startup_arch:
    cli
    ; setting up Page Tables
    ; Identity Mapping first GB
    mov ax, 0x7000
    mov es, ax

    xor edi, edi
    xor eax, eax
    mov ecx, 6 * 4096 / 4 ;PML4, PDP, 4 PD / moves 4 Bytes at once
    cld
    rep stosd

    xor edi, edi
    ;Link first PML4 and second to last PML4 to PDP
    mov DWORD [es:edi], 0x71000 | 1 << 1 | 1
    mov DWORD [es:edi + 510*8], 0x71000 | 1 << 1 | 1
    add edi, 0x1000
    ;Link last PML4 to PML4
    mov DWORD [es:edi - 8], 0x70000 | 1 << 1 | 1
    ;Link first four PDP to PD
    mov DWORD [es:edi], 0x72000 | 1 << 1 | 1
    mov DWORD [es:edi + 8], 0x73000 | 1 << 1 | 1
    mov DWORD [es:edi + 16], 0x74000 | 1 << 1 | 1
    mov DWORD [es:edi + 24], 0x75000 | 1 << 1 | 1
    add edi, 0x1000
    ;Link all PD's (512 per PDP, 2MB each)y
    mov ebx, 1 << 7 | 1 << 1 | 1
    mov ecx, 4*512
.setpd:
    mov [es:edi], ebx
    add ebx, 0x200000
    add edi, 8
    loop .setpd

    xor ax, ax
    mov es, ax

    ;cr3 holds pointer to PML4
    mov edi, 0x70000
    mov cr3, edi

    ;enable FXSAVE/FXRSTOR, Page Global, Page Address Extension, and Page Size Extension
    mov eax, cr4
    or eax, 1 << 9 | 1 << 7 | 1 << 5 | 1 << 4
    mov cr4, eax

    ; load protected mode GDT
    lgdt [gdtr]

    mov ecx, 0xC0000080               ; Read from the EFER MSR.
    rdmsr
    or eax, 1 << 11 | 1 << 8          ; Set the Long-Mode-Enable and NXE bit.
    wrmsr

    ;enabling paging and protection simultaneously
    mov ebx, cr0
    or ebx, 1 << 31 | 1 << 16 | 1                ;Bit 31: Paging, Bit 16: write protect kernel, Bit 0: Protected Mode
    mov cr0, ebx

    ; far jump to enable Long Mode and load CS with 64 bit segment
    jmp gdt.kernel_code:long_mode

USE64
long_mode:
    ; load all the other segments with 64 bit data segments
    mov rax, gdt.kernel_data
    mov ds, rax
    mov es, rax
    mov fs, rax
    mov gs, rax
    mov ss, rax

    ; stack_base
    mov rsi, 0xFFFFFF0000080000
    mov [args.stack_base], rsi
    ; stack_size
    mov rcx, 0x1F000
    mov [args.stack_size], rcx

    ; set stack pointer
    mov rsp, rsi
    add rsp, rcx

    ; copy env to stack
;%ifdef KERNEL
;    mov rsi, 0
;    mov rcx, 0
;%else
;    mov rsi, redoxfs.env
;    mov rcx, redoxfs.env.end - redoxfs.env
;%endif
;    mov [args.env_size], rcx
;.copy_env:
;    cmp rcx, 0
;    je .no_env
;    dec rcx
;    mov al, [rsi + rcx]
;    dec rsp
;    mov [rsp], al
;    jmp .copy_env
.no_env:
    mov rsi, 0xFFFFFF0000080000
    mov [args.env_base], rsi

    ; align stack
    and rsp, 0xFFFFFFFFFFFFFFF0

    ; set args
    mov rdi, args

    ; entry point
    mov rax, [args.kernel_base]
    call [rax + 0x18]
.halt:
    cli
    hlt
    jmp .halt

gdtr:
    dw gdt.end + 1  ; size
    dq gdt          ; offset

gdt:
.null equ $ - gdt
    dq 0

.kernel_code equ $ - gdt
istruc GDTEntry
    at GDTEntry.limitl, dw 0
    at GDTEntry.basel, dw 0
    at GDTEntry.basem, db 0
    at GDTEntry.attribute, db attrib.present | attrib.user | attrib.code
    at GDTEntry.flags__limith, db flags.long_mode
    at GDTEntry.baseh, db 0
iend

.kernel_data equ $ - gdt
istruc GDTEntry
    at GDTEntry.limitl, dw 0
    at GDTEntry.basel, dw 0
    at GDTEntry.basem, db 0
; AMD System Programming Manual states that the writeable bit is ignored in long mode, but ss can not be set to this descriptor without it
    at GDTEntry.attribute, db attrib.present | attrib.user | attrib.writable
    at GDTEntry.flags__limith, db 0
    at GDTEntry.baseh, db 0
iend

.end equ $ - gdt
		

;	;mov eax, [_start.start_lba]
;	;mov bx, 0xc000
;	;mov cx, [_start.num_sectors]
;	;xor dx, dx
;	;call load
;	;call test
;	;call print_char 
;	;call print_line
;	;mov si, hello
;	;call print
;	;switch back to long mode
;	mov eax, cr0
;	or eax, 80000001h
;	mov cr0, eax
;	jmp DESC_LONG:comp_again
;       	;db           66h
;       	;db           0EAh
;       	;dd           comp_again
;       	;dw           DESC_LONG
;
;test:
;	mov al, [0xd1fe]
;        cmp al, 0x55
;        jnz error
;        mov al, [0xd1ff]
;        cmp al, 0xaa
;        jnz error
;        xor eax, eax 
;	ret
;; load some sectors from disk to a buffer in memory
;; buffer has to be below 1MiB
;; IN
;;   ax: start sector
;;   bx: offset of buffer
;;   cx: number of sectors (512 Bytes each)
;;   dx: segment of buffer
;; CLOBBER
;;   ax, bx, cx, dx, si
;; TODO rewrite to (eventually) move larger parts at once
;; if that is done increase buffer_size_sectors in startup-common to that (max 0x80000 - startup_end)
;load:
;	cmp cx, 127
;	jbe .good_size
;
;	pusha
;	mov cx, 127
;	call load
;	popa
;	add eax, 127
;	add dx, 127 * 512 / 16
;	sub cx, 127
;
;	jmp load
;.good_size:
;	mov [DAPACK.addr], eax
;	mov [DAPACK.buf], bx
;	mov [DAPACK.count], cx
;	mov [DAPACK.seg], dx
;
;	;call print_dapack
;
;	mov dl, [_start.drive]
;	mov si, DAPACK
;	mov ah, 0x42
;	int 0x13
;	jc error
;	ret
;
;disk: db 0x80
;error:
;	mov eax, 0xdeadbeef
;	ret
;
;DAPACK:
;        db 0x10
;        db 0
;.count: dw 0 ; int 13 resets this to # of blocks actually read/written
;.buf:   dw 0 ; memory buffer destination address (0:7c00)
;.seg:   dw 0 ; in memory page zero
;.addr:  dq 0 ; put the lba to read in this spot
;
;
;USE64
;comp_again:
;	lgdt [_start.gdtv64]                    ;restore gdt
;	mov ax, DESC_DATA                   ;read YOUR DATA descriptor to selectors
;	mov ds, ax
;	mov es, ax
;	mov gs, ax
;	mov ss, ax
;	mov ax, 0x18
;	mov fs, ax
;	lidt [_start.idt64]                        ;restore idt
;	mov rsp, qword [_start.stckptr]           ;restore stack
;	;must be a non rip-relative jump
;	;sti
;	pop rbp
;	ret

