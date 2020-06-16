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
  
  test_that("setters/getters and copying work") {
    GContext* gc = gcontext_new();
    
    gcontext_set_color(gc, "blue");
    expect_true(strcmp(gcontext_color(gc), "blue") == 0);
    
    gcontext_set_fill(gc, "red");
    expect_true(strcmp(gcontext_fill(gc), "red") == 0);

    gcontext_set_fontfamily(gc, "Times New Roman");
    expect_true(strcmp(gcontext_fontfamily(gc), "Times New Roman") == 0);

    gcontext_set_fontface(gc, 3);
    expect_true(gcontext_fontface(gc) == 3);
    
    GContext* gc2 = gcontext_copy(gc);
    expect_true(strcmp(gcontext_color(gc2), "blue") == 0);
    expect_true(strcmp(gcontext_fill(gc2), "red") == 0);
    expect_true(strcmp(gcontext_fontfamily(gc2), "Times New Roman") == 0);
    expect_true(gcontext_fontface(gc2) == 3);
    
    gcontext_delete(gc);
    gcontext_delete(gc2);
  }
  
}