#' @export
render_markdown <- function(text, css) .Call(C_render_markdown, text, css)
