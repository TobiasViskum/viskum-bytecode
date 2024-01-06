pub const OPCODES: [&str; 8] = [
    "OpReturn       = 0",
    "OpConstant     = 1",
    "OpConstantLong = 2",
    "OpNegate       = 3",
    "OpAdd          = 4",
    "OpSubtract     = 5",
    "OpMultiply     = 6",
    "OpDivide       = 7",
];

pub const TOKENTYPES: [&str; 45] = [
    "TokenLeftParen            <=>  left parenthesis",
    "TokenRightParen           <=>  right parenthesis",
    "TokenLeftBrace            <=>  left curly brace",
    "TokenRightBrace           <=>  right curly brace",
    "TokenComma                <=>  comma",
    "TokenMinus                <=>  minus",
    "TokenDot                  <=>  dot",
    "TokenPlus                 <=>  plus",
    "TokenSemicolon            <=>  semicolon",
    "TokenSlash                <=>  divide",
    "TokenStar                 <=>  multiply",
    "TokenPower                <=>  power",

    "TokenStringStart          <=>  string start",
    "TokenStringEnd            <=>  string end",
    "TokenInterpolationStart   <=>  interpolation start",
    "TokenInterpolationEnd     <=>  interpolation end",

    "TokenBang                 <=>  not",
    "TokenBangEqual            <=>  not equal",
    "TokenEqual                <=>  equal",
    "TokenEqualEqual           <=>  equal equal",
    "TokenGreater              <=>  greater than",
    "TokenGreaterEqual         <=>  greater than or equal",
    "TokenLess                 <=>  less than",
    "TokenLessEqual            <=>  less than or equal",

    "TokenIdentifier           <=>  identifier",
    "TokenString               <=>  string literal",
    "TokenNumber               <=>  number literal",

    "TokenAnd                  <=>  and",
    "TokenClass                <=>  class",
    "TokenElse                 <=>  else",
    "TokenFalse                <=>  false",
    "TokenFor                  <=>  for",
    "TokenFn                   <=>  function",
    "TokenIf                   <=>  if",
    "TokenNull                 <=>  null",
    "TokenOr                   <=>  or",
    "TokenPrint                <=>  print",
    "TokenReturn               <=>  return",
    "TokenThis                 <=>  this",
    "TokenTrue                 <=>  true",
    "TokenLet                  <=>  let",
    "TokenWhile                <=>  while",

    "TokenExprEnd              <=>  end of expression",
    "TokenError(String)        <=>  error",
    "TokenEof                  <=>  end of file",
];
