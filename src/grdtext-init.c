#include <R_ext/Rdynload.h>
#include <R.h>
#include <Rinternals.h>

SEXP add_(SEXP x_, SEXP y_);
SEXP named_list_(SEXP x_, SEXP y_);
SEXP random_unif_(SEXP n, SEXP min, SEXP max);
SEXP test_(SEXP n_);

void R_init_gridtext(DllInfo *info) {
  R_RegisterCCallable("grdtext", "add", (DL_FUNC) &add_);
  R_RegisterCCallable("grdtext", "named_list", (DL_FUNC) &named_list_);
  R_RegisterCCallable("grdtext", "random_unif", (DL_FUNC) &random_unif_);
  R_RegisterCCallable("grdtext", "test", (DL_FUNC) &test_);
}
