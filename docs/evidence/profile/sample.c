/* Test-only Linux x86-64 flat CPU sampler. Never linked into the decoder.
 * SIGPROF samples the interrupted instruction, without allocating or doing
 * I/O in the handler. Intended for these single-threaded benchmark programs. */
#define _GNU_SOURCE
#include <link.h>
#include <signal.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <ucontext.h>
#include <unistd.h>

#define CAPACITY 1000000
static uintptr_t pcs[CAPACITY];
static volatile sig_atomic_t used, dropped;
static const char *destination;
static void sample(int signal, siginfo_t *info, void *context) {
    (void)signal; (void)info;
    ucontext_t *uc = context;
    sig_atomic_t i = used;
    if (i < CAPACITY) {
        pcs[i] = (uintptr_t)uc->uc_mcontext.gregs[REG_RIP];
        used = i + 1;
    } else if (dropped < 2000000000) ++dropped;
}
static int module(struct dl_phdr_info *info, size_t size, void *out) {
    (void)size;
    char executable[4096];
    const char *name = info->dlpi_name;
    if (!name || !*name) {
        ssize_t n = readlink("/proc/self/exe", executable, sizeof executable - 1);
        if (n < 0) return 0;
        executable[n] = 0; name = executable;
    }
    for (int i = 0; i < info->dlpi_phnum; ++i) {
        const ElfW(Phdr) *p = &info->dlpi_phdr[i];
        if (p->p_type == PT_LOAD && (p->p_flags & PF_X))
            fprintf(out, "module %lx %lx %lx %s\n",
                    (unsigned long)info->dlpi_addr,
                    (unsigned long)(info->dlpi_addr + p->p_vaddr),
                    (unsigned long)(info->dlpi_addr + p->p_vaddr + p->p_memsz), name);
    }
    return 0;
}
__attribute__((constructor)) static void start(void) {
    destination = getenv("TIFF_PROFILE_OUTPUT");
    if (!destination) return;
    struct sigaction action = {0};
    action.sa_sigaction = sample;
    action.sa_flags = SA_SIGINFO | SA_RESTART;
    sigemptyset(&action.sa_mask);
    struct itimerval timer = {{0, 1000}, {0, 1000}};
    if (sigaction(SIGPROF, &action, NULL) || setitimer(ITIMER_PROF, &timer, NULL)) {
        perror("CPU profiler"); _exit(125);
    }
}
__attribute__((destructor)) static void finish(void) {
    if (!destination) return;
    struct itimerval timer = {{0, 0}, {0, 0}};
    setitimer(ITIMER_PROF, &timer, NULL);
    sigset_t blocked;
    sigemptyset(&blocked); sigaddset(&blocked, SIGPROF);
    sigprocmask(SIG_BLOCK, &blocked, NULL);
    FILE *out = fopen(destination, "w");
    if (!out) { perror("CPU profile output"); _exit(125); }
    fprintf(out, "samples %d dropped %d interval_us 1000\n", used, dropped);
    dl_iterate_phdr(module, out);
    for (sig_atomic_t i = 0; i < used; ++i)
        fprintf(out, "%lx\n", (unsigned long)pcs[i]);
    if (fclose(out)) { perror("CPU profile close"); _exit(125); }
}
