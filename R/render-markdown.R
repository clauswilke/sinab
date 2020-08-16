#' Render HTML or Markdown to a list of grid grobs
#' 
#' Render HTML or Markdown to a list of grid grobs.
#' @param text HTML or Markdown to render
#' @param css The CSS to use when rendering `text`
#' @param width_in Width of the drawing area, in inches 
#' @param height_in Height of the drawing area, in inches
#' @export
render_markdown <- function(text, css = "", width_in = 5, height_in = 5) {
  .Call(C_render_markdown, text, css, width_in, height_in)
}
