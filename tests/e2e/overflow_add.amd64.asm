TEXT overflow_add.main(SB)
  LEAQ  -224(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $208, SP
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L2
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L3
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
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
  MOVQ  -40(BP), CX
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
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  ADDQ  CX, AX
  JOS   L4
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
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
  MOVQ  -128(BP), CX
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
  MOVQ  $0, AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
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
