library(ggplot2)
library(dplyr)
library(glue)
library(rlang)
library(sinab)

element_html <- function(css = NULL, family = NULL, face = NULL, size = NULL, colour = NULL, fill = NULL,
                         linetype = NULL, linewidth = NULL, hjust = NULL, vjust = NULL, lineheight = NULL,
                         margin = NULL, width = NULL, height = NULL, color = NULL, 
                         debug = FALSE, inherit.blank = FALSE) {
  if (!is.null(color))
    colour <- color
  
  # doesn't work with other values at this time
  hjust <- 0
  vjust <- 1

  structure(
    list(
      css = css,
      family = family, face = face, size = size, colour = colour, fill = fill,
      linetype = linetype, linewidth = linewidth, hjust = hjust, vjust = vjust,
      lineheight = lineheight, margin = margin, width = width, height = height,
      debug = debug, inherit.blank = inherit.blank),
    class = c("element_html", "element_text", "element")
  )
}

element_grob.element_html <- function(element, label = "", x = NULL, y = NULL,
                                      family = NULL, face = NULL, colour = NULL, size = NULL,
                                      hjust = NULL, vjust = NULL, lineheight = NULL,
                                      margin = NULL, ...) {
  if (is.null(label))
    return(ggplot2::zeroGrob())
  
  # for now we ignore hjust and vjust, it doesn't work yet
  hj <- 0
  vj <- 1

  css <- element$css %||% ""

  html_grob(
    label, x = x, y = y, hjust = hj, vjust = vj,
    width = element$width, height = element$height,
    css = css
  )
}


css <- '
p { text-align: center; padding-top: 2px; }
.drv-f { background-color: darkslategray; color: white; }
.drv-r { background-color: lightsteelblue; }
.drv-4 { background-color: darkkhaki; }
'

mpg %>%
  mutate(facet_label = glue('<p class = "drv-{drv}">{class}</p>')) %>%
  ggplot(aes(cty, hwy)) + 
  geom_point() +
  facet_wrap(~facet_label) +
  theme_bw() +
  theme(
    strip.background = element_blank(),
    strip.text = element_html(css = css)
  )



