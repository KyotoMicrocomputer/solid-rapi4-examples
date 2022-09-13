#include <array>
#include <algorithm>
#include <limits>
#include <kernel.h>
#include <solid_log.h>
#include <solid_sockets.h>
#include <cassert>

namespace {
    
constexpr size_t NUM_WORKERS = std::min<size_t>(8, TBIT_FLGPTN);

using Buffer = std::array<char, 4096>;

/** Used by the acceptor to pass an incoming FD to a worker task */
std::array<SOLID_NET_FD, NUM_WORKERS> g_incoming_fds;

/** The message buffers used by the worker tasks */
std::array<Buffer, NUM_WORKERS> g_buffers;

/** An eventflag object indicating which workers are ready to accept new jobs */
ID g_ready_worker_flags;

inline ER
svc_perror(const char *expr, ER ercd)
{
    if (ercd < 0) {
        SOLID_LOG_printf("error: %s: failed with error code %d\n", expr, ercd);
        abort();
    }

    return ercd;
}

#define SVC_PERROR(expr) svc_perror(#expr, (expr))

/** Log the result of `SOLID_NET_GetLastError()` */
void report_last_net_error(const char *op_name)
{
    const char *msg = SOLID_NET_StrError(SOLID_NET_GetLastError());
    SOLID_LOG_printf("error: %s: %s\n", op_name, msg);
}

/** Serve one client. Doesn't take the ownership of the FD. */
void serve_client(SOLID_NET_FD client_fd, Buffer& buffer)
{
    static constexpr SOLID_NET_TIMEVAL timeout = {
        .tv_sec = 30,
        .tv_usec = 0,
    };

    if (SOLID_NET_SetSockOpt(client_fd,
        SOLID_NET_SOCKOPTLEVEL_SOCKET,
        SOLID_NET_SOCKOPT_SO_SNDTIMEO,
        &timeout,
        sizeof timeout))
    {
        report_last_net_error("serve_client: SOLID_NET_SetSockOpt(SO_SNDTIMEO)");
        return;
    }
    
    if (SOLID_NET_SetSockOpt(client_fd,
        SOLID_NET_SOCKOPTLEVEL_SOCKET,
        SOLID_NET_SOCKOPT_SO_RCVTIMEO,
        &timeout,
        sizeof timeout))
    {
        report_last_net_error("serve_client: SOLID_NET_SetSockOpt(SO_RCVTIMEO)");
        return;
    }

    while (true) {
        // Read data from the socket
        const ssize_t num_read_bytes = SOLID_NET_Read(client_fd, buffer.data(), buffer.size());
        if (num_read_bytes <= 0) {
            if (num_read_bytes < 0) {
                report_last_net_error("serve_client: SOLID_NET_Read");
            } else {
                if (SOLID_NET_Shutdown(client_fd, SOLID_NET_SHUTDOWN_BOTH)) {
                    report_last_net_error("serve_client: SOLID_NET_Shutdown");
                }
            }
            return;
        }

        // Write back the data to the socket
        size_t cursor = 0;
        while (cursor < num_read_bytes) {
            const ssize_t num_written_bytes = SOLID_NET_Write(
                client_fd, buffer.data() + cursor, num_read_bytes - cursor);
            if (num_written_bytes <= 0) {
                if (num_written_bytes < 0) {
                    report_last_net_error("serve_client: SOLID_NET_Write");
                }
                return;
            }
            cursor += num_written_bytes;
        }
    }
}

/** The entry point for worker tasks */
void worker_task(intptr_t exinf)
{
    const size_t worker_i = exinf;

    // We own this `client_fd` now
    const SOLID_NET_FD client_fd = g_incoming_fds[worker_i];

    // Serve the client
    SOLID_LOG_printf(
        "info: worker_task: worker %zd is serving client FD %d\n", worker_i, client_fd);
    serve_client(client_fd, g_buffers[worker_i]);
    SOLID_LOG_printf(
        "info: worker_task: worker %zd finished serving client FD %d\n", worker_i, client_fd);

    // Since we own `client_fd` now, it's up to us to close it
    if (SOLID_NET_Close(client_fd)) {
        report_last_net_error("SOLID_NET_Close");
    }

    // Tell the acceptor that we're ready to accept a new client
    SVC_PERROR(set_flg(g_ready_worker_flags, FLGPTN(1) << worker_i));
}

} // namespace

/** The root task entry point */
extern "C" void slo_main()
{
    // The bitmask to specify all workers
    const FLGPTN all_workers_mask = NUM_WORKERS == std::numeric_limits<FLGPTN>::digits
        ? ~FLGPTN(0) : (FLGPTN(1) << NUM_WORKERS) - 1;

    // Create an eventflag object to track free workers
    static constexpr T_CFLG ready_worker_flags_opts = {
        .flgatr = 0,
        .iflgptn = all_workers_mask,
    };
    g_ready_worker_flags = SVC_PERROR(acre_flg(&ready_worker_flags_opts));

    // Create worker tasks
    std::array<ID, NUM_WORKERS> worker_tasks;
    for (size_t i = 0; i < NUM_WORKERS; ++i) {
        ID proc = ID(i) % SOLID_CORE_MAX;
        const T_CTSK worker_task_opts = {
            .tskatr = 0,
            .exinf = intptr_t(i),
            .task = worker_task,
            .itskpri = 10,
            .stksz = 4096,
            .stk = NULL,
            .iprcid = proc + 1,
            .affinity = uint_t(1) << proc,
        };
        worker_tasks[i] = SVC_PERROR(acre_tsk(&worker_task_opts));
    }

    // Create an accepting socket
    SOLID_NET_FD acceptor_fd = SOLID_NET_Socket(
        SOLID_NET_SA_FAMILY_INET,
        SOLID_NET_SOCKET_TYPE_STREAM,
        SOLID_NET_IPPROTO_TCP);
    if (acceptor_fd == SOLID_NET_INVALID_SOCKET) {
        report_last_net_error("SOLID_NET_Socket");
        abort();
    }

    // Enable local address reuse
    const int one = 1;
    if (SOLID_NET_SetSockOpt(acceptor_fd,
        SOLID_NET_SOCKOPTLEVEL_SOCKET,
        SOLID_NET_SOCKOPT_SO_REUSEADDR,
        &one,
        sizeof one))
    {
        report_last_net_error("SOLID_NET_SetSockOpt");
        abort();
    }

    // Bind the accepting socket
    const SOLID_NET_SOCKADDR_IN bind_addr = {
        .sin_len = sizeof(SOLID_NET_SOCKADDR_IN),
        .sin_family = SOLID_NET_SA_FAMILY_INET,
        .sin_port = htons(7777),
        .sin_addr = {0}, // 0.0.0.0
    };

    if (SOLID_NET_Bind(
        acceptor_fd, 
        reinterpret_cast<const SOLID_NET_SOCKADDR *>(&bind_addr), 
        bind_addr.sin_len))
    {
        report_last_net_error("SOLID_NET_Bind");
        abort();
    }

    SOLID_LOG_printf("info: Starting TCP echo server on 0.0.0.0:%d\n", int(ntohs(bind_addr.sin_port)));

    if (SOLID_NET_Listen(acceptor_fd, 16)) {
        report_last_net_error("SOLID_NET_Listen");
        abort();
    }

    // Accept clients
    while (true) {
        // Find a free worker
        FLGPTN flgptn;
        SVC_PERROR(wai_flg(g_ready_worker_flags, all_workers_mask, TWF_ORW, &flgptn));
        assert(flgptn != 0);

        size_t worker_i = __builtin_ctz(flgptn);

        // Accept a client
        SOLID_NET_FD client_fd = SOLID_NET_Accept(acceptor_fd, NULL, NULL);
        if (client_fd == SOLID_NET_INVALID_SOCKET) {
            report_last_net_error("SOLID_NET_Accept");

            // An accept failure is mostly non-fatal, so continue
            continue;
        }

        // Activate the worker, passing the ownership of `client_fd`
        //
        // Note: The worker task might be still active when we observe that the corresponding bit
        // of `g_ready_worker_flags` is set. That's okay because `act_tsk` can queue activation
        // requests.
        g_incoming_fds[worker_i] = client_fd;
        SVC_PERROR(clr_flg(g_ready_worker_flags, ~(FLGPTN(1) << worker_i)));
        SVC_PERROR(act_tsk(worker_tasks[worker_i]));
    }
}
