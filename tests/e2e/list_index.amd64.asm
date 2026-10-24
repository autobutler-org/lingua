TEXT list_index.pick(SB)
  LEAQ  -256(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $240, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  24(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.list_index(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -24(BP)
  MOVQ  $0, AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -128(BP)
  MOVQ  $1, AX
  MOVQ  AX, -136(BP)
  MOVQ  -128(BP), AX
  MOVQ  -136(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -56(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -64(BP)
  MOVQ  24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -80(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $40, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $4, AX
  MOVQ  AX, -112(BP)
  MOVQ  -104(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -104(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -104(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -104(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -104(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -152(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -160(BP)
  MOVQ  24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -176(BP)
  MOVQ  $32, AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $3, AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  -200(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -192(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -192(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -192(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT list_index.main(SB)
  LEAQ  -816(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L11
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $800, SP
  MOVQ  $2, AX
  MOVQ  AX, -8(BP)
  MOVQ  $3, AX
  MOVQ  AX, -16(BP)
  MOVQ  $5, AX
  MOVQ  AX, -24(BP)
  MOVQ  $7, AX
  MOVQ  AX, -32(BP)
  MOVQ  $40, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $4, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -48(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -48(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -48(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -48(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -48(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $0, AX
  MOVQ  AX, -72(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $3, AX
  MOVQ  AX, -96(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $4, AX
  MOVQ  AX, -120(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $-1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $8, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $0, AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  $0, AX
  MOVQ  AX, -192(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 8(SP)
  CALL  list_index.pick(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  $10, AX
  MOVQ  AX, -248(BP)
  MOVQ  $20, AX
  MOVQ  AX, -256(BP)
  MOVQ  $30, AX
  MOVQ  AX, -264(BP)
  MOVQ  $32, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $3, AX
  MOVQ  AX, -288(BP)
  MOVQ  -280(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -280(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -280(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -280(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  $2, AX
  MOVQ  AX, -296(BP)
  MOVQ  $3, AX
  MOVQ  AX, -304(BP)
  MOVQ  -296(BP), AX
  MOVQ  -304(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -312(BP)
  MOVQ  -280(BP), AX
  MOVQ  -312(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  AX, -336(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -344(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
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
  MOVQ  -344(BP), CX
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
  MOVQ  $1, AX
  MOVQ  AX, -408(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.list_index(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -424(BP)
  MOVQ  $0, AX
  MOVQ  AX, -432(BP)
  MOVQ  -424(BP), AX
  MOVQ  -432(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -440(BP)
  MOVQ  -440(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -400(BP), AX
  MOVQ  AX, -496(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -504(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  $24, AX
  MOVQ  AX, -520(BP)
  MOVQ  -520(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -528(BP)
  MOVQ  $2, AX
  MOVQ  AX, -536(BP)
  MOVQ  -528(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -528(BP), AX
  MOVQ  -504(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -528(BP), AX
  MOVQ  -512(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -528(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -560(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -568(BP)
  MOVQ  $24, AX
  MOVQ  AX, -576(BP)
  MOVQ  -576(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  $2, AX
  MOVQ  AX, -592(BP)
  MOVQ  -584(BP), AX
  MOVQ  -592(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -584(BP), AX
  MOVQ  -560(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -584(BP), AX
  MOVQ  -568(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -584(BP), AX
  MOVQ  AX, -600(BP)
  MOVQ  $5, AX
  MOVQ  AX, -616(BP)
  MOVQ  -600(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -616(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.list_index(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -624(BP)
  MOVQ  -624(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -632(BP)
  MOVQ  $0, AX
  MOVQ  AX, -640(BP)
  MOVQ  -632(BP), AX
  MOVQ  -640(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L2
  MOVQ  -416(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -464(BP)
  MOVQ  $1, AX
  MOVQ  AX, -472(BP)
  MOVQ  -464(BP), AX
  MOVQ  -472(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -416(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -448(BP)
  MOVQ  -448(BP), AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, -400(BP)
  JMP   L1
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -416(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -488(BP)
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  MOVQ  -608(BP), AX
  MOVQ  AX, -712(BP)
  LEAQ  str.8(SB), AX
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
  MOVQ  -720(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -736(BP), AX
  MOVQ  -712(BP), CX
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
LABEL L7
  MOVQ  -624(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -672(BP)
  MOVQ  $1, AX
  MOVQ  AX, -680(BP)
  MOVQ  -672(BP), AX
  MOVQ  -680(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L9
LABEL L8
  MOVQ  -624(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -656(BP)
  MOVQ  -656(BP), AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  AX, -608(BP)
  JMP   L6
LABEL L9
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  MOVQ  -624(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -696(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  AX, -608(BP)
  JMP   L6
LABEL L11
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
