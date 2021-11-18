; Fibonacci time!

; Ok so we learned a lot of things, now it's time to use them!
; In this example we will compute the Fibonacci sequence.
; For those who don't now the fibonacci sequence is a sequence of numbers
; where the next number is the sum of the previous two numbers.
; 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 ... and so on.
; For this example we will use everything we learned so far.
; It may take you a bit of time to undersand the code, but a good advice is
; to just get yourself a pen and some paper and start writing down the steps, the stack and the register.

; You can run this program by running "qstack 6-fibonacci"

0 & . 1 & . 0 > & $ ! + $ 0 $ & . 1 <
; Output:
; 0
; 1
; 1
; 2
; 3
; 5
; 8
; 13
; 21
; 34
; 55
; 89
; 144
; ... And so on up to the overflow error!