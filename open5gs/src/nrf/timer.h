/*
 * Copyright (C) 2019 by Sukchan Lee <acetcom@gmail.com>
 *
 * This file is part of Open5GS.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef NRF_TIMER_H
#define NRF_TIMER_H

#include "ogs-core.h"

#ifdef __cplusplus
extern "C" {
#endif

/* forward declaration */
typedef enum {
    NRF_TIMER_BASE = 0,

    NRF_TIMER_NF_INSTANCE_NO_HEARTBEAT,
    NRF_TIMER_SUBSCRIPTION_VALIDITY,

    MAX_NUM_OF_NRF_TIMER,

} nrf_timer_e;

typedef struct nrf_timer_cfg_s {
    int max_count;
    ogs_time_t duration;
} nrf_timer_cfg_t;

nrf_timer_cfg_t *nrf_timer_cfg(nrf_timer_e id);

const char *nrf_timer_get_name(nrf_timer_e id);

void nrf_timer_nf_instance_no_heartbeat(void *data);
void nrf_timer_subscription_validity(void *data);

#ifdef __cplusplus
}
#endif

#endif /* NRF_TIMER_H */
