#include <ft2build.h>

#define FT_INCLUDE_ERR_PROTOS
#define FT_ERRORDEF(e, v, s) const int e = v;
#define FT_MODERRDEF(e, v, s) const int FT_Mod_Err_##e = v;

#include FT_FREETYPE_H
#include FT_MODULE_H
#include FT_SFNT_NAMES_H
#include FT_MULTIPLE_MASTERS_H
