TEXT or_return_mapped.parse_quantity(SB)
  LEAQ  -224(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $208, SP
  MOVQ  $1, AX
  MOVQ  AX, -16(BP)
  MOVQ  $99, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_text(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -40(BP)
  MOVQ  $0, AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  -48(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  $16, AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -168(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -168(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -32(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -80(BP)
  MOVQ  $1, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  -88(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -104(BP)
  LEAQ  var.or_return_mapped.OrderError.BadQuantity(SB), AX
  MOVQ  AX, -112(BP)
  MOVQ  $1, AX
  MOVQ  AX, -120(BP)
  MOVQ  $16, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -136(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -136(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L6
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT or_return_mapped.render(SB)
  LEAQ  -240(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L7
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $224, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.parse_quantity(SB)
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
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -112(BP)
  MOVQ  $1, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -56(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  $32, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $3, AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -88(BP), AX
  MOVQ  16(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -88(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -144(BP)
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  -144(BP), AX
  MOVQ  -152(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  TESTQ AX, AX
  JNE   L6
  JMP   L4
LABEL L6
  LEAQ  str.1(SB), AX
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
  MOVQ  16(BP), CX
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
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L7
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT or_return_mapped.main(SB)
  LEAQ  -496(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L11
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $480, SP
  LEAQ  str.2(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.render(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.parse_quantity(SB)
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
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -128(BP), AX
  MOVQ  AX, -232(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -240(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  $24, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $2, AX
  MOVQ  AX, -272(BP)
  MOVQ  -264(BP), AX
  MOVQ  -272(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -264(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -264(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -264(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -304(BP)
  MOVQ  -304(BP), AX
  MOVQ  AX, 0(SP)
  CALL  or_return_mapped.parse_quantity(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -320(BP)
  MOVQ  $0, AX
  MOVQ  AX, -328(BP)
  MOVQ  -320(BP), AX
  MOVQ  -328(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L2
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
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -144(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, -128(BP)
  JMP   L1
LABEL L4
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -144(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -216(BP)
  MOVQ  $0, AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L6
  MOVQ  -296(BP), AX
  MOVQ  AX, -400(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -408(BP)
  MOVQ  -400(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  $24, AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $2, AX
  MOVQ  AX, -440(BP)
  MOVQ  -432(BP), AX
  MOVQ  -440(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -432(BP), AX
  MOVQ  -408(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -432(BP), AX
  MOVQ  -416(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  -448(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -456(BP)
  MOVQ  $0, AX
  MOVQ  AX, -464(BP)
  MOVQ  -464(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  MOVQ  -312(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -360(BP)
  MOVQ  $1, AX
  MOVQ  AX, -368(BP)
  MOVQ  -360(BP), AX
  MOVQ  -368(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -376(BP)
  MOVQ  -376(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L9
LABEL L8
  MOVQ  -312(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, -352(BP)
  MOVQ  -352(BP), AX
  MOVQ  AX, -296(BP)
  JMP   L6
LABEL L9
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  MOVQ  -312(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -384(BP)
  MOVQ  $0, AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L11
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
