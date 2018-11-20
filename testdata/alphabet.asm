ldi sp, MEMORY_END // Setup stack

call print_alphabet
halt

print_alphabet:
ldi r0, 0
ldi r1, 65 // A in decimal
loop:
cmpi r0, 26
breq end
stb r1, CONSOLEIO_START // Mapped ConsoleIO device
inc r1
inc r0
jmp loop
end:
ret
