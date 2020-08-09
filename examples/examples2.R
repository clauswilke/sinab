# cases with rendering errors. in some (but not all) cases filled
# boxes are too wide. not yet clear what the pattern is.

library(grdtext)
library(grid)

css <-
'
.brown { color:red; font-family: "Comic Sans MS";}
em { color:green;   background-color: cornsilk;}
strong em { color:blue; font-family: monospace; }
strong .brown { 
  color:brown; font-size: 24px; /*background-color: cornsilk;*/
}'

mdtext <-
  'The **very quick <span class="brown">brown</span>
*fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)

css <-
  '
.brown { color:red; font-family: "Comic Sans MS";}
em { color:green;   background-color: skyblue;}
strong em { color:blue; font-family: monospace; }
strong .brown { 
  color:brown; font-size: 24px; background-color: cornsilk;
}'

mdtext <-
  'The **very quick <span class="brown">brown</span>
*fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)


css <-
  '
strong { background-color: skyblue; }
.brown { color:red; font-family: "Comic Sans MS";}
em { color:green; }
strong em { color:blue; font-family: monospace; }
strong .brown { 
  color:brown; font-size: 24px; background-color: cornsilk;
}'

mdtext <-
  'The **very quick <span class="brown">brown</span>
*fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext, css)
grid.newpage()
grid.draw(g)

mdtext2 <-
  'The **very quick <span class="brown">brown</span> *fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.'

g <- render_markdown(mdtext2, css)
grid.newpage()
grid.draw(g)

md_to_html(mdtext)
md_to_html(mdtext2)
