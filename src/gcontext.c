#include "renderer.h"

/* GContext is similar to R_GE_gcontext from R_ext/GraphicsEngine.h */

/* create a new graphics context object */
GContext* gcontext_new() {
  GContext* gc = (GContext*) Calloc(1, GContext);

  strcpy(gc->color, "black");
  strcpy(gc->fill, "black");
  strcpy(gc->fontfamily, "");
  gc->fontface = 1;
  gc->fontsize = 12;
  gc->lineheight = 1.2;
  return gc;
}

/* create a new graphics context object via copying */
GContext* gcontext_copy(GContext* source) {
  GContext* gc = (GContext*) Calloc(1, GContext);
  
  strcpy(gc->color, source->color);
  strcpy(gc->fill, source->fill);
  strcpy(gc->fontfamily, source->fontfamily);
  gc->fontface = source->fontface;
  gc->fontsize = source->fontsize;
  gc->lineheight = source->lineheight;
  return gc;
}

/* delete a graphics context object */
void gcontext_delete(GContext* gc) {
  Free(gc);
}
 
/* setters/accessors */
void gcontext_set_color(GContext* gc, const char* color) {
  strcpy(gc->color, color);
}

const char* gcontext_color(GContext* gc) {
  return gc->color;
}

void gcontext_set_fontfamily(GContext* gc, const char* fontfamily) {
  strcpy(gc->fontfamily, fontfamily);
}

const char* gcontext_fontfamily(GContext* gc) {
  return gc->fontfamily;
}
