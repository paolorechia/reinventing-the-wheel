from typing import Any, Iterator


class EmptyListException(Exception):
    pass


class Node:
    def __init__(self, value: Any):
        self.next: None | Node = None
        self.value: Any = value


class LinkedList:
    def __init__(self):
        self.head = Node(None)
        self.size = 0

    @property
    def empty(self) -> bool:
        return self.size == 0

    def __len__(self) -> int:
        return self.size

    def _check_bounds(self, index):
        if index < 0:
            raise IndexError(f"Index not found: {index}")

        if index >= self.size:
            raise IndexError(f"Index not found: {index}")

    def __iter__(self) -> Iterator[Any]:
        current = self.head
        while current.next is not None:
            current = current.next
            yield current.value

    def __getitem__(self, key) -> Any:
        return self.get(key).value

    def append(self, value):
        current = self.head
        while current.next is not None:
            current = current.next

        current.next = Node(value)
        self.size += 1

    def get(self, index):
        self._check_bounds(index)

        current = self.head
        current_index = -1

        while current.next is not None:
            current = current.next
            current_index += 1
            if current_index == index:
                return current

    def delete(self, index) -> Node:
        self._check_bounds(index)

        current = self.head
        current_index = -1

        to_delete = None
        while current.next is not None:
            previous = current
            current = current.next
            current_index += 1

            if current_index == index:
                to_delete = current
                break

        previous.next = to_delete.next
        self.size -= 1
        return to_delete

    def pop_head(self) -> Node:
        if self.empty:
            raise EmptyListException

        old_head = self.head.next
        self.head.next = self.head.next.next

        self.size -= 1
        return old_head
