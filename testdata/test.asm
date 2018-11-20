ldi sp, MEMORY_END // Setup stack

ldi r0, CONSOLEIO_START
ldi r1, 65
stb r1, r0
halt
