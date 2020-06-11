#include <R.h>
#include <Rinternals.h>
#include <stdlib.h> // for NULL
#include <R_ext/Rdynload.h>

#include "grid_renderer.h"

/* test.c */
extern SEXP add_(SEXP, SEXP);
extern SEXP named_list_(SEXP, SEXP);
extern SEXP random_unif_(SEXP, SEXP, SEXP);

static const R_CallMethodDef CallEntries[] = {
  {"add_", (DL_FUNC) &add_, 2},
  {"named_list_", (DL_FUNC) &named_list_, 2},
  {"random_unif_", (DL_FUNC) &random_unif_, 3},
  {"gr_string_metrics", (DL_FUNC) &gr_string_metrics, 0},
  {"test_gr_create_release", (DL_FUNC) &test_gr_create_release, 1},
  {"test_gr_draw_text", (DL_FUNC) &test_gr_draw_text, 0},
  {"gpar_empty", (DL_FUNC) &gpar_empty, 0},
  {"text_grob", (DL_FUNC) &text_grob, 6},
  {"unit_in", (DL_FUNC) &unit_in, 1},
  {"test_gpar_gcontext", (DL_FUNC) &test_gpar_gcontext, 0},
  {NULL, NULL, 0}
};

void R_init_grdtext(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
