; What the func!

; Now to make our life a bit easier we will learn how to make functions
; Functions can be declared easily in the following way:
; function_name { ... }
; You can then call them just by writing the function name.
; You can also use function inside other functions or even inside themselves.
; The following example shows how to use a function inside another function in order to count up to 10.

; You can run this program by running "qstack 7-functions"

; Function declaration
count {
    1 + & . & 9 - > count 0 <
}
; Actual running code
0 count
; Output:
; 1
; 2
; 3
; 4
; 5
; 6
; 7
; 8
; 9
; 10