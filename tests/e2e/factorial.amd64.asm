TEXT factorial.factorial(SB)
  LEAQ  -96(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $80, SP
  MOVQ  $1, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETLE AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -32(BP)
  MOVQ  16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L5
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  factorial.factorial(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  16(BP), AX
  MOVQ  -48(BP), CX
  IMULQ CX, AX
  JOS   L6
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT factorial.main(SB)
  LEAQ  -240(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $224, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  $6, AX
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
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -64(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  factorial.factorial(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $32, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $3, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -96(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $1, AX
  MOVQ  AX, -128(BP)
  MOVQ  -32(BP), AX
  MOVQ  -128(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -32(BP)
  JMP   L1
LABEL L3
  LEAQ  str.1(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  $20, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  factorial.factorial(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $24, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $2, AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 16(AX)
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
  MOVQ  $0, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
