test_that("grid renderer create/release", {
  x <- .Call("test_gr_create_release", 1)
  expect_identical(length(x), 1L)
  expect_identical(x[[1]], "test")
  expect_s3_class(x, "gList")

  x <- .Call("test_gr_create_release", 0)
  expect_identical(length(x), 0L)
  expect_s3_class(x, "gList")
  
  x <- .Call("test_gr_create_release", 100)
  expect_identical(length(x), 100L)
  expect_identical(x[[75]], "test")
  expect_s3_class(x, "gList")
})
