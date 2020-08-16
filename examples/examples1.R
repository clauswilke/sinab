library(sinab)
library(grid)

css <-
  '
p { line-height: 1.0; background-color: #eee; font-family: serif;
    padding: 5px; }
.box { background-color: skyblue; }
.brown { color:red; font-family: "Comic Sans MS";}
em { color:green;   background-color: cornsilk;
  vertical-align: super; line-height: 2.0 }
strong { background-color: lightsalmon; white-space: normal; }
strong em { color:blue; font-family: monospace; }
strong .brown { color:brown; font-size: 24px;
                background-color: skyblue; }
p.box { padding: 0px; border: dotted 3px; margin: 10px; }
'

mdtext <- 'The **very quick <span class="brown">brown brown brown brown brown brown</span>
*fox fox fox fox*** jumps *over* the <span style="color:#0000ff80">lazy
dog.</span><br>The quick <span class="brown">brown</span> fox.

<p class="box">
A box with border.
</p>
'

g <- html_grob(mdtext, css = css)
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

g <- html_grob(
  mdtext,
  css = "p {font-family: serif; background-color: #eee; text-align: right;}"
)
grid.newpage()
grid.draw(g)


md_to_html("This is *a* **test**.")


mdtext <- "
Here are a few examples of word-wrapped and non-word-wrapped text. First
regular text pre-formatted:
<pre>
The quick brown
  fox jumps over
    the lazy dog.
</pre>
Now some inline code: `let a = 5; let b = 6; let c = 7;`
It gets wrapped.

We can also write block code. Notice how the long comment runs beyond
the box limits:
```
let a = 5;
let b = 6;
let c = 7; // and a really really long comment
```
"

css <- '
pre { background-color: #eee;
      border-left: 5px solid #888; padding: 6px; }
p   { background-color: #def; margin-top: 16px; margin-bottom: 4px;
      padding: 4px; }
code { border: 1px solid #AAA; 
       padding: 2px; }
pre code { border: none; padding: 0px; }
'

g <- html_grob(mdtext, css = css)
grid.newpage()
grid.draw(g)


