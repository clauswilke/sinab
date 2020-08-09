#include <R.h>
#include <Rinternals.h>

#include "renderer.h"

// Import C headers for rust API
#include "sinab/sinab.h"

SEXP C_render_markdown(SEXP text_str, SEXP css_str) {
  double ymax = rdev_device_height();
  RenderDevice* rdev = rdev_new(ymax);
  
  if (xlength(text_str) >= 1 && xlength(css_str) >= 1 ) {
    mdl_test_renderer(
      rdev,
      Rf_translateCharUTF8(STRING_ELT(text_str, 0)),
      Rf_translateCharUTF8(STRING_ELT(css_str, 0))
    );
  }
  
  return rdev_release(rdev);
}
