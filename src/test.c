#include <R.h>
#include <Rinternals.h>

#include "renderer.h"

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP C_test_rust() {
  GR_Object* gro = gr_new();
  test_renderer(gro);
  return gr_release(gro);
}
