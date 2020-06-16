test_that("grid renderer new/release", {
  x <- .Call("test_gr_new_release", 1)
  expect_identical(length(x), 1L)
  expect_identical(x[[1]], "test")
  expect_s3_class(x, "gList")

  x <- .Call("test_gr_new_release", 0)
  expect_identical(length(x), 0L)
  expect_s3_class(x, "gList")
  
  x <- .Call("test_gr_new_release", 100)
  expect_identical(length(x), 100L)
  expect_identical(x[[75]], "test")
  expect_s3_class(x, "gList")
})
