TEXT main_param_name.main(SB)
  LEAQ  -240(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $224, SP
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -24(BP)
  MOVQ  $0, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L1
LABEL L1
  MOVQ  -40(BP), AX
  MOVQ  -24(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -56(BP)
  MOVQ  -40(BP), AX
  MOVQ  -56(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -16(BP), AX
  MOVQ  -64(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  $24, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $2, AX
  MOVQ  AX, -112(BP)
  MOVQ  -104(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -104(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -104(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  $1, AX
  MOVQ  AX, -136(BP)
  MOVQ  -40(BP), AX
  MOVQ  -136(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -40(BP)
  JMP   L1
LABEL L3
  MOVQ  16(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -144(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -152(BP)
  MOVQ  16(BP), AX
  MOVQ  24(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  $32, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $3, AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 24(AX)
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
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
