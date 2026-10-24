TEXT bool_compare.show(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.1(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT bool_compare.main(SB)
  LEAQ  -928(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $912, SP
  MOVQ  $3, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $5, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
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
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -120(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETGT AX
  MOVBQZX AX, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $24, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $2, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -152(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -152(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  $3, AX
  MOVQ  AX, -192(BP)
  MOVQ  -16(BP), AX
  MOVQ  -192(BP), CX
  CMPQ  AX, CX
  SETLE AX
  MOVBQZX AX, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $24, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $2, AX
  MOVQ  AX, -232(BP)
  MOVQ  -224(BP), AX
  MOVQ  -232(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -224(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -224(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -256(BP)
  MOVQ  $4, AX
  MOVQ  AX, -264(BP)
  MOVQ  -16(BP), AX
  MOVQ  -264(BP), CX
  CMPQ  AX, CX
  SETGE AX
  MOVBQZX AX, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $24, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  $2, AX
  MOVQ  AX, -304(BP)
  MOVQ  -296(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -296(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -296(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -328(BP)
  MOVQ  $3, AX
  MOVQ  AX, -336(BP)
  MOVQ  -16(BP), AX
  MOVQ  -336(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $24, AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  $2, AX
  MOVQ  AX, -376(BP)
  MOVQ  -368(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -368(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -368(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -400(BP)
  MOVQ  $3, AX
  MOVQ  AX, -408(BP)
  MOVQ  -16(BP), AX
  MOVQ  -408(BP), CX
  CMPQ  AX, CX
  SETNE AX
  MOVBQZX AX, AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -424(BP)
  MOVQ  $24, AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -440(BP)
  MOVQ  $2, AX
  MOVQ  AX, -448(BP)
  MOVQ  -440(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -440(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -440(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -472(BP)
  MOVQ  -16(BP), AX
  NEGQ  AX
  JOS   L2
  MOVQ  AX, -480(BP)
  MOVQ  -32(BP), AX
  NEGQ  AX
  JOS   L3
  MOVQ  AX, -488(BP)
  MOVQ  -480(BP), AX
  MOVQ  -488(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -504(BP)
  MOVQ  $24, AX
  MOVQ  AX, -512(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  $2, AX
  MOVQ  AX, -528(BP)
  MOVQ  -520(BP), AX
  MOVQ  -528(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -520(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -520(BP), AX
  MOVQ  -504(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -520(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  MOVQ  -536(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -544(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -552(BP)
  MOVQ  $2, AX
  MOVQ  AX, -560(BP)
  MOVQ  -16(BP), AX
  MOVQ  -560(BP), CX
  ADDQ  CX, AX
  JOS   L4
  MOVQ  AX, -568(BP)
  MOVQ  -568(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -576(BP)
  MOVQ  -576(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  $24, AX
  MOVQ  AX, -592(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -600(BP)
  MOVQ  $2, AX
  MOVQ  AX, -608(BP)
  MOVQ  -600(BP), AX
  MOVQ  -608(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -600(BP), AX
  MOVQ  -552(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -600(BP), AX
  MOVQ  -584(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -600(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -616(BP)
  MOVQ  -616(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -624(BP)
  LEAQ  str.11(SB), AX
  MOVQ  AX, -632(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -640(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -640(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -648(BP)
  MOVQ  $1, AX
  MOVQ  AX, -656(BP)
  MOVQ  -648(BP), AX
  MOVQ  -656(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -672(BP)
  MOVQ  $24, AX
  MOVQ  AX, -680(BP)
  MOVQ  -680(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -688(BP)
  MOVQ  $2, AX
  MOVQ  AX, -696(BP)
  MOVQ  -688(BP), AX
  MOVQ  -696(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -688(BP), AX
  MOVQ  -632(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -688(BP), AX
  MOVQ  -672(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -712(BP)
  LEAQ  str.12(SB), AX
  MOVQ  AX, -720(BP)
  LEAQ  str.13(SB), AX
  MOVQ  AX, -728(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -728(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -736(BP)
  MOVQ  $1, AX
  MOVQ  AX, -744(BP)
  MOVQ  -736(BP), AX
  MOVQ  -744(BP), CX
  CMPQ  AX, CX
  SETNE AX
  MOVBQZX AX, AX
  MOVQ  AX, -752(BP)
  MOVQ  -752(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  $24, AX
  MOVQ  AX, -768(BP)
  MOVQ  -768(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -776(BP)
  MOVQ  $2, AX
  MOVQ  AX, -784(BP)
  MOVQ  -776(BP), AX
  MOVQ  -784(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -776(BP), AX
  MOVQ  -720(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -776(BP), AX
  MOVQ  -760(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -776(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -792(BP)
  MOVQ  -792(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -800(BP)
  LEAQ  str.14(SB), AX
  MOVQ  AX, -808(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -816(BP)
  MOVQ  $1, AX
  MOVQ  AX, -824(BP)
  MOVQ  -816(BP), AX
  MOVQ  -824(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -832(BP)
  MOVQ  -832(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bool_compare.show(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -840(BP)
  MOVQ  $24, AX
  MOVQ  AX, -848(BP)
  MOVQ  -848(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -856(BP)
  MOVQ  $2, AX
  MOVQ  AX, -864(BP)
  MOVQ  -856(BP), AX
  MOVQ  -864(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -856(BP), AX
  MOVQ  -808(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -856(BP), AX
  MOVQ  -840(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -856(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -872(BP)
  MOVQ  -872(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -880(BP)
  MOVQ  $0, AX
  MOVQ  AX, -888(BP)
  MOVQ  -888(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
