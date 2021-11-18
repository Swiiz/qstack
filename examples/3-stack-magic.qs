; There is more cool things to do with the stack

; To be ready to take full control over the stack,
; you need to know 3 more tokens for manipulating it:
; - "!" : to pop the stack
; - "&" : to duplicate the top value on the stack
; - "@" : to rotate the stack
; The exclamation mark token works just as ".", but it doesn't
; show anything onto the console.
; The ampersand token simply duplicates the top value on the stack.
; For example "1 &" will push "1" and then "1" again onto the stack.
; The at token rotates the stack. Placing the top value underneath the stack.
; For example "1 2 3 @ . . ." will print "3 1 2" onto the console.
; The stack is a LIFO structure, so the order of the operations is important.

; You can run this program by running "qstack 3-stack-magic"

1 & 2 & 3 & ! @ ! @ . . . .

; Output:
; 1
; 1
; 3
; 2