TEXT money.percent(SB)
  LEAQ  -64(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $48, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  24(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  32(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 40(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT order.line_subtotal(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  -16(BP), CX
  IMULQ CX, AX
  JOS   L2
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT order.order_total(SB)
  LEAQ  -336(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L14
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $320, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -32(BP)
  MOVQ  $0, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  JMP   L1
LABEL L1
  MOVQ  -48(BP), AX
  MOVQ  -32(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -64(BP)
  MOVQ  -48(BP), AX
  MOVQ  -64(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -72(BP)
  MOVQ  -24(BP), AX
  MOVQ  -72(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  order.line_subtotal(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -16(BP), AX
  MOVQ  -96(BP), CX
  ADDQ  CX, AX
  JOS   L15
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $1, AX
  MOVQ  AX, -112(BP)
  MOVQ  -48(BP), AX
  MOVQ  -112(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -48(BP)
  JMP   L1
LABEL L3
  MOVQ  24(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -128(BP)
  MOVQ  $0, AX
  MOVQ  AX, -136(BP)
  MOVQ  -128(BP), AX
  MOVQ  -136(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  TESTQ AX, AX
  JNE   L6
  JMP   L5
LABEL L4
  MOVQ  -120(BP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $0, AX
  MOVQ  AX, -264(BP)
  MOVQ  -16(BP), AX
  MOVQ  -256(BP), CX
  SUBQ  CX, AX
  JOS   L16
  MOVQ  AX, -272(BP)
  MOVQ  -264(BP), AX
  MOVQ  -272(BP), CX
  CMPQ  AX, CX
  SETGE AX
  MOVBQZX AX, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  TESTQ AX, AX
  JNE   L11
  JMP   L12
LABEL L5
  MOVQ  24(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  $1, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  -168(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L6
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, -120(BP)
  JMP   L4
LABEL L7
  MOVQ  24(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -216(BP)
  MOVQ  $2, AX
  MOVQ  AX, -224(BP)
  MOVQ  -216(BP), AX
  MOVQ  -224(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L9
LABEL L8
  MOVQ  24(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, -192(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, -120(BP)
  JMP   L4
LABEL L9
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  MOVQ  24(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -120(BP)
  JMP   L4
LABEL L11
  MOVQ  -264(BP), AX
  MOVQ  AX, -280(BP)
  JMP   L13
LABEL L12
  MOVQ  -272(BP), AX
  MOVQ  AX, -280(BP)
  JMP   L13
LABEL L13
  MOVQ  -280(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L14
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L15
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L16
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)

TEXT order.parse_discount(SB)
  LEAQ  -816(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L25
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $800, SP
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, -24(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $1, AX
  MOVQ  AX, -48(BP)
  MOVQ  -40(BP), AX
  MOVQ  -48(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  var.order.ParseError.Empty(SB), AX
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
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.1(SB), AX
  MOVQ  AX, -96(BP)
  LEAQ  site.8(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.split(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -120(BP)
  MOVQ  $1, AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  -128(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  TESTQ AX, AX
  JNE   L6
  JMP   L5
LABEL L4
  MOVQ  $0, AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L5
  MOVQ  -112(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -200(BP)
  MOVQ  $2, AX
  MOVQ  AX, -208(BP)
  MOVQ  -200(BP), AX
  MOVQ  -208(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  TESTQ AX, AX
  JNE   L9
  JMP   L8
LABEL L6
  MOVQ  -112(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -144(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -152(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L5
LABEL L7
  LEAQ  var.order.Discount.NoDiscount(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  $0, AX
  MOVQ  AX, -176(BP)
  MOVQ  $16, AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -192(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -192(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L8
  MOVQ  -112(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -456(BP)
  MOVQ  $2, AX
  MOVQ  AX, -464(BP)
  MOVQ  -456(BP), AX
  MOVQ  -464(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -472(BP)
  MOVQ  -472(BP), AX
  TESTQ AX, AX
  JNE   L17
  JMP   L16
LABEL L9
  MOVQ  -112(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -224(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -232(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L8
LABEL L10
  MOVQ  -112(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $1, AX
  MOVQ  AX, -272(BP)
  MOVQ  $100, AX
  MOVQ  AX, -280(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_text(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -296(BP)
  MOVQ  $0, AX
  MOVQ  AX, -304(BP)
  MOVQ  -296(BP), AX
  MOVQ  -304(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  TESTQ AX, AX
  JNE   L13
  JMP   L12
LABEL L11
  MOVQ  -264(BP), AX
  MOVQ  AX, -400(BP)
  MOVQ  $1, AX
  MOVQ  AX, -408(BP)
  MOVQ  $16, AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  -408(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -424(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $0, AX
  MOVQ  AX, -432(BP)
  MOVQ  $16, AX
  MOVQ  AX, -440(BP)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  -448(BP), AX
  MOVQ  -432(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -448(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -448(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L12
  MOVQ  -288(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -336(BP)
  MOVQ  $1, AX
  MOVQ  AX, -344(BP)
  MOVQ  -336(BP), AX
  MOVQ  -344(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -352(BP)
  MOVQ  -352(BP), AX
  TESTQ AX, AX
  JNE   L15
  JMP   L14
LABEL L13
  MOVQ  -288(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  AX, -264(BP)
  JMP   L11
LABEL L14
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L15
  MOVQ  -288(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -360(BP)
  LEAQ  var.order.ParseError.BadQuantity(SB), AX
  MOVQ  AX, -368(BP)
  MOVQ  $1, AX
  MOVQ  AX, -376(BP)
  MOVQ  $16, AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -392(BP), AX
  MOVQ  -368(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -392(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L16
  MOVQ  $2, AX
  MOVQ  AX, -712(BP)
  MOVQ  $16, AX
  MOVQ  AX, -720(BP)
  MOVQ  -720(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -728(BP)
  MOVQ  -728(BP), AX
  MOVQ  -712(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -728(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $1, AX
  MOVQ  AX, -736(BP)
  MOVQ  $16, AX
  MOVQ  AX, -744(BP)
  MOVQ  -744(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -752(BP)
  MOVQ  -752(BP), AX
  MOVQ  -736(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -752(BP), AX
  MOVQ  -728(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -752(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L17
  MOVQ  -112(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -480(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -488(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -488(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  TESTQ AX, AX
  JNE   L18
  JMP   L16
LABEL L18
  MOVQ  -112(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, -512(BP)
  MOVQ  $0, AX
  MOVQ  AX, -528(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -536(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -536(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_text(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -552(BP)
  MOVQ  $0, AX
  MOVQ  AX, -560(BP)
  MOVQ  -552(BP), AX
  MOVQ  -560(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -568(BP)
  MOVQ  -568(BP), AX
  TESTQ AX, AX
  JNE   L21
  JMP   L20
LABEL L19
  MOVQ  -520(BP), AX
  MOVQ  AX, -656(BP)
  MOVQ  $2, AX
  MOVQ  AX, -664(BP)
  MOVQ  $16, AX
  MOVQ  AX, -672(BP)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -680(BP)
  MOVQ  -680(BP), AX
  MOVQ  -664(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -680(BP), AX
  MOVQ  -656(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $0, AX
  MOVQ  AX, -688(BP)
  MOVQ  $16, AX
  MOVQ  AX, -696(BP)
  MOVQ  -696(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  -688(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -704(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -704(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L20
  MOVQ  -544(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -592(BP)
  MOVQ  $1, AX
  MOVQ  AX, -600(BP)
  MOVQ  -592(BP), AX
  MOVQ  -600(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  TESTQ AX, AX
  JNE   L23
  JMP   L22
LABEL L21
  MOVQ  -544(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -576(BP)
  MOVQ  -576(BP), AX
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, -520(BP)
  JMP   L19
LABEL L22
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
LABEL L23
  MOVQ  -544(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -616(BP)
  LEAQ  var.order.ParseError.BadQuantity(SB), AX
  MOVQ  AX, -624(BP)
  MOVQ  $1, AX
  MOVQ  AX, -632(BP)
  MOVQ  $16, AX
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  MOVQ  -632(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -648(BP), AX
  MOVQ  -624(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -648(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L24
  LEAQ  site.11(SB), AX
  JMP   rt.fault(SB)
LABEL L25
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)

TEXT main.apply_code(SB)
  LEAQ  -240(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L6
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $224, SP
  MOVQ  24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  order.parse_discount(SB)
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
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  -16(BP), AX
  MOVQ  0(AX), AX
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
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -56(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -64(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $40, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $4, AX
  MOVQ  AX, -112(BP)
  MOVQ  -104(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -104(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -104(BP), AX
  MOVQ  24(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -104(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -104(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.13(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -152(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -160(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  $32, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  $3, AX
  MOVQ  AX, -192(BP)
  MOVQ  -184(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -184(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -184(BP), AX
  MOVQ  24(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -184(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.12(SB), AX
  JMP   rt.fault(SB)

TEXT main.main(SB)
  LEAQ  -1408(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $1392, SP
  LEAQ  str.8(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $200, AX
  MOVQ  AX, -16(BP)
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  $24, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -40(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -40(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 16(AX)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -48(BP)
  MOVQ  $500, AX
  MOVQ  AX, -56(BP)
  MOVQ  $2, AX
  MOVQ  AX, -64(BP)
  MOVQ  $24, AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -80(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -80(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  $24, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $2, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -96(BP), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -120(BP)
  LEAQ  var.order.Discount.NoDiscount(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
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
  MOVQ  -120(BP), CX
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
  LEAQ  str.11(SB), AX
  MOVQ  AX, -192(BP)
  MOVQ  $10, AX
  MOVQ  AX, -200(BP)
  MOVQ  $1, AX
  MOVQ  AX, -208(BP)
  MOVQ  $16, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -224(BP), AX
  MOVQ  -200(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  $24, AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $2, AX
  MOVQ  AX, -264(BP)
  MOVQ  -256(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -256(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -256(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  LEAQ  str.12(SB), AX
  MOVQ  AX, -288(BP)
  MOVQ  $150, AX
  MOVQ  AX, -296(BP)
  MOVQ  $2, AX
  MOVQ  AX, -304(BP)
  MOVQ  $16, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -320(BP), AX
  MOVQ  -296(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  $24, AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $2, AX
  MOVQ  AX, -360(BP)
  MOVQ  -352(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -352(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -352(BP), AX
  MOVQ  -336(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -352(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  LEAQ  str.13(SB), AX
  MOVQ  AX, -384(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -392(BP)
  MOVQ  $200, AX
  MOVQ  AX, -400(BP)
  MOVQ  $1, AX
  MOVQ  AX, -408(BP)
  MOVQ  $24, AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  -392(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -424(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -424(BP), AX
  MOVQ  -408(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  $16, AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -440(BP)
  MOVQ  $1, AX
  MOVQ  AX, -448(BP)
  MOVQ  -440(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -440(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $500, AX
  MOVQ  AX, -456(BP)
  MOVQ  $2, AX
  MOVQ  AX, -464(BP)
  MOVQ  $16, AX
  MOVQ  AX, -472(BP)
  MOVQ  -472(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  -464(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -480(BP), AX
  MOVQ  -456(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -488(BP)
  MOVQ  -488(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -496(BP)
  MOVQ  $24, AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  $2, AX
  MOVQ  AX, -520(BP)
  MOVQ  -512(BP), AX
  MOVQ  -520(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -512(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -512(BP), AX
  MOVQ  -496(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -528(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  LEAQ  str.14(SB), AX
  MOVQ  AX, -544(BP)
  MOVQ  $8, AX
  MOVQ  AX, -552(BP)
  MOVQ  -552(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -560(BP)
  MOVQ  $0, AX
  MOVQ  AX, -568(BP)
  MOVQ  -560(BP), AX
  MOVQ  -568(BP), CX
  MOVQ  CX, 0(AX)
  LEAQ  var.order.Discount.NoDiscount(SB), AX
  MOVQ  AX, -576(BP)
  MOVQ  -560(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -576(BP), AX
  MOVQ  AX, 8(SP)
  CALL  order.order_total(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -592(BP)
  MOVQ  $24, AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -608(BP)
  MOVQ  $2, AX
  MOVQ  AX, -616(BP)
  MOVQ  -608(BP), AX
  MOVQ  -616(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -608(BP), AX
  MOVQ  -544(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -608(BP), AX
  MOVQ  -592(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -608(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -624(BP)
  MOVQ  -624(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -632(BP)
  LEAQ  str.15(SB), AX
  MOVQ  AX, -640(BP)
  MOVQ  $10, AX
  MOVQ  AX, -648(BP)
  MOVQ  $1200, AX
  MOVQ  AX, -656(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -664(BP)
  MOVQ  -648(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -656(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -664(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -672(BP)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -680(BP)
  MOVQ  $24, AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $2, AX
  MOVQ  AX, -704(BP)
  MOVQ  -696(BP), AX
  MOVQ  -704(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -696(BP), AX
  MOVQ  -640(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -696(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -696(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -712(BP)
  MOVQ  -712(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -720(BP)
  LEAQ  str.16(SB), AX
  MOVQ  AX, -728(BP)
  MOVQ  $1, AX
  MOVQ  AX, -736(BP)
  MOVQ  $150, AX
  MOVQ  AX, -744(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -752(BP)
  MOVQ  -736(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -744(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -752(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -768(BP)
  MOVQ  $24, AX
  MOVQ  AX, -776(BP)
  MOVQ  -776(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -784(BP)
  MOVQ  $2, AX
  MOVQ  AX, -792(BP)
  MOVQ  -784(BP), AX
  MOVQ  -792(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -784(BP), AX
  MOVQ  -728(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -784(BP), AX
  MOVQ  -768(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -784(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -800(BP)
  MOVQ  -800(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -808(BP)
  LEAQ  str.17(SB), AX
  MOVQ  AX, -816(BP)
  MOVQ  $49, AX
  MOVQ  AX, -824(BP)
  MOVQ  $1, AX
  MOVQ  AX, -832(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -840(BP)
  MOVQ  -824(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -832(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -840(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -848(BP)
  MOVQ  -848(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -856(BP)
  MOVQ  $24, AX
  MOVQ  AX, -864(BP)
  MOVQ  -864(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -872(BP)
  MOVQ  $2, AX
  MOVQ  AX, -880(BP)
  MOVQ  -872(BP), AX
  MOVQ  -880(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -872(BP), AX
  MOVQ  -816(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -872(BP), AX
  MOVQ  -856(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -872(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -888(BP)
  MOVQ  -888(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -896(BP)
  LEAQ  str.18(SB), AX
  MOVQ  AX, -904(BP)
  MOVQ  $50, AX
  MOVQ  AX, -912(BP)
  MOVQ  $1, AX
  MOVQ  AX, -920(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -928(BP)
  MOVQ  -912(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -920(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -928(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -936(BP)
  MOVQ  -936(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -944(BP)
  MOVQ  $24, AX
  MOVQ  AX, -952(BP)
  MOVQ  -952(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -960(BP)
  MOVQ  $2, AX
  MOVQ  AX, -968(BP)
  MOVQ  -960(BP), AX
  MOVQ  -968(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -960(BP), AX
  MOVQ  -904(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -960(BP), AX
  MOVQ  -944(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -960(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -976(BP)
  MOVQ  -976(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -984(BP)
  LEAQ  str.19(SB), AX
  MOVQ  AX, -992(BP)
  MOVQ  $33, AX
  MOVQ  AX, -1000(BP)
  MOVQ  $1000, AX
  MOVQ  AX, -1008(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -1016(BP)
  MOVQ  -1000(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -1008(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -1016(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -1024(BP)
  MOVQ  -1024(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1032(BP)
  MOVQ  $24, AX
  MOVQ  AX, -1040(BP)
  MOVQ  -1040(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1048(BP)
  MOVQ  $2, AX
  MOVQ  AX, -1056(BP)
  MOVQ  -1048(BP), AX
  MOVQ  -1056(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1048(BP), AX
  MOVQ  -992(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1048(BP), AX
  MOVQ  -1032(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -1048(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1064(BP)
  MOVQ  -1064(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1072(BP)
  LEAQ  str.20(SB), AX
  MOVQ  AX, -1080(BP)
  MOVQ  $100, AX
  MOVQ  AX, -1088(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -1096(BP)
  LEAQ  var.money.Rounding.half_up(SB), AX
  MOVQ  AX, -1104(BP)
  MOVQ  -1088(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -1096(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -1104(BP), AX
  MOVQ  AX, 16(SP)
  CALL  money.percent(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -1112(BP)
  MOVQ  -1112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1120(BP)
  MOVQ  $24, AX
  MOVQ  AX, -1128(BP)
  MOVQ  -1128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1136(BP)
  MOVQ  $2, AX
  MOVQ  AX, -1144(BP)
  MOVQ  -1136(BP), AX
  MOVQ  -1144(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1136(BP), AX
  MOVQ  -1080(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1136(BP), AX
  MOVQ  -1120(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -1136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1152(BP)
  MOVQ  -1152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1160(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -1168(BP)
  LEAQ  str.21(SB), AX
  MOVQ  AX, -1176(BP)
  LEAQ  str.22(SB), AX
  MOVQ  AX, -1184(BP)
  LEAQ  str.23(SB), AX
  MOVQ  AX, -1192(BP)
  LEAQ  str.24(SB), AX
  MOVQ  AX, -1200(BP)
  LEAQ  str.25(SB), AX
  MOVQ  AX, -1208(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -1216(BP)
  MOVQ  $64, AX
  MOVQ  AX, -1224(BP)
  MOVQ  -1224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1232(BP)
  MOVQ  $7, AX
  MOVQ  AX, -1240(BP)
  MOVQ  -1232(BP), AX
  MOVQ  -1240(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1168(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1176(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1184(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1192(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1200(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1208(BP), CX
  MOVQ  CX, 48(AX)
  MOVQ  -1232(BP), AX
  MOVQ  -1216(BP), CX
  MOVQ  CX, 56(AX)
  MOVQ  -1232(BP), AX
  MOVQ  AX, -1248(BP)
  MOVQ  -1248(BP), AX
  MOVQ  AX, -1256(BP)
  MOVQ  -1256(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -1264(BP)
  MOVQ  $0, AX
  MOVQ  AX, -1272(BP)
  MOVQ  -1272(BP), AX
  MOVQ  AX, -1280(BP)
  JMP   L1
LABEL L1
  MOVQ  -1280(BP), AX
  MOVQ  -1264(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -1288(BP)
  MOVQ  -1288(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  $3, AX
  MOVQ  AX, -1296(BP)
  MOVQ  -1280(BP), AX
  MOVQ  -1296(BP), CX
  SHLQ  CX, AX
  MOVQ  AX, -1304(BP)
  MOVQ  -1256(BP), AX
  MOVQ  -1304(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -1312(BP)
  MOVQ  -1312(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -1320(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -1320(BP), AX
  MOVQ  AX, 8(SP)
  CALL  main.apply_code(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -1328(BP)
  MOVQ  -1328(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -1336(BP)
  MOVQ  $1, AX
  MOVQ  AX, -1344(BP)
  MOVQ  -1280(BP), AX
  MOVQ  -1344(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -1280(BP)
  JMP   L1
LABEL L3
  MOVQ  $0, AX
  MOVQ  AX, -1352(BP)
  MOVQ  -1352(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.14(SB), AX
  JMP   rt.fault(SB)
