; The last most important thing

; To be able to do anything really useful, we need to be able to swap things around.
; Luckily, we can use the "$" token to swap the value on top of the stack with
; a value inside the only register we have. (Initialized to 0 by default)
; For example, the following code: "1 $ $ ." do the following:
; 1. Push 1 onto the stack
; 2. Swap the top value from the stack (1) with the value in the register (0)
; 3. Do the same thing again, so the top value is now 1 again
; 4. Print the value on the top of the stack (1)
; This token is super useful, because it allows us to do swap the top two values
; on the stack.
; For example, the following code: "$ ! $ 0 $" swaps the top two values on the stack

; You can run this program by running "qstack 5-swap"

1 2
$ ! $ 0 $
. .
; Output:
; 2
; 1