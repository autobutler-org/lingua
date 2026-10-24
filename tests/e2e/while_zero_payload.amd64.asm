TEXT while_zero_payload.main(SB)
  LEAQ  -304(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L10
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $288, SP
  LEAQ  var.while_zero_payload.Phase.Warmup(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  JMP   L1
LABEL L1
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
  JNE   L2
  JMP   L3
LABEL L2
  LEAQ  str.0(SB), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $2, AX
  MOVQ  AX, -64(BP)
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
  MOVQ  -88(BP), AX
  MOVQ  AX, -16(BP)
  JMP   L1
LABEL L3
  JMP   L4
LABEL L4
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -96(BP)
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L6
LABEL L5
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, -128(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $24, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $2, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -160(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -160(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  $1, AX
  MOVQ  AX, -192(BP)
  MOVQ  -128(BP), AX
  MOVQ  -192(BP), CX
  CMPQ  AX, CX
  SETGT AX
  MOVBQZX AX, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L8
LABEL L6
  LEAQ  str.2(SB), AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $0, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  MOVQ  $1, AX
  MOVQ  AX, -208(BP)
  MOVQ  -128(BP), AX
  MOVQ  -208(BP), CX
  SUBQ  CX, AX
  JOS   L11
  MOVQ  AX, -216(BP)
  MOVQ  $1, AX
  MOVQ  AX, -224(BP)
  MOVQ  $16, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  -224(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -240(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -240(BP), AX
  MOVQ  AX, -16(BP)
  JMP   L9
LABEL L8
  LEAQ  var.while_zero_payload.Phase.Cooldown(SB), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -16(BP)
  JMP   L9
LABEL L9
  JMP   L4
LABEL L10
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
