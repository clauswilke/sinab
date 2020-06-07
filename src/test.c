#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>


SEXP test_(SEXP x_, SEXP y_) {
  double x = Rf_asReal(x_);
  double y = Rf_asReal(y_);
  
  double sum = x + y;
  
  return Rf_ScalarReal(sum);
}

/* Move eventually to grdtext-init.c */

#include <R_ext/Rdynload.h>

void R_init_gridtext(DllInfo *info) {
  R_RegisterCCallable("grdtext", "test", (DL_FUNC) &test_);
}