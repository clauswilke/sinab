#include "renderer.h"

/* Create a grid render device. Must be deleted with rdev_release().*/
RenderDevice* rdev_new(double y0) {
  RenderDevice* rdev = (RenderDevice*) Calloc(1, RenderDevice);
  rdev->capacity = 5;
  
  rdev->grobs = PROTECT(allocVector(VECSXP, rdev->capacity));
  R_PreserveObject(rdev->grobs);
  UNPROTECT(1);
  rdev->size = 0;
  
  rdev->y0 = y0;

  /* initialize bounding box; initially it is not set. */  
  rdev->bb_ymin = 0;
  rdev->bb_ymax = 0;
  rdev->bb_xmin = 0;
  rdev->bb_xmax = 0;
  rdev->bb_set = false;
  
  return rdev;
}

/* Consume the grid renderer object and return a vector of grobs.*/
SEXP rdev_release(RenderDevice* rdev) {
  /* In general, the capacity is larger than the current size
   * of the list, so we copy the relevant part into a new list.
   */
  SEXP grobs_old, grobs_new, cl;
  grobs_old = rdev->grobs;
  grobs_new = PROTECT(allocVector(VECSXP, rdev->size));
  
  for (R_xlen_t i = 0; i < rdev->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  
  /* if bounding box is set, record in an attribute */
  if (rdev->bb_set) {
    SEXP bb_xmin, bb_ymin, bb_xmax, bb_ymax, bbox;
    bb_xmin = PROTECT(ScalarReal(rdev->bb_xmin));
    bb_ymin = PROTECT(ScalarReal(rdev->bb_ymin));
    bb_xmax = PROTECT(ScalarReal(rdev->bb_xmax));
    bb_ymax = PROTECT(ScalarReal(rdev->bb_ymax));
    
    const char *names[] = {"xmin", "ymin", "xmax", "ymax", ""};
    bbox = PROTECT(Rf_mkNamed(VECSXP, names));
    SET_VECTOR_ELT(bbox, 0, bb_xmin);
    SET_VECTOR_ELT(bbox, 1, bb_ymin);
    SET_VECTOR_ELT(bbox, 2, bb_xmax);
    SET_VECTOR_ELT(bbox, 3, bb_ymax);

    setAttrib(grobs_new, install("bbox"), bbox);
    UNPROTECT(5);
  }
  
  /* set class to "gList" */
  cl = PROTECT(mkString("gList"));
  classgets(grobs_new, cl);

  /* release rdev object, we're done */
  Free(rdev);
  
  /* unprotect remaining objects */
  UNPROTECT(2);
  
  return grobs_new;
}

/* Grow the capacity of the list of grobs. Internal. */
void rdev_grow_capacity(RenderDevice* rdev) {
  /* Whenever we're running out of space, we grow the
   * capacity by doubling the grobs vector size
   */
  SEXP grobs_old, grobs_new;
  R_xlen_t cap_new = 2*rdev->capacity;
  grobs_old = rdev->grobs;
  PROTECT(grobs_new = allocVector(VECSXP, cap_new));
  
  /* We only need to copy to size, not to capacity */
  for (R_xlen_t i = 0; i < rdev->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  R_PreserveObject(grobs_new);
  rdev->grobs = grobs_new;
  rdev->capacity = cap_new;
  UNPROTECT(1);
}

/* Add a SEXP to the list of grobs. Internal. */
void rdev_add_SEXP(RenderDevice* rdev, SEXP s) {
  if (rdev->size == rdev->capacity) {
    rdev_grow_capacity(rdev);
    /*
    warning("Doubling list capacity. New capacity: %i", rdev->capacity);
    */
  }
  
  SET_VECTOR_ELT(rdev->grobs, rdev->size, s);
  rdev->size += 1;
}

void rdev_draw_text(RenderDevice* rdev, const char* label, double x, double y, const GContext *gc) {
  SEXP slabel, sx, sy, sxu, syu, hjust, vjust, gp, grob;
  
  PROTECT(slabel = mkString(label));
  PROTECT(sx = ScalarReal(x));
  PROTECT(sxu = unit_in(sx));
  PROTECT(sy = ScalarReal(rdev->y0 - y)); /* invert y coordinate system */
  PROTECT(syu = unit_in(sy));
  PROTECT(hjust = ScalarReal(0));
  PROTECT(vjust = ScalarReal(0));
  PROTECT(gp = gpar_gcontext(gc));
  
  PROTECT(grob = text_grob(slabel, sxu, syu, hjust, vjust, gp));
  
  rdev_add_SEXP(rdev, grob);
  
  UNPROTECT(9);
}

// x, y: top left corner
void rdev_draw_rect(RenderDevice* rdev, double x, double y, double width, double height, const GContext *gc) {
  SEXP sx, sy, sxu, syu, sw, sh, swu, shu, hjust, vjust, gp, grob;
  
  PROTECT(sx = ScalarReal(x));
  PROTECT(sxu = unit_in(sx));
  PROTECT(sy = ScalarReal(rdev->y0 - y)); /* invert y coordinate system */
  PROTECT(syu = unit_in(sy));
  PROTECT(sw = ScalarReal(width));
  PROTECT(swu = unit_in(sw));
  PROTECT(sh = ScalarReal(height));
  PROTECT(shu = unit_in(sh));
  PROTECT(hjust = ScalarReal(0));
  PROTECT(vjust = ScalarReal(1));
  PROTECT(gp = gpar_gcontext(gc));
    
  PROTECT(grob = rect_grob(sxu, syu, swu, shu, hjust, vjust, gp));
    
  rdev_add_SEXP(rdev, grob);
    
  UNPROTECT(12);
}

// n: number of points specified by *x, *y
void rdev_draw_line(RenderDevice* rdev, const double *x, const double *y, unsigned int n, const GContext *gc) {
  SEXP sx, sy, sxu, syu, gp, grob;
  
  PROTECT(sx = allocVector(REALSXP, n));
  PROTECT(sy = allocVector(REALSXP, n));
  
  // copy coordinates into R vectors, inverting y coordinate system
  double y0 = rdev->y0;
  double* px = REAL(sx);
  double* py = REAL(sy);
  for (int i = 0; i < n; ++i) {
    px[i] = x[i];
    py[i] = y0 - y[i];
  }
  
  PROTECT(sxu = unit_in(sx));
  PROTECT(syu = unit_in(sy));
  PROTECT(gp = gpar_gcontext(gc));
    
  PROTECT(grob = lines_grob(sxu, syu, gp));
    
  rdev_add_SEXP(rdev, grob);
    
  UNPROTECT(6);
}


/* Bounding boxes are recorded manually, rather than automatically
 * upon drawing rectangles or lines or text, so that the client code
 * has full control over what counts towards the bounding box (e.g.
 * this allows margins to count towards bounding boxes, or margins
 * to be negative).
 */

void rdev_record_bbox(RenderDevice* rdev, double xmin, double ymin, double xmax, double ymax) {
  /* invert y axis */
  ymin = rdev->y0 - ymin;
  ymax = rdev->y0 - ymax;
  
  /* make sure min values are always smaller than max values */
  if (xmin > xmax) {
    double tmp = xmin;
    xmin = xmax;
    xmax = tmp;
  }
  if (ymin > ymax) {
    double tmp = ymin;
    ymin = ymax;
    ymax = tmp;
  }
  
  if (rdev->bb_set) {
    /* bounding box exists already, enlarge if necessary */
    if (xmin < rdev->bb_xmin) rdev->bb_xmin = xmin;
    if (ymin < rdev->bb_ymin) rdev->bb_ymin = ymin;
    if (xmax > rdev->bb_xmax) rdev->bb_xmax = xmax;
    if (ymax > rdev->bb_ymax) rdev->bb_ymax = ymax;
  } else {
    rdev->bb_xmin = xmin;
    rdev->bb_ymin = ymin;
    rdev->bb_xmax = xmax;
    rdev->bb_ymax = ymax;
    rdev->bb_set = true;
  }
}

/* Calls GEStrMetric() and returns results in 
 * variables ascent, descent, width. These values are returned
 * in inches.
 */

void rdev_string_metrics(const char* label, const GContext *gc,
                         double *ascent, double *descent, double *width) {
  pGEDevDesc dev = GEcurrentDevice();
  
  /* set up R graphics context from grid renderer graphics context */
  R_GE_gcontext R_gc = {
     .cex = 1
  };
  strcpy(R_gc.fontfamily, gc->fontfamily);
  R_gc.fontface = gc->fontface;
  R_gc.ps = gc->fontsize;
    
  double a, d, w; 
  GEStrMetric(label, CE_UTF8, &R_gc,
              &a, &d, &w, dev);
  
  /* Convert from device units to inches */
  GEUnit u = GE_INCHES;
  *width = GEfromDeviceWidth(w, u, dev);
  *ascent = GEfromDeviceHeight(a, u, dev);
  *descent = GEfromDeviceHeight(d, u, dev);
}

/* returns the height of the current R graphics device, in inches */
double rdev_device_height() {
  pGEDevDesc dev = GEcurrentDevice();
  return fromDeviceY(toDeviceY(1, GE_NDC, dev), GE_INCHES, dev);
}

/* Test routines */

SEXP test_rdev_new_release(SEXP n_) {
  int n = asInteger(n_);
  
  RenderDevice* rdev = rdev_new(0);
  
  for (int i = 0; i<n; i++) {
    SEXP s;
    PROTECT(s = mkString("test"));
    rdev_add_SEXP(rdev, s);
    UNPROTECT(1);
  }
    
  return rdev_release(rdev);
}
