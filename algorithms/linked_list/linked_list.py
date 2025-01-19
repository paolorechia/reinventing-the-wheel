from typing import Any

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


    def append(self, value):
        current = self.head
        while current.next is not None:
            current = current.next

        current.next = Node(value)
        self.size += 1


    def pop(self):
        if self.empty:
            raise EmptyListException

        old_head = self.head.next
        self.head.next = self.head.next.next

        self.size -= 1
        return old_head

