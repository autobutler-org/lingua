TEXT or_return.lookup(SB)
  LEAQ  -336(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L10
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $320, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $24, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  $2, AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -24(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -24(BP), AX
  MOVQ  16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $1, AX
  MOVQ  AX, -72(BP)
  MOVQ  -64(BP), AX
  MOVQ  -72(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $4, AX
  MOVQ  AX, -88(BP)
  MOVQ  $0, AX
  MOVQ  AX, -96(BP)
  MOVQ  $16, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.2(SB), AX
  MOVQ  AX, -120(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  $1, AX
  MOVQ  AX, -136(BP)
  MOVQ  -128(BP), AX
  MOVQ  -136(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L4
  MOVQ  $5, AX
  MOVQ  AX, -152(BP)
  MOVQ  $0, AX
  MOVQ  AX, -160(BP)
  MOVQ  $16, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L5
  JMP   L6
LABEL L6
  LEAQ  str.3(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $1, AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  -200(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L8
LABEL L7
  LEAQ  str.3(SB), AX
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
  MOVQ  $1, AX
  MOVQ  AX, -248(BP)
  MOVQ  $16, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -264(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -264(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L8
  JMP   L9
LABEL L9
  LEAQ  var.or_return.AppError.Missing(SB), AX
  MOVQ  AX, -272(BP)
  MOVQ  $1, AX
  MOVQ  AX, -280(BP)
  MOVQ  $16, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -296(BP), AX
  MOVQ  -272(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -296(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L10
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT or_return.area_of(SB)
  LEAQ  -320(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L11
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $304, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.lookup(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
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
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, -128(BP)
  MOVQ  24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.lookup(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -152(BP)
  MOVQ  $0, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L2
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  $1, AX
  MOVQ  AX, -72(BP)
  MOVQ  -64(BP), AX
  MOVQ  -72(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  $16, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -120(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -120(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L6
  MOVQ  -136(BP), AX
  MOVQ  AX, -256(BP)
  MOVQ  -128(BP), AX
  MOVQ  -256(BP), CX
  IMULQ CX, AX
  JOS   L12
  MOVQ  AX, -264(BP)
  MOVQ  $0, AX
  MOVQ  AX, -272(BP)
  MOVQ  $16, AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  -272(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -288(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -288(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  MOVQ  -144(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -192(BP)
  MOVQ  $1, AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  -200(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L9
LABEL L8
  MOVQ  -144(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, -136(BP)
  JMP   L6
LABEL L9
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  MOVQ  -144(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $1, AX
  MOVQ  AX, -232(BP)
  MOVQ  $16, AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  -232(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -248(BP), AX
  MOVQ  -224(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -248(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L11
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L12
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)

TEXT or_return.explain(SB)
  LEAQ  -160(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $144, SP
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
  LEAQ  str.4(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.5(SB), AX
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
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)

TEXT or_return.render(SB)
  LEAQ  -176(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $160, SP
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
  MOVQ  AX, -104(BP)
  MOVQ  $1, AX
  MOVQ  AX, -112(BP)
  MOVQ  -104(BP), AX
  MOVQ  -112(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
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
  MOVQ  -56(BP), CX
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
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.explain(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)

TEXT or_return.main(SB)
  LEAQ  -176(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $160, SP
  LEAQ  str.1(SB), AX
  MOVQ  AX, -8(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  CALL  or_return.area_of(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 8(SP)
  CALL  or_return.area_of(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -88(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 8(SP)
  CALL  or_return.area_of(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return.render(SB)
  MOVQ  8(SP), AX
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
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
