std
---

## std::collections

Sequences

- `Vec`
- `VecDeque`
- `LinkedList`

Maps

- `HashMap`
- `BTreeMap`

Set

- `HashSet`
- `BTreeSet`

Misc

- `BinaryHeap`

| | get(i) | insert(i) | remove(i)  | append | split_off(i) |
| --- | --- | --- | --- | --- | --- |
| Vec | O(1) | O(n-i)* | O(n-i) | O(m)* | O(n-i) |
| VecDeque | O(1) | O(min(i, n-i))* | O(min(i, n-i)) | O(m)* | O(min(i, n-i)) |
| LinkedList | O(min(i, n-i)) | O(min(i, n-i)) | O(min(i, n-i)) | O(1) | O(min(i, n-i)) |
| HashMap | O(1)~ | O(1)~* | O(1)~ | - | - |
| BTreeMap | O(log(n)) | O(log(n)) | O(log(n)) | O(log(n)) | O(n+m) |
