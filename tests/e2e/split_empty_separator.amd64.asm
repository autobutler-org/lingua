TEXT split_empty_separator.main(SB)
  LEAQ  -272(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $256, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -24(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -40(BP)
  LEAQ  site.1(SB), AX
  MOVQ  AX, -48(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.split(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
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
  MOVQ  -24(BP), CX
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
  LEAQ  site.2(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.split(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, -144(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -152(BP)
  MOVQ  -144(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  $24, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  $2, AX
  MOVQ  AX, -192(BP)
  MOVQ  -184(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -184(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -184(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $0, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
