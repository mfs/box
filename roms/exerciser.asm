; 4001 exerciser program
wrr
bbl 15
fim 5 65
jms 222 ; ld_mk

jms 229 ; ck_idx

fin 0
db 254
jms 238 ; ck_fin

jms 229 ; ck_idx
jms 238 ; ck_fin

jms 229 ; ck_idx



; ...


* = 201 ; temp until rest of program entered
; at address 201
src 5
rd0
jcn az 215 ; a=0 not sure how to assemble these conditionals
ldm 8
src 0
wmp
clb
src 5
wr0
jcn tn 211 ; T=1
jun 32

; subroutines

; at address 222
ld_mk
	src 5
	ld 11
	clc
	wmp
	ral
	xch 11
	bbl 0


; at address 229
ck_idx
	src 0
	src 1
	src 2
	src 3
	src 4
	src 5
	src 6
	src 7
	bbl 0

; at address 238
ck_fin
	fin 1
	fin 2
	fin 3
	fin 4
	fin 5
	fin 6
	fin 7
	fin 0
	bbl 0

; at address 247
ck_dcl
	ld 4
	ral
	dcl
	xch 4
	rdr
	bbl 0



; data at address 253
nop
.byte 255
.byte 0




