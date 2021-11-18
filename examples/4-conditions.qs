; Condition and loops

; In qstack there is no if statement or while and for loops
; there is only two tokens ">" and "<".
; As you can see, they are basically the same, except that they have a different direction.
; Secondly, they only activate if the value on top of the stack is greater than 0.
; The stack is then popped and if the token activated then the program jumps to the next,
; token with the opposite direction, which is not linked with an other token inside the two parents.
; For example: "> ! 0 > . < . <", in this example the tokens link this way:
;               ^     ^   ^   ^
;               |     |   |   |
;               |     +---+   | 
;               +-------------+
; Also you can create an infinite loop super easily this way:
; "0 > ........ 1 <"
; The first token will not activate as the value on the top of the stack is 0.
; When the program reaches the "<" token, the value on the top of the stack is 1.
; And so the program jumps to the ">" token, rerunning the commands inside the loop.
; The program will continue to do this for ever. But we could use mathematical operators
; to decide whenever to stop the loop.

; You can run this program by running "qstack 4-conditions"

; Prints "1" if 6 is greater than 5.
5 6 - > 1 . >
; Output:
; 1