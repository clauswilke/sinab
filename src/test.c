#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>


SEXP add_(SEXP x_, SEXP y_) {
  double x = Rf_asReal(x_);
  double y = Rf_asReal(y_);
  
  double sum = x + y;
  
  return Rf_ScalarReal(sum);
}

SEXP named_list_(SEXP x_, SEXP y_) {
  /* Construct named result list from variables containing the results */
  const char *names[] = {"x", "y", ""};                   /* note the null string */
  SEXP res = PROTECT(Rf_mkNamed(VECSXP, names));  /* list of length 2 */
  SET_VECTOR_ELT(res, 0, x_);
  SET_VECTOR_ELT(res, 1, y_);
  UNPROTECT(1);
  return res;
}

/* Move eventually to grdtext-init.c */

#include <R_ext/Rdynload.h>

void R_init_gridtext(DllInfo *info) {
  R_RegisterCCallable("grdtext", "add", (DL_FUNC) &add_);
  R_RegisterCCallable("grdtext", "named_list", (DL_FUNC) &named_list_);
}
