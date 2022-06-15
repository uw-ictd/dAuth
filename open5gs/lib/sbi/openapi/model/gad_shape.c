
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "gad_shape.h"

OpenAPI_gad_shape_t *OpenAPI_gad_shape_create(
    OpenAPI_supported_gad_shapes_t *shape
)
{
    OpenAPI_gad_shape_t *gad_shape_local_var = ogs_malloc(sizeof(OpenAPI_gad_shape_t));
    ogs_assert(gad_shape_local_var);

    gad_shape_local_var->shape = shape;

    return gad_shape_local_var;
}

void OpenAPI_gad_shape_free(OpenAPI_gad_shape_t *gad_shape)
{
    if (NULL == gad_shape) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_supported_gad_shapes_free(gad_shape->shape);
    ogs_free(gad_shape);
}

cJSON *OpenAPI_gad_shape_convertToJSON(OpenAPI_gad_shape_t *gad_shape)
{
    cJSON *item = NULL;

    if (gad_shape == NULL) {
        ogs_error("OpenAPI_gad_shape_convertToJSON() failed [GADShape]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *shape_local_JSON = OpenAPI_supported_gad_shapes_convertToJSON(gad_shape->shape);
    if (shape_local_JSON == NULL) {
        ogs_error("OpenAPI_gad_shape_convertToJSON() failed [shape]");
        goto end;
    }
    cJSON_AddItemToObject(item, "shape", shape_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_gad_shape_convertToJSON() failed [shape]");
        goto end;
    }

end:
    return item;
}

OpenAPI_gad_shape_t *OpenAPI_gad_shape_parseFromJSON(cJSON *gad_shapeJSON)
{
    OpenAPI_gad_shape_t *gad_shape_local_var = NULL;
    cJSON *shape = cJSON_GetObjectItemCaseSensitive(gad_shapeJSON, "shape");
    if (!shape) {
        ogs_error("OpenAPI_gad_shape_parseFromJSON() failed [shape]");
        goto end;
    }

    OpenAPI_supported_gad_shapes_t *shape_local_nonprim = NULL;
    shape_local_nonprim = OpenAPI_supported_gad_shapes_parseFromJSON(shape);

    gad_shape_local_var = OpenAPI_gad_shape_create (
        shape_local_nonprim
    );

    return gad_shape_local_var;
end:
    return NULL;
}

OpenAPI_gad_shape_t *OpenAPI_gad_shape_copy(OpenAPI_gad_shape_t *dst, OpenAPI_gad_shape_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_gad_shape_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_gad_shape_convertToJSON() failed");
        return NULL;
    }

    content = cJSON_Print(item);
    cJSON_Delete(item);

    if (!content) {
        ogs_error("cJSON_Print() failed");
        return NULL;
    }

    item = cJSON_Parse(content);
    ogs_free(content);
    if (!item) {
        ogs_error("cJSON_Parse() failed");
        return NULL;
    }

    OpenAPI_gad_shape_free(dst);
    dst = OpenAPI_gad_shape_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

