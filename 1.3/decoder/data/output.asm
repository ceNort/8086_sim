bits 16
ADD BX, [BX + SI]
ADD BX, [BP + 0]
ADD SI, 2
ADD BP, 2
ADD CX, 8
ADD BX, [BP + 0]
ADD CX, [BX + 2]
ADD BH, [BP + SI + 4]
ADD DI, [BP + DI + 6]
ADD [BX + SI], BX
ADD [BP + 0], BX
ADD [BP + 0], BX
ADD [BX + 2], CX
ADD [BP + SI + 4], BH
ADD [BP + DI + 6], DI
ADD BYTE [BX], 34
ADD WORD [BP + SI + 1000], 29
ADD AX, [BP + 0]
ADD AL, [BX + SI]
ADD AX, BX
ADD AL, AH
ADD AX, 1000
ADD AL, -30
ADD AL, 9
SUB BX, [BX + SI]
SUB BX, [BP + 0]
SUB SI, 2
SUB BP, 2
SUB CX, 8
SUB BX, [BP + 0]
SUB CX, [BX + 2]
SUB BH, [BP + SI + 4]
SUB DI, [BP + DI + 6]
SUB [BX + SI], BX
SUB [BP + 0], BX
SUB [BP + 0], BX
SUB [BX + 2], CX
SUB [BP + SI + 4], BH
SUB [BP + DI + 6], DI
SUB BYTE [BX], 34
SUB WORD [BX + DI], 29
SUB AX, [BP + 0]
SUB AL, [BX + SI]
SUB AX, BX
SUB AL, AH
SUB AX, 1000
SUB AL, -30
SUB AL, 9
CMP BX, [BX + SI]
CMP BX, [BP + 0]
CMP SI, 2
CMP BP, 2
CMP CX, 8
CMP BX, [BP + 0]
CMP CX, [BX + 2]
CMP BH, [BP + SI + 4]
CMP DI, [BP + DI + 6]
CMP [BX + SI], BX
CMP [BP + 0], BX
CMP [BP + 0], BX
CMP [BX + 2], CX
CMP [BP + SI + 4], BH
CMP [BP + DI + 6], DI
CMP BYTE [BX], 34
CMP WORD [BP], 29
CMP AX, [BP + 0]
CMP AL, [BX + SI]
CMP AX, BX
CMP AL, AH
CMP AX, 1000
CMP AL, -30
CMP AL, 9
JNZ label, 2
JNZ label, -4
JNZ label, -6
JNZ label, -4
JE label, -2
JL label, -4
JLE label, -6
JB label, -8
JBE label, -10
JP label, -12
JO label, -14
JS label, -16
JNZ label, -18
JNL label, -20
JNLE label, -22
JNB label, -24
JNBE label, -26
JNP label, -28
JNO label, -30
JNS label, -32
LOOP label, -34
LOOPZ label, -36
LOOPNZ label, -38
JCXZ label, -40
