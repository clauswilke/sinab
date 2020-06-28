#include <R.h>
#include <Rinternals.h>

#include "renderer.h"

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP C_render_markdown(SEXP str) {
  double ymax = rdev_device_height();
  RenderDevice* rdev = rdev_new(ymax);
  
  if (xlength(str) >= 1) {
    mdl_test_renderer(rdev, Rf_translateCharUTF8(STRING_ELT(str, 0)));
  }
  
  return rdev_release(rdev);
}
