#define FT_INCLUDE_ERR_PROTOS
#define FT_ERRORDEF(e, v, s) const int e = v;
#define FT_MODERRDEF(e, v, s) const int FT_Mod_Err_##e = v;

#include <freetype/freetype.h>
#include <freetype/ftmm.h>
#include <freetype/ftmodapi.h>
#include <freetype/ftsnames.h>
