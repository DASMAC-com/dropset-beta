# Sectors

Market account data begins with a [`MarketHeader`]
followed by a contiguous array of fixed-size sectors. Each
sector holds one of three node types ([`Order`],
[`Seat`], or [`StackNode`]), but a node does not
necessarily occupy the entire sector.

Because the market account is at a [fixed position] in the
input buffer, its data offsets are persisted across
transactions. `MarketHeader` stores absolute SBPF pointers
into the sector array (`seats`, `asks`, `bids`, `top`,
`next`) that remain valid without recomputation.

```txt
+----------------+----------+----------+----------+-----+
| MarketHeader   | Sector 0 | Sector 1 | Sector 2 | ... |
+----------------+----------+----------+----------+-----+
```

A `Sector` is a byte buffer sized to the largest node type:

<Include rs="interface::common::memory#sector"/>

The first byte of every sector is a `NodeTag` discriminant
that identifies its contents:

<Include rs="interface::common::memory#node_tag"/>

## Order

A sector holding an [`Order`] is an active node in
one of the market's order trees.

```txt
+-----+--------------------------------------------------+
| tag |                   (unused)                       |
+-----+--------------------------------------------------+
```

<Include rs="interface::order#order" collapsed/>

## Seat

A sector holding a [`Seat`] is an active node in
the market's seat tree.

```txt
+-----+--------+------+-------+------+-----+------+------+
| tag | parent | left | right | user | ... | asks | bids |
+-----+--------+------+-------+------+-----+------+------+
```

<Include rs="interface::seat#seat" collapsed/>

## StackNode

A sector holding a `StackNode` is a freed sector on the
free sector stack.

```txt
+-----+------+-------------------------------------------+
| tag | next |                (unused)                   |
+-----+------+-------------------------------------------+
```

<Include rs="interface::stack#stack_node" collapsed/>

## Allocation

`MarketHeader.next` points to the next unallocated sector
in the memory map. `MarketHeader.top` points to the top of
the free sector stack. When a sector is freed, it becomes a
`StackNode` pushed onto the stack. New allocations pop from
the stack first; when the stack is empty, `next` advances.

[`MarketHeader`]: markets
[`Order`]: orders
[`Seat`]: seats
[`StackNode`]: #stacknode
[fixed position]: inputs#input-buffer
