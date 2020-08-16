#ifdef __cplusplus
extern "C" {
#endif

  extern char* sinab_md_to_html(const char*);
  extern void sinab_free_cstring(char *); /* call to deallocate char* returned from rust */

  extern void sinab_test_renderer(void *render_device, const char* text, const char* css,
                                double width_px, double height_px);
    
#ifdef __cplusplus
}
#endif
