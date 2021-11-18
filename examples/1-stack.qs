; Basics of qstack

; Qstack is a stack based language, which means that the stack is the only way to store data. (or almost)
; Think about the stack as an infinite array.
; You can push data onto the stack, and pop data off of the stack.
; To push data to the stack, simply write down a number.
; Note that in qstack you separate the tokens with spaces.
; For example: "1 2 3 4 5" , the stack would then look like this: [1, 2, 3, 4, 5]
; To show and pop data from the stack, you can use "."
; For example "1 2 3 .", would output "3" into the console,
; and the stack would look like this (after the program finished): [1, 2]

; You can run this program by running "qstack 1-stack"

1 . 2 . 3 .
; Output:
; 1
; 2
; 3