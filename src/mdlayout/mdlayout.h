#ifdef __cplusplus
extern "C" {
#endif

    extern char* md_to_html(const char*);
    extern void free_rust_cstring(char *); /* call to deallocate char* returned from rust */

    extern void test_renderer();
    
#ifdef __cplusplus
}
#endif