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

#define STRING_BUFFER_SIZE 201 /* 201 is number used by R */

typedef struct {
  char color[STRING_BUFFER_SIZE];        /* Drawing color: lines, text, etc. */
  char fill[STRING_BUFFER_SIZE];         /* Fill color */
  double fontsize;       /* Font size in points (R_GE_gcontext.ps) */
  double lineheight;     /* Line height (in multiples of fontsize) */
  int fontface;          /* Font face:
                          *  plain = 1, bold = 2,
                          *  italic = 3, bold italic = 4 
                          */
  char fontfamily[STRING_BUFFER_SIZE];  /* Font family */
} GR_GContext;


/*
 * Function declarations
 */

/* grid_renderer.c */
extern GR_Object* gr_create();
extern SEXP gr_release(GR_Object*); 
extern void gr_gcontext_defaults(GR_Object*, GR_GContext*);
extern void gr_draw_text(GR_Object*, const char* label, double x, double y, const GR_GContext *);
extern void gr_string_metrics(GR_Object*, const char* label, const GR_GContext *,
                              double *ascent, double *descent, double *width);
extern SEXP test_gr_create_release(SEXP);
extern SEXP test_gr_draw_text();
extern SEXP test_gpar_gcontext();
  

/* r-callbacks.c */
extern SEXP text_grob(SEXP, SEXP, SEXP, SEXP, SEXP, SEXP);
extern SEXP gpar_empty();
extern SEXP gpar_gcontext(const GR_GContext *);
extern SEXP unit_in(SEXP);

#endif
