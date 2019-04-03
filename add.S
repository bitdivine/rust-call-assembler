.text
.globl my_adder
my_adder:
        addq %rdi, %rsi
        movq %rsi, %rax
        ret
