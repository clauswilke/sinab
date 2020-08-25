# visual tests for various CSS properties

context("CSS properties")

## This test suite is not reliable on CRAN or CI
#skip_on_cran()
#skip_on_ci()
# Reference images are not distributed with the package
skip_if(!file.exists(here::here("tests", "png-refs")), "development only")

# set to true to rebuild all references instead of testing them
rebuild <- FALSE

test_that("CSS selectors", {
  css <- '
*            { padding: 4px; }
.class       { color: navy; }
#id          { background-color: skyblue; }
[attribute]  { border: solid 1px navy; }
em           { background-color: lavender; }
span > em    { font-size: 30px; }
'
  text <- '
<p class = "class">The quick <span id = "id">brown fox</span>
jumps over the <span attribute = "value">lazy dog.</span></p><br>
<p id = "id">The quick <em>brown fox</em> jumps over the
<span><em>lazy dog.</em></span></p>
'
  
  expect_img_match("css selectors", draw_html(text, css = css), rebuild = rebuild)
})
