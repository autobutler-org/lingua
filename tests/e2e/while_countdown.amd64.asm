TEXT while_countdown.main(SB)
  LEAQ  -400(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L10
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $384, SP
  MOVQ  $3, AX
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
  LEAQ  str.0(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $24, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $2, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -80(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L3
  LEAQ  str.1(SB), AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  LEAQ  var.while_countdown.Countdown.Liftoff(SB), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, -232(BP)
  JMP   L7
LABEL L4
  LEAQ  var.while_countdown.Countdown.Liftoff(SB), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L6
LABEL L5
  MOVQ  $1, AX
  MOVQ  AX, -168(BP)
  MOVQ  -80(BP), AX
  MOVQ  -168(BP), CX
  SUBQ  CX, AX
  JOS   L11
  MOVQ  AX, -176(BP)
  MOVQ  $0, AX
  MOVQ  AX, -184(BP)
  MOVQ  $16, AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -200(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -200(BP), AX
  MOVQ  AX, -40(BP)
  JMP   L6
LABEL L6
  JMP   L1
LABEL L7
  MOVQ  -232(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -240(BP)
  MOVQ  $0, AX
  MOVQ  AX, -248(BP)
  MOVQ  -240(BP), AX
  MOVQ  -248(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L9
LABEL L8
  MOVQ  -232(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, -272(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -280(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  MOVQ  $24, AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $2, AX
  MOVQ  AX, -312(BP)
  MOVQ  -304(BP), AX
  MOVQ  -312(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -304(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -304(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -304(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  LEAQ  var.while_countdown.Countdown.Liftoff(SB), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, -232(BP)
  JMP   L7
LABEL L9
  LEAQ  str.3(SB), AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $0, AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L10
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
