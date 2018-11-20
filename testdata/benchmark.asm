ldi sp, MEMORY_END // Setup stack

ldi r0, 10000000

loop2:
cmpi r0, 0
breq stop
call print_alphabet
dec r0
jmp loop2
stop:
halt

print_alphabet:
push r0
push r1
ldi r0, 0
ldi r1, 65 // A in decimal
loop:
cmpi r0, 26
breq end
inc r1
inc r0
jmp loop
end:
pop r1
pop r0
ret
