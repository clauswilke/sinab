#include <R.h>
#include <Rinternals.h>

#include "renderer.h"

// Import C headers for rust API
#include "sinab/sinab.h"

SEXP C_render_markdown(SEXP text_str, SEXP css_str, SEXP width_in, SEXP height_in) {
  /*
  double ymax = rdev_device_height();
  RenderDevice* rdev = rdev_new(ymax);
   */
  RenderDevice* rdev = rdev_new(0);
  
  double* w = REAL(width_in);
  double* h = REAL(height_in);
  
  if (xlength(text_str) >= 1 && xlength(css_str) >= 1 ) {
    sinab_test_renderer(
      rdev,
      Rf_translateCharUTF8(STRING_ELT(text_str, 0)),
      Rf_translateCharUTF8(STRING_ELT(css_str, 0)),
      *w * 96, *h * 96
    );
  }
  
  return rdev_release(rdev);
}
