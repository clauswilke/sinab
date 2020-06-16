#include <R.h>
#include <Rinternals.h>

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP test_rust() {
  test_renderer();
  
  return R_NilValue;
}
