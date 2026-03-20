.globl entrypoint

# Included in order of dependency.
.include "common/discriminant.s"
.include "common/error.s"
.include "common/memory.s"
.include "entrypoint.s"
.include "market/register.s"
