TEXT list_basics.main(SB)
  LEAQ  -1296(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L10
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $1280, SP
  MOVQ  $1, AX
  MOVQ  AX, -8(BP)
  MOVQ  $2, AX
  MOVQ  AX, -16(BP)
  MOVQ  $3, AX
  MOVQ  AX, -24(BP)
  MOVQ  $32, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $3, AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -40(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -40(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -40(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -40(BP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $4, AX
  MOVQ  AX, -64(BP)
  MOVQ  $5, AX
  MOVQ  AX, -72(BP)
  MOVQ  $24, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $2, AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -88(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -88(BP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $24, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $2, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -160(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -160(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -200(BP)
  MOVQ  $0, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, -216(BP)
  JMP   L1
LABEL L1
  MOVQ  -216(BP), AX
  MOVQ  -200(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -232(BP)
  MOVQ  -216(BP), AX
  MOVQ  -232(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -240(BP)
  MOVQ  -192(BP), AX
  MOVQ  -240(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $16, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $1, AX
  MOVQ  AX, -288(BP)
  MOVQ  -280(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -280(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $1, AX
  MOVQ  AX, -312(BP)
  MOVQ  -216(BP), AX
  MOVQ  -312(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -216(BP)
  JMP   L1
LABEL L3
  MOVQ  $8, AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  $0, AX
  MOVQ  AX, -336(BP)
  MOVQ  -328(BP), AX
  MOVQ  -336(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -328(BP), AX
  MOVQ  AX, -344(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -352(BP)
  MOVQ  -344(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  $24, AX
  MOVQ  AX, -376(BP)
  MOVQ  -376(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -384(BP)
  MOVQ  $2, AX
  MOVQ  AX, -392(BP)
  MOVQ  -384(BP), AX
  MOVQ  -392(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -384(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -384(BP), AX
  MOVQ  -368(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -408(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  AX, -424(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -432(BP)
  MOVQ  -424(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -440(BP)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  $24, AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  MOVQ  $2, AX
  MOVQ  AX, -472(BP)
  MOVQ  -464(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -464(BP), AX
  MOVQ  -432(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -464(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -464(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -488(BP)
  MOVQ  $8, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -504(BP)
  MOVQ  $0, AX
  MOVQ  AX, -512(BP)
  MOVQ  -504(BP), AX
  MOVQ  -512(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  -520(BP), AX
  MOVQ  AX, -528(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -536(BP)
  MOVQ  -528(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  $24, AX
  MOVQ  AX, -560(BP)
  MOVQ  -560(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -568(BP)
  MOVQ  $2, AX
  MOVQ  AX, -576(BP)
  MOVQ  -568(BP), AX
  MOVQ  -576(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -568(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -568(BP), AX
  MOVQ  -552(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -568(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -592(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  MOVQ  AX, -616(BP)
  MOVQ  -616(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -624(BP)
  MOVQ  $0, AX
  MOVQ  AX, -632(BP)
  MOVQ  -632(BP), AX
  MOVQ  AX, -640(BP)
  JMP   L4
LABEL L4
  MOVQ  -640(BP), AX
  MOVQ  -624(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L6
LABEL L5
  MOVQ  $3, AX
  MOVQ  AX, -656(BP)
  MOVQ  -640(BP), AX
  MOVQ  -656(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -664(BP)
  MOVQ  -616(BP), AX
  MOVQ  -664(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -672(BP)
  MOVQ  -672(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -680(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -688(BP)
  MOVQ  -680(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $24, AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -712(BP)
  MOVQ  $2, AX
  MOVQ  AX, -720(BP)
  MOVQ  -712(BP), AX
  MOVQ  -720(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -712(BP), AX
  MOVQ  -688(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -712(BP), AX
  MOVQ  -696(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -712(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -728(BP)
  MOVQ  -728(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -736(BP)
  MOVQ  $1, AX
  MOVQ  AX, -744(BP)
  MOVQ  -640(BP), AX
  MOVQ  -744(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -640(BP)
  JMP   L4
LABEL L6
  LEAQ  str.5(SB), AX
  MOVQ  AX, -752(BP)
  MOVQ  $7, AX
  MOVQ  AX, -760(BP)
  MOVQ  $8, AX
  MOVQ  AX, -768(BP)
  MOVQ  $9, AX
  MOVQ  AX, -776(BP)
  MOVQ  $32, AX
  MOVQ  AX, -784(BP)
  MOVQ  -784(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -792(BP)
  MOVQ  $3, AX
  MOVQ  AX, -800(BP)
  MOVQ  -792(BP), AX
  MOVQ  -800(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -792(BP), AX
  MOVQ  -760(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -792(BP), AX
  MOVQ  -768(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -792(BP), AX
  MOVQ  -776(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -792(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -808(BP)
  MOVQ  -808(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -816(BP)
  MOVQ  $24, AX
  MOVQ  AX, -824(BP)
  MOVQ  -824(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -832(BP)
  MOVQ  $2, AX
  MOVQ  AX, -840(BP)
  MOVQ  -832(BP), AX
  MOVQ  -840(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -832(BP), AX
  MOVQ  -752(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -832(BP), AX
  MOVQ  -816(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -832(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -848(BP)
  MOVQ  -848(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -856(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -864(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -872(BP)
  MOVQ  $24, AX
  MOVQ  AX, -880(BP)
  MOVQ  -880(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -888(BP)
  MOVQ  $2, AX
  MOVQ  AX, -896(BP)
  MOVQ  -888(BP), AX
  MOVQ  -896(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -888(BP), AX
  MOVQ  -864(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -888(BP), AX
  MOVQ  -872(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -888(BP), AX
  MOVQ  AX, -904(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -912(BP)
  MOVQ  -904(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -920(BP)
  MOVQ  -920(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -928(BP)
  MOVQ  $24, AX
  MOVQ  AX, -936(BP)
  MOVQ  -936(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -944(BP)
  MOVQ  $2, AX
  MOVQ  AX, -952(BP)
  MOVQ  -944(BP), AX
  MOVQ  -952(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -944(BP), AX
  MOVQ  -912(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -944(BP), AX
  MOVQ  -928(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -944(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -960(BP)
  MOVQ  -960(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -968(BP)
  MOVQ  $1, AX
  MOVQ  AX, -976(BP)
  MOVQ  $2, AX
  MOVQ  AX, -984(BP)
  MOVQ  $24, AX
  MOVQ  AX, -992(BP)
  MOVQ  -992(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1000(BP)
  MOVQ  $2, AX
  MOVQ  AX, -1008(BP)
  MOVQ  -1000(BP), AX
  MOVQ  -1008(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1000(BP), AX
  MOVQ  -976(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1000(BP), AX
  MOVQ  -984(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  $3, AX
  MOVQ  AX, -1016(BP)
  MOVQ  $16, AX
  MOVQ  AX, -1024(BP)
  MOVQ  -1024(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1032(BP)
  MOVQ  $1, AX
  MOVQ  AX, -1040(BP)
  MOVQ  -1032(BP), AX
  MOVQ  -1040(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1032(BP), AX
  MOVQ  -1016(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $8, AX
  MOVQ  AX, -1048(BP)
  MOVQ  -1048(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1056(BP)
  MOVQ  $0, AX
  MOVQ  AX, -1064(BP)
  MOVQ  -1056(BP), AX
  MOVQ  -1064(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  $32, AX
  MOVQ  AX, -1072(BP)
  MOVQ  -1072(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1080(BP)
  MOVQ  $3, AX
  MOVQ  AX, -1088(BP)
  MOVQ  -1080(BP), AX
  MOVQ  -1088(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1080(BP), AX
  MOVQ  -1000(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1080(BP), AX
  MOVQ  -1032(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -1080(BP), AX
  MOVQ  -1056(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -1080(BP), AX
  MOVQ  AX, -1096(BP)
  MOVQ  -1096(BP), AX
  MOVQ  AX, -1104(BP)
  MOVQ  -1104(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -1112(BP)
  MOVQ  $0, AX
  MOVQ  AX, -1120(BP)
  MOVQ  -1120(BP), AX
  MOVQ  AX, -1128(BP)
  JMP   L7
LABEL L7
  MOVQ  -1128(BP), AX
  MOVQ  -1112(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -1136(BP)
  MOVQ  -1136(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L9
LABEL L8
  MOVQ  $3, AX
  MOVQ  AX, -1144(BP)
  MOVQ  -1128(BP), AX
  MOVQ  -1144(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -1152(BP)
  MOVQ  -1104(BP), AX
  MOVQ  -1152(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -1160(BP)
  MOVQ  -1160(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -1168(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -1176(BP)
  MOVQ  -1168(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -1184(BP)
  MOVQ  -1184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1192(BP)
  MOVQ  $24, AX
  MOVQ  AX, -1200(BP)
  MOVQ  -1200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1208(BP)
  MOVQ  $2, AX
  MOVQ  AX, -1216(BP)
  MOVQ  -1208(BP), AX
  MOVQ  -1216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1208(BP), AX
  MOVQ  -1176(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1208(BP), AX
  MOVQ  -1192(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -1208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1224(BP)
  MOVQ  -1224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1232(BP)
  MOVQ  $1, AX
  MOVQ  AX, -1240(BP)
  MOVQ  -1128(BP), AX
  MOVQ  -1240(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -1128(BP)
  JMP   L7
LABEL L9
  MOVQ  $0, AX
  MOVQ  AX, -1248(BP)
  MOVQ  -1248(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L10
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
