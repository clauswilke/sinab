#' @export
render_markdown <- function(text) .Call(C_render_markdown, text)
