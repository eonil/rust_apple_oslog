#include "RE_stub.h"
#include <os/log.h>

void RE_stub_oslog_default_public(char const* msg) {
    os_log_with_type(OS_LOG_DEFAULT, OS_LOG_TYPE_DEFAULT, "%{public}s", msg);
}
