TEXT int_div_rem.main(SB)
  LEAQ  -992(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $976, SP
  MOVQ  $7, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $-7, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $2, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $-2, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -16(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L2
  CMPQ  CX, $-1
  JNE   L4
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L3
LABEL L4
  CQO
  IDIVQ CX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $16, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -32(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L5
  CMPQ  CX, $-1
  JNE   L7
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L6
LABEL L7
  CQO
  IDIVQ CX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $16, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $1, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -152(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  -16(BP), AX
  MOVQ  -64(BP), CX
  TESTQ CX, CX
  JEQ   L8
  CMPQ  CX, $-1
  JNE   L10
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L9
LABEL L10
  CQO
  IDIVQ CX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $16, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $1, AX
  MOVQ  AX, -216(BP)
  MOVQ  -208(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -208(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  -32(BP), AX
  MOVQ  -64(BP), CX
  TESTQ CX, CX
  JEQ   L11
  CMPQ  CX, $-1
  JNE   L13
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L12
LABEL L13
  CQO
  IDIVQ CX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  $16, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $1, AX
  MOVQ  AX, -272(BP)
  MOVQ  -264(BP), AX
  MOVQ  -272(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -264(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  -16(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L14
  CMPQ  CX, $-1
  JNE   L16
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L15
LABEL L16
  CQO
  IDIVQ CX
  MOVQ  DX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $16, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  $1, AX
  MOVQ  AX, -328(BP)
  MOVQ  -320(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -320(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  -32(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L17
  CMPQ  CX, $-1
  JNE   L19
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L18
LABEL L19
  CQO
  IDIVQ CX
  MOVQ  DX, -352(BP)
  MOVQ  -352(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -360(BP)
  MOVQ  $16, AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  MOVQ  $1, AX
  MOVQ  AX, -384(BP)
  MOVQ  -376(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -376(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  -16(BP), AX
  MOVQ  -64(BP), CX
  TESTQ CX, CX
  JEQ   L20
  CMPQ  CX, $-1
  JNE   L22
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L21
LABEL L22
  CQO
  IDIVQ CX
  MOVQ  DX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  $16, AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $1, AX
  MOVQ  AX, -440(BP)
  MOVQ  -432(BP), AX
  MOVQ  -440(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -432(BP), AX
  MOVQ  -416(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  -32(BP), AX
  MOVQ  -64(BP), CX
  TESTQ CX, CX
  JEQ   L23
  CMPQ  CX, $-1
  JNE   L25
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L24
LABEL L25
  CQO
  IDIVQ CX
  MOVQ  DX, -464(BP)
  MOVQ  -464(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -472(BP)
  MOVQ  $16, AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -488(BP)
  MOVQ  $1, AX
  MOVQ  AX, -496(BP)
  MOVQ  -488(BP), AX
  MOVQ  -496(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -488(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -488(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  -32(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L26
  CMPQ  CX, $-1
  JNE   L28
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L27
LABEL L28
  CQO
  IDIVQ CX
  MOVQ  AX, -520(BP)
  MOVQ  -520(BP), AX
  MOVQ  -48(BP), CX
  IMULQ CX, AX
  JOS   L29
  MOVQ  AX, -528(BP)
  MOVQ  -32(BP), AX
  MOVQ  -48(BP), CX
  TESTQ CX, CX
  JEQ   L30
  CMPQ  CX, $-1
  JNE   L32
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L31
LABEL L32
  CQO
  IDIVQ CX
  MOVQ  DX, -536(BP)
  MOVQ  -528(BP), AX
  MOVQ  -536(BP), CX
  ADDQ  CX, AX
  JOS   L33
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  $16, AX
  MOVQ  AX, -560(BP)
  MOVQ  -560(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -568(BP)
  MOVQ  $1, AX
  MOVQ  AX, -576(BP)
  MOVQ  -568(BP), AX
  MOVQ  -576(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -568(BP), AX
  MOVQ  -552(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -568(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  -584(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -592(BP)
  MOVQ  $0, AX
  MOVQ  AX, -600(BP)
  MOVQ  -600(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L34
  CMPQ  CX, $-1
  JNE   L36
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L35
LABEL L36
  CQO
  IDIVQ CX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -616(BP)
  MOVQ  $16, AX
  MOVQ  AX, -624(BP)
  MOVQ  -624(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -632(BP)
  MOVQ  $1, AX
  MOVQ  AX, -640(BP)
  MOVQ  -632(BP), AX
  MOVQ  -640(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -632(BP), AX
  MOVQ  -616(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -632(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -656(BP)
  MOVQ  $1, AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L37
  CMPQ  CX, $-1
  JNE   L39
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L38
LABEL L39
  CQO
  IDIVQ CX
  MOVQ  AX, -672(BP)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -680(BP)
  MOVQ  $16, AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $1, AX
  MOVQ  AX, -704(BP)
  MOVQ  -696(BP), AX
  MOVQ  -704(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -696(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  $100, AX
  MOVQ  AX, -728(BP)
  MOVQ  -728(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L40
  CMPQ  CX, $-1
  JNE   L42
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L41
LABEL L42
  CQO
  IDIVQ CX
  MOVQ  AX, -736(BP)
  MOVQ  -736(BP), AX
  MOVQ  -16(BP), CX
  IMULQ CX, AX
  JOS   L43
  MOVQ  AX, -744(BP)
  MOVQ  -744(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -752(BP)
  MOVQ  $16, AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -768(BP)
  MOVQ  $1, AX
  MOVQ  AX, -776(BP)
  MOVQ  -768(BP), AX
  MOVQ  -776(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -768(BP), AX
  MOVQ  -752(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -768(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -784(BP)
  MOVQ  -784(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -792(BP)
  MOVQ  $100, AX
  MOVQ  AX, -800(BP)
  MOVQ  -800(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L44
  CMPQ  CX, $-1
  JNE   L46
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L45
LABEL L46
  CQO
  IDIVQ CX
  MOVQ  DX, -808(BP)
  MOVQ  $4, AX
  MOVQ  AX, -816(BP)
  MOVQ  -808(BP), AX
  MOVQ  -816(BP), CX
  TESTQ CX, CX
  JEQ   L47
  CMPQ  CX, $-1
  JNE   L49
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L48
LABEL L49
  CQO
  IDIVQ CX
  MOVQ  DX, -824(BP)
  MOVQ  -824(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -832(BP)
  MOVQ  $16, AX
  MOVQ  AX, -840(BP)
  MOVQ  -840(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -848(BP)
  MOVQ  $1, AX
  MOVQ  AX, -856(BP)
  MOVQ  -848(BP), AX
  MOVQ  -856(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -848(BP), AX
  MOVQ  -832(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -848(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -864(BP)
  MOVQ  -864(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -872(BP)
  MOVQ  $2, AX
  MOVQ  AX, -880(BP)
  MOVQ  $100, AX
  MOVQ  AX, -888(BP)
  MOVQ  -888(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L50
  CMPQ  CX, $-1
  JNE   L52
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L51
LABEL L52
  CQO
  IDIVQ CX
  MOVQ  AX, -896(BP)
  MOVQ  -880(BP), AX
  MOVQ  -896(BP), CX
  ADDQ  CX, AX
  JOS   L53
  MOVQ  AX, -904(BP)
  MOVQ  -904(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -912(BP)
  MOVQ  $16, AX
  MOVQ  AX, -920(BP)
  MOVQ  -920(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -928(BP)
  MOVQ  $1, AX
  MOVQ  AX, -936(BP)
  MOVQ  -928(BP), AX
  MOVQ  -936(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -928(BP), AX
  MOVQ  -912(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -928(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -944(BP)
  MOVQ  -944(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -952(BP)
  MOVQ  $0, AX
  MOVQ  AX, -960(BP)
  MOVQ  -960(BP), AX
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
  LEAQ  site.1.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.2.1(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L9
  LEAQ  site.3.1(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L12
  LEAQ  site.4.1(SB), AX
  JMP   rt.fault(SB)
LABEL L14
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L15
  LEAQ  site.5.1(SB), AX
  JMP   rt.fault(SB)
LABEL L17
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L18
  LEAQ  site.6.1(SB), AX
  JMP   rt.fault(SB)
LABEL L20
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
LABEL L21
  LEAQ  site.7.1(SB), AX
  JMP   rt.fault(SB)
LABEL L23
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L24
  LEAQ  site.8.1(SB), AX
  JMP   rt.fault(SB)
LABEL L26
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L27
  LEAQ  site.9.1(SB), AX
  JMP   rt.fault(SB)
LABEL L29
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
LABEL L30
  LEAQ  site.11(SB), AX
  JMP   rt.fault(SB)
LABEL L31
  LEAQ  site.11.1(SB), AX
  JMP   rt.fault(SB)
LABEL L33
  LEAQ  site.12(SB), AX
  JMP   rt.fault(SB)
LABEL L34
  LEAQ  site.13(SB), AX
  JMP   rt.fault(SB)
LABEL L35
  LEAQ  site.13.1(SB), AX
  JMP   rt.fault(SB)
LABEL L37
  LEAQ  site.14(SB), AX
  JMP   rt.fault(SB)
LABEL L38
  LEAQ  site.14.1(SB), AX
  JMP   rt.fault(SB)
LABEL L40
  LEAQ  site.15(SB), AX
  JMP   rt.fault(SB)
LABEL L41
  LEAQ  site.15.1(SB), AX
  JMP   rt.fault(SB)
LABEL L43
  LEAQ  site.16(SB), AX
  JMP   rt.fault(SB)
LABEL L44
  LEAQ  site.17(SB), AX
  JMP   rt.fault(SB)
LABEL L45
  LEAQ  site.17.1(SB), AX
  JMP   rt.fault(SB)
LABEL L47
  LEAQ  site.18(SB), AX
  JMP   rt.fault(SB)
LABEL L48
  LEAQ  site.18.1(SB), AX
  JMP   rt.fault(SB)
LABEL L50
  LEAQ  site.19(SB), AX
  JMP   rt.fault(SB)
LABEL L51
  LEAQ  site.19.1(SB), AX
  JMP   rt.fault(SB)
LABEL L53
  LEAQ  site.20(SB), AX
  JMP   rt.fault(SB)
