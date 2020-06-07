#' @useDynLib grdtext test_
#' @export
test <- function(x, y) .Call(test_, x, y)