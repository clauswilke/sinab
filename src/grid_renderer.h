#ifndef GRID_RENDERER_H
#define GRID_RENDERER_H 

#include <R.h>
#include <Rinternals.h>
#include <R_ext/GraphicsEngine.h>

/* 
 * Grid renderer object.
 */

typedef struct {
  SEXP grobs;
  R_xlen_t size;
  R_xlen_t capacity;
} GR_Object;

/* 
 * Grid renderer graphics context. Similar to R_GE_gcontext
 * from R_ext/GraphicsEngine.h.
 */

typedef struct {
  char color[40];        /* Drawing color: lines, text, etc. */
  char fill[40];         /* Fill color */
  double fontsize;       /* Font size in points (R_GE_gcontext.ps) */
  double lineheight;     /* Line height (in multiples of fontsize) */
  int fontface;          /* Font face:
                          *  plain = 1, bold = 2,
                          *  italic = 3, bold italic = 4 
                          */
  char fontfamily[201];  /* Font family */
} GR_GContext;


/*
 * Function declarations
 */

/* grid_renderer.c */
extern SEXP gr_string_metrics();
extern SEXP test_gr_create_release(SEXP);
extern SEXP test_gpar_gcontext();

/* r-callbacks.c */
extern SEXP text_grob(SEXP, SEXP, SEXP, SEXP, SEXP, SEXP);
extern SEXP gpar_empty();
extern SEXP gpar_gcontext(GR_GContext *);


#endif
