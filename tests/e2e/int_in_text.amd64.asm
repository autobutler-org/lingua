TEXT int_in_text.bounds_name(SB)
  LEAQ  -96(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $80, SP
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  -24(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  $1, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  str.1(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT int_in_text.probe(SB)
  LEAQ  -272(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $256, SP
  MOVQ  $-10, AX
  MOVQ  AX, -16(BP)
  MOVQ  $10, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_text(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -40(BP)
  MOVQ  $0, AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  -48(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -32(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -136(BP)
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $40, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $4, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, -168(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_in_text.bounds_name(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $40, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $4, AX
  MOVQ  AX, -216(BP)
  MOVQ  -208(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -208(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -208(BP), AX
  MOVQ  16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -208(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -208(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT int_in_text.main(SB)
  LEAQ  -304(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $288, SP
  LEAQ  str.4(SB), AX
  MOVQ  AX, -8(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -24(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -40(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -56(BP)
  LEAQ  str.11(SB), AX
  MOVQ  AX, -64(BP)
  LEAQ  str.12(SB), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.13(SB), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.14(SB), AX
  MOVQ  AX, -88(BP)
  LEAQ  str.15(SB), AX
  MOVQ  AX, -96(BP)
  LEAQ  str.16(SB), AX
  MOVQ  AX, -104(BP)
  LEAQ  str.17(SB), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.18(SB), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.19(SB), AX
  MOVQ  AX, -128(BP)
  LEAQ  str.20(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  $144, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $17, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -152(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -152(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -152(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -152(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -152(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -152(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 48(AX)
  MOVQ  -152(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 56(AX)
  MOVQ  -152(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 64(AX)
  MOVQ  -152(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 72(AX)
  MOVQ  -152(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 80(AX)
  MOVQ  -152(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 88(AX)
  MOVQ  -152(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 96(AX)
  MOVQ  -152(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 104(AX)
  MOVQ  -152(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 112(AX)
  MOVQ  -152(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 120(AX)
  MOVQ  -152(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 128(AX)
  MOVQ  -152(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 136(AX)
  MOVQ  -152(BP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -184(BP)
  MOVQ  $0, AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, -200(BP)
  JMP   L1
LABEL L1
  MOVQ  -200(BP), AX
  MOVQ  -184(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -216(BP)
  MOVQ  -200(BP), AX
  MOVQ  -216(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -224(BP)
  MOVQ  -176(BP), AX
  MOVQ  -224(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_in_text.probe(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $1, AX
  MOVQ  AX, -264(BP)
  MOVQ  -200(BP), AX
  MOVQ  -264(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -200(BP)
  JMP   L1
LABEL L3
  MOVQ  $0, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
