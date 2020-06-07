#' @useDynLib grdtext add_
#' @export
add <- function(x, y) .Call(add_, x, y)

#' @useDynLib grdtext named_list_
#' @export
named_list <- function(x, y) .Call(named_list_, x, y)