
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "geographic_area.h"

OpenAPI_geographic_area_t *OpenAPI_geographic_area_create(
    OpenAPI_supported_gad_shapes_t *shape,
    OpenAPI_geographical_coordinates_t *point,
    float uncertainty,
    OpenAPI_uncertainty_ellipse_t *uncertainty_ellipse,
    int confidence,
    OpenAPI_list_t *point_list,
    double altitude,
    float uncertainty_altitude,
    int inner_radius,
    float uncertainty_radius,
    int offset_angle,
    int included_angle
)
{
    OpenAPI_geographic_area_t *geographic_area_local_var = ogs_malloc(sizeof(OpenAPI_geographic_area_t));
    ogs_assert(geographic_area_local_var);

    geographic_area_local_var->shape = shape;
    geographic_area_local_var->point = point;
    geographic_area_local_var->uncertainty = uncertainty;
    geographic_area_local_var->uncertainty_ellipse = uncertainty_ellipse;
    geographic_area_local_var->confidence = confidence;
    geographic_area_local_var->point_list = point_list;
    geographic_area_local_var->altitude = altitude;
    geographic_area_local_var->uncertainty_altitude = uncertainty_altitude;
    geographic_area_local_var->inner_radius = inner_radius;
    geographic_area_local_var->uncertainty_radius = uncertainty_radius;
    geographic_area_local_var->offset_angle = offset_angle;
    geographic_area_local_var->included_angle = included_angle;

    return geographic_area_local_var;
}

void OpenAPI_geographic_area_free(OpenAPI_geographic_area_t *geographic_area)
{
    if (NULL == geographic_area) {
        return;
    }
    OpenAPI_lnode_t *node;
    OpenAPI_supported_gad_shapes_free(geographic_area->shape);
    OpenAPI_geographical_coordinates_free(geographic_area->point);
    OpenAPI_uncertainty_ellipse_free(geographic_area->uncertainty_ellipse);
    OpenAPI_list_for_each(geographic_area->point_list, node) {
        OpenAPI_geographical_coordinates_free(node->data);
    }
    OpenAPI_list_free(geographic_area->point_list);
    ogs_free(geographic_area);
}

cJSON *OpenAPI_geographic_area_convertToJSON(OpenAPI_geographic_area_t *geographic_area)
{
    cJSON *item = NULL;

    if (geographic_area == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [GeographicArea]");
        return NULL;
    }

    item = cJSON_CreateObject();
    cJSON *shape_local_JSON = OpenAPI_supported_gad_shapes_convertToJSON(geographic_area->shape);
    if (shape_local_JSON == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [shape]");
        goto end;
    }
    cJSON_AddItemToObject(item, "shape", shape_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [shape]");
        goto end;
    }

    cJSON *point_local_JSON = OpenAPI_geographical_coordinates_convertToJSON(geographic_area->point);
    if (point_local_JSON == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [point]");
        goto end;
    }
    cJSON_AddItemToObject(item, "point", point_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [point]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "uncertainty", geographic_area->uncertainty) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [uncertainty]");
        goto end;
    }

    cJSON *uncertainty_ellipse_local_JSON = OpenAPI_uncertainty_ellipse_convertToJSON(geographic_area->uncertainty_ellipse);
    if (uncertainty_ellipse_local_JSON == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [uncertainty_ellipse]");
        goto end;
    }
    cJSON_AddItemToObject(item, "uncertaintyEllipse", uncertainty_ellipse_local_JSON);
    if (item->child == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [uncertainty_ellipse]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "confidence", geographic_area->confidence) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [confidence]");
        goto end;
    }

    cJSON *point_listList = cJSON_AddArrayToObject(item, "pointList");
    if (point_listList == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [point_list]");
        goto end;
    }

    OpenAPI_lnode_t *point_list_node;
    if (geographic_area->point_list) {
        OpenAPI_list_for_each(geographic_area->point_list, point_list_node) {
            cJSON *itemLocal = OpenAPI_geographical_coordinates_convertToJSON(point_list_node->data);
            if (itemLocal == NULL) {
                ogs_error("OpenAPI_geographic_area_convertToJSON() failed [point_list]");
                goto end;
            }
            cJSON_AddItemToArray(point_listList, itemLocal);
        }
    }

    if (cJSON_AddNumberToObject(item, "altitude", geographic_area->altitude) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [altitude]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "uncertaintyAltitude", geographic_area->uncertainty_altitude) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [uncertainty_altitude]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "innerRadius", geographic_area->inner_radius) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [inner_radius]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "uncertaintyRadius", geographic_area->uncertainty_radius) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [uncertainty_radius]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "offsetAngle", geographic_area->offset_angle) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [offset_angle]");
        goto end;
    }

    if (cJSON_AddNumberToObject(item, "includedAngle", geographic_area->included_angle) == NULL) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed [included_angle]");
        goto end;
    }

end:
    return item;
}

OpenAPI_geographic_area_t *OpenAPI_geographic_area_parseFromJSON(cJSON *geographic_areaJSON)
{
    OpenAPI_geographic_area_t *geographic_area_local_var = NULL;
    cJSON *shape = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "shape");
    if (!shape) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [shape]");
        goto end;
    }

    OpenAPI_supported_gad_shapes_t *shape_local_nonprim = NULL;
    shape_local_nonprim = OpenAPI_supported_gad_shapes_parseFromJSON(shape);

    cJSON *point = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "point");
    if (!point) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [point]");
        goto end;
    }

    OpenAPI_geographical_coordinates_t *point_local_nonprim = NULL;
    point_local_nonprim = OpenAPI_geographical_coordinates_parseFromJSON(point);

    cJSON *uncertainty = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "uncertainty");
    if (!uncertainty) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty]");
        goto end;
    }

    if (!cJSON_IsNumber(uncertainty)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty]");
        goto end;
    }

    cJSON *uncertainty_ellipse = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "uncertaintyEllipse");
    if (!uncertainty_ellipse) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty_ellipse]");
        goto end;
    }

    OpenAPI_uncertainty_ellipse_t *uncertainty_ellipse_local_nonprim = NULL;
    uncertainty_ellipse_local_nonprim = OpenAPI_uncertainty_ellipse_parseFromJSON(uncertainty_ellipse);

    cJSON *confidence = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "confidence");
    if (!confidence) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [confidence]");
        goto end;
    }

    if (!cJSON_IsNumber(confidence)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [confidence]");
        goto end;
    }

    cJSON *point_list = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "pointList");
    if (!point_list) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [point_list]");
        goto end;
    }

    OpenAPI_list_t *point_listList;
    cJSON *point_list_local_nonprimitive;
    if (!cJSON_IsArray(point_list)){
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [point_list]");
        goto end;
    }

    point_listList = OpenAPI_list_create();

    cJSON_ArrayForEach(point_list_local_nonprimitive, point_list ) {
        if (!cJSON_IsObject(point_list_local_nonprimitive)) {
            ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [point_list]");
            goto end;
        }
        OpenAPI_geographical_coordinates_t *point_listItem = OpenAPI_geographical_coordinates_parseFromJSON(point_list_local_nonprimitive);

        if (!point_listItem) {
            ogs_error("No point_listItem");
            OpenAPI_list_free(point_listList);
            goto end;
        }

        OpenAPI_list_add(point_listList, point_listItem);
    }

    cJSON *altitude = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "altitude");
    if (!altitude) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [altitude]");
        goto end;
    }

    if (!cJSON_IsNumber(altitude)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [altitude]");
        goto end;
    }

    cJSON *uncertainty_altitude = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "uncertaintyAltitude");
    if (!uncertainty_altitude) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty_altitude]");
        goto end;
    }

    if (!cJSON_IsNumber(uncertainty_altitude)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty_altitude]");
        goto end;
    }

    cJSON *inner_radius = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "innerRadius");
    if (!inner_radius) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [inner_radius]");
        goto end;
    }

    if (!cJSON_IsNumber(inner_radius)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [inner_radius]");
        goto end;
    }

    cJSON *uncertainty_radius = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "uncertaintyRadius");
    if (!uncertainty_radius) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty_radius]");
        goto end;
    }

    if (!cJSON_IsNumber(uncertainty_radius)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [uncertainty_radius]");
        goto end;
    }

    cJSON *offset_angle = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "offsetAngle");
    if (!offset_angle) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [offset_angle]");
        goto end;
    }

    if (!cJSON_IsNumber(offset_angle)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [offset_angle]");
        goto end;
    }

    cJSON *included_angle = cJSON_GetObjectItemCaseSensitive(geographic_areaJSON, "includedAngle");
    if (!included_angle) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [included_angle]");
        goto end;
    }

    if (!cJSON_IsNumber(included_angle)) {
        ogs_error("OpenAPI_geographic_area_parseFromJSON() failed [included_angle]");
        goto end;
    }

    geographic_area_local_var = OpenAPI_geographic_area_create (
        shape_local_nonprim,
        point_local_nonprim,
        
        uncertainty->valuedouble,
        uncertainty_ellipse_local_nonprim,
        
        confidence->valuedouble,
        point_listList,
        
        altitude->valuedouble,
        
        uncertainty_altitude->valuedouble,
        
        inner_radius->valuedouble,
        
        uncertainty_radius->valuedouble,
        
        offset_angle->valuedouble,
        
        included_angle->valuedouble
    );

    return geographic_area_local_var;
end:
    return NULL;
}

OpenAPI_geographic_area_t *OpenAPI_geographic_area_copy(OpenAPI_geographic_area_t *dst, OpenAPI_geographic_area_t *src)
{
    cJSON *item = NULL;
    char *content = NULL;

    ogs_assert(src);
    item = OpenAPI_geographic_area_convertToJSON(src);
    if (!item) {
        ogs_error("OpenAPI_geographic_area_convertToJSON() failed");
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

    OpenAPI_geographic_area_free(dst);
    dst = OpenAPI_geographic_area_parseFromJSON(item);
    cJSON_Delete(item);

    return dst;
}

