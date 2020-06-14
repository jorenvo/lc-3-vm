.ORIG x3000                        ; this is the address in memory where the program will be loaded
AND R0, R0, #0                     ; zero R0
ADD R0, R0, #7                     ; R0 + 7 into R0
ADD R0, R0, 1                      ; R0 + 1 into R0
.END                               ; mark the end of the file (R0 should be 8)