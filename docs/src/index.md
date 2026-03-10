# Dropset

<!-- markdownlint-disable MD013 -->

> [!important] Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)

<!-- markdownlint-enable MD013 -->

<script setup>
const quicksort = `\\begin{algorithm}
\\caption{Quicksort}
\\begin{algorithmic}
\\PROCEDURE{Quicksort}{$A, p, r$}
    \\IF{$p < r$}
        \\STATE $q \\gets$ \\CALL{Partition}{$A, p, r$}
        \\STATE \\CALL{Quicksort}{$A, p, q - 1$}
        \\STATE \\CALL{Quicksort}{$A, q + 1, r$}
    \\ENDIF
\\ENDPROCEDURE
\\end{algorithmic}
\\end{algorithm}`;
</script>

<Pseudocode :code="quicksort" />
