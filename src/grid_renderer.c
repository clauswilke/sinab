#include <R.h>
#include <Rinternals.h>

typedef struct GRO {
  SEXP grobs;
  R_xlen_t size;
  R_xlen_t capacity;
} GR_Object;

/* Create a grid renderer object. Must be deleted with gr_release().*/
GR_Object* gr_create() {
  GR_Object* gro = (GR_Object*) Calloc(1, GR_Object);
  gro->capacity = 5;
  
  gro->grobs = PROTECT(allocVector(VECSXP, gro->capacity));
  R_PreserveObject(gro->grobs);
  UNPROTECT(1);
  gro->size = 0;
  
  return gro;
}

/* Consume the grid renderer object and return a vector of grobs.*/
SEXP gr_release(GR_Object* gro) {
  /* In general, the capacity is larger than the current size
   * of the list, so we copy the relevant part into a new list.
   */
  SEXP grobs_old = gro->grobs;
  SEXP grobs_new = PROTECT(allocVector(VECSXP, gro->size));
  
  for (R_xlen_t i = 0; i < gro->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  Free(gro);
  
  /* set class to "gList" */
  setAttrib(grobs_new, R_ClassSymbol, mkString("gList"));
    
  UNPROTECT(1);
  
  return grobs_new;
}

/* Grow the capacity of the list of grobs. Internal. */
void gr_grow_capacity(GR_Object* gro) {
  /* Whenever we're running out of space, we grow the
   * capacity by doubling the grobs vector size
   */
  R_xlen_t cap_new = 2*gro->capacity;
  SEXP grobs_old = gro->grobs;
  SEXP grobs_new = PROTECT(allocVector(VECSXP, cap_new));
  
  /* We only need to copy to size, not to capacity */
  for (R_xlen_t i = 0; i < gro->size; i++) {
    SET_VECTOR_ELT(grobs_new, i, VECTOR_ELT(grobs_old, i));
  }
  
  R_ReleaseObject(grobs_old);
  R_PreserveObject(grobs_new);
  gro->grobs = grobs_new;
  gro->capacity = cap_new;
  UNPROTECT(1);
}

/* Add a SEXP to the list of grobs. Internal. */
void gr_add_SEXP(GR_Object* gro, SEXP s) {
  if (gro->size == gro->capacity) {
    gr_grow_capacity(gro);
    /*
    warning("Doubling list capacity. New capacity: %i", gro->capacity);
    */
  }
  
  SET_VECTOR_ELT(gro->grobs, gro->size, s);
  gro->size += 1;
}


SEXP test_(SEXP n_) {
  int n = asInteger(n_);
  
  GR_Object* gro = gr_create();
  
  for (int i = 0; i<n; i++) {
    SEXP s = mkString("test");
    gr_add_SEXP(gro, s);
  }
    
  return gr_release(gro);
}


