library(sinab)
library(grid)

mdtext <-
'The **very quick <span class="brown">brown brown brown brown brown
brown</span> *fox fox fox fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

css <-
'
p      { font-family: serif; line-height: 1.2; background-color: #eee; }
.box   { background-color: skyblue; }
.brown { color:red; font-family: "Comic Sans MS";}
em     { color:green; background-color: cornsilk;
         vertical-align: super; line-height: 2.0; }
strong { background-color: lightsalmon; }
strong em { color:blue; font-family: monospace; }
strong .brown { 
  color:brown; font-size: 24px; background-color: skyblue;
}'

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



words <- rep("test", 60)

ragg::agg_capture()
microbenchmark::microbenchmark(
  render_markdown(mdtext, css),
  lapply(words, textGrob )
)
dev.off()

md_to_html("This is *a* **test**.")


mdtext <- "
<pre>
The quick brown
  fox jumps over
    the lazy dog.
</pre>
Some inline code: `let a = 5; let b = 6; let c = 7;`
<br>
And some block code:
```
let a = 5;
let b = 6;
let c = 7; // and a really really long comment
```
"

css <- "pre {background-color: #eee; margin-top: 5px;}
p {background-color: #def; margin-top: 5px;}"

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)

