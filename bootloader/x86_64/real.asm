; Taken from https://forum.osdev.org/viewtopic.php?f=1&t=23125
ORG 0xb000
SECTION .bios
USE64
%define retfq o64 retf
_start:
	cli
	push rbp
	mov rbp, rsp 
	; 64 bit C calling convention
	mov [.start_lba], edi
	mov [.num_sectors], esi
	mov [.drive], edx

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

.start_lba:
	dd 	0
	align 	16, db 0
.drive: 
	dd 	0
	align 	16, db 0
.num_sectors:
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
	mov eax, cr0
	and eax, 7FFFFFFEh
	mov cr0, eax
	
	mov esp, 0xafff
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	jmp 0h:real

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
	mov eax, [start_lba]
	mov bx, 0xc000
	mov cx, [num_sectors]
	xor dx, dx
	call load
	call test
	;call print_char 
	;call print_line
	;mov si, hello
	;call print
	;switch back to long mode
	mov eax, cr0
	or eax, 80000001h
	mov cr0, eax
	jmp DESC_LONG:comp_again
       	;db           66h
       	;db           0EAh
       	;dd           comp_again
       	;dw           DESC_LONG

test:
	mov al, [0xd1fe]
        cmp al, 0x55
        jnz error
        mov al, [0xd1ff]
        cmp al, 0xaa
        jnz error
        xor eax, eax 
	ret
; load some sectors from disk to a buffer in memory
; buffer has to be below 1MiB
; IN
;   ax: start sector
;   bx: offset of buffer
;   cx: number of sectors (512 Bytes each)
;   dx: segment of buffer
; CLOBBER
;   ax, bx, cx, dx, si
; TODO rewrite to (eventually) move larger parts at once
; if that is done increase buffer_size_sectors in startup-common to that (max 0x80000 - startup_end)
load:
	cmp cx, 127
	jbe .good_size

	pusha
	mov cx, 127
	call load
	popa
	add eax, 127
	add dx, 127 * 512 / 16
	sub cx, 127

	jmp load
.good_size:
	mov [DAPACK.addr], eax
	mov [DAPACK.buf], bx
	mov [DAPACK.count], cx
	mov [DAPACK.seg], dx

	;call print_dapack

	mov dl, [disk]
	mov si, DAPACK
	mov ah, 0x42
	int 0x13
	jc error
	ret

disk: db 0x80
error:
	mov eax, 0xdeadbeef
	ret

DAPACK:
        db 0x10
        db 0
.count: dw 0 ; int 13 resets this to # of blocks actually read/written
.buf:   dw 0 ; memory buffer destination address (0:7c00)
.seg:   dw 0 ; in memory page zero
.addr:  dq 0 ; put the lba to read in this spot


USE64
comp_again:
	lgdt [_start.gdtv64]                    ;restore gdt
	mov ax, DESC_DATA                   ;read YOUR DATA descriptor to selectors
	mov ds, ax
	mov es, ax
	mov gs, ax
	mov ss, ax
	mov ax, 0x18
	mov fs, ax
	lidt [_start.idt64]                        ;restore idt
	mov rsp, qword [_start.stckptr]           ;restore stack
	;must be a non rip-relative jump
	sti
	pop rbp
	ret
