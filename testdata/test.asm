ldi sp, 0x100
ldi r0, 10
call foo
jmp end
foo:
ldi r1, 10
push r1
pop r2
ret
end:
halt