#' Convert markdown to html
#' 
#' Convert a markdown string into an html string. The markdown conversion
#' adheres to the commonmark standard with tables extension.
#' @param text Character vector. If of length > 1, each string is
#'   separately converted.
#' @return Character vector of html fragments.
#' @references 
#' https://commonmark.org/
#' @export
md_to_html <- function(text) {
  text <- as.character(text)
  .Call(C_md_to_html, text)
}