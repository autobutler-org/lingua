TEXT comptime_const.next_pow2(SB)
  LEAQ  -288(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L15
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $272, SP
  MOVQ  $1, AX
  MOVQ  AX, -8(BP)
  MOVQ  $0, AX
  MOVQ  AX, -16(BP)
  MOVQ  $16, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -32(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L1
LABEL L1
  MOVQ  -40(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  $0, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  -40(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  16(BP), CX
  CMPQ  AX, CX
  SETGE AX
  MOVBQZX AX, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L3
  MOVQ  -40(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  $1, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  -168(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  TESTQ AX, AX
  JNE   L9
  JMP   L8
LABEL L4
  MOVQ  $1, AX
  MOVQ  AX, -96(BP)
  MOVQ  $16, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L6
LABEL L5
  MOVQ  -80(BP), AX
  MOVQ  -80(BP), CX
  ADDQ  CX, AX
  JOS   L16
  MOVQ  AX, -120(BP)
  MOVQ  $0, AX
  MOVQ  AX, -128(BP)
  MOVQ  $16, AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -144(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -144(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L6
LABEL L6
  JMP   L1
LABEL L7
  MOVQ  -152(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L8
  MOVQ  -40(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -224(BP)
  MOVQ  $0, AX
  MOVQ  AX, -232(BP)
  MOVQ  -224(BP), AX
  MOVQ  -232(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  TESTQ AX, AX
  JNE   L14
  JMP   L13
LABEL L9
  MOVQ  -40(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $1, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  -192(BP), CX
  CMPQ  AX, CX
  SETGE AX
  MOVBQZX AX, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L11
LABEL L10
  MOVQ  -200(BP), AX
  MOVQ  AX, -208(BP)
  JMP   L12
LABEL L11
  MOVQ  -192(BP), AX
  MOVQ  AX, -208(BP)
  JMP   L12
LABEL L12
  MOVQ  -208(BP), AX
  MOVQ  AX, -152(BP)
  JMP   L7
LABEL L13
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L14
  MOVQ  -40(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -248(BP)
  MOVQ  $1, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, -152(BP)
  JMP   L7
LABEL L15
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L16
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT comptime_const.squares(SB)
  LEAQ  -160(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $144, SP
  MOVQ  $8, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -16(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $0, AX
  MOVQ  AX, -40(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -56(BP)
  JMP   L1
LABEL L1
  MOVQ  -56(BP), AX
  MOVQ  -48(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  -56(BP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  -72(BP), CX
  IMULQ CX, AX
  JOS   L5
  MOVQ  AX, -80(BP)
  MOVQ  $16, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $1, AX
  MOVQ  AX, -120(BP)
  MOVQ  -56(BP), AX
  MOVQ  -120(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -56(BP)
  JMP   L1
LABEL L3
  MOVQ  -32(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)

TEXT comptime_const.main(SB)
  LEAQ  -880(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L9
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $864, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $128, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  $24, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $2, AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -40(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -40(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $0, AX
  MOVQ  AX, -88(BP)
  MOVQ  $1, AX
  MOVQ  AX, -96(BP)
  MOVQ  $4, AX
  MOVQ  AX, -104(BP)
  MOVQ  $9, AX
  MOVQ  AX, -112(BP)
  MOVQ  $16, AX
  MOVQ  AX, -120(BP)
  MOVQ  $48, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $5, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -136(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -136(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -136(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -136(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -136(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -136(BP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  $0, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, -176(BP)
  JMP   L1
LABEL L1
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -192(BP)
  MOVQ  -176(BP), AX
  MOVQ  -192(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -200(BP)
  MOVQ  -152(BP), AX
  MOVQ  -200(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -216(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -224(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  $24, AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  $2, AX
  MOVQ  AX, -256(BP)
  MOVQ  -248(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -248(BP), AX
  MOVQ  -224(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -248(BP), AX
  MOVQ  -232(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  MOVQ  $1, AX
  MOVQ  AX, -280(BP)
  MOVQ  -176(BP), AX
  MOVQ  -280(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -176(BP)
  JMP   L1
LABEL L3
  LEAQ  str.3(SB), AX
  MOVQ  AX, -288(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -296(BP)
  MOVQ  $4, AX
  MOVQ  AX, -304(BP)
  MOVQ  $16, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  -296(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -320(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -320(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -328(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -336(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -344(BP)
  MOVQ  $4, AX
  MOVQ  AX, -352(BP)
  MOVQ  $16, AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  -344(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -368(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -368(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -376(BP)
  MOVQ  -376(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -384(BP)
  MOVQ  $40, AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  $4, AX
  MOVQ  AX, -408(BP)
  MOVQ  -400(BP), AX
  MOVQ  -408(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -400(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -400(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -400(BP), AX
  MOVQ  -336(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -400(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -400(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -424(BP)
  MOVQ  $42, AX
  MOVQ  AX, -440(BP)
  MOVQ  $0, AX
  MOVQ  AX, -448(BP)
  MOVQ  $16, AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  MOVQ  -464(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -464(BP), AX
  MOVQ  -440(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -464(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -472(BP)
  MOVQ  $0, AX
  MOVQ  AX, -480(BP)
  MOVQ  -472(BP), AX
  MOVQ  -480(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -488(BP)
  MOVQ  -488(BP), AX
  TESTQ AX, AX
  JNE   L6
  JMP   L5
LABEL L4
  LEAQ  str.8(SB), AX
  MOVQ  AX, -616(BP)
  MOVQ  $1024, AX
  MOVQ  AX, -624(BP)
  MOVQ  -624(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -632(BP)
  MOVQ  $24, AX
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -648(BP)
  MOVQ  $2, AX
  MOVQ  AX, -656(BP)
  MOVQ  -648(BP), AX
  MOVQ  -656(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -648(BP), AX
  MOVQ  -616(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -648(BP), AX
  MOVQ  -632(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -648(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -672(BP)
  MOVQ  $8, AX
  MOVQ  AX, -680(BP)
  MOVQ  $1, AX
  MOVQ  AX, -688(BP)
  MOVQ  -680(BP), AX
  MOVQ  -688(BP), CX
  ADDQ  CX, AX
  JOS   L10
  MOVQ  AX, -696(BP)
  MOVQ  -696(BP), AX
  MOVQ  AX, -704(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -712(BP)
  MOVQ  -704(BP), AX
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
  MOVQ  $128, AX
  MOVQ  AX, -768(BP)
  MOVQ  -768(BP), AX
  MOVQ  AX, -776(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -784(BP)
  MOVQ  -776(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -792(BP)
  MOVQ  $24, AX
  MOVQ  AX, -800(BP)
  MOVQ  -800(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -808(BP)
  MOVQ  $2, AX
  MOVQ  AX, -816(BP)
  MOVQ  -808(BP), AX
  MOVQ  -816(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -808(BP), AX
  MOVQ  -784(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -808(BP), AX
  MOVQ  -792(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -808(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -824(BP)
  MOVQ  -824(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -832(BP)
  MOVQ  $0, AX
  MOVQ  AX, -840(BP)
  MOVQ  -840(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L5
  MOVQ  -464(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -568(BP)
  MOVQ  $1, AX
  MOVQ  AX, -576(BP)
  MOVQ  -568(BP), AX
  MOVQ  -576(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L6
  MOVQ  -464(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, -504(BP)
  LEAQ  str.6(SB), AX
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
  MOVQ  -560(BP), AX
  MOVQ  AX, -432(BP)
  JMP   L4
LABEL L7
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  MOVQ  -464(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -592(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  MOVQ  AX, -432(BP)
  JMP   L4
LABEL L9
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
