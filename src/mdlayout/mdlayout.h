#ifdef __cplusplus
extern "C" {
#endif

    extern char * string_from_rust();
    extern void free_rust_cstring(char *); /* call to deallocate char* returned from rust */

#ifdef __cplusplus
}
#endif