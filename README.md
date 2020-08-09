
<!-- README.md is generated from README.Rmd. Please edit that file -->

# SINAB is not a browser

A basic html rendering engine for R, written in Rust. The purpose is not
to write a browser, but rather to provide the ability to render simple,
static html documents to an R graphics device.

An initial rendering engine has been implemented, though it is for
demonstration purposes only at this time:

``` r
library(sinab)
library(grid)

css <-
'
p { line-height: 1.5; background-color: #eee; }
.box { background-color: skyblue; }
.brown { color:red; font-family: "Comic Sans MS";}
em { color:green;   background-color: cornsilk;}
strong { background-color: lightsalmon; }
strong em { color:blue; font-family: monospace; }
strong .brown { 
  color:brown; font-size: 24px; background-color: skyblue;
}'

mdtext <-
'The **very quick <span class="brown">brown brown brown brown brown brown brown brown</span>
*fox fox fox fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)
```

<img src="man/figures/README-unnamed-chunk-2-1.png" width="100%" />

Simple markdown-to-html conversion is also implemented:

``` r
md_to_html("This is *a* **test**.")
#> [1] "<p>This is <em>a</em> <strong>test</strong>.</p>\n"
```
