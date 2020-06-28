#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP C_md_to_html(SEXP str) {
  R_xlen_t n = xlength(str);
  SEXP out;
  PROTECT(out = allocVector(STRSXP, n));
  
  for (R_xlen_t i = 0; i < n; i++) {
    char *s = mdl_md_to_html(Rf_translateCharUTF8(STRING_ELT(str, i)));
    SEXP rs = PROTECT(Rf_mkCharCE(s, CE_UTF8)); 
    mdl_free_cstring(s); /* make sure the raw string we were given is properly deallocated */
    SET_STRING_ELT(out, i, rs);
    UNPROTECT(1);
  }
  UNPROTECT(1);
  
  return out;
}
