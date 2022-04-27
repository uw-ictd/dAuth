/*
 * Copyright (C) 2019,2020 by Sukchan Lee <acetcom@gmail.com>
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

#include <stdint.h>
#include <sys/eventfd.h>
#include <unistd.h>

#include "event.h"
#include "sbi-path.h"
#include "dauth-c-binding.h"

static int grpc_notify_fd;
static ogs_thread_t *grpc_thread;
static ogs_thread_t *event_thread;
static void ausf_grpc_main(void *data);
static void ausf_event_main(void *data);
static int initialized = 0;


/*
Callback triggered when the grpc_notify_fd is triggered. Must occur before
events are consumed from the queue to catch all signals from the grpc callback
queue thread.
*/
static void notify_cb (short when, ogs_socket_t fd, void *data) {
    int64_t grpc_event_counter;
    ogs_debug("GRPC notify callback");
    /* Reset the grpc event signal if needed. Do not check how many bytes
    read, since the value does not actually matter */
    ogs_assert(grpc_notify_fd);
    read(grpc_notify_fd, &grpc_event_counter, sizeof(grpc_event_counter));
}

int ausf_initialize()
{
    int rv;

    grpc_notify_fd = eventfd(0, EFD_NONBLOCK);
    ogs_assert(grpc_notify_fd);

    ausf_context_init();
    ausf_event_init();
    ogs_sbi_context_init();

    rv = ogs_sbi_context_parse_config("ausf", "nrf");
    if (rv != OGS_OK) return rv;

    rv = ausf_context_parse_config();
    if (rv != OGS_OK) return rv;

    rv = ogs_log_config_domain(
            ogs_app()->logger.domain, ogs_app()->logger.level);
    if (rv != OGS_OK) return rv;

    rv = ausf_sbi_open();
    if (rv != OGS_OK) return rv;

    /* Ignoring return since the eventfd poll will never be cancelled or checked. */
    ogs_pollset_add(ogs_app()->pollset, OGS_POLLIN, grpc_notify_fd, notify_cb, NULL);

    event_thread = ogs_thread_create(ausf_event_main, NULL);
    if (!event_thread) return OGS_ERROR;
    grpc_thread = ogs_thread_create(ausf_grpc_main, NULL);
    if (!grpc_thread) return OGS_ERROR;

    initialized = 1;

    return OGS_OK;
}

static ogs_timer_t *t_termination_holding = NULL;

static void event_termination(void)
{
    ogs_sbi_nf_instance_t *nf_instance = NULL;

    /* Sending NF Instance De-registeration to NRF */
    ogs_list_for_each(&ogs_sbi_self()->nf_instance_list, nf_instance)
        ausf_nf_fsm_fini(nf_instance);

    /* Starting holding timer */
    t_termination_holding = ogs_timer_add(ogs_app()->timer_mgr, NULL, NULL);
    ogs_assert(t_termination_holding);
#define TERMINATION_HOLDING_TIME ogs_time_from_msec(300)
    ogs_timer_start(t_termination_holding, TERMINATION_HOLDING_TIME);

    /* Sending termination event to the queue */
    ogs_queue_term(ogs_app()->queue);
    ogs_pollset_notify(ogs_app()->pollset);
}

void ausf_terminate(void)
{
    if (!initialized) return;

    /* Daemon terminating */
    event_termination();
    grpc_client_shutdown();

    ogs_thread_destroy(grpc_thread);
    ogs_thread_destroy(event_thread);

    ogs_timer_delete(t_termination_holding);

    ausf_sbi_close();

    ausf_context_final();
    ogs_sbi_context_final();

    ausf_event_final(); /* Destroy event */

    if (grpc_notify_fd) {
        close(grpc_notify_fd);
        grpc_notify_fd = 0;
    }
}


static void ausf_event_main(void *data)
{
    ogs_fsm_t ausf_sm;
    int rv;

    ogs_fsm_create(&ausf_sm, ausf_state_initial, ausf_state_final);
    ogs_fsm_init(&ausf_sm, 0);

    for ( ;; ) {
        ogs_pollset_poll(ogs_app()->pollset,
                ogs_timer_mgr_next(ogs_app()->timer_mgr));

        /*
         * After ogs_pollset_poll(), ogs_timer_mgr_expire() must be called.
         *
         * The reason is why ogs_timer_mgr_next() can get the corrent value
         * when ogs_timer_stop() is called internally in ogs_timer_mgr_expire().
         *
         * You should not use event-queue before ogs_timer_mgr_expire().
         * In this case, ogs_timer_mgr_expire() does not work
         * because 'if rv == OGS_DONE' statement is exiting and
         * not calling ogs_timer_mgr_expire().
         */
        ogs_timer_mgr_expire(ogs_app()->timer_mgr);

        for ( ;; ) {
            ausf_event_t *e = NULL;

            rv = ogs_queue_trypop(ogs_app()->queue, (void**)&e);
            ogs_assert(rv != OGS_ERROR);

            if (rv == OGS_DONE)
                goto done;

            if (rv == OGS_RETRY)
                break;

            ogs_assert(e);
            ogs_fsm_dispatch(&ausf_sm, e);
            ausf_event_free(e);
        }
    }
done:

    ogs_fsm_fini(&ausf_sm, 0);
    ogs_fsm_delete(&ausf_sm);
}

static void ausf_grpc_main(void *data)
{
    uint64_t event_ctr=1;

    for ( ;; ) {
        void* rpc_tag = NULL;
        bool ok = wait_for_next_rpc_event(&rpc_tag);
        if (!ok) {
            ogs_error("wait_for_next_rpc_event not ok, shutting down");
            break;
        }
        ogs_assert(rpc_tag);

        ausf_event_t *e = NULL;
        int rv;

        e = ausf_event_new(AUSF_EVT_RPC_COMPLETION);
        ogs_assert(e);

        e->rpc_tag = rpc_tag;
        rv = ogs_queue_push(ogs_app()->queue, e);

        /* Write to the eventfd only after the event has been successfully
        pushed to the queue to wake the consumer thread if necessary. */
        ogs_assert(grpc_notify_fd);
        ogs_assert(write(grpc_notify_fd, &event_ctr, sizeof(event_ctr)) == 8);

        if (rv != OGS_OK) {
            ogs_warn("ogs_queue_push() failed:%d", (int)rv);
            ausf_event_free(e);
        }
    }
}
