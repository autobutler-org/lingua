TEXT tail_recursion.depth(SB)
  LEAQ  -96(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $80, SP
LABEL L0
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  24(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  -24(BP), CX
  SUBQ  CX, AX
  JOS   L5
  MOVQ  AX, -32(BP)
  MOVQ  $1, AX
  MOVQ  AX, -40(BP)
  MOVQ  24(BP), AX
  MOVQ  -40(BP), CX
  ADDQ  CX, AX
  JOS   L6
  MOVQ  AX, -48(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 24(BP)
  JMP   L0
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT tail_recursion.count(SB)
  LEAQ  -112(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $96, SP
LABEL L0
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  24(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  24(BP), AX
  MOVQ  -24(BP), CX
  ADDQ  CX, AX
  JOS   L5
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $1, AX
  MOVQ  AX, -48(BP)
  MOVQ  16(BP), AX
  MOVQ  -48(BP), CX
  SUBQ  CX, AX
  JOS   L6
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 24(BP)
  JMP   L0
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)

TEXT tail_recursion.main(SB)
  LEAQ  -208(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $192, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $100000000, AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 8(SP)
  CALL  tail_recursion.depth(SB)
  MOVQ  16(SP), AX
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
  MOVQ  -8(BP), CX
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
  LEAQ  str.1(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  $100000000, AX
  MOVQ  AX, -96(BP)
  MOVQ  $0, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 8(SP)
  CALL  tail_recursion.count(SB)
  MOVQ  16(SP), AX
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
  MOVQ  -88(BP), CX
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
  MOVQ  $0, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
