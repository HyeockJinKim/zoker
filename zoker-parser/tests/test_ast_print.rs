use zoker_parser::{parser, print};

#[test]
fn test_print_number() {
    let num = parser::parse_expression("66").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ Number : 66 ] ");
    assert_eq!(ast.print_ast(), "[ Number : 66 ] \n");
}

#[test]
fn test_print_arithmetic_expression1() {
    let num = parser::parse_expression("22 + 66").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ BinaryExpression ] ");
    assert_eq!(ast.print_ast(), "            [ BinaryExpression ]              \n[ Number : 22 ] [ binop : + ] [ Number : 66 ] \n");
}

#[test]
fn test_print_arithmetic_expression2() {
    let num = parser::parse_expression("22 + 66 * 33").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ BinaryExpression ] ");
    assert_eq!(ast.print_ast(), "                           [ BinaryExpression ]                             \n[ Number : 22 ] [ binop : + ]             [ BinaryExpression ]              \n                              [ Number : 66 ] [ binop : * ] [ Number : 33 ] \n");
}

#[test]
fn test_print_arithmetic_expression3() {
    let num = parser::parse_expression("22 * (1 + 2) - 66 * 33 % 3").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                                                       [ BinaryExpression ]                                                                        \n                          [ BinaryExpression ]                            [ binop : - ]                            [ BinaryExpression ]                            \n[ Number : 22 ] [ binop : * ]            [ BinaryExpression ]                                       [ BinaryExpression ]              [ binop : % ] [ Number : 3 ] \n                              [ Number : 1 ] [ binop : + ] [ Number : 2 ]               [ Number : 66 ] [ binop : * ] [ Number : 33 ]                              \n");
}

#[test]
fn test_print_assign_expression1() {
    let num = parser::parse_expression("a = 22 + 3 * 2").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                             [ AssignExpression ]                                              \n[ Identifier : a ] [ assign-op : = ]                           [ BinaryExpression ]                            \n                                     [ Number : 22 ] [ binop : + ]            [ BinaryExpression ]             \n                                                                   [ Number : 3 ] [ binop : * ] [ Number : 2 ] \n");
}

#[test]
fn test_print_assign_expression2() {
    let num = parser::parse_expression("a = b = 2").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                  [ AssignExpression ]                                   \n[ Identifier : a ] [ assign-op : = ]                [ AssignExpression ]                 \n                                     [ Identifier : b ] [ assign-op : = ] [ Number : 2 ] \n");
}

#[test]
fn test_print_comparison_expression1() {
    let num = parser::parse_expression("2 < a").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                [ BinaryExpression ]                 \n[ Number : 2 ] [ compare-op : < ] [ Identifier : a ] \n");
}

#[test]
fn test_print_comparison_expression2() {
    let num = parser::parse_expression("(a + 2 >= 3) == (2 < a && b < c)").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                                                                                          [ BinaryExpression ]                                                                                                           \n                               [ BinaryExpression ]                                [ compare-op : == ]                                                       [ BinaryExpression ]                                                        \n             [ BinaryExpression ]               [ compare-op : >= ] [ Number : 3 ]                                     [ BinaryExpression ]                 [ logical-op : && ]                   [ BinaryExpression ]                   \n[ Identifier : a ] [ binop : + ] [ Number : 2 ]                                                        [ Number : 2 ] [ compare-op : < ] [ Identifier : a ]                     [ Identifier : b ] [ compare-op : < ] [ Identifier : c ] \n");
}

#[test]
fn test_print_if_statement_ast1() {
    let stmt = parser::parse_statement("if a < 2 { a = 3; } else { a = 1; }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                   [ If-else Expression ]                                                                    \n                [ BinaryExpression ]                               [ Compound Statement ]                              [ Compound Statement ]                \n[ Identifier : a ] [ compare-op : < ] [ Number : 2 ]                [ AssignExpression ]                                [ AssignExpression ]                 \n                                                     [ Identifier : a ] [ assign-op : = ] [ Number : 3 ] [ Identifier : a ] [ assign-op : = ] [ Number : 1 ] \n");
}

#[test]
fn test_print_if_statement_ast2() {
    let stmt = parser::parse_statement("b = if a < 2 { 1 } else { 0 }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                         [ AssignExpression ]                                                           \n[ Identifier : b ] [ assign-op : = ]                                       [ If-else Expression ]                                       \n                                                     [ BinaryExpression ]                 [ Compound Statement ] [ Compound Statement ] \n                                     [ Identifier : a ] [ compare-op : < ] [ Number : 2 ]     [ Number : 1 ]         [ Number : 0 ]     \n");
}

#[test]
fn test_print_if_statement_ast3() {
    let stmt = parser::parse_statement("if a < 2 { a = 2; b = 2; }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                     [ If Expression ]                                                                       \n                [ BinaryExpression ]                                                         [ Compound Statement ]                                          \n[ Identifier : a ] [ compare-op : < ] [ Number : 2 ]                [ AssignExpression ]                                [ AssignExpression ]                 \n                                                     [ Identifier : a ] [ assign-op : = ] [ Number : 2 ] [ Identifier : b ] [ assign-op : = ] [ Number : 2 ] \n");
}

#[test]
fn test_print_if_statement_ast4() {
    let stmt = parser::parse_statement("abs_plus = if a > 0 { c = if b > 0 { b } else { -b }; a + c } else { c = if b > 0 { b } else { -b }; -a + c }")
        .unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                                                                                                                                                                                                [ AssignExpression ]                                                                                                                                                                                                                                                 \n[ Identifier : abs_plus ] [ assign-op : = ]                                                                                                                                                                                                                          [ If-else Expression ]                                                                                                                                                                                                                          \n                                                            [ BinaryExpression ]                                                                                                       [ Compound Statement ]                                                                                                                                                                                    [ Compound Statement ]                                                                                              \n                                            [ Identifier : a ] [ compare-op : > ] [ Number : 0 ]                                                              [ AssignExpression ]                                                                              [ BinaryExpression ]                                                                              [ AssignExpression ]                                                                                    [ BinaryExpression ]                       \n                                                                                                 [ Identifier : c ] [ assign-op : = ]                                           [ If-else Expression ]                                           [ Identifier : a ] [ binop : + ] [ Identifier : c ] [ Identifier : c ] [ assign-op : = ]                                           [ If-else Expression ]                                                [ UnaryExpression ]       [ binop : + ] [ Identifier : c ] \n                                                                                                                                                      [ BinaryExpression ]                 [ Compound Statement ]     [ Compound Statement ]                                                                                                              [ BinaryExpression ]                 [ Compound Statement ]     [ Compound Statement ]     [ Identifier : a ] [ uop : - ]                                  \n                                                                                                                                      [ Identifier : b ] [ compare-op : > ] [ Number : 0 ]   [ Identifier : b ]        [ UnaryExpression ]                                                                                                [ Identifier : b ] [ compare-op : > ] [ Number : 0 ]   [ Identifier : b ]        [ UnaryExpression ]                                                                       \n                                                                                                                                                                                                                  [ Identifier : b ] [ uop : - ]                                                                                                                                                                      [ Identifier : b ] [ uop : - ]                                                                 \n");
}

#[test]
fn test_print_for_each_statement_ast1() {
    let stmt =
        parser::parse_statement("max = for i in vec { if max < i { i } else { max } }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                                 [ AssignExpression ]                                                                                   \n[ Identifier : max ] [ assign-op : = ]                                                                [ For Expression ]                                                                \n                                       [ Identifier : i ] [ Identifier : vec ]                                          [ Compound Statement ]                                          \n                                                                                                                        [ If-else Expression ]                                          \n                                                                                                  [ BinaryExpression ]                    [ Compound Statement ] [ Compound Statement ] \n                                                                               [ Identifier : max ] [ compare-op : < ] [ Identifier : i ]   [ Identifier : i ]    [ Identifier : max ]  \n");
}

#[test]
fn test_print_for_each_statement_ast2() {
    let stmt = parser::parse_statement("sum += for i in vec { i }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                         [ AssignExpression ]                                          \n[ Identifier : sum ] [ assign-op : += ]                       [ For Expression ]                       \n                                        [ Identifier : i ] [ Identifier : vec ] [ Compound Statement ] \n                                                                                  [ Identifier : i ]   \n");
}

#[test]
fn test_print_function_statement_ast1() {
    let stmt = parser::parse_global_statement("function two() { 3 }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                       [ Function Statement ]                         \n[ Identifier : two ] [ Parameters Expression ] [ Compound Statement ] \n                                                   [ Number : 3 ]     \n");
}

#[test]
fn test_print_function_statement_ast2() {
    let stmt = parser::parse_global_statement("function plus(uint i, int j) { i + j }").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                               [ Function Statement ]                                                                \n[ Identifier : plus ]                         [ Parameters Expression ]                                        [ Compound Statement ]                \n                            [ Initializer Statement ]            [ Initializer Statement ]                      [ BinaryExpression ]                 \n                      [ type : uint256 ] [ Identifier : i ] [ type : int256 ] [ Identifier : j ] [ Identifier : i ] [ binop : + ] [ Identifier : j ] \n");
}

#[test]
fn test_print_function_call_expression_ast1() {
    let stmt = parser::parse_statement("f()").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "       [ Function Call Expression ]         \n[ Identifier : f ] [ Arguments Expression ] \n");
}

#[test]
fn test_print_function_call_expression_ast2() {
    let stmt = parser::parse_statement("f(i, j)").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "              [ Function Call Expression ]               \n[ Identifier : f ]       [ Arguments Expression ]        \n                   [ Identifier : i ] [ Identifier : j ] \n");
}

#[test]
fn test_print_initializer_expression_ast1() {
    let stmt = parser::parse_statement("uint i").unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(
        ast.print_ast(),
        "      [ Initializer Statement ]       \n[ type : uint256 ] [ Identifier : i ] \n"
    );
}

#[test]
fn test_print_initializer_expression_ast2() {
    let stmt = parser::parse_program("uint i;").unwrap();
    let ast = print::program_to_str(&stmt);
    assert_eq!(
        ast.print_ast(),
        "             [ Program ]              \n      [ Initializer Statement ]       \n[ type : uint256 ] [ Identifier : i ] \n"
    );
}

#[test]
fn test_print_program_ast1() {
    let stmt = parser::parse_program("uint i;\nuint a = 3;").unwrap();
    let ast = print::program_to_str(&stmt);
    assert_eq!(
        ast.print_ast(),
        "                                       [ Program ]                                         \n      [ Initializer Statement ]                    [ Initializer Statement ]               \n[ type : uint256 ] [ Identifier : i ] [ type : uint256 ] [ Identifier : a ] [ Number : 3 ] \n"
    );
}
