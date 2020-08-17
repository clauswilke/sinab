#' Render HTML or Markdown
#' 
#' Render HTML or Markdown
#' @param text HTML or Markdown text to render
#' @param x,y x and y position
#' @param width,height width and height
#' @param hjust,vjust horizontal and vertical justification relative
#'  to `x` and `y`
#' @param css CSS specification to use for rendering
#' @param vp viewport
#' @export
html_grob <- function(text, x = unit(0.1, "npc"), y = unit(0.9, "npc"),
                      width = unit(0.8, "npc"), height = unit(0.8, "npc"),
                      hjust = 0, vjust = 1, css = "", vp = NULL) {
  # make sure we can handle input text even if provided as factor
  text <- as.character(text)
  # convert NAs to empty strings
  text <- ifelse(is.na(text), "", text)
  
  gTree(
    text = text,
    x = x,
    y = y,
    width = width,
    height = height,
    hjust = hjust,
    vjust = vjust,
    css = css,
    vp = vp,
    cl = "html_grob"
  )
}

#' @export
makeContext.html_grob <- function(x) {
  x$width_inch <- current_width(x, x$width)
  # for now, we're using the specified height,
  # not the calculated height
  x$height_inch <- current_height(x, x$height)
  
  vp <- viewport(x$x, x$y, just = c(x$hjust, 1-x$vjust))
  if (is.null(x$vp)) {
    x$vp <- vp
  } else {
    x$vp <- vpStack(x$vp, vp)
  }
  x
}

#' @export
makeContent.html_grob <- function(x) {
  grobs <- render_markdown(x$text, x$css, x$width_inch, x$height_inch)
  
  setChildren(x, grobs)
}



#' @export
heightDetails.html_grob <- function(x) {
  unit(x$height_inch, "inches")
}

#' @export
widthDetails.html_grob <- function(x) {
  unit(x$width_inch, "inches")
}

#' @export
ascentDetails.html_grob <- function(x) {
  unit(x$height_inch, "inches")
}

#' @export
descentDetails.html_grob <- function(x) {
  unit(0, "inches")
}