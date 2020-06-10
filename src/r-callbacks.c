#include <R.h>
#include <Rinternals.h>


SEXP get_namespace(const char* namesp) {
  SEXP out, call, namesp_sexp;
  
  PROTECT(namesp_sexp = mkString(namesp));
  PROTECT(call = lang2(install("getNamespace"), namesp_sexp));
  out = eval(call, R_GlobalEnv);
  
  UNPROTECT(2);
  return out;
}

/* Call grid::gpar() without any arguments */
SEXP gpar_empty() {
  SEXP out, grid, fun, call;
  
  PROTECT(grid = get_namespace("grid"));
  PROTECT(fun = findFun(install("gpar"), grid));
  PROTECT(call = lang1(fun));
  out = eval(call, R_GlobalEnv);
  
  UNPROTECT(3);
  return out;
}

/* Call grid::textGrob() */
SEXP text_grob(SEXP label, SEXP x, SEXP y, SEXP hjust, SEXP vjust, SEXP gp) {
  SEXP out, grid, fun, call, s;
  
  PROTECT(grid = get_namespace("grid"));
  PROTECT(fun = findFun(install("textGrob"), grid));
  
  PROTECT(call = allocVector(LANGSXP, 7)); 
  SETCAR(call, fun);  
  
  s = CDR(call);
  SETCAR(s, label);
  SET_TAG(s, install("label"));
  
  s = CDR(s);
  SETCAR(s, x);
  SET_TAG(s, install("x"));
  
  s = CDR(s);
  SETCAR(s, y);
  SET_TAG(s, install("y"));

  s = CDR(s);
  SETCAR(s, hjust);
  SET_TAG(s, install("hjust"));
  
  s = CDR(s);
  SETCAR(s, vjust);
  SET_TAG(s, install("vjust"));

  s = CDR(s);
  SETCAR(s, gp);
  SET_TAG(s, install("gp"));
  
  out = eval(call, R_GlobalEnv);
  
  UNPROTECT(3);
  return out;
}

