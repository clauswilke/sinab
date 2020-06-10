#include <R.h>
#include <Rinternals.h>
#include <stdlib.h> // for NULL
#include <R_ext/Rdynload.h>

/* test.c */
extern SEXP add_(SEXP, SEXP);
extern SEXP named_list_(SEXP, SEXP);
extern SEXP random_unif_(SEXP, SEXP, SEXP);

/* grid_renderer.c */
extern SEXP test_(SEXP);

/* r-callbacks.c */
extern SEXP gpar_empty();
extern SEXP text_grob(SEXP, SEXP, SEXP, SEXP, SEXP, SEXP);

static const R_CallMethodDef CallEntries[] = {
  {"add_", (DL_FUNC) &add_, 2},
  {"named_list_", (DL_FUNC) &named_list_, 2},
  {"random_unif_", (DL_FUNC) &random_unif_, 3},
  {"test_", (DL_FUNC) &test_, 1},
  {"gpar_empty", (DL_FUNC) &gpar_empty, 0},
  {"text_grob", (DL_FUNC) &text_grob, 6},
  {NULL, NULL, 0}
};

void R_init_grdtext(DllInfo *dll) {
  R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
  R_useDynamicSymbols(dll, FALSE);
}
