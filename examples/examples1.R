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
'The **very quick <span class="brown">brown brown brown brown brown brown</span>
*fox fox fox fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)


mdtext <- "Lorem ipsum dolor sit amet, consectetur adipiscing
elit, sed do eiusmod tempor incididunt ut labore et dolore magna
aliqua. Ut enim ad minim veniam, quis nostrud exercitation
ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis
aute irure dolor in reprehenderit in voluptate velit esse cillum
dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat
non proident, sunt in culpa qui officia deserunt mollit anim id
est laborum."

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)



words <- rep("test", 13)

microbenchmark::microbenchmark(
  render_markdown(mdtext, css),
  lapply(words, textGrob )
)


microbenchmark::microbenchmark(grid.draw(g))

md_to_html("This is *a* **test**.")

css <- "
.example > * {
  background-color: rebeccapurple;
  color: white;
}"

mdtext <- '
  <div class="example">
I am wrapped in an anonymous box 
<p>I am in the paragraph</p>
I am wrapped in an anonymous box.
</div>
'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)
