; Hello world!

; Finally we reached what is usually the first thing you learn in a new programming language.
; How to print hello world!
; This language is not really made to play with text, but i'm sure using some well made functions,
; you could do some really interesting things.
; You can use the ":" token to print a number as it's corresponding unicode character.
; For example, "32 :" will print a space.
; Furthermore, you can use the * token to translate character to their numerical equivalent and push it to the stack.
; For example, "*A :" will print a A.
; Lastly, you can chain character after the "*" token to push every character's numerical equivalent to the stack.
; For example, "*ABC" is equivalent to "65 66 67".
; But note that with how the stack works, "*Hello : : : :" will print "Hello" in reverse order.
; That is why we create a reverse function in this program, which reverse the order of the stack (Only work for stack of size 5). 

; You can run this program by running "qstack 8-helloworld"

swap { $ ! $ 0 $ }
w { @ swap }
reverse { w w w @ @ w w @ @ @ w }
print { 0 > & : < }

*Hello reverse 0 @ print
32 :
*World reverse 0 @ print
32 : *! :