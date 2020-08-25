write_png <- function(filename, p) {
  # store current device
  cur_dev <- grDevices::dev.cur()
  
  # open png output device  
  ragg::agg_png(
    filename = filename,
    width = 480,
    height = 480,
    units = "px",
    background = "white",
    res = 150
  )
  new_dev <- grDevices::dev.cur()   # store new device
  
  # make sure we always clean up properly, even if something causes an error
  on.exit({
    grDevices::dev.off(new_dev)
    if (cur_dev > 1) grDevices::dev.set(cur_dev) # only set cur device if not null device
  })
  
  eval(p)
  invisible()
}

standardize_name <- function(name) {
  # convert to lower case, replace all non-alphanum
  # characters with "-".
  name <- gsub("[^a-z0-9]", "-", tolower(name))
  # remove multiple "-"s in a row
  gsub("-+-", "-", name)
}

create_reference <- function(name, expr) {
  name <- paste0(standardize_name(name), ".png")
  
  # create directory if doesn't exist already
  dir <- here::here("tests", "png-refs")
  dir.create(dir, showWarnings = FALSE)
  
  fname <- file.path(dir, name)
  write_png(fname, expr)
}

diff_to_reference <- function(name, expr, fuzz = 0) {
  name_std <- standardize_name(name)
  fname <- paste0(name_std, ".png")
  dir <- here::here("tests", "png-refs")
  reference_file <- file.path(dir, fname)
  
  # if reference doesn't exist, can't compare
  if (!file.exists(reference_file)) {
    warning(
      paste0('reference image not found for case "', name, '"'),
      call. = FALSE
    )
    return(invisible())
  }
  
  
  test_file <- tempfile(pattern = name_std, fileext = ".png")
  write_png(test_file, expr)
  
  magick::image_compare(
    magick::image_read(test_file),
    magick::image_read(reference_file),
    metric = "AE",
    fuzz = fuzz
  )
}

expect_img_match <- function(name, expr, fuzz = 0, show = FALSE, rebuild = FALSE) {
  if (isTRUE(rebuild)) {
    create_reference(name, expr)
    return()
  }
  
  diff <- diff_to_reference(name, expr, fuzz)
  
  if (is.null(diff)) {
    warning(
      paste0('reference image not found for case "', name, '"'),
      call. = FALSE
    )
    return(invisible())
  }

  error <- attributes(diff)$distortion
  if (error > 0) {
    if (isTRUE(show)) {
      print(diff)
    }
    testthat::expect(
      FALSE,
      paste0(
        "reference image doesn't match for \"", name,
        '"; deviation metric: ', error
      )
    )
  } else {
    testthat::expect(TRUE, "reference image matches")
  }
}
