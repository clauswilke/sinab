#define R_NO_REMAP
#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP test_rust() {
  test_renderer();
  
  return R_NilValue;
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
