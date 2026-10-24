TEXT layout.add(SB)
  LEAQ  -32(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $16, SP
  MOVQ  16(BP), AX
  MOVQ  24(BP), CX
  ADDQ  CX, AX
  JOS   L2
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT layout.main(SB)
  LEAQ  -176(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $160, SP
  MOVQ  $40, AX
  MOVQ  AX, -8(BP)
  MOVQ  $2, AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  CALL  layout.add(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $24, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $2, AX
  MOVQ  AX, -72(BP)
  MOVQ  -64(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -64(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -64(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 16(AX)
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
  LEAQ  str.1(SB), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $0, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
