TEXT int_at_least.bounds_name(SB)
  LEAQ  -96(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $80, SP
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
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  str.1(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT int_at_least.at_least_zero(SB)
  LEAQ  -272(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $256, SP
  MOVQ  $0, AX
  MOVQ  AX, -16(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_int(SB)
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
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -32(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -136(BP)
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, -72(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
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
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, -168(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.bounds_name(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $32, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $3, AX
  MOVQ  AX, -216(BP)
  MOVQ  -208(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -208(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -208(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -208(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT int_at_least.to_cents(SB)
  LEAQ  -224(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $208, SP
  MOVQ  $0, AX
  MOVQ  AX, -16(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_int(SB)
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
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -32(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
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
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)

TEXT int_at_least.main(SB)
  LEAQ  -960(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L24
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $944, SP
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $-9223372036854775807, AX
  MOVQ  AX, -24(BP)
  MOVQ  $1, AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L25
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $5, AX
  MOVQ  AX, -56(BP)
  MOVQ  $0, AX
  MOVQ  AX, -64(BP)
  MOVQ  $-1, AX
  MOVQ  AX, -72(BP)
  MOVQ  $48, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $5, AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -88(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -88(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -88(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -88(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -88(BP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -120(BP)
  MOVQ  $0, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -136(BP)
  JMP   L1
LABEL L1
  MOVQ  -136(BP), AX
  MOVQ  -120(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -152(BP)
  MOVQ  -136(BP), AX
  MOVQ  -152(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -160(BP)
  MOVQ  -112(BP), AX
  MOVQ  -160(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.at_least_zero(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $1, AX
  MOVQ  AX, -200(BP)
  MOVQ  -136(BP), AX
  MOVQ  -200(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -136(BP)
  JMP   L1
LABEL L3
  MOVQ  $100, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, -216(BP)
  MOVQ  $99, AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, -232(BP)
  MOVQ  $100, AX
  MOVQ  AX, -248(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -256(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_int(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -272(BP)
  MOVQ  $0, AX
  MOVQ  AX, -280(BP)
  MOVQ  -272(BP), AX
  MOVQ  -280(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  TESTQ AX, AX
  JNE   L6
  JMP   L5
LABEL L4
  MOVQ  -240(BP), AX
  MOVQ  AX, -344(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -352(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -360(BP)
  MOVQ  $24, AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  MOVQ  $2, AX
  MOVQ  AX, -384(BP)
  MOVQ  -376(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -376(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -376(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -376(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  $100, AX
  MOVQ  AX, -416(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -424(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_int(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -440(BP)
  MOVQ  $0, AX
  MOVQ  AX, -448(BP)
  MOVQ  -440(BP), AX
  MOVQ  -448(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  TESTQ AX, AX
  JNE   L11
  JMP   L10
LABEL L5
  MOVQ  -264(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -312(BP)
  MOVQ  $1, AX
  MOVQ  AX, -320(BP)
  MOVQ  -312(BP), AX
  MOVQ  -320(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L6
  MOVQ  -264(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, -304(BP)
  MOVQ  -304(BP), AX
  MOVQ  AX, -240(BP)
  JMP   L4
LABEL L7
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  MOVQ  -264(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -336(BP)
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L9
  MOVQ  -408(BP), AX
  MOVQ  AX, -528(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -536(BP)
  MOVQ  $24, AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  $2, AX
  MOVQ  AX, -560(BP)
  MOVQ  -552(BP), AX
  MOVQ  -560(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -552(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -552(BP), AX
  MOVQ  -528(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -552(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -568(BP)
  MOVQ  -568(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -576(BP)
  MOVQ  $250, AX
  MOVQ  AX, -592(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.to_cents(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -608(BP)
  MOVQ  $0, AX
  MOVQ  AX, -616(BP)
  MOVQ  -608(BP), AX
  MOVQ  -616(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -624(BP)
  MOVQ  -624(BP), AX
  TESTQ AX, AX
  JNE   L16
  JMP   L15
LABEL L10
  MOVQ  -432(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -480(BP)
  MOVQ  $1, AX
  MOVQ  AX, -488(BP)
  MOVQ  -480(BP), AX
  MOVQ  -488(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  TESTQ AX, AX
  JNE   L13
  JMP   L12
LABEL L11
  MOVQ  -432(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -464(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -472(BP)
  MOVQ  -472(BP), AX
  MOVQ  AX, -408(BP)
  JMP   L9
LABEL L12
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
LABEL L13
  MOVQ  -432(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, -512(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.bounds_name(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  -520(BP), AX
  MOVQ  AX, -408(BP)
  JMP   L9
LABEL L14
  MOVQ  -584(BP), AX
  MOVQ  AX, -680(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -688(BP)
  MOVQ  -680(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $24, AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -712(BP)
  MOVQ  $2, AX
  MOVQ  AX, -720(BP)
  MOVQ  -712(BP), AX
  MOVQ  -720(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -712(BP), AX
  MOVQ  -688(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -712(BP), AX
  MOVQ  -696(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -712(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -728(BP)
  MOVQ  -728(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -736(BP)
  MOVQ  $-250, AX
  MOVQ  AX, -752(BP)
  MOVQ  -752(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.to_cents(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -768(BP)
  MOVQ  $0, AX
  MOVQ  AX, -776(BP)
  MOVQ  -768(BP), AX
  MOVQ  -776(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -784(BP)
  MOVQ  -784(BP), AX
  TESTQ AX, AX
  JNE   L21
  JMP   L20
LABEL L15
  MOVQ  -600(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -648(BP)
  MOVQ  $1, AX
  MOVQ  AX, -656(BP)
  MOVQ  -648(BP), AX
  MOVQ  -656(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  TESTQ AX, AX
  JNE   L18
  JMP   L17
LABEL L16
  MOVQ  -600(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -632(BP)
  MOVQ  -632(BP), AX
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  AX, -584(BP)
  JMP   L14
LABEL L17
  LEAQ  site.12(SB), AX
  JMP   rt.fault(SB)
LABEL L18
  MOVQ  -600(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -672(BP)
  LEAQ  site.11(SB), AX
  JMP   rt.fault(SB)
LABEL L19
  MOVQ  -744(BP), AX
  MOVQ  AX, -856(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -864(BP)
  MOVQ  $24, AX
  MOVQ  AX, -872(BP)
  MOVQ  -872(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -880(BP)
  MOVQ  $2, AX
  MOVQ  AX, -888(BP)
  MOVQ  -880(BP), AX
  MOVQ  -888(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -880(BP), AX
  MOVQ  -864(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -880(BP), AX
  MOVQ  -856(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -880(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -896(BP)
  MOVQ  -896(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -904(BP)
  MOVQ  $0, AX
  MOVQ  AX, -912(BP)
  MOVQ  -912(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L20
  MOVQ  -760(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -808(BP)
  MOVQ  $1, AX
  MOVQ  AX, -816(BP)
  MOVQ  -808(BP), AX
  MOVQ  -816(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -824(BP)
  MOVQ  -824(BP), AX
  TESTQ AX, AX
  JNE   L23
  JMP   L22
LABEL L21
  MOVQ  -760(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -792(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -800(BP)
  MOVQ  -800(BP), AX
  MOVQ  AX, -744(BP)
  JMP   L19
LABEL L22
  LEAQ  site.13(SB), AX
  JMP   rt.fault(SB)
LABEL L23
  MOVQ  -760(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -832(BP)
  MOVQ  -832(BP), AX
  MOVQ  AX, -840(BP)
  MOVQ  -840(BP), AX
  MOVQ  AX, 0(SP)
  CALL  int_at_least.bounds_name(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -848(BP)
  MOVQ  -848(BP), AX
  MOVQ  AX, -744(BP)
  JMP   L19
LABEL L24
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L25
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
