#include <R.h>
#include <Rinternals.h>
#include <stdlib.h> // for NULL
#include <R_ext/Rdynload.h>

#include "renderer.h"

/* test.c */
extern SEXP C_grobs_from_rust(); 

/* markdown.c */
extern SEXP C_md_to_html(SEXP);

/* from testthat */
extern SEXP run_testthat_tests();

static const R_CallMethodDef CallEntries[] = {
  {"C_md_to_html", (DL_FUNC) &C_md_to_html, 1},
  {"C_grobs_from_rust", (DL_FUNC) &C_grobs_from_rust, 0},
  {"rdev_string_metrics", (DL_FUNC) &rdev_string_metrics, 0},
  {"test_rdev_new_release", (DL_FUNC) &test_rdev_new_release, 1},
  {"test_rdev_draw_text", (DL_FUNC) &test_rdev_draw_text, 0},
  {"gpar_empty", (DL_FUNC) &gpar_empty, 0},
  {"text_grob", (DL_FUNC) &text_grob, 6},
  {"unit_in", (DL_FUNC) &unit_in, 1},
  {"test_gpar_gcontext", (DL_FUNC) &test_gpar_gcontext, 0},
  {"run_testthat_tests", (DL_FUNC) &run_testthat_tests, 0},
  {NULL, NULL, 0}
};

void R_init_grdtext(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
