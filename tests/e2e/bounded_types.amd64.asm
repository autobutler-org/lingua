TEXT bounded_types.widen(SB)
  LEAQ  -32(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $16, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT bounded_types.half(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  $2, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  TESTQ CX, CX
  JEQ   L2
  CMPQ  CX, $-1
  JNE   L4
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L3
LABEL L4
  CQO
  IDIVQ CX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
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
  LEAQ  site.2.1(SB), AX
  JMP   rt.fault(SB)

TEXT bounded_types.last_digit(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  $10, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  TESTQ CX, CX
  JEQ   L2
  CMPQ  CX, $-1
  JNE   L4
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L3
LABEL L4
  CQO
  IDIVQ CX
  MOVQ  DX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.4.1(SB), AX
  JMP   rt.fault(SB)

TEXT bounded_types.signed_digit(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  $10, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  TESTQ CX, CX
  JEQ   L2
  CMPQ  CX, $-1
  JNE   L4
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L3
LABEL L4
  CQO
  IDIVQ CX
  MOVQ  DX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.6.1(SB), AX
  JMP   rt.fault(SB)

TEXT bounded_types.triple(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  $3, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  IMULQ CX, AX
  JOS   L2
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)

TEXT bounded_types.main(SB)
  LEAQ  -768(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $752, SP
  MOVQ  $7, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.widen(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $24, AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $2, AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -56(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -56(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $85, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, -96(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.half(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $24, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $2, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -136(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -136(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  $1234, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.last_digit(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $24, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $2, AX
  MOVQ  AX, -216(BP)
  MOVQ  -208(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -208(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -208(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -240(BP)
  MOVQ  $-1234, AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.signed_digit(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $24, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $2, AX
  MOVQ  AX, -288(BP)
  MOVQ  -280(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -280(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -280(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 16(AX)
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
  LEAQ  str.4(SB), AX
  MOVQ  AX, -312(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.triple(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  $24, AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  $2, AX
  MOVQ  AX, -352(BP)
  MOVQ  -344(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -344(BP), AX
  MOVQ  -312(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -344(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -376(BP)
  MOVQ  $100, AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  bounded_types.triple(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
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
  MOVQ  -376(BP), CX
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
  MOVQ  -96(BP), AX
  MOVQ  AX, -448(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -456(BP)
  MOVQ  $100, AX
  MOVQ  AX, -464(BP)
  MOVQ  -448(BP), AX
  MOVQ  -464(BP), CX
  SUBQ  CX, AX
  JOS   L5
  MOVQ  AX, -472(BP)
  MOVQ  -472(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -480(BP)
  MOVQ  $24, AX
  MOVQ  AX, -488(BP)
  MOVQ  -488(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -496(BP)
  MOVQ  $2, AX
  MOVQ  AX, -504(BP)
  MOVQ  -496(BP), AX
  MOVQ  -504(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -496(BP), AX
  MOVQ  -456(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -496(BP), AX
  MOVQ  -480(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  $0, AX
  MOVQ  AX, -528(BP)
  MOVQ  $3, AX
  MOVQ  AX, -536(BP)
  MOVQ  -536(BP), AX
  MOVQ  AX, -544(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, -552(BP)
  JMP   L1
LABEL L1
  MOVQ  -552(BP), AX
  MOVQ  -544(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -560(BP)
  MOVQ  -560(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  -552(BP), AX
  MOVQ  AX, -568(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -576(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -584(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -592(BP)
  MOVQ  $32, AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -608(BP)
  MOVQ  $3, AX
  MOVQ  AX, -616(BP)
  MOVQ  -608(BP), AX
  MOVQ  -616(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -608(BP), AX
  MOVQ  -576(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -608(BP), AX
  MOVQ  -584(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -608(BP), AX
  MOVQ  -592(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  $3, AX
  MOVQ  AX, -624(BP)
  MOVQ  -568(BP), AX
  MOVQ  -624(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -632(BP)
  MOVQ  -608(BP), AX
  MOVQ  -632(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  MOVQ  AX, -656(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -664(BP)
  MOVQ  -568(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -672(BP)
  LEAQ  str.11(SB), AX
  MOVQ  AX, -680(BP)
  MOVQ  $40, AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $4, AX
  MOVQ  AX, -704(BP)
  MOVQ  -696(BP), AX
  MOVQ  -704(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -696(BP), AX
  MOVQ  -664(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -696(BP), AX
  MOVQ  -672(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -696(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -696(BP), AX
  MOVQ  -656(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -696(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -712(BP)
  MOVQ  -712(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -720(BP)
  MOVQ  $1, AX
  MOVQ  AX, -728(BP)
  MOVQ  -552(BP), AX
  MOVQ  -728(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -552(BP)
  JMP   L1
LABEL L3
  MOVQ  $0, AX
  MOVQ  AX, -736(BP)
  MOVQ  -736(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
