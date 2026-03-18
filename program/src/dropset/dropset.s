.globl entrypoint

# Included in order of dependency.
.include "discriminant.s"
.include "error.s"
.include "entrypoint.s"
.include "market/register.s"
