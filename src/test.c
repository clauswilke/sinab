#include <R.h>
#include <Rinternals.h>

#include "renderer.h"

// Import C headers for rust API
#include "mdlayout/mdlayout.h"

SEXP C_grobs_from_rust() {
  double ymax = rdev_device_height();
  RenderDevice* rdev = rdev_new(ymax);
  test_renderer(rdev);
  return rdev_release(rdev);
}
