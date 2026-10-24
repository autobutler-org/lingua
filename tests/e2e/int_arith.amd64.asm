TEXT int_arith.main(SB)
  LEAQ  -816(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $800, SP
  MOVQ  $7, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $3, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L2
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $16, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $1, AX
  MOVQ  AX, -72(BP)
  MOVQ  -64(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -64(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L3
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $16, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $1, AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -120(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -32(BP), AX
  MOVQ  -16(BP), CX
  SUBQ  CX, AX
  JOS   L4
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $16, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $1, AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  IMULQ CX, AX
  JOS   L5
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  MOVQ  $16, AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  $1, AX
  MOVQ  AX, -240(BP)
  MOVQ  -232(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -232(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $2, AX
  MOVQ  AX, -264(BP)
  MOVQ  -32(BP), AX
  MOVQ  -264(BP), CX
  IMULQ CX, AX
  JOS   L6
  MOVQ  AX, -272(BP)
  MOVQ  -16(BP), AX
  MOVQ  -272(BP), CX
  ADDQ  CX, AX
  JOS   L7
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  MOVQ  $16, AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $1, AX
  MOVQ  AX, -312(BP)
  MOVQ  -304(BP), AX
  MOVQ  -312(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -304(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -304(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L8
  MOVQ  AX, -336(BP)
  MOVQ  $2, AX
  MOVQ  AX, -344(BP)
  MOVQ  -336(BP), AX
  MOVQ  -344(BP), CX
  IMULQ CX, AX
  JOS   L9
  MOVQ  AX, -352(BP)
  MOVQ  -352(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -360(BP)
  MOVQ  $16, AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  MOVQ  $1, AX
  MOVQ  AX, -384(BP)
  MOVQ  -376(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -376(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -376(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L10
  MOVQ  AX, -408(BP)
  MOVQ  $1, AX
  MOVQ  AX, -416(BP)
  MOVQ  -408(BP), AX
  MOVQ  -416(BP), CX
  SUBQ  CX, AX
  JOS   L11
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $16, AX
  MOVQ  AX, -440(BP)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  $1, AX
  MOVQ  AX, -456(BP)
  MOVQ  -448(BP), AX
  MOVQ  -456(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -448(BP), AX
  MOVQ  -432(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -448(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  MOVQ  -464(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -472(BP)
  MOVQ  $2, AX
  MOVQ  AX, -480(BP)
  MOVQ  $3, AX
  MOVQ  AX, -488(BP)
  MOVQ  -480(BP), AX
  MOVQ  -488(BP), CX
  IMULQ CX, AX
  JOS   L12
  MOVQ  AX, -496(BP)
  MOVQ  $4, AX
  MOVQ  AX, -504(BP)
  MOVQ  $5, AX
  MOVQ  AX, -512(BP)
  MOVQ  -504(BP), AX
  MOVQ  -512(BP), CX
  IMULQ CX, AX
  JOS   L13
  MOVQ  AX, -520(BP)
  MOVQ  -496(BP), AX
  MOVQ  -520(BP), CX
  ADDQ  CX, AX
  JOS   L14
  MOVQ  AX, -528(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  MOVQ  $16, AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  $1, AX
  MOVQ  AX, -560(BP)
  MOVQ  -552(BP), AX
  MOVQ  -560(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -552(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -552(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -568(BP)
  MOVQ  -568(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -576(BP)
  MOVQ  -16(BP), AX
  NEGQ  AX
  JOS   L15
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L16
  MOVQ  AX, -592(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -600(BP)
  MOVQ  $16, AX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -616(BP)
  MOVQ  $1, AX
  MOVQ  AX, -624(BP)
  MOVQ  -616(BP), AX
  MOVQ  -624(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -616(BP), AX
  MOVQ  -600(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -616(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -632(BP)
  MOVQ  -632(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -640(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L17
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  NEGQ  AX
  JOS   L18
  MOVQ  AX, -656(BP)
  MOVQ  -656(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -664(BP)
  MOVQ  $16, AX
  MOVQ  AX, -672(BP)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -680(BP)
  MOVQ  $1, AX
  MOVQ  AX, -688(BP)
  MOVQ  -680(BP), AX
  MOVQ  -688(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -680(BP), AX
  MOVQ  -664(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -680(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  -696(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -704(BP)
  MOVQ  -32(BP), AX
  NEGQ  AX
  JOS   L19
  MOVQ  AX, -712(BP)
  MOVQ  -16(BP), AX
  MOVQ  -712(BP), CX
  IMULQ CX, AX
  JOS   L20
  MOVQ  AX, -720(BP)
  MOVQ  -720(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -728(BP)
  MOVQ  $16, AX
  MOVQ  AX, -736(BP)
  MOVQ  -736(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -744(BP)
  MOVQ  $1, AX
  MOVQ  AX, -752(BP)
  MOVQ  -744(BP), AX
  MOVQ  -752(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -744(BP), AX
  MOVQ  -728(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -744(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -768(BP)
  MOVQ  $0, AX
  MOVQ  AX, -776(BP)
  MOVQ  -776(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
LABEL L9
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
LABEL L12
  LEAQ  site.11(SB), AX
  JMP   rt.fault(SB)
LABEL L13
  LEAQ  site.12(SB), AX
  JMP   rt.fault(SB)
LABEL L14
  LEAQ  site.13(SB), AX
  JMP   rt.fault(SB)
LABEL L15
  LEAQ  site.14(SB), AX
  JMP   rt.fault(SB)
LABEL L16
  LEAQ  site.15(SB), AX
  JMP   rt.fault(SB)
LABEL L17
  LEAQ  site.16(SB), AX
  JMP   rt.fault(SB)
LABEL L18
  LEAQ  site.17(SB), AX
  JMP   rt.fault(SB)
LABEL L19
  LEAQ  site.18(SB), AX
  JMP   rt.fault(SB)
LABEL L20
  LEAQ  site.19(SB), AX
  JMP   rt.fault(SB)
