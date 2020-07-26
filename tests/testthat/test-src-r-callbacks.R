test_that("gpar_empty()", {
  x <- .Call(gpar_empty)
  y <- grid::gpar()
  expect_identical(x, y)
})

test_that("gpar_gcontext()", {
  x <- .Call(test_gpar_gcontext)
  y <- grid::gpar(
    col = "red", fill = "#F0F0F0F0",
    fontfamily = "Helvetica", fontface = 1L,
    fontsize = 12, lineheight = 1.2
  )
  expect_identical(x, y)
})

test_that("unit_in()", {
  x <- .Call(unit_in, 4)
  y <- grid::unit(4, "inches")
  expect_identical(x, y)
})

test_that("text_grob()", {
  label = "hello"
  x <- 2
  y <- 5
  hjust <- 0.5
  vjust <- 1
  gp <- grid::gpar(fill = "blue")
  g1 <- .Call(text_grob, label, x, y, hjust, vjust, gp)
  g2 <- grid::textGrob(
    label, x, y, hjust = hjust, vjust = vjust, gp = gp
  )
  g2$name <- g1$name # all grobs have a unique name
  expect_identical(g1, g2)
})

test_that("rect_grob()", {
  x <- 2
  y <- 5
  width <- 3
  height <- 4
  hjust <- 0.5
  vjust <- 1
  gp <- grid::gpar(fill = "blue")
  g1 <- .Call(rect_grob, x, y, width, height, hjust, vjust, gp)
  g2 <- grid::rectGrob(
    x, y, width, height, hjust = hjust, vjust = vjust, gp = gp
  )
  g2$name <- g1$name # all grobs have a unique name
  expect_identical(g1, g2)
})