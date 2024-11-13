#include <ft2build.h>

#define FT_INCLUDE_ERR_PROTOS
#define FT_ERRORDEF( e, v, s )  e = v,
#define FT_ERROR_START_LIST     enum FT_ERR : FT_Error {
#define FT_ERROR_END_LIST       FT_ERR_CAT( FT_ERR_PREFIX, Max ) };

#include FT_FREETYPE_H
#include FT_MODULE_H
#include FT_SFNT_NAMES_H
#include FT_TRUETYPE_IDS_H
#include FT_MULTIPLE_MASTERS_H
