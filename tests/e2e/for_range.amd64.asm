TEXT for_range.main(SB)
  LEAQ  -800(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L19
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $784, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  $3, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -32(BP)
  JMP   L1
LABEL L1
  MOVQ  -32(BP), AX
  MOVQ  -24(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  -32(BP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $24, AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $2, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -80(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -80(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $1, AX
  MOVQ  AX, -112(BP)
  MOVQ  -32(BP), AX
  MOVQ  -112(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -32(BP)
  JMP   L1
LABEL L3
  MOVQ  $5, AX
  MOVQ  AX, -120(BP)
  MOVQ  $5, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, -144(BP)
  JMP   L4
LABEL L4
  MOVQ  -144(BP), AX
  MOVQ  -136(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L6
LABEL L5
  MOVQ  -144(BP), AX
  MOVQ  AX, -160(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $24, AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $2, AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  -200(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -192(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -192(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  MOVQ  $1, AX
  MOVQ  AX, -224(BP)
  MOVQ  -144(BP), AX
  MOVQ  -224(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -144(BP)
  JMP   L4
LABEL L6
  MOVQ  $-2, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, -240(BP)
  MOVQ  $1, AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, -272(BP)
  JMP   L7
LABEL L7
  MOVQ  -272(BP), AX
  MOVQ  -264(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L9
LABEL L8
  MOVQ  -272(BP), AX
  MOVQ  AX, -288(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -296(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $24, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  $2, AX
  MOVQ  AX, -328(BP)
  MOVQ  -320(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -320(BP), AX
  MOVQ  -296(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -320(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  $1, AX
  MOVQ  AX, -352(BP)
  MOVQ  -272(BP), AX
  MOVQ  -352(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -272(BP)
  JMP   L7
LABEL L9
  MOVQ  -240(BP), AX
  MOVQ  AX, -360(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, -368(BP)
  JMP   L10
LABEL L10
  MOVQ  -368(BP), AX
  MOVQ  -360(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -376(BP)
  MOVQ  -376(BP), AX
  TESTQ AX, AX
  JNE   L11
  JMP   L12
LABEL L11
  MOVQ  -368(BP), AX
  MOVQ  AX, -384(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -392(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  $24, AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  $2, AX
  MOVQ  AX, -424(BP)
  MOVQ  -416(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -416(BP), AX
  MOVQ  -392(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -416(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -440(BP)
  MOVQ  $1, AX
  MOVQ  AX, -448(BP)
  MOVQ  -368(BP), AX
  MOVQ  -448(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -368(BP)
  JMP   L10
LABEL L12
  MOVQ  $-3, AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, -464(BP)
  MOVQ  $0, AX
  MOVQ  AX, -472(BP)
  MOVQ  -464(BP), AX
  MOVQ  AX, -480(BP)
  MOVQ  -472(BP), AX
  MOVQ  AX, -488(BP)
  JMP   L13
LABEL L13
  MOVQ  -488(BP), AX
  MOVQ  -480(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  TESTQ AX, AX
  JNE   L14
  JMP   L15
LABEL L14
  MOVQ  -488(BP), AX
  MOVQ  AX, -504(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -512(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  $24, AX
  MOVQ  AX, -528(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  MOVQ  $2, AX
  MOVQ  AX, -544(BP)
  MOVQ  -536(BP), AX
  MOVQ  -544(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -536(BP), AX
  MOVQ  -512(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -536(BP), AX
  MOVQ  -520(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -536(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  -552(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -560(BP)
  MOVQ  $1, AX
  MOVQ  AX, -568(BP)
  MOVQ  -488(BP), AX
  MOVQ  -568(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -488(BP)
  JMP   L13
LABEL L15
  MOVQ  $2, AX
  MOVQ  AX, -576(BP)
  MOVQ  -576(BP), AX
  MOVQ  AX, -584(BP)
  MOVQ  $0, AX
  MOVQ  AX, -592(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, -600(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, -608(BP)
  JMP   L16
LABEL L16
  MOVQ  -608(BP), AX
  MOVQ  -600(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -616(BP)
  MOVQ  -616(BP), AX
  TESTQ AX, AX
  JNE   L17
  JMP   L18
LABEL L17
  MOVQ  -608(BP), AX
  MOVQ  AX, -624(BP)
  MOVQ  $10, AX
  MOVQ  AX, -632(BP)
  MOVQ  -584(BP), AX
  MOVQ  -632(BP), CX
  ADDQ  CX, AX
  JOS   L20
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  AX, -584(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -648(BP)
  MOVQ  -624(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -656(BP)
  MOVQ  $24, AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -672(BP)
  MOVQ  $2, AX
  MOVQ  AX, -680(BP)
  MOVQ  -672(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -672(BP), AX
  MOVQ  -648(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -672(BP), AX
  MOVQ  -656(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $1, AX
  MOVQ  AX, -704(BP)
  MOVQ  -608(BP), AX
  MOVQ  -704(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -608(BP)
  JMP   L16
LABEL L18
  LEAQ  str.4(SB), AX
  MOVQ  AX, -712(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -720(BP)
  MOVQ  $24, AX
  MOVQ  AX, -728(BP)
  MOVQ  -728(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -736(BP)
  MOVQ  $2, AX
  MOVQ  AX, -744(BP)
  MOVQ  -736(BP), AX
  MOVQ  -744(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -736(BP), AX
  MOVQ  -712(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -736(BP), AX
  MOVQ  -720(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -736(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -752(BP)
  MOVQ  -752(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  $0, AX
  MOVQ  AX, -768(BP)
  MOVQ  -768(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L19
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L20
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
