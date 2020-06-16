#include <testthat.h>
#include <string.h>

#include "renderer.h"

context("graphics context") {
  test_that("default values are correct") {
    GContext* gc = gcontext_new();
    
    expect_true(strcmp(gc->color, "black") == 0);
    expect_true(strcmp(gc->fill, "black") == 0);
    expect_true(strcmp(gc->fontfamily, "") == 0);
    expect_true(gc->fontface == 1);
    expect_true(gc->fontsize == 12);
    expect_true(gc->lineheight == 1.2);
    
    gcontext_delete(gc);
  }
  
  test_that("setters/getters work") {
    GContext* gc = gcontext_new();
    
    gcontext_set_color(gc, "blue");
    expect_true(strcmp(gcontext_color(gc), "blue") == 0);
    /*
    gcontext_set_fill(gc, color);
    expect_true(strcmp(gcontext_fill(gc), color));
    */
    gcontext_set_fontfamily(gc, "Times New Roman");
    expect_true(strcmp(gcontext_color(gc), "Times New Roman"));

    gcontext_delete(gc);
  }
  
}