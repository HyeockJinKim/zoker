use zoker_parser::{print, zoker};

#[test]
fn test_print_number() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("66").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ Number : 66 ] ");
    assert_eq!(ast.print_ast(), "[ Number : 66 ] \n");
}

#[test]
fn test_print_arithmetic_expression1() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("22 + 66").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ BinaryExpression ] ");
    assert_eq!(ast.print_ast(), "            [ BinaryExpression ]             \n[ Number : 22 ] [ binop : + ] [ Number : 66 ] \n");
}

#[test]
fn test_print_arithmetic_expression2() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("22 + 66 * 33").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.str(), "[ BinaryExpression ] ");
    assert_eq!(ast.print_ast(), "                           [ BinaryExpression ]                            \n[ Number : 22 ] [ binop : + ]             [ BinaryExpression ]             \n                              [ Number : 66 ] [ binop : * ] [ Number : 33 ] \n");
}

#[test]
fn test_print_arithmetic_expression3() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("22 * (1 + 2) - 66 * 33 % 3").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                                                       [ BinaryExpression ]                                                                        \n                          [ BinaryExpression ]                           [ binop : - ]                            [ BinaryExpression ]                            \n[ Number : 22 ] [ binop : * ]            [ BinaryExpression ]                                      [ BinaryExpression ]             [ binop : % ] [ Number : 3 ] \n                              [ Number : 1 ] [ binop : + ] [ Number : 2 ]               [ Number : 66 ] [ binop : * ] [ Number : 33 ]                              \n");
}

#[test]
fn test_print_assign_expression() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("a = 22 + 3 * 2").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                             [ AssignExpression ]                                              \n[ Identifier : a ] [ assign-op : = ]                           [ BinaryExpression ]                           \n                                     [ Number : 22 ] [ binop : + ]            [ BinaryExpression ]            \n                                                                   [ Number : 3 ] [ binop : * ] [ Number : 2 ] \n");
}

#[test]
fn test_print_assign_expression2() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("a = b = 2").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                  [ AssignExpression ]                                   \n[ Identifier : a ] [ assign-op : = ]                [ AssignExpression ]                \n                                     [ Identifier : b ] [ assign-op : = ] [ Number : 2 ] \n");
}

#[test]
fn test_print_comparison_expression1() {
    use zoker::ExpressionParser as parser;
    let num = parser::new().parse("2 < a").unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                [ BinaryExpression ]                 \n[ Number : 2 ] [ compare-op : < ] [ Identifier : a ] \n");
}

#[test]
fn test_print_comparison_expression2() {
    use zoker::ExpressionParser as parser;
    let num = parser::new()
        .parse("(a + 2 >= 3) == (2 < a && b < c)")
        .unwrap();

    let ast = print::expr_to_str(&num.node);
    assert_eq!(ast.print_ast(), "                                                                                                          [ BinaryExpression ]                                                                                                           \n                               [ BinaryExpression ]                                [ compare-op : == ]                                                       [ BinaryExpression ]                                                       \n             [ BinaryExpression ]              [ compare-op : >= ] [ Number : 3 ]                                     [ BinaryExpression ]                 [ logical-op : && ]                   [ BinaryExpression ]                   \n[ Identifier : a ] [ binop : + ] [ Number : 2 ]                                                        [ Number : 2 ] [ compare-op : < ] [ Identifier : a ]                     [ Identifier : b ] [ compare-op : < ] [ Identifier : c ] \n");
}

#[test]
fn test_print_if_statement_ast1() {
    use zoker::StatementParser as parser;
    let stmt = parser::new()
        .parse("if a < 2 { a = 3; } else { a = 1; }")
        .unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                   [ If-else Expression ]                                                                    \n                [ BinaryExpression ]                               [ Compound Statement ]                             [ Compound Statement ]               \n[ Identifier : a ] [ compare-op : < ] [ Number : 2 ]                [ AssignExpression ]                               [ AssignExpression ]                \n                                                     [ Identifier : a ] [ assign-op : = ] [ Number : 3 ] [ Identifier : a ] [ assign-op : = ] [ Number : 1 ] \n");
}

#[test]
fn test_print_if_statement_ast2() {
    use zoker::StatementParser as parser;
    let stmt = parser::new()
        .parse("b = if a < 2 { 1 } else { 0 }")
        .unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                         [ AssignExpression ]                                                          \n[ Identifier : b ] [ assign-op : = ]                                       [ If-else Expression ]                                       \n                                                     [ BinaryExpression ]                 [ Compound Statement ] [ Compound Statement ] \n                                     [ Identifier : a ] [ compare-op : < ] [ Number : 2 ] [ Number : 1 ] [ Number : 0 ] \n");
}

#[test]
fn test_print_if_statement_ast3() {
    use zoker::StatementParser as parser;
    let stmt = parser::new()
        .parse("if a < 2 { a = 2; b = 2; }")
        .unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                     [ If Expression ]                                                                      \n                [ BinaryExpression ]                                                         [ Compound Statement ]                                         \n[ Identifier : a ] [ compare-op : < ] [ Number : 2 ]                [ AssignExpression ]                               [ AssignExpression ]                \n                                                     [ Identifier : a ] [ assign-op : = ] [ Number : 2 ] [ Identifier : b ] [ assign-op : = ] [ Number : 2 ] \n");
}

#[test]
fn test_print_if_statement_ast4() {
    use zoker::StatementParser as parser;
    let stmt = parser::new()
        .parse("abs_plus = if a > 0 { c = if b > 0 { b } else { -b }; a + c } else { c = if b > 0 { b } else { -b }; -a + c }")
        .unwrap();

    let ast = print::stmt_to_str(&stmt.node);
    assert_eq!(ast.print_ast(), "                                                                                                                                                                                                                                                [ AssignExpression ]                                                                                                                                                                                                                                                 \n[ Identifier : abs_plus ] [ assign-op : = ]                                                                                                                                                                                                                          [ If-else Expression ]                                                                                                                                                                                                                          \n                                                            [ BinaryExpression ]                                                                                                       [ Compound Statement ]                                                                                                                                                                                   [ Compound Statement ]                                                                                             \n                                            [ Identifier : a ] [ compare-op : > ] [ Number : 0 ]                                                              [ AssignExpression ]                                                                             [ BinaryExpression ]                                                                             [ AssignExpression ]                                                                                   [ BinaryExpression ]                      \n                                                                                                 [ Identifier : c ] [ assign-op : = ]                                           [ If-else Expression ]                                           [ Identifier : a ] [ binop : + ] [ Identifier : c ] [ Identifier : c ] [ assign-op : = ]                                           [ If-else Expression ]                                                [ UnaryExpression ]      [ binop : + ] [ Identifier : c ] \n                                                                                                                                                      [ BinaryExpression ]                 [ Compound Statement ]     [ Compound Statement ]                                                                                                              [ BinaryExpression ]                 [ Compound Statement ]     [ Compound Statement ]     [ Identifier : a ] [ uop : - ]                                  \n                                                                                                                                      [ Identifier : b ] [ compare-op : > ] [ Number : 0 ] [ Identifier : b ]      [ UnaryExpression ]                                                                                               [ Identifier : b ] [ compare-op : > ] [ Number : 0 ] [ Identifier : b ]      [ UnaryExpression ]                                                                      \n                                                                                                                                                                                                              [ Identifier : b ] [ uop : - ]                                                                                                                                                                  [ Identifier : b ] [ uop : - ]                                                                 \n");
}
