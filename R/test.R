#' @export
add <- function(x, y) .Call(add_, x, y)

#' @export
named_list <- function(x, y) .Call(named_list_, x, y)

#' @export
random_unif <- function(n, min, max) .Call(random_unif_, n, min, max)

#' @export
test <- function(n) .Call(test_, n)
