#ifdef __cplusplus
extern "C" {
#endif

  extern char* mdl_md_to_html(const char*);
  extern void mdl_free_cstring(char *); /* call to deallocate char* returned from rust */

  extern void mdl_test_renderer(void *render_device, const char* text, const char* css);
    
#ifdef __cplusplus
}
#endif
