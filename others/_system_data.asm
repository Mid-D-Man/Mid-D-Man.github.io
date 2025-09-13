; file: sysinfo.asm
; Build:
;   nasm -felf64 sysinfo.asm -o sysinfo.o
;   ld -o sysinfo sysinfo.o
; Run:
;   ./sysinfo

BITS 64
GLOBAL _start

SECTION .data
    ; Labels
    lbl_title_uname:    db "=== uname ===", 10, 0
    lbl_sysname:        db "Kernel: ", 0
    lbl_release:        db "Release: ", 0
    lbl_version:        db "Version: ", 0
    lbl_machine:        db "Machine: ", 0

    lbl_title_sysinfo:  db 10, "=== sysinfo ===", 10, 0
    lbl_uptime:         db "Uptime (s): ", 0
    lbl_totalram:       db "Total RAM (units): ", 0
    lbl_freeram:        db "Free RAM (units): ", 0
    lbl_procs:          db "Processes: ", 0
    lbl_memunit:        db "mem_unit (bytes per unit): ", 0

    nl:                 db 10, 0
    sep:                db "------------------------", 10, 0

SECTION .bss
    ; utsname structure (Linux): 6 fields * 65 bytes each = 390 bytes
    ; Offsets: sysname=0, nodename=65, release=130, version=195, machine=260, domainname=325
    utsname_buf:    resb 390

    ; sysinfo structure buffer (enough room, 128 bytes)
    sysinfo_buf:    resb 128

    ; Integer-to-string buffer (decimal, up to 20 digits + newline)
    itoa_buf:       resb 32

SECTION .text

; ------------------------------------------------------------
; write(fd=1, rsi=ptr, rdx=len)
print:
    mov rax, 1
    mov rdi, 1
    syscall
    ret

; ------------------------------------------------------------
; print a zero-terminated C string at RSI
; computes length, then calls write
print_cstr:
    push rsi
    mov rdi, rsi          ; save base
    xor rcx, rcx
.find0:
    cmp byte [rsi], 0
    je .gotlen
    inc rsi
    inc rcx
    jmp .find0
.gotlen:
    mov rsi, rdi
    mov rdx, rcx
    call print
    pop rsi
    ret

; ------------------------------------------------------------
; print newline
print_nl:
    mov rsi, nl
    call print_cstr
    ret

; ------------------------------------------------------------
; print label (C string) then another C string
print_label_and_cstr:
    ; rsi = label ptr, rdx = cstr ptr
    push rdx
    call print_cstr
    pop rsi
    call print_cstr
    call print_nl
    ret

; ------------------------------------------------------------
; unsigned 64-bit integer in RAX -> decimal string in itoa_buf (zero-terminated)
; returns RSI = pointer to zero-terminated string
utoa_dec:
    mov rbx, 10
    lea rdi, [itoa_buf + 31]
    mov byte [rdi], 0
    dec rdi
    xor rcx, rcx
    cmp rax, 0
    jne .conv
    mov byte [rdi], '0'
    jmp .done

.conv:
    ; divide RAX by 10 repeatedly, store digits backward
    xor rdx, rdx
    div rbx            ; RAX = RAX/10, RDX = RAX%10
    add dl, '0'
    mov byte [rdi], dl
    dec rdi
    test rax, rax
    jne .conv

    inc rdi
    jmp .finish

.done:
    ; single zero digit already placed at [rdi]
.finish:
    mov rsi, rdi
    ret

; ------------------------------------------------------------
; print label + unsigned number in RAX (decimal) + newline
print_label_and_u64:
    ; rsi = label ptr, rax = number
    push rax
    push rsi
    call print_cstr
    pop rsi             ; restore label ptr (unused now)
    pop rax
    call utoa_dec
    call print_cstr
    call print_nl
    ret

; ------------------------------------------------------------
; Extract and print a null-terminated field inside a struct
; rdi = base pointer, rax = field offset, rsi = label ptr
print_struct_cfield:
    push rdi
    add rdi, rax
    ; rdi now points to C string field
    mov rdx, rdi
    mov rsi, rsi        ; label unchanged
    ; print label then field
    mov rsi, rsi        ; keep label in rsi
    mov rsi, rsi
    ; Use helper that expects rsi=label, rdx=cstr
    ; We’ll use stack to pass rdx
    push rdx
    call print_cstr     ; print label
    pop rsi             ; rsi = cstr
    call print_cstr     ; print field
    call print_nl
    pop rdi
    ret

_start:
    ; ---- uname ----
    mov rax, 63                 ; sys_uname
    mov rdi, utsname_buf
    syscall

    mov rsi, lbl_title_uname
    call print_cstr

    ; Offsets inside utsname
    ; sysname=0, release=130, version=195, machine=260
    ; print Kernel
    mov rsi, lbl_sysname
    mov rdi, utsname_buf
    mov rax, 0
    call print_struct_cfield

    ; Release
    mov rsi, lbl_release
    mov rdi, utsname_buf
    mov rax, 130
    call print_struct_cfield

    ; Version
    mov rsi, lbl_version
    mov rdi, utsname_buf
    mov rax, 195
    call print_struct_cfield

    ; Machine
    mov rsi, lbl_machine
    mov rdi, utsname_buf
    mov rax, 260
    call print_struct_cfield

    mov rsi, lbl_title_sysinfo
    call print_cstr

    ; ---- sysinfo ----
    mov rax, 99                 ; sys_sysinfo
    mov rdi, sysinfo_buf
    syscall

    ; sysinfo offsets on x86-64 (typical):
    ; 0: uptime (long)
    ; 32: totalram (unsigned long)
    ; 40: freeram  (unsigned long)
    ; 80: procs (unsigned short)
    ; 104: mem_unit (unsigned int)
    ;
    ; Note: totalram/freeram are in units of mem_unit. To get bytes, multiply.

    ; Uptime
    mov rsi, lbl_uptime
    mov rax, [sysinfo_buf + 0]
    call print_label_and_u64

    ; Total RAM (units)
    mov rsi, lbl_totalram
    mov rax, [sysinfo_buf + 32]
    call print_label_and_u64

    ; Free RAM (units)
    mov rsi, lbl_freeram
    mov rax, [sysinfo_buf + 40]
    call print_label_and_u64

    ; Processes (16-bit), zero-extend
    movzx rax, word [sysinfo_buf + 80]
    mov rsi, lbl_procs
    call print_label_and_u64

    ; mem_unit (bytes per unit)
    mov eax, dword [sysinfo_buf + 104]
    mov rsi, lbl_memunit
    call print_label_and_u64

    ; Exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
