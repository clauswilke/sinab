#' @useDynLib gridtest.rewrite test_
test <- function(x, y) .Call(test_, x, y)