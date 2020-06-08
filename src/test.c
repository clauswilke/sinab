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



SEXP random_unif_(SEXP n, SEXP min, SEXP max) {
  SEXP stats;
  PROTECT(
    stats = Rf_eval(
      Rf_lang2(
        Rf_install("getNamespace"),
        Rf_ScalarString(Rf_mkChar("stats"))
      ),
      R_GlobalEnv
    )
  );
  
  SEXP r_call;
  PROTECT(r_call = Rf_allocVector(LANGSXP, 4)); 
  SETCAR(
    r_call,
    Rf_findFun(Rf_install("runif"), stats)
  );  
  
  SETCADR(r_call, n);
  SET_TAG(CDR(r_call), Rf_install("n"));
  
  SETCADDR(r_call, min);
  SET_TAG(CDDR(r_call), Rf_install("min"));
  
  SETCADDDR(r_call, max);
  SET_TAG(CDR(CDDR(r_call)), Rf_install("max"));
  
  SEXP randoms;
  PROTECT(
    randoms = Rf_eval(r_call, stats)
  );  
  
  UNPROTECT(3);
  return(randoms);
}

