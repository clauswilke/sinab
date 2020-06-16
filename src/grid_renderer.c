#include "renderer.h"

/* Create a grid renderer object. Must be deleted with gr_release().*/
GR_Object* gr_new() {
  GR_Object* gro = (GR_Object*) Calloc(1, GR_Object);
  gro->capacity = 5;
  
  gro->grobs = PROTECT(allocVector(VECSXP, gro->capacity));
  R_PreserveObject(gro->grobs);
  UNPROTECT(1);
  gro->size = 0;
  
  return gro;
}

/* Consume the grid renderer object and return a vector of grobs.*/
SEXP gr_release(GR_Object* gro) {
  /* In general, the capacity is larger than the current size
   * of the list, so we copy the relevant part into a new list.
   */
  SEXP grobs_old, grobs_new, cl;
  grobs_old = gro->grobs;
  PROTECT(grobs_new = allocVector(VECSXP, gro->size));
  
  for (R_xlen_t i = 0; i < gro->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  Free(gro);
  
  /* set class to "gList" */
  PROTECT(cl = mkString("gList"));
  classgets(grobs_new, cl);
  UNPROTECT(2);

  return grobs_new;
}

/* Grow the capacity of the list of grobs. Internal. */
void gr_grow_capacity(GR_Object* gro) {
  /* Whenever we're running out of space, we grow the
   * capacity by doubling the grobs vector size
   */
  SEXP grobs_old, grobs_new;
  R_xlen_t cap_new = 2*gro->capacity;
  grobs_old = gro->grobs;
  PROTECT(grobs_new = allocVector(VECSXP, cap_new));
  
  /* We only need to copy to size, not to capacity */
  for (R_xlen_t i = 0; i < gro->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  R_PreserveObject(grobs_new);
  gro->grobs = grobs_new;
  gro->capacity = cap_new;
  UNPROTECT(1);
}

/* Add a SEXP to the list of grobs. Internal. */
void gr_add_SEXP(GR_Object* gro, SEXP s) {
  if (gro->size == gro->capacity) {
    gr_grow_capacity(gro);
    /*
    warning("Doubling list capacity. New capacity: %i", gro->capacity);
    */
  }
  
  SET_VECTOR_ELT(gro->grobs, gro->size, s);
  gro->size += 1;
}

void gr_draw_text(GR_Object* gro, const char* label, double x, double y, const GContext *gc) {
  SEXP slabel, sx, sy, sxu, syu, hjust, vjust, gp, grob;
  
  PROTECT(slabel = mkString(label));
  PROTECT(sx = ScalarReal(x));
  PROTECT(sxu = unit_in(sx));
  PROTECT(sy = ScalarReal(y));
  PROTECT(syu = unit_in(sy));
  PROTECT(hjust = ScalarReal(0));
  PROTECT(vjust = ScalarReal(0));
  PROTECT(gp = gpar_gcontext(gc));
  
  PROTECT(grob = text_grob(slabel, sxu, syu, hjust, vjust, gp));
  
  gr_add_SEXP(gro, grob);
  
  UNPROTECT(9);
}

/* Calls GEStrMetric() and returns results in 
 * variables ascent, descent, width. These values are returned
 * in inches.
 */

void gr_string_metrics(GR_Object* gro, const char* label, const GContext *gc,
                       double *ascent, double *descent, double *width) {
  pGEDevDesc dev = GEcurrentDevice();
  
  /* set up R graphics context from grid renderer graphics context */
  R_GE_gcontext R_gc = {
     .cex = 1
  };
  strcpy(R_gc.fontfamily, gc->fontfamily);
  R_gc.fontface = gc->fontface;
  R_gc.ps = gc->fontsize;
    
  double a, d, w; 
  GEStrMetric(label, CE_UTF8, &R_gc,
              &a, &d, &w, dev);
  
  /* Convert from device units to inches */
  GEUnit u = GE_INCHES;
  *width = GEfromDeviceWidth(w, u, dev);
  *ascent = GEfromDeviceWidth(a, u, dev);
  *descent = GEfromDeviceWidth(d, u, dev);
}

  
/* Test routines */

SEXP test_gr_new_release(SEXP n_) {
  int n = asInteger(n_);
  
  GR_Object* gro = gr_new();
  
  for (int i = 0; i<n; i++) {
    SEXP s;
    PROTECT(s = mkString("test"));
    gr_add_SEXP(gro, s);
    UNPROTECT(1);
  }
    
  return gr_release(gro);
}

SEXP test_gr_draw_text() {
  GR_Object* gro = gr_new();
  GContext* gc = gcontext_new();

  double a, d, w_word, w_space, x;
  /* get width of a space */
  gr_string_metrics(gro, " ", gc, &a, &d, &w_space);

  /* draw first word */
  x = 0.5;
  gr_draw_text(gro, "Hello", x, 2, gc);
  
  /* advance x */
  gr_string_metrics(gro, "Hello", gc, &a, &d, &w_word);
  x = x + w_word + w_space;
  
  /* draw second word */
  GContext* gc2 = gcontext_copy(gc);
  gcontext_set_color(gc2, "blue");
  gcontext_set_fontfamily(gc2, "Times");
  gr_draw_text(gro, "World", x, 2, gc2);
  
  /* advance x */
  gr_string_metrics(gro, "World", gc2, &a, &d, &w_word);
  x = x + w_word + w_space;
  
  /* draw remainder */
  gcontext_set_color(gc, "red");
  gr_draw_text(gro, "in red!", x, 2, gc);
  
  /* delete graphics context */
  gcontext_delete(gc);
  gcontext_delete(gc2);
  return gr_release(gro);
}
