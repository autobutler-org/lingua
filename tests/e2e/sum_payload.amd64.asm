TEXT sum_payload.area(SB)
  LEAQ  -176(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L8
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $160, SP
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -8(BP)
  MOVQ  $0, AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  -16(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -56(BP)
  MOVQ  $1, AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  -64(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  -40(BP), CX
  IMULQ CX, AX
  JOS   L9
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -120(BP)
  MOVQ  $2, AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  -128(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L6
LABEL L5
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  IMULQ CX, AX
  JOS   L10
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L6
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  MOVQ  $0, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L8
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L9
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT sum_payload.main(SB)
  LEAQ  -320(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $304, SP
  MOVQ  $4, AX
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
  MOVQ  $3, AX
  MOVQ  AX, -40(BP)
  MOVQ  $7, AX
  MOVQ  AX, -48(BP)
  MOVQ  $16, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -64(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $1, AX
  MOVQ  AX, -72(BP)
  MOVQ  $16, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -88(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 8(AX)
  LEAQ  var.sum_payload.Shape.Dot(SB), AX
  MOVQ  AX, -96(BP)
  MOVQ  $32, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $3, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -144(BP)
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, -160(BP)
  JMP   L1
LABEL L1
  MOVQ  -160(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -176(BP)
  MOVQ  -160(BP), AX
  MOVQ  -176(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -184(BP)
  MOVQ  -136(BP), AX
  MOVQ  -184(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -200(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -208(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  sum_payload.area(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $24, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  $2, AX
  MOVQ  AX, -248(BP)
  MOVQ  -240(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -240(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -240(BP), AX
  MOVQ  -224(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $1, AX
  MOVQ  AX, -272(BP)
  MOVQ  -160(BP), AX
  MOVQ  -272(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -160(BP)
  JMP   L1
LABEL L3
  MOVQ  $0, AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
